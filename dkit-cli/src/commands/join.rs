use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use dkit_core::format::csv::{CsvReader, CsvWriter};
use dkit_core::format::env::{EnvReader, EnvWriter};
use dkit_core::format::hcl::{HclReader, HclWriter};
use dkit_core::format::html::HtmlWriter;
use dkit_core::format::ini::{IniReader, IniWriter};
use dkit_core::format::json::{JsonReader, JsonWriter};
use dkit_core::format::jsonl::{JsonlReader, JsonlWriter};
use dkit_core::format::markdown::MarkdownWriter;
use dkit_core::format::msgpack::{MsgpackReader, MsgpackWriter};
use dkit_core::format::plist::{PlistReader, PlistWriter};
use dkit_core::format::properties::{PropertiesReader, PropertiesWriter};
use dkit_core::format::toml::{TomlReader, TomlWriter};
use dkit_core::format::xml::{XmlReader, XmlWriter};
use dkit_core::format::yaml::{YamlReader, YamlWriter};
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, Format, FormatOptions,
    FormatReader, FormatWriter,
};
use dkit_core::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl JoinType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "inner" => Ok(JoinType::Inner),
            "left" => Ok(JoinType::Left),
            "right" => Ok(JoinType::Right),
            "full" | "outer" | "full-outer" => Ok(JoinType::Full),
            _ => bail!(
                "Unknown join type: '{s}'\n  Hint: supported types are inner, left, right, full"
            ),
        }
    }
}

pub struct JoinArgs<'a> {
    pub left: &'a Path,
    pub right: &'a Path,
    pub on: &'a str,
    pub join_type: JoinType,
    pub to: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub pretty: bool,
    pub compact: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

/// Parse the --on key specification.
/// "field" => (field, field) — same key name in both files
/// "left_field=right_field" => (left_field, right_field) — different key names
fn parse_join_key(on: &str) -> Result<(String, String)> {
    if let Some((left, right)) = on.split_once('=') {
        let left = left.trim();
        let right = right.trim();
        if left.is_empty() || right.is_empty() {
            bail!("Invalid --on format: '{on}'\n  Hint: use 'field' or 'left_field=right_field'");
        }
        Ok((left.to_string(), right.to_string()))
    } else {
        let key = on.trim();
        if key.is_empty() {
            bail!("--on cannot be empty");
        }
        Ok((key.to_string(), key.to_string()))
    }
}

/// Extract rows (Vec<Object>) from a Value.
/// If the value is an Array of Objects, return them.
/// If the value is a single Object, wrap it in a vec.
fn extract_rows(value: Value, source: &str) -> Result<Vec<indexmap::IndexMap<String, Value>>> {
    match value {
        Value::Array(arr) => {
            let mut rows = Vec::with_capacity(arr.len());
            for (i, item) in arr.into_iter().enumerate() {
                match item {
                    Value::Object(map) => rows.push(map),
                    _ => bail!("{source}: row {i} is not an object — join requires tabular data (array of objects)"),
                }
            }
            Ok(rows)
        }
        Value::Object(map) => Ok(vec![map]),
        _ => bail!(
            "{source}: expected array of objects or a single object for join, got {}",
            value_type_name(&value)
        ),
    }
}

fn value_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        _ => "unknown",
    }
}

/// Build a lookup index from the right table: key_value -> Vec<row_index>
fn build_index(
    rows: &[indexmap::IndexMap<String, Value>],
    key: &str,
) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, row) in rows.iter().enumerate() {
        let key_val = row.get(key).map(value_to_key_string).unwrap_or_default();
        index.entry(key_val).or_default().push(i);
    }
    index
}

/// Convert a Value to a string for use as a hash key.
fn value_to_key_string(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        _ => format!("{v:?}"),
    }
}

