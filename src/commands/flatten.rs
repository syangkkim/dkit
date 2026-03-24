use std::fs;
use std::io::{self, Read, Write as _};
use std::path::Path;

use anyhow::{bail, Context, Result};
use indexmap::IndexMap;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, ParquetWriteOptions, SqliteOptions,
};
use crate::format::csv::{CsvReader, CsvWriter};
use crate::format::html::HtmlWriter;
use crate::format::json::{JsonReader, JsonWriter};
use crate::format::jsonl::{JsonlReader, JsonlWriter};
use crate::format::markdown::MarkdownWriter;
use crate::format::msgpack::{MsgpackReader, MsgpackWriter};
use crate::format::toml::{TomlReader, TomlWriter};
use crate::format::xml::{XmlReader, XmlWriter};
use crate::format::yaml::{YamlReader, YamlWriter};
use crate::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader, FormatWriter,
};
use crate::value::Value;

/// Array index format for flattening
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayFormat {
    /// `items.0.name`
    Index,
    /// `items[0].name`
    Bracket,
}

impl ArrayFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "index" => Ok(Self::Index),
            "bracket" => Ok(Self::Bracket),
            other => bail!(
                "Unknown array format: '{}'\n  Hint: supported formats are index, bracket",
                other
            ),
        }
    }
}

pub struct FlattenArgs<'a> {
    pub input: &'a str,
    pub separator: &'a str,
    pub array_format: ArrayFormat,
    pub max_depth: Option<usize>,
    pub from: Option<&'a str>,
    pub format: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub pretty: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

pub struct UnflattenArgs<'a> {
    pub input: &'a str,
    pub separator: &'a str,
    pub from: Option<&'a str>,
    pub format: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub pretty: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

pub fn run_flatten(args: &FlattenArgs) -> Result<()> {
    let (value, source_format) = read_input(
        args.input,
        args.from,
        args.delimiter,
        args.no_header,
        &args.encoding_opts,
        &args.excel_opts,
        &args.sqlite_opts,
    )?;

    let result = flatten_value(&value, args.separator, args.array_format, args.max_depth);

    let output_format = match args.format {
        Some(f) => Format::from_str(f)?,
        None => source_format,
    };

    let write_options = FormatOptions {
        delimiter: args.delimiter,
        no_header: args.no_header,
        pretty: args.pretty,
        ..Default::default()
    };

    write_output(&result, output_format, &write_options, args.output)
}

pub fn run_unflatten(args: &UnflattenArgs) -> Result<()> {
    let (value, source_format) = read_input(
        args.input,
        args.from,
        args.delimiter,
        args.no_header,
        &args.encoding_opts,
        &args.excel_opts,
        &args.sqlite_opts,
    )?;

    let result = unflatten_value(&value, args.separator);

    let output_format = match args.format {
        Some(f) => Format::from_str(f)?,
        None => source_format,
    };

    let write_options = FormatOptions {
        delimiter: args.delimiter,
        no_header: args.no_header,
        pretty: args.pretty,
        ..Default::default()
    };

    write_output(&result, output_format, &write_options, args.output)
}

// ── Flatten logic ──

fn flatten_value(
    value: &Value,
    separator: &str,
    array_format: ArrayFormat,
    max_depth: Option<usize>,
) -> Value {
    match value {
        Value::Object(_) => {
            let mut result = IndexMap::new();
            flatten_recursive(
                value,
                "",
                separator,
                array_format,
                max_depth,
                0,
                &mut result,
            );
            Value::Object(result)
        }
        Value::Array(arr) => {
            let flattened: Vec<Value> = arr
                .iter()
                .map(|item| flatten_value(item, separator, array_format, max_depth))
                .collect();
            Value::Array(flattened)
        }
        other => other.clone(),
    }
}

