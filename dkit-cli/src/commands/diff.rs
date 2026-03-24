use std::path::Path;

use anyhow::{bail, Result};
use colored::Colorize;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::json::JsonReader;
use dkit_core::format::jsonl::JsonlReader;
use dkit_core::format::msgpack::MsgpackReader;
use dkit_core::format::toml::TomlReader;
use dkit_core::format::xml::XmlReader;
use dkit_core::format::yaml::YamlReader;
use dkit_core::format::{default_delimiter, detect_format, Format, FormatOptions, FormatReader};
use dkit_core::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum DiffMode {
    Structural,
    Value,
    Key,
}

impl DiffMode {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "structural" => Ok(DiffMode::Structural),
            "value" => Ok(DiffMode::Value),
            "key" => Ok(DiffMode::Key),
            other => bail!("Unknown diff mode: '{other}'. Valid modes: structural, value, key"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffOutputFormat {
    Unified,
    SideBySide,
    Json,
    Summary,
}

impl DiffOutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "unified" => Ok(DiffOutputFormat::Unified),
            "side-by-side" | "sidebyside" => Ok(DiffOutputFormat::SideBySide),
            "json" => Ok(DiffOutputFormat::Json),
            "summary" => Ok(DiffOutputFormat::Summary),
            other => bail!(
                "Unknown diff format: '{other}'. Valid formats: unified, side-by-side, json, summary"
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArrayDiffStrategy {
    Index,
    Value,
    Key(String),
}

impl ArrayDiffStrategy {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "index" => Ok(ArrayDiffStrategy::Index),
            "value" => Ok(ArrayDiffStrategy::Value),
            _ if s.starts_with("key=") => {
                let field = s.strip_prefix("key=").unwrap();
                if field.is_empty() {
                    bail!("key= strategy requires a field name, e.g. --array-diff key=id");
                }
                Ok(ArrayDiffStrategy::Key(field.to_string()))
            }
            other => bail!(
                "Unknown array-diff strategy: '{other}'. Valid options: index, value, key=<field>"
            ),
        }
    }
}

pub struct DiffArgs<'a> {
    pub file1: &'a Path,
    pub file2: &'a Path,
    pub path: Option<&'a str>,
    pub quiet: bool,
    pub mode: DiffMode,
    pub diff_format: DiffOutputFormat,
    pub array_diff: ArrayDiffStrategy,
    pub ignore_order: bool,
    pub ignore_case: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

struct DiffOptions<'a> {
    array_diff: &'a ArrayDiffStrategy,
    ignore_order: bool,
    ignore_case: bool,
}

/// diff 서브커맨드 실행. 차이가 있으면 true, 없으면 false 반환.
pub fn run(args: &DiffArgs) -> Result<bool> {
    let value1 = read_value_from_path(
        args.file1,
        &args.encoding_opts,
        &args.excel_opts,
        &args.sqlite_opts,
    )?;
    let value2 = read_value_from_path(
        args.file2,
        &args.encoding_opts,
        &args.excel_opts,
        &args.sqlite_opts,
    )?;

    let (v1, v2) = match args.path {
        Some(path_expr) => (
            resolve_path(&value1, path_expr)?,
            resolve_path(&value2, path_expr)?,
        ),
        None => (value1, value2),
    };

    let opts = DiffOptions {
        array_diff: &args.array_diff,
        ignore_order: args.ignore_order,
        ignore_case: args.ignore_case,
    };

    let all_diffs = compute_diff("", &v1, &v2, &opts);

    // Filter entries based on mode
    let display_entries: Vec<&DiffEntry> = match args.mode {
        DiffMode::Structural => all_diffs.iter().collect(),
        DiffMode::Value => all_diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Changed { .. }))
            .collect(),
        DiffMode::Key => all_diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Added { .. } | DiffEntry::Removed { .. }))
            .collect(),
    };

    let has_diff = display_entries
        .iter()
        .any(|d| !matches!(d, DiffEntry::Unchanged { .. }));

    if !has_diff {
        if !args.quiet {
            println!("No differences found.");
        }
        return Ok(false);
    }

    if args.quiet {
        return Ok(true);
    }

    match args.diff_format {
        DiffOutputFormat::Unified => print_unified(&display_entries),
        DiffOutputFormat::SideBySide => print_side_by_side(&display_entries),
        DiffOutputFormat::Json => print_json_format(&display_entries)?,
        DiffOutputFormat::Summary => print_summary(&display_entries),
    }

    Ok(true)
}