/// Merge a left row and right row, prefixing conflicting keys.
fn merge_row(
    left: &indexmap::IndexMap<String, Value>,
    right: &indexmap::IndexMap<String, Value>,
    left_key: &str,
    right_key: &str,
) -> indexmap::IndexMap<String, Value> {
    let mut merged = indexmap::IndexMap::new();

    // Collect all field names to detect conflicts
    let right_keys: std::collections::HashSet<&str> = right.keys().map(|s| s.as_str()).collect();

    for (k, v) in left {
        if k == left_key && left_key != right_key {
            // Join key from left side — add as-is
            merged.insert(k.clone(), v.clone());
        } else if right_keys.contains(k.as_str()) && k != left_key {
            // Conflict: field exists in both sides and is not the join key
            merged.insert(format!("left_{k}"), v.clone());
        } else {
            merged.insert(k.clone(), v.clone());
        }
    }

    for (k, v) in right {
        if k == right_key && left_key == right_key {
            // Same join key name — already added from left side, skip
            continue;
        } else if k == right_key {
            // Different join key name on right — skip (left key already present)
            continue;
        } else if left.contains_key(k) {
            // Conflict
            merged.insert(format!("right_{k}"), v.clone());
        } else {
            merged.insert(k.clone(), v.clone());
        }
    }

    merged
}

/// Create a null-filled row from a template (using the other side's columns).
fn null_row(template: &[String]) -> indexmap::IndexMap<String, Value> {
    template.iter().map(|k| (k.clone(), Value::Null)).collect()
}

/// Collect unique column names from rows.
fn collect_columns(rows: &[indexmap::IndexMap<String, Value>]) -> Vec<String> {
    let mut cols = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for row in rows {
        for k in row.keys() {
            if seen.insert(k.clone()) {
                cols.push(k.clone());
            }
        }
    }
    cols
}

/// Execute the join operation.
fn execute_join(
    left_rows: Vec<indexmap::IndexMap<String, Value>>,
    right_rows: Vec<indexmap::IndexMap<String, Value>>,
    left_key: &str,
    right_key: &str,
    join_type: JoinType,
) -> Vec<indexmap::IndexMap<String, Value>> {
    let right_index = build_index(&right_rows, right_key);
    let right_cols = collect_columns(&right_rows);
    let left_cols = collect_columns(&left_rows);

    // Filter out the join key from the template columns
    let right_template: Vec<String> = right_cols
        .iter()
        .filter(|c| c.as_str() != right_key)
        .cloned()
        .collect();
    let left_template: Vec<String> = left_cols
        .iter()
        .filter(|c| c.as_str() != left_key)
        .cloned()
        .collect();

    let mut result = Vec::new();
    let mut right_matched = vec![false; right_rows.len()];

    for left_row in &left_rows {
        let key_val = left_row
            .get(left_key)
            .map(value_to_key_string)
            .unwrap_or_default();

        if let Some(indices) = right_index.get(&key_val) {
            for &ri in indices {
                right_matched[ri] = true;
                result.push(merge_row(left_row, &right_rows[ri], left_key, right_key));
            }
        } else {
            // No match on right side
            match join_type {
                JoinType::Left | JoinType::Full => {
                    let null_right = null_row(&right_template);
                    result.push(merge_row(left_row, &null_right, left_key, right_key));
                }
                _ => {} // Inner, Right: skip unmatched left rows
            }
        }
    }

    // For right/full join: add unmatched right rows
    if join_type == JoinType::Right || join_type == JoinType::Full {
        for (ri, row) in right_rows.iter().enumerate() {
            if !right_matched[ri] {
                let null_left = null_row(&left_template);
                // For right-only rows, we need the join key
                let mut merged = indexmap::IndexMap::new();
                // Add left null columns
                for (k, v) in &null_left {
                    merged.insert(k.clone(), v.clone());
                }
                // Add the join key from right side (using left key name for consistency)
                if let Some(key_val) = row.get(right_key) {
                    merged.insert(left_key.to_string(), key_val.clone());
                }
                // Add right columns (excluding right join key)
                for (k, v) in row {
                    if k == right_key {
                        continue;
                    }
                    if null_left.contains_key(k) {
                        merged.insert(format!("right_{k}"), v.clone());
                    } else {
                        merged.insert(k.clone(), v.clone());
                    }
                }
                result.push(merged);
            }
        }
    }

    result
}