fn flatten_recursive(
    value: &Value,
    prefix: &str,
    separator: &str,
    array_format: ArrayFormat,
    max_depth: Option<usize>,
    current_depth: usize,
    result: &mut IndexMap<String, Value>,
) {
    // If we've reached max depth, store the value as-is
    if let Some(max) = max_depth {
        if current_depth >= max {
            result.insert(prefix.to_string(), value.clone());
            return;
        }
    }

    match value {
        Value::Object(map) => {
            if map.is_empty() {
                if !prefix.is_empty() {
                    result.insert(prefix.to_string(), value.clone());
                }
                return;
            }
            for (key, val) in map {
                let new_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}{separator}{key}")
                };
                flatten_recursive(
                    val,
                    &new_key,
                    separator,
                    array_format,
                    max_depth,
                    current_depth + 1,
                    result,
                );
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                if !prefix.is_empty() {
                    result.insert(prefix.to_string(), value.clone());
                }
                return;
            }
            for (i, val) in arr.iter().enumerate() {
                let new_key = match array_format {
                    ArrayFormat::Index => {
                        if prefix.is_empty() {
                            format!("{i}")
                        } else {
                            format!("{prefix}{separator}{i}")
                        }
                    }
                    ArrayFormat::Bracket => {
                        if prefix.is_empty() {
                            format!("[{i}]")
                        } else {
                            format!("{prefix}[{i}]")
                        }
                    }
                };
                flatten_recursive(
                    val,
                    &new_key,
                    separator,
                    array_format,
                    max_depth,
                    current_depth + 1,
                    result,
                );
            }
        }
        _ => {
            result.insert(prefix.to_string(), value.clone());
        }
    }
}

// ── Unflatten logic ──

fn unflatten_value(value: &Value, separator: &str) -> Value {
    match value {
        Value::Object(map) => {
            let mut root = Value::Object(IndexMap::new());
            for (key, val) in map {
                let segments = parse_key_segments(key, separator);
                set_nested(&mut root, &segments, val.clone());
            }
            root
        }
        Value::Array(arr) => {
            let unflattened: Vec<Value> = arr
                .iter()
                .map(|item| unflatten_value(item, separator))
                .collect();
            Value::Array(unflattened)
        }
        other => other.clone(),
    }
}

/// Parse a flat key into segments, handling both dot notation and bracket notation.
/// Examples:
///   "a.b.c" with separator "." -> ["a", "b", "c"]
///   "items[0].name" with separator "." -> ["items", "0", "name"]
///   "a.0.b" with separator "." -> ["a", "0", "b"]
fn parse_key_segments(key: &str, separator: &str) -> Vec<String> {
    let mut segments = Vec::new();

    for part in key.split(separator) {
        if part.is_empty() {
            continue;
        }
        // Handle bracket notation: "items[0]" -> "items", "0"
        let mut remaining = part;
        while !remaining.is_empty() {
            if let Some(bracket_start) = remaining.find('[') {
                if bracket_start > 0 {
                    segments.push(remaining[..bracket_start].to_string());
                }
                if let Some(bracket_end) = remaining.find(']') {
                    let index_str = &remaining[bracket_start + 1..bracket_end];
                    segments.push(index_str.to_string());
                    remaining = &remaining[bracket_end + 1..];
                } else {
                    // Malformed bracket, treat as literal
                    segments.push(remaining.to_string());
                    break;
                }
            } else {
                segments.push(remaining.to_string());
                break;
            }
        }
    }

    segments
}

/// Determine if a segment represents an array index
fn is_array_index(segment: &str) -> bool {
    !segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit())
}

/// Set a value at a nested path in a Value tree
fn set_nested(root: &mut Value, segments: &[String], val: Value) {
    if segments.is_empty() {
        return;
    }

    if segments.len() == 1 {
        match root {
            Value::Object(map) => {
                map.insert(segments[0].clone(), val);
            }
            Value::Array(arr) => {
                if let Ok(idx) = segments[0].parse::<usize>() {
                    while arr.len() <= idx {
                        arr.push(Value::Null);
                    }
                    arr[idx] = val;
                }
            }
            _ => {}
        }
        return;
    }

    let current = &segments[0];
    let next = &segments[1];

    // Determine the type of the next container
    let next_is_array = is_array_index(next);

    match root {
        Value::Object(map) => {
            let entry = map.entry(current.clone()).or_insert_with(|| {
                if next_is_array {
                    Value::Array(Vec::new())
                } else {
                    Value::Object(IndexMap::new())
                }
            });
            // Ensure correct type
            if next_is_array && !matches!(entry, Value::Array(_)) {
                *entry = Value::Array(Vec::new());
            } else if !next_is_array
                && !matches!(entry, Value::Object(_))
                && !matches!(entry, Value::Array(_))
            {
                *entry = Value::Object(IndexMap::new());
            }
            set_nested(entry, &segments[1..], val);
        }
        Value::Array(arr) => {
            if let Ok(idx) = current.parse::<usize>() {
                while arr.len() <= idx {
                    arr.push(Value::Null);
                }
                if matches!(arr[idx], Value::Null) {
                    arr[idx] = if next_is_array {
                        Value::Array(Vec::new())
                    } else {
                        Value::Object(IndexMap::new())
                    };
                }
                set_nested(&mut arr[idx], &segments[1..], val);
            }
        }
        _ => {}
    }
}