/// 파일 경로에서 Value를 읽는다
fn read_value_from_path(
    path: &Path,
    encoding_opts: &EncodingOptions,
    excel_opts: &ExcelOptions,
    sqlite_opts: &SqliteOptions,
) -> Result<Value> {
    let format = detect_format(path)?;
    let delimiter = default_delimiter(path);
    let options = FormatOptions {
        delimiter,
        ..Default::default()
    };

    if format == Format::Msgpack {
        let bytes = read_file_bytes(path)?;
        MsgpackReader.read_from_bytes(&bytes)
    } else if format == Format::Xlsx {
        let bytes = read_file_bytes(path)?;
        read_xlsx_from_bytes(&bytes, excel_opts)
    } else if format == Format::Sqlite {
        read_sqlite_from_path(path, sqlite_opts)
    } else if format == Format::Parquet {
        let bytes = read_file_bytes(path)?;
        read_parquet_from_bytes(&bytes)
    } else {
        let content = read_file_with_encoding(path, encoding_opts)?;
        read_value(&content, format, &options)
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

/// 경로 표현식으로 Value 탐색
fn resolve_path(value: &Value, path_expr: &str) -> Result<Value> {
    let path_expr = path_expr.trim();
    if path_expr.is_empty() || path_expr == "." {
        return Ok(value.clone());
    }

    let path_expr = path_expr.strip_prefix('.').unwrap_or(path_expr);
    let mut current = value.clone();

    for segment in path_expr.split('.') {
        if segment.is_empty() {
            continue;
        }

        // Handle array index: field[0]
        if let Some(bracket_pos) = segment.find('[') {
            let field = &segment[..bracket_pos];
            if !field.is_empty() {
                current = access_field(&current, field)?;
            }
            let idx_str = segment[bracket_pos + 1..]
                .strip_suffix(']')
                .unwrap_or(&segment[bracket_pos + 1..]);
            let idx: usize = idx_str
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid array index: {idx_str}"))?;
            current = access_index(&current, idx)?;
        } else {
            current = access_field(&current, segment)?;
        }
    }

    Ok(current)
}

fn access_field(value: &Value, field: &str) -> Result<Value> {
    match value {
        Value::Object(map) => match map.get(field) {
            Some(v) => Ok(v.clone()),
            None => bail!("Field '{field}' not found"),
        },
        _ => bail!("Cannot access field '{field}' on non-object value"),
    }
}

fn access_index(value: &Value, idx: usize) -> Result<Value> {
    match value {
        Value::Array(arr) => match arr.get(idx) {
            Some(v) => Ok(v.clone()),
            None => bail!("Array index {idx} out of bounds (length: {})", arr.len()),
        },
        _ => bail!("Cannot index non-array value"),
    }
}

/// diff 결과 하나의 엔트리
enum DiffEntry {
    Unchanged {
        path: String,
        value: String,
    },
    Added {
        path: String,
        value: String,
    },
    Removed {
        path: String,
        value: String,
    },
    Changed {
        path: String,
        old: String,
        new: String,
    },
}

/// 두 Value 간 재귀적 비교
fn compute_diff(
    path_prefix: &str,
    left: &Value,
    right: &Value,
    opts: &DiffOptions,
) -> Vec<DiffEntry> {
    let mut entries = Vec::new();

    match (left, right) {
        (Value::Object(map1), Value::Object(map2)) => {
            for (key, v1) in map1 {
                let child_path = if path_prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{path_prefix}.{key}")
                };

                match map2.get(key) {
                    Some(v2) => {
                        entries.extend(compute_diff(&child_path, v1, v2, opts));
                    }
                    None => {
                        entries.extend(collect_removed(&child_path, v1));
                    }
                }
            }
            for (key, v2) in map2 {
                if !map1.contains_key(key) {
                    let child_path = if path_prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{path_prefix}.{key}")
                    };
                    entries.extend(collect_added(&child_path, v2));
                }
            }
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            let arr1_cmp = if opts.ignore_order {
                let mut s = arr1.clone();
                s.sort_by_key(|a| a.to_string());
                s
            } else {
                arr1.clone()
            };
            let arr2_cmp = if opts.ignore_order {
                let mut s = arr2.clone();
                s.sort_by_key(|a| a.to_string());
                s
            } else {
                arr2.clone()
            };

            match opts.array_diff {
                ArrayDiffStrategy::Index => {
                    entries.extend(compare_arrays_index(
                        path_prefix,
                        &arr1_cmp,
                        &arr2_cmp,
                        opts,
                    ));
                }
                ArrayDiffStrategy::Value => {
                    entries.extend(compare_arrays_value(
                        path_prefix,
                        &arr1_cmp,
                        &arr2_cmp,
                        opts,
                    ));
                }
                ArrayDiffStrategy::Key(ref field) => {
                    entries.extend(compare_arrays_key(path_prefix, arr1, arr2, field, opts));
                }
            }
        }
        _ => {
            if values_equal(left, right, opts.ignore_case) {
                entries.push(DiffEntry::Unchanged {
                    path: path_prefix.to_string(),
                    value: format_scalar(left),
                });
            } else {
                entries.push(DiffEntry::Changed {
                    path: path_prefix.to_string(),
                    old: format_scalar(left),
                    new: format_scalar(right),
                });
            }
        }
    }

    entries
}