pub fn run(args: &JoinArgs) -> Result<()> {
    let (left_key, right_key) = parse_join_key(args.on)?;
    let join_type = args.join_type;

    // Read left and right files
    let left_value = read_input(args.left, args)?;
    let right_value = read_input(args.right, args)?;

    // Extract rows
    let left_rows = extract_rows(left_value, &args.left.display().to_string())?;
    let right_rows = extract_rows(right_value, &args.right.display().to_string())?;

    // Validate that join keys exist in at least some rows
    if !left_rows.is_empty() && !left_rows.iter().any(|r| r.contains_key(&left_key)) {
        bail!(
            "Join key '{}' not found in left file ({})\n  Hint: available fields are: {}",
            left_key,
            args.left.display(),
            left_rows[0]
                .keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if !right_rows.is_empty() && !right_rows.iter().any(|r| r.contains_key(&right_key)) {
        bail!(
            "Join key '{}' not found in right file ({})\n  Hint: available fields are: {}",
            right_key,
            args.right.display(),
            right_rows[0]
                .keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // Execute join
    let joined = execute_join(left_rows, right_rows, &left_key, &right_key, join_type);

    // Convert to Value::Array
    let result = Value::Array(joined.into_iter().map(Value::Object).collect());

    // Determine output format
    let target_format = match args.to {
        Some(f) => Format::from_str(f)?,
        None => match args.output {
            Some(p) => detect_format(p).unwrap_or_else(|_| detect_format(args.left).unwrap()),
            None => detect_format(args.left)?,
        },
    };

    let write_delimiter = args
        .delimiter
        .or_else(|| args.to.and_then(default_delimiter_for_format));
    let write_options = FormatOptions {
        delimiter: write_delimiter,
        no_header: args.no_header,
        pretty: if args.compact {
            false
        } else {
            args.pretty || !args.compact
        },
        compact: args.compact,
        flow_style: false,
        root_element: None,
        styled: false,
        full_html: false,
        indent: None,
        sort_keys: false,
        template: None,
        template_file: None,
    };

    if target_format == Format::Msgpack {
        let bytes = MsgpackWriter.write_bytes(&result)?;
        if let Some(out_path) = args.output {
            if let Some(parent) = out_path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create directory {}", parent.display())
                    })?;
                }
            }
            fs::write(out_path, &bytes)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            use std::io::Write as _;
            std::io::stdout()
                .write_all(&bytes)
                .context("Failed to write to stdout")?;
        }
    } else {
        let output_str = write_value(&result, target_format, &write_options)?;
        if let Some(out_path) = args.output {
            if let Some(parent) = out_path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create directory {}", parent.display())
                    })?;
                }
            }
            fs::write(out_path, &output_str)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            print!("{output_str}");
        }
    }

    Ok(())
}