// ── Input reading (shared with sample.rs pattern) ──

fn read_input(
    input: &str,
    from: Option<&str>,
    delimiter: Option<char>,
    no_header: bool,
    encoding_opts: &EncodingOptions,
    excel_opts: &ExcelOptions,
    sqlite_opts: &SqliteOptions,
) -> Result<(Value, Format)> {
    if input == "-" {
        if from == Some("msgpack") || from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            let value = MsgpackReader.read_from_bytes(&buf)?;
            Ok((value, Format::Msgpack))
        } else {
            let buf = read_stdin_with_encoding(encoding_opts)?;
            let (format, sniffed_delimiter) = match from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&buf)?,
            };
            let auto_delimiter =
                sniffed_delimiter.or_else(|| from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: delimiter.or(auto_delimiter),
                no_header,
                ..Default::default()
            };
            let value = read_value(&buf, format, &read_options)?;
            Ok((value, format))
        }
    } else {
        let format = match from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(Path::new(input))?,
        };
        if format == Format::Msgpack {
            let bytes = read_file_bytes(Path::new(input))?;
            let value = MsgpackReader.read_from_bytes(&bytes)?;
            Ok((value, format))
        } else if format == Format::Xlsx {
            let bytes = read_file_bytes(Path::new(input))?;
            let value = read_xlsx_from_bytes(&bytes, excel_opts)?;
            Ok((value, format))
        } else if format == Format::Sqlite {
            let value = read_sqlite_from_path(Path::new(input), sqlite_opts)?;
            Ok((value, format))
        } else if format == Format::Parquet {
            let bytes = read_file_bytes(Path::new(input))?;
            let value = read_parquet_from_bytes(&bytes)?;
            Ok((value, format))
        } else {
            let content = read_file_with_encoding(Path::new(input), encoding_opts)?;
            let auto_delimiter = default_delimiter(Path::new(input));
            let read_options = FormatOptions {
                delimiter: delimiter.or(auto_delimiter),
                no_header,
                ..Default::default()
            };
            let value = read_value(&content, format, &read_options)?;
            Ok((value, format))
        }
    }
}

fn read_stdin_with_encoding(opts: &EncodingOptions) -> Result<String> {
    if opts.encoding.is_some() || opts.detect_encoding {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .context("Failed to read from stdin")?;
        super::decode_bytes(&buf, opts)
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read from stdin")?;
        Ok(buf)
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
        Format::Msgpack => MsgpackReader.read(content),
        Format::Xlsx => {
            bail!("Excel files must be read as binary; use file path input instead of stdin")
        }
        Format::Sqlite => {
            bail!("SQLite files must be read from a file path, not from text input")
        }
        Format::Parquet => {
            bail!("Parquet files must be read from a file path, not from text input")
        }
        Format::Markdown => bail!("Markdown is an output-only format and cannot be used as input"),
        Format::Html => bail!("HTML is an output-only format and cannot be used as input"),
        Format::Table => bail!("Table is an output-only format and cannot be used as input"),
    }
}

// ── Output writing ──