/// 인덱스 기반 배열 비교
fn compare_arrays_index(
    path_prefix: &str,
    arr1: &[Value],
    arr2: &[Value],
    opts: &DiffOptions,
) -> Vec<DiffEntry> {
    let mut entries = Vec::new();
    let max_len = arr1.len().max(arr2.len());
    for i in 0..max_len {
        let child_path = format!("{path_prefix}[{i}]");
        match (arr1.get(i), arr2.get(i)) {
            (Some(v1), Some(v2)) => {
                entries.extend(compute_diff(&child_path, v1, v2, opts));
            }
            (Some(v1), None) => {
                entries.extend(collect_removed(&child_path, v1));
            }
            (None, Some(v2)) => {
                entries.extend(collect_added(&child_path, v2));
            }
            (None, None) => unreachable!(),
        }
    }
    entries
}

/// 값 기반 배열 비교 (순서 무관, 집합처럼 비교)
fn compare_arrays_value(
    path_prefix: &str,
    arr1: &[Value],
    arr2: &[Value],
    opts: &DiffOptions,
) -> Vec<DiffEntry> {
    let mut entries = Vec::new();
    let mut matched2 = vec![false; arr2.len()];

    for v1 in arr1 {
        let found = arr2
            .iter()
            .enumerate()
            .find(|(j, v2)| !matched2[*j] && values_equal(v1, v2, opts.ignore_case));
        match found {
            Some((j, _)) => {
                matched2[j] = true;
                entries.push(DiffEntry::Unchanged {
                    path: path_prefix.to_string(),
                    value: format_scalar(v1),
                });
            }
            None => {
                entries.push(DiffEntry::Removed {
                    path: path_prefix.to_string(),
                    value: format_scalar(v1),
                });
            }
        }
    }

    for (j, v2) in arr2.iter().enumerate() {
        if !matched2[j] {
            entries.push(DiffEntry::Added {
                path: path_prefix.to_string(),
                value: format_scalar(v2),
            });
        }
    }

    entries
}

/// 키 필드 기반 배열 비교 (지정한 필드값으로 매칭)
fn compare_arrays_key(
    path_prefix: &str,
    arr1: &[Value],
    arr2: &[Value],
    key_field: &str,
    opts: &DiffOptions,
) -> Vec<DiffEntry> {
    let mut entries = Vec::new();
    let mut matched2 = vec![false; arr2.len()];

    for (i, v1) in arr1.iter().enumerate() {
        let key1 = get_key_value(v1, key_field);
        let match_idx = arr2.iter().enumerate().position(|(j, v2)| {
            !matched2[j] && key1.is_some() && get_key_value(v2, key_field) == key1
        });

        match match_idx {
            Some(j) => {
                matched2[j] = true;
                let item_key = key1.unwrap_or_else(|| i.to_string());
                let item_path = format!("{path_prefix}[{key_field}={item_key}]");
                entries.extend(compute_diff(&item_path, v1, &arr2[j], opts));
            }
            None => {
                let item_path = format!("{path_prefix}[{i}]");
                entries.extend(collect_removed(&item_path, v1));
            }
        }
    }

    for (j, v2) in arr2.iter().enumerate() {
        if !matched2[j] {
            let item_path = format!("{path_prefix}[{}]", j);
            entries.extend(collect_added(&item_path, v2));
        }
    }

    entries
}