fn read_input(path: &Path, args: &JoinArgs) -> Result<Value> {
    let format = detect_format(path)?;
    let read_delimiter = args.delimiter.or_else(|| default_delimiter(path));
    let read_options = FormatOptions {
        delimiter: read_delimiter,
        no_header: args.no_header,
        ..Default::default()
    };
    if format == Format::Msgpack {
        let bytes = read_file_bytes(path)?;
        MsgpackReader.read_from_bytes(&bytes)
    } else if format == Format::Xlsx {
        let bytes = read_file_bytes(path)?;
        read_xlsx_from_bytes(&bytes, &args.excel_opts)
    } else if format == Format::Sqlite {
        read_sqlite_from_path(path, &args.sqlite_opts)
    } else if format == Format::Parquet {
        let bytes = read_file_bytes(path)?;
        read_parquet_from_bytes(&bytes)
    } else {
        let content = read_file_with_encoding(path, &args.encoding_opts)?;
        read_value(&content, format, &read_options)
    }
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Jsonl => JsonlReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
        Format::Xml => XmlReader::default().read(content),
        Format::Env => EnvReader.read(content),
        Format::Ini => IniReader.read(content),
        Format::Properties => PropertiesReader.read(content),
        Format::Hcl => HclReader.read(content),
        Format::Plist => PlistReader.read(content),
        Format::Msgpack => MsgpackReader.read(content),
        _ => bail!("Unsupported input format for join: {format}"),
    }
}