fn write_output(
    value: &Value,
    format: Format,
    options: &FormatOptions,
    output: Option<&Path>,
) -> Result<()> {
    if format == Format::Msgpack {
        let bytes = MsgpackWriter.write_bytes(value)?;
        if let Some(out_path) = output {
            fs::write(out_path, &bytes)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            io::stdout()
                .write_all(&bytes)
                .context("Failed to write to stdout")?;
        }
    } else if format == Format::Parquet {
        let bytes = super::write_parquet_to_bytes(value, &ParquetWriteOptions::default())?;
        if let Some(out_path) = output {
            fs::write(out_path, &bytes)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            io::stdout()
                .write_all(&bytes)
                .context("Failed to write Parquet to stdout")?;
        }
    } else {
        let result = write_value(value, format, options)?;
        if let Some(out_path) = output {
            fs::write(out_path, &result)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else if result.ends_with('\n') {
            print!("{result}");
        } else {
            println!("{result}");
        }
    }
    Ok(())
}

fn write_value(value: &Value, format: Format, options: &FormatOptions) -> Result<String> {
    match format {
        Format::Json => JsonWriter::new(options.clone()).write(value),
        Format::Jsonl => JsonlWriter.write(value),
        Format::Csv => CsvWriter::new(options.clone()).write(value),
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty, options.root_element.clone()).write(value),
        Format::Msgpack => MsgpackWriter.write(value),
        Format::Xlsx => bail!("Excel is an input-only format and cannot be used as output"),
        Format::Sqlite => bail!("SQLite is an input-only format and cannot be used as output"),
        Format::Parquet => {
            bail!("Internal error: Parquet output should be handled via write_output")
        }
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── flatten tests ──

    #[test]
    fn test_flatten_simple_object() {
        let mut inner = IndexMap::new();
        inner.insert("b".to_string(), Value::Integer(1));
        let mut obj = IndexMap::new();
        obj.insert("a".to_string(), Value::Object(inner));
        let value = Value::Object(obj);

        let result = flatten_value(&value, ".", ArrayFormat::Index, None);
        let map = result.as_object().unwrap();
        assert_eq!(map.get("a.b").unwrap(), &Value::Integer(1));
    }

    #[test]
    fn test_flatten_nested_object() {
        // {"a": {"b": {"c": 1}}}
        let mut c_obj = IndexMap::new();
        c_obj.insert("c".to_string(), Value::Integer(1));
        let mut b_obj = IndexMap::new();
        b_obj.insert("b".to_string(), Value::Object(c_obj));
        let mut obj = IndexMap::new();
        obj.insert("a".to_string(), Value::Object(b_obj));
        let value = Value::Object(obj);

        let result = flatten_value(&value, ".", ArrayFormat::Index, None);
        let map = result.as_object().unwrap();
        assert_eq!(map.get("a.b.c").unwrap(), &Value::Integer(1));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_flatten_with_array_index_format() {
        // {"items": [{"name": "a"}, {"name": "b"}]}
        let mut item1 = IndexMap::new();
        item1.insert("name".to_string(), Value::String("a".to_string()));
        let mut item2 = IndexMap::new();
        item2.insert("name".to_string(), Value::String("b".to_string()));
        let mut obj = IndexMap::new();
        obj.insert(
            "items".to_string(),
            Value::Array(vec![Value::Object(item1), Value::Object(item2)]),
        );
        let value = Value::Object(obj);

        let result = flatten_value(&value, ".", ArrayFormat::Index, None);
        let map = result.as_object().unwrap();
        assert_eq!(
            map.get("items.0.name").unwrap(),
            &Value::String("a".to_string())
        );
        assert_eq!(
            map.get("items.1.name").unwrap(),
            &Value::String("b".to_string())
        );
    }

    #[test]
    fn test_flatten_with_array_bracket_format() {
        let mut item1 = IndexMap::new();
        item1.insert("name".to_string(), Value::String("a".to_string()));
        let mut obj = IndexMap::new();
        obj.insert(
            "items".to_string(),
            Value::Array(vec![Value::Object(item1)]),
        );
        let value = Value::Object(obj);

        let result = flatten_value(&value, ".", ArrayFormat::Bracket, None);
        let map = result.as_object().unwrap();
        assert_eq!(
            map.get("items[0].name").unwrap(),
            &Value::String("a".to_string())
        );
    }

    #[test]
    fn test_flatten_custom_separator() {
        let mut inner = IndexMap::new();
        inner.insert("b".to_string(), Value::Integer(1));
        let mut obj = IndexMap::new();
        obj.insert("a".to_string(), Value::Object(inner));
        let value = Value::Object(obj);

        let result = flatten_value(&value, "/", ArrayFormat::Index, None);
        let map = result.as_object().unwrap();
        assert_eq!(map.get("a/b").unwrap(), &Value::Integer(1));
    }

    #[test]
    fn test_flatten_max_depth() {
        // {"a": {"b": {"c": 1}}} with max_depth=1
        let mut c_obj = IndexMap::new();
        c_obj.insert("c".to_string(), Value::Integer(1));
        let mut b_obj = IndexMap::new();
        b_obj.insert("b".to_string(), Value::Object(c_obj.clone()));
        let mut obj = IndexMap::new();
        obj.insert("a".to_string(), Value::Object(b_obj.clone()));
        let value = Value::Object(obj);

        // max_depth=1: recurse 1 level, so "a" maps to {"b":{"c":1}}
        let result = flatten_value(&value, ".", ArrayFormat::Index, Some(1));
        let map = result.as_object().unwrap();
        assert_eq!(map.get("a").unwrap(), &Value::Object(b_obj));

        // max_depth=2: recurse 2 levels, so "a.b" maps to {"c":1}
        let result2 = flatten_value(&value, ".", ArrayFormat::Index, Some(2));
        let map2 = result2.as_object().unwrap();
        assert_eq!(map2.get("a.b").unwrap(), &Value::Object(c_obj));
    }

    #[test]
    fn test_flatten_empty_object() {
        let value = Value::Object(IndexMap::new());
        let result = flatten_value(&value, ".", ArrayFormat::Index, None);
        let map = result.as_object().unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn test_flatten_mixed_types() {
        let mut obj = IndexMap::new();
        obj.insert("name".to_string(), Value::String("test".to_string()));
        obj.insert("count".to_string(), Value::Integer(42));
        obj.insert("active".to_string(), Value::Bool(true));
        obj.insert("data".to_string(), Value::Null);
        let value = Value::Object(obj);

        let result = flatten_value(&value, ".", ArrayFormat::Index, None);
        let map = result.as_object().unwrap();
        assert_eq!(map.len(), 4);
        assert_eq!(map.get("name").unwrap(), &Value::String("test".to_string()));
        assert_eq!(map.get("count").unwrap(), &Value::Integer(42));
    }

    #[test]
    fn test_flatten_array_of_objects() {
        let mut item = IndexMap::new();
        item.insert("x".to_string(), Value::Integer(1));
        let arr = Value::Array(vec![Value::Object(item.clone()), Value::Object(item)]);

        let result = flatten_value(&arr, ".", ArrayFormat::Index, None);
        let items = result.as_array().unwrap();
        assert_eq!(items.len(), 2);
        for item in items {
            let map = item.as_object().unwrap();
            assert_eq!(map.get("x").unwrap(), &Value::Integer(1));
        }
    }

    // ── unflatten tests ──

    #[test]
    fn test_unflatten_simple() {
        let mut obj = IndexMap::new();
        obj.insert("a.b".to_string(), Value::Integer(1));
        let value = Value::Object(obj);

        let result = unflatten_value(&value, ".");
        let map = result.as_object().unwrap();
        let a = map.get("a").unwrap().as_object().unwrap();
        assert_eq!(a.get("b").unwrap(), &Value::Integer(1));
    }

    #[test]
    fn test_unflatten_deep() {
        let mut obj = IndexMap::new();
        obj.insert("a.b.c".to_string(), Value::Integer(1));
        let value = Value::Object(obj);

        let result = unflatten_value(&value, ".");
        let a = result.as_object().unwrap().get("a").unwrap().clone();
        let b = a.as_object().unwrap().get("b").unwrap().clone();
        let c = b.as_object().unwrap().get("c").unwrap();
        assert_eq!(c, &Value::Integer(1));
    }

    #[test]
    fn test_unflatten_with_array_index() {
        let mut obj = IndexMap::new();
        obj.insert("items.0.name".to_string(), Value::String("a".to_string()));
        obj.insert("items.1.name".to_string(), Value::String("b".to_string()));
        let value = Value::Object(obj);

        let result = unflatten_value(&value, ".");
        let items = result
            .as_object()
            .unwrap()
            .get("items")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(
            items[0].as_object().unwrap().get("name").unwrap(),
            &Value::String("a".to_string())
        );
        assert_eq!(
            items[1].as_object().unwrap().get("name").unwrap(),
            &Value::String("b".to_string())
        );
    }

    #[test]
    fn test_unflatten_with_bracket_notation() {
        let mut obj = IndexMap::new();
        obj.insert("items[0].name".to_string(), Value::String("a".to_string()));
        obj.insert("items[1].name".to_string(), Value::String("b".to_string()));
        let value = Value::Object(obj);

        let result = unflatten_value(&value, ".");
        let items = result
            .as_object()
            .unwrap()
            .get("items")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_unflatten_custom_separator() {
        let mut obj = IndexMap::new();
        obj.insert("a/b".to_string(), Value::Integer(1));
        let value = Value::Object(obj);

        let result = unflatten_value(&value, "/");
        let a = result
            .as_object()
            .unwrap()
            .get("a")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(a.get("b").unwrap(), &Value::Integer(1));
    }

    #[test]
    fn test_roundtrip_flatten_unflatten() {
        // Build: {"a": {"b": 1}, "c": {"d": {"e": 2}}}
        let mut d_obj = IndexMap::new();
        d_obj.insert("e".to_string(), Value::Integer(2));
        let mut c_obj = IndexMap::new();
        c_obj.insert("d".to_string(), Value::Object(d_obj));
        let mut b_obj = IndexMap::new();
        b_obj.insert("b".to_string(), Value::Integer(1));
        let mut obj = IndexMap::new();
        obj.insert("a".to_string(), Value::Object(b_obj));
        obj.insert("c".to_string(), Value::Object(c_obj));
        let original = Value::Object(obj);

        let flat = flatten_value(&original, ".", ArrayFormat::Index, None);
        let restored = unflatten_value(&flat, ".");
        assert_eq!(original, restored);
    }

    #[test]
    fn test_roundtrip_with_arrays() {
        // {"items": [{"name": "a"}, {"name": "b"}]}
        let mut item1 = IndexMap::new();
        item1.insert("name".to_string(), Value::String("a".to_string()));
        let mut item2 = IndexMap::new();
        item2.insert("name".to_string(), Value::String("b".to_string()));
        let mut obj = IndexMap::new();
        obj.insert(
            "items".to_string(),
            Value::Array(vec![Value::Object(item1), Value::Object(item2)]),
        );
        let original = Value::Object(obj);

        let flat = flatten_value(&original, ".", ArrayFormat::Index, None);
        let restored = unflatten_value(&flat, ".");
        assert_eq!(original, restored);
    }

    #[test]
    fn test_parse_key_segments_simple() {
        let segments = parse_key_segments("a.b.c", ".");
        assert_eq!(segments, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_key_segments_bracket() {
        let segments = parse_key_segments("items[0].name", ".");
        assert_eq!(segments, vec!["items", "0", "name"]);
    }

    #[test]
    fn test_parse_key_segments_mixed() {
        let segments = parse_key_segments("a[0].b[1].c", ".");
        assert_eq!(segments, vec!["a", "0", "b", "1", "c"]);
    }

    #[test]
    fn test_flatten_preserves_scalar_at_root() {
        let value = Value::Integer(42);
        let result = flatten_value(&value, ".", ArrayFormat::Index, None);
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_unflatten_array_of_objects() {
        let mut obj1 = IndexMap::new();
        obj1.insert("a.b".to_string(), Value::Integer(1));
        let mut obj2 = IndexMap::new();
        obj2.insert("a.b".to_string(), Value::Integer(2));
        let arr = Value::Array(vec![Value::Object(obj1), Value::Object(obj2)]);

        let result = unflatten_value(&arr, ".");
        let items = result.as_array().unwrap();
        assert_eq!(items.len(), 2);
        let a1 = items[0]
            .as_object()
            .unwrap()
            .get("a")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(a1.get("b").unwrap(), &Value::Integer(1));
    }
}