/// 두 Value가 같은지 비교 (ignore_case 옵션 고려)
fn values_equal(left: &Value, right: &Value, ignore_case: bool) -> bool {
    if ignore_case {
        match (left, right) {
            (Value::String(s1), Value::String(s2)) => s1.to_lowercase() == s2.to_lowercase(),
            _ => left == right,
        }
    } else {
        left == right
    }
}

/// 객체에서 키 필드의 값을 문자열로 가져오기
fn get_key_value(value: &Value, key_field: &str) -> Option<String> {
    match value {
        Value::Object(map) => map.get(key_field).map(format_scalar),
        _ => None,
    }
}

/// 삭제된 Value의 모든 리프 노드를 DiffEntry::Removed로 수집
fn collect_removed(path: &str, value: &Value) -> Vec<DiffEntry> {
    collect_flat_entries(path, value, true)
}

/// 추가된 Value의 모든 리프 노드를 DiffEntry::Added로 수집
fn collect_added(path: &str, value: &Value) -> Vec<DiffEntry> {
    collect_flat_entries(path, value, false)
}

fn collect_flat_entries(path: &str, value: &Value, is_removed: bool) -> Vec<DiffEntry> {
    let mut entries = Vec::new();
    match value {
        Value::Object(map) => {
            for (key, v) in map {
                let child_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                entries.extend(collect_flat_entries(&child_path, v, is_removed));
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let child_path = format!("{path}[{i}]");
                entries.extend(collect_flat_entries(&child_path, v, is_removed));
            }
        }
        _ => {
            let formatted = format_scalar(value);
            if is_removed {
                entries.push(DiffEntry::Removed {
                    path: path.to_string(),
                    value: formatted,
                });
            } else {
                entries.push(DiffEntry::Added {
                    path: path.to_string(),
                    value: formatted,
                });
            }
        }
    }
    entries
}

/// 스칼라 값을 표시용 문자열로 변환
fn format_scalar(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => {
            if f.fract() == 0.0 && f.is_finite() {
                format!("{f:.1}")
            } else {
                f.to_string()
            }
        }
        Value::Array(_) | Value::Object(_) => value.to_string(),
    }
}

// ─── 출력 함수들 ──────────────────────────────────────────────────────────────

/// unified diff 형식 출력 (기본)
fn print_unified(entries: &[&DiffEntry]) {
    for entry in entries {
        match entry {
            DiffEntry::Unchanged { path, value } => {
                println!("  {path}: {value} (unchanged)");
            }
            DiffEntry::Added { path, value } => {
                println!("{}", format!("+ {path}: {value} (added)").green());
            }
            DiffEntry::Removed { path, value } => {
                println!("{}", format!("- {path}: {value} (removed)").red());
            }
            DiffEntry::Changed { path, old, new } => {
                println!("{}", format!("- {path}: {old}").red());
                println!("{}", format!("+ {path}: {new}").yellow());
            }
        }
    }
}