fn write_value(value: &Value, format: Format, options: &FormatOptions) -> Result<String> {
    match format {
        Format::Json => JsonWriter::new(options.clone()).write(value),
        Format::Jsonl => JsonlWriter.write(value),
        Format::Csv => CsvWriter::new(options.clone()).write(value),
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty, options.root_element.clone()).write(value),
        Format::Env => EnvWriter.write(value),
        Format::Ini => IniWriter.write(value),
        Format::Properties => PropertiesWriter.write(value),
        Format::Hcl => HclWriter.write(value),
        Format::Plist => PlistWriter.write(value),
        Format::Msgpack => MsgpackWriter.write(value),
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
        _ => bail!("Unsupported output format: {format}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_row(pairs: &[(&str, Value)]) -> indexmap::IndexMap<String, Value> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn test_parse_join_key_same() {
        let (l, r) = parse_join_key("user_id").unwrap();
        assert_eq!(l, "user_id");
        assert_eq!(r, "user_id");
    }

    #[test]
    fn test_parse_join_key_different() {
        let (l, r) = parse_join_key("id=user_id").unwrap();
        assert_eq!(l, "id");
        assert_eq!(r, "user_id");
    }

    #[test]
    fn test_parse_join_key_empty() {
        assert!(parse_join_key("").is_err());
        assert!(parse_join_key("=foo").is_err());
        assert!(parse_join_key("foo=").is_err());
    }

    #[test]
    fn test_inner_join() {
        let left = vec![
            make_row(&[
                ("id", Value::Integer(1)),
                ("name", Value::String("Alice".into())),
            ]),
            make_row(&[
                ("id", Value::Integer(2)),
                ("name", Value::String("Bob".into())),
            ]),
            make_row(&[
                ("id", Value::Integer(3)),
                ("name", Value::String("Charlie".into())),
            ]),
        ];
        let right = vec![
            make_row(&[("id", Value::Integer(1)), ("amount", Value::Integer(100))]),
            make_row(&[("id", Value::Integer(2)), ("amount", Value::Integer(200))]),
            make_row(&[("id", Value::Integer(4)), ("amount", Value::Integer(400))]),
        ];

        let result = execute_join(left, right, "id", "id", JoinType::Inner);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get("name"), Some(&Value::String("Alice".into())));
        assert_eq!(result[0].get("amount"), Some(&Value::Integer(100)));
        assert_eq!(result[1].get("name"), Some(&Value::String("Bob".into())));
        assert_eq!(result[1].get("amount"), Some(&Value::Integer(200)));
    }

    #[test]
    fn test_left_join() {
        let left = vec![
            make_row(&[
                ("id", Value::Integer(1)),
                ("name", Value::String("Alice".into())),
            ]),
            make_row(&[
                ("id", Value::Integer(3)),
                ("name", Value::String("Charlie".into())),
            ]),
        ];
        let right = vec![make_row(&[
            ("id", Value::Integer(1)),
            ("amount", Value::Integer(100)),
        ])];

        let result = execute_join(left, right, "id", "id", JoinType::Left);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get("amount"), Some(&Value::Integer(100)));
        assert_eq!(result[1].get("amount"), Some(&Value::Null));
    }

    #[test]
    fn test_right_join() {
        let left = vec![make_row(&[
            ("id", Value::Integer(1)),
            ("name", Value::String("Alice".into())),
        ])];
        let right = vec![
            make_row(&[("id", Value::Integer(1)), ("amount", Value::Integer(100))]),
            make_row(&[("id", Value::Integer(4)), ("amount", Value::Integer(400))]),
        ];

        let result = execute_join(left, right, "id", "id", JoinType::Right);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get("name"), Some(&Value::String("Alice".into())));
        assert_eq!(result[1].get("name"), Some(&Value::Null));
        assert_eq!(result[1].get("amount"), Some(&Value::Integer(400)));
    }

    #[test]
    fn test_full_join() {
        let left = vec![
            make_row(&[
                ("id", Value::Integer(1)),
                ("name", Value::String("Alice".into())),
            ]),
            make_row(&[
                ("id", Value::Integer(3)),
                ("name", Value::String("Charlie".into())),
            ]),
        ];
        let right = vec![
            make_row(&[("id", Value::Integer(1)), ("amount", Value::Integer(100))]),
            make_row(&[("id", Value::Integer(4)), ("amount", Value::Integer(400))]),
        ];

        let result = execute_join(left, right, "id", "id", JoinType::Full);
        assert_eq!(result.len(), 3);
        // id=1: matched
        assert_eq!(result[0].get("name"), Some(&Value::String("Alice".into())));
        assert_eq!(result[0].get("amount"), Some(&Value::Integer(100)));
        // id=3: left only
        assert_eq!(
            result[1].get("name"),
            Some(&Value::String("Charlie".into()))
        );
        assert_eq!(result[1].get("amount"), Some(&Value::Null));
        // id=4: right only
        assert_eq!(result[2].get("name"), Some(&Value::Null));
        assert_eq!(result[2].get("amount"), Some(&Value::Integer(400)));
    }

    #[test]
    fn test_join_different_keys() {
        let left = vec![make_row(&[
            ("user_id", Value::Integer(1)),
            ("name", Value::String("Alice".into())),
        ])];
        let right = vec![make_row(&[
            ("uid", Value::Integer(1)),
            ("amount", Value::Integer(100)),
        ])];

        let result = execute_join(left, right, "user_id", "uid", JoinType::Inner);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("user_id"), Some(&Value::Integer(1)));
        assert_eq!(result[0].get("name"), Some(&Value::String("Alice".into())));
        assert_eq!(result[0].get("amount"), Some(&Value::Integer(100)));
        // uid should not appear (it's the right join key with a different name)
        assert!(!result[0].contains_key("uid"));
    }

    #[test]
    fn test_join_many_to_many() {
        let left = vec![
            make_row(&[
                ("id", Value::Integer(1)),
                ("name", Value::String("Alice".into())),
            ]),
            make_row(&[
                ("id", Value::Integer(1)),
                ("name", Value::String("Alice2".into())),
            ]),
        ];
        let right = vec![
            make_row(&[
                ("id", Value::Integer(1)),
                ("item", Value::String("A".into())),
            ]),
            make_row(&[
                ("id", Value::Integer(1)),
                ("item", Value::String("B".into())),
            ]),
        ];

        let result = execute_join(left, right, "id", "id", JoinType::Inner);
        // 2 left * 2 right = 4 combinations
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_extract_rows_array() {
        let value = Value::Array(vec![Value::Object(make_row(&[("a", Value::Integer(1))]))]);
        let rows = extract_rows(value, "test").unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_extract_rows_single_object() {
        let value = Value::Object(make_row(&[("a", Value::Integer(1))]));
        let rows = extract_rows(value, "test").unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_extract_rows_non_object() {
        let value = Value::String("not tabular".into());
        assert!(extract_rows(value, "test").is_err());
    }
}