/// side-by-side 형식 출력
fn print_side_by_side(entries: &[&DiffEntry]) {
    // 컬럼 너비 계산
    let path_width = entries
        .iter()
        .map(|e| match e {
            DiffEntry::Unchanged { path, .. }
            | DiffEntry::Added { path, .. }
            | DiffEntry::Removed { path, .. }
            | DiffEntry::Changed { path, .. } => path.len(),
        })
        .max()
        .unwrap_or(4)
        .max(4);

    let val_width = 24usize;

    // 헤더
    println!(
        "{:<path_width$}  {:<val_width$}  {:<val_width$}  STATUS",
        "PATH",
        "LEFT",
        "RIGHT",
        path_width = path_width,
        val_width = val_width
    );
    println!("{}", "-".repeat(path_width + val_width * 2 + 12));

    for entry in entries {
        match entry {
            DiffEntry::Unchanged { path, value } => {
                println!(
                    "{:<path_width$}  {:<val_width$}  {:<val_width$}  =",
                    path,
                    truncate(value, val_width),
                    truncate(value, val_width),
                    path_width = path_width,
                    val_width = val_width
                );
            }
            DiffEntry::Added { path, value } => {
                let line = format!(
                    "{:<path_width$}  {:<val_width$}  {:<val_width$}  +",
                    path,
                    "(absent)",
                    truncate(value, val_width),
                    path_width = path_width,
                    val_width = val_width
                );
                println!("{}", line.green());
            }
            DiffEntry::Removed { path, value } => {
                let line = format!(
                    "{:<path_width$}  {:<val_width$}  {:<val_width$}  -",
                    path,
                    truncate(value, val_width),
                    "(absent)",
                    path_width = path_width,
                    val_width = val_width
                );
                println!("{}", line.red());
            }
            DiffEntry::Changed { path, old, new } => {
                let line = format!(
                    "{:<path_width$}  {:<val_width$}  {:<val_width$}  ~",
                    path,
                    truncate(old, val_width),
                    truncate(new, val_width),
                    path_width = path_width,
                    val_width = val_width
                );
                println!("{}", line.yellow());
            }
        }
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
}

/// JSON 형식 출력
fn print_json_format(entries: &[&DiffEntry]) -> Result<()> {
    let mut items = Vec::new();
    for entry in entries {
        let obj = match entry {
            DiffEntry::Unchanged { path, value } => {
                format!(
                    r#"{{"path":{},"type":"unchanged","value":{}}}"#,
                    serde_json::to_string(path)?,
                    serde_json::to_string(value)?
                )
            }
            DiffEntry::Added { path, value } => {
                format!(
                    r#"{{"path":{},"type":"added","value":{}}}"#,
                    serde_json::to_string(path)?,
                    serde_json::to_string(value)?
                )
            }
            DiffEntry::Removed { path, value } => {
                format!(
                    r#"{{"path":{},"type":"removed","value":{}}}"#,
                    serde_json::to_string(path)?,
                    serde_json::to_string(value)?
                )
            }
            DiffEntry::Changed { path, old, new } => {
                format!(
                    r#"{{"path":{},"type":"changed","old":{},"new":{}}}"#,
                    serde_json::to_string(path)?,
                    serde_json::to_string(old)?,
                    serde_json::to_string(new)?
                )
            }
        };
        items.push(obj);
    }
    println!("[{}]", items.join(",\n "));
    Ok(())
}

/// summary 형식 출력
fn print_summary(entries: &[&DiffEntry]) {
    let mut added = 0usize;
    let mut removed = 0usize;
    let mut changed = 0usize;
    let mut unchanged = 0usize;

    for entry in entries {
        match entry {
            DiffEntry::Added { .. } => added += 1,
            DiffEntry::Removed { .. } => removed += 1,
            DiffEntry::Changed { .. } => changed += 1,
            DiffEntry::Unchanged { .. } => unchanged += 1,
        }
    }

    println!(
        "Summary: {} added, {} removed, {} changed, {} unchanged",
        added.to_string().green(),
        removed.to_string().red(),
        changed.to_string().yellow(),
        unchanged
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn obj(pairs: Vec<(&str, Value)>) -> Value {
        let mut map = IndexMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v);
        }
        Value::Object(map)
    }

    fn default_opts() -> DiffOptions<'static> {
        DiffOptions {
            array_diff: &ArrayDiffStrategy::Index,
            ignore_order: false,
            ignore_case: false,
        }
    }

    #[test]
    fn test_identical_values_no_diff() {
        let v = obj(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
        ]);
        let opts = default_opts();
        let diffs = compute_diff("", &v, &v, &opts);
        assert!(diffs
            .iter()
            .all(|d| matches!(d, DiffEntry::Unchanged { .. })));
    }

    #[test]
    fn test_changed_scalar() {
        let v1 = obj(vec![("port", Value::Integer(5432))]);
        let v2 = obj(vec![("port", Value::Integer(3306))]);
        let opts = default_opts();
        let diffs = compute_diff("", &v1, &v2, &opts);
        assert_eq!(diffs.len(), 1);
        assert!(matches!(&diffs[0], DiffEntry::Changed { path, old, new }
            if path == "port" && old == "5432" && new == "3306"));
    }

    #[test]
    fn test_added_field() {
        let v1 = obj(vec![("host", Value::String("localhost".into()))]);
        let v2 = obj(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
        ]);
        let opts = default_opts();
        let diffs = compute_diff("", &v1, &v2, &opts);
        assert_eq!(diffs.len(), 2);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { path, .. } if path == "host"));
        assert!(matches!(&diffs[1], DiffEntry::Added { path, value }
            if path == "port" && value == "5432"));
    }

    #[test]
    fn test_removed_field() {
        let v1 = obj(vec![
            ("host", Value::String("localhost".into())),
            ("debug", Value::Bool(true)),
        ]);
        let v2 = obj(vec![("host", Value::String("localhost".into()))]);
        let opts = default_opts();
        let diffs = compute_diff("", &v1, &v2, &opts);
        assert_eq!(diffs.len(), 2);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { path, .. } if path == "host"));
        assert!(matches!(&diffs[1], DiffEntry::Removed { path, value }
            if path == "debug" && value == "true"));
    }

    #[test]
    fn test_nested_object_diff() {
        let v1 = obj(vec![(
            "database",
            obj(vec![
                ("host", Value::String("localhost".into())),
                ("port", Value::Integer(5432)),
            ]),
        )]);
        let v2 = obj(vec![(
            "database",
            obj(vec![
                ("host", Value::String("localhost".into())),
                ("port", Value::Integer(3306)),
            ]),
        )]);
        let opts = default_opts();
        let diffs = compute_diff("", &v1, &v2, &opts);
        assert_eq!(diffs.len(), 2);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { path, .. } if path == "database.host"));
        assert!(matches!(&diffs[1], DiffEntry::Changed { path, .. } if path == "database.port"));
    }

    #[test]
    fn test_array_diff_index() {
        let v1 = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let v2 = Value::Array(vec![Value::Integer(1), Value::Integer(99)]);
        let opts = default_opts();
        let diffs = compute_diff("items", &v1, &v2, &opts);
        assert_eq!(diffs.len(), 3);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { path, .. } if path == "items[0]"));
        assert!(matches!(&diffs[1], DiffEntry::Changed { path, .. } if path == "items[1]"));
        assert!(matches!(&diffs[2], DiffEntry::Removed { path, .. } if path == "items[2]"));
    }

    #[test]
    fn test_ignore_case() {
        let v1 = Value::String("Hello".into());
        let v2 = Value::String("hello".into());
        let opts_sensitive = DiffOptions {
            array_diff: &ArrayDiffStrategy::Index,
            ignore_order: false,
            ignore_case: false,
        };
        let diffs = compute_diff("field", &v1, &v2, &opts_sensitive);
        assert!(matches!(&diffs[0], DiffEntry::Changed { .. }));

        let opts_insensitive = DiffOptions {
            array_diff: &ArrayDiffStrategy::Index,
            ignore_order: false,
            ignore_case: true,
        };
        let diffs = compute_diff("field", &v1, &v2, &opts_insensitive);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { .. }));
    }

    #[test]
    fn test_ignore_order() {
        let arr_strategy = ArrayDiffStrategy::Index;
        let v1 = Value::Array(vec![
            Value::Integer(3),
            Value::Integer(1),
            Value::Integer(2),
        ]);
        let v2 = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let opts_no_ignore = DiffOptions {
            array_diff: &arr_strategy,
            ignore_order: false,
            ignore_case: false,
        };
        let diffs = compute_diff("arr", &v1, &v2, &opts_no_ignore);
        let changed = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Changed { .. }))
            .count();
        assert!(changed > 0);

        let opts_ignore = DiffOptions {
            array_diff: &arr_strategy,
            ignore_order: true,
            ignore_case: false,
        };
        let diffs = compute_diff("arr", &v1, &v2, &opts_ignore);
        assert!(diffs
            .iter()
            .all(|d| matches!(d, DiffEntry::Unchanged { .. })));
    }

    #[test]
    fn test_array_diff_value() {
        let arr_strategy = ArrayDiffStrategy::Value;
        let v1 = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let v2 = Value::Array(vec![
            Value::Integer(3),
            Value::Integer(1),
            Value::Integer(4),
        ]);
        let opts = DiffOptions {
            array_diff: &arr_strategy,
            ignore_order: false,
            ignore_case: false,
        };
        let diffs = compute_diff("arr", &v1, &v2, &opts);
        let added = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Added { .. }))
            .count();
        let removed = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Removed { .. }))
            .count();
        let unchanged = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Unchanged { .. }))
            .count();
        assert_eq!(added, 1); // 4 added
        assert_eq!(removed, 1); // 2 removed
        assert_eq!(unchanged, 2); // 1, 3 unchanged
    }

    #[test]
    fn test_array_diff_key() {
        let arr_strategy = ArrayDiffStrategy::Key("id".to_string());
        let v1 = Value::Array(vec![
            obj(vec![
                ("id", Value::Integer(1)),
                ("name", Value::String("Alice".into())),
            ]),
            obj(vec![
                ("id", Value::Integer(2)),
                ("name", Value::String("Bob".into())),
            ]),
        ]);
        let v2 = Value::Array(vec![
            obj(vec![
                ("id", Value::Integer(2)),
                ("name", Value::String("Bobby".into())),
            ]),
            obj(vec![
                ("id", Value::Integer(3)),
                ("name", Value::String("Carol".into())),
            ]),
        ]);
        let opts = DiffOptions {
            array_diff: &arr_strategy,
            ignore_order: false,
            ignore_case: false,
        };
        let diffs = compute_diff("users", &v1, &v2, &opts);
        // id=1: removed (Alice)
        // id=2: name changed (Bob -> Bobby)
        // id=3: added (Carol)
        let added = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Added { .. }))
            .count();
        let removed = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Removed { .. }))
            .count();
        let changed = diffs
            .iter()
            .filter(|d| matches!(d, DiffEntry::Changed { .. }))
            .count();
        assert!(removed >= 1);
        assert!(added >= 1);
        assert!(changed >= 1);
    }

    #[test]
    fn test_scalar_type_change() {
        let v1 = Value::String("hello".into());
        let v2 = Value::Integer(42);
        let opts = default_opts();
        let diffs = compute_diff("field", &v1, &v2, &opts);
        assert_eq!(diffs.len(), 1);
        assert!(matches!(&diffs[0], DiffEntry::Changed { path, old, new }
            if path == "field" && old == "\"hello\"" && new == "42"));
    }

    #[test]
    fn test_resolve_path_simple() {
        let v = obj(vec![(
            "database",
            obj(vec![("host", Value::String("localhost".into()))]),
        )]);
        let result = resolve_path(&v, ".database.host").unwrap();
        assert_eq!(result, Value::String("localhost".into()));
    }

    #[test]
    fn test_resolve_path_root() {
        let v = Value::Integer(42);
        let result = resolve_path(&v, ".").unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_format_scalar_values() {
        assert_eq!(format_scalar(&Value::Null), "null");
        assert_eq!(format_scalar(&Value::Bool(true)), "true");
        assert_eq!(format_scalar(&Value::Integer(42)), "42");
        assert_eq!(format_scalar(&Value::Float(3.14)), "3.14");
        assert_eq!(format_scalar(&Value::String("hello".into())), "\"hello\"");
    }

    #[test]
    fn test_diff_mode_parse() {
        assert_eq!(
            DiffMode::from_str("structural").unwrap(),
            DiffMode::Structural
        );
        assert_eq!(DiffMode::from_str("value").unwrap(), DiffMode::Value);
        assert_eq!(DiffMode::from_str("key").unwrap(), DiffMode::Key);
        assert!(DiffMode::from_str("unknown").is_err());
    }

    #[test]
    fn test_diff_output_format_parse() {
        assert_eq!(
            DiffOutputFormat::from_str("unified").unwrap(),
            DiffOutputFormat::Unified
        );
        assert_eq!(
            DiffOutputFormat::from_str("side-by-side").unwrap(),
            DiffOutputFormat::SideBySide
        );
        assert_eq!(
            DiffOutputFormat::from_str("json").unwrap(),
            DiffOutputFormat::Json
        );
        assert_eq!(
            DiffOutputFormat::from_str("summary").unwrap(),
            DiffOutputFormat::Summary
        );
        assert!(DiffOutputFormat::from_str("unknown").is_err());
    }

    #[test]
    fn test_array_diff_strategy_parse() {
        assert!(matches!(
            ArrayDiffStrategy::from_str("index").unwrap(),
            ArrayDiffStrategy::Index
        ));
        assert!(matches!(
            ArrayDiffStrategy::from_str("value").unwrap(),
            ArrayDiffStrategy::Value
        ));
        assert!(
            matches!(ArrayDiffStrategy::from_str("key=id").unwrap(), ArrayDiffStrategy::Key(f) if f == "id")
        );
        assert!(ArrayDiffStrategy::from_str("key=").is_err());
        assert!(ArrayDiffStrategy::from_str("unknown").is_err());
    }
}
