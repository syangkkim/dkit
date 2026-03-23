use std::path::Path;

use anyhow::{bail, Result};
use colored::Colorize;

use super::{read_file, read_file_bytes};
use crate::format::csv::CsvReader;
use crate::format::json::JsonReader;
use crate::format::jsonl::JsonlReader;
use crate::format::msgpack::MsgpackReader;
use crate::format::toml::TomlReader;
use crate::format::xml::XmlReader;
use crate::format::yaml::YamlReader;
use crate::format::{default_delimiter, detect_format, Format, FormatOptions, FormatReader};
use crate::value::Value;

pub struct DiffArgs<'a> {
    pub file1: &'a Path,
    pub file2: &'a Path,
    pub path: Option<&'a str>,
    pub quiet: bool,
}

/// diff 서브커맨드 실행. 차이가 있으면 true, 없으면 false 반환.
pub fn run(args: &DiffArgs) -> Result<bool> {
    let value1 = read_value_from_path(args.file1)?;
    let value2 = read_value_from_path(args.file2)?;

    // --path 옵션으로 중첩 데이터 접근
    let (v1, v2) = match args.path {
        Some(path_expr) => (
            resolve_path(&value1, path_expr)?,
            resolve_path(&value2, path_expr)?,
        ),
        None => (value1, value2),
    };

    if v1 == v2 {
        if !args.quiet {
            println!("No differences found.");
        }
        return Ok(false);
    }

    if args.quiet {
        return Ok(true);
    }

    // 차이 출력
    let diffs = compute_diff("", &v1, &v2);
    for entry in &diffs {
        print_diff_entry(entry);
    }

    Ok(true)
}

/// 파일 경로에서 Value를 읽는다
fn read_value_from_path(path: &Path) -> Result<Value> {
    let format = detect_format(path)?;
    let delimiter = default_delimiter(path);
    let options = FormatOptions {
        delimiter,
        ..Default::default()
    };

    if format == Format::Msgpack {
        let bytes = read_file_bytes(path)?;
        MsgpackReader.read_from_bytes(&bytes)
    } else {
        let content = read_file(path)?;
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
        Format::Markdown => bail!("Markdown is an output-only format and cannot be used as input"),
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

/// 두 Value 간 재귀적 비교. path_prefix는 현재까지의 경로 문자열.
fn compute_diff(path_prefix: &str, left: &Value, right: &Value) -> Vec<DiffEntry> {
    let mut entries = Vec::new();

    match (left, right) {
        // 두 Object 비교
        (Value::Object(map1), Value::Object(map2)) => {
            // map1에 있는 키들
            for (key, v1) in map1 {
                let child_path = if path_prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{path_prefix}.{key}")
                };

                match map2.get(key) {
                    Some(v2) => {
                        entries.extend(compute_diff(&child_path, v1, v2));
                    }
                    None => {
                        entries.extend(collect_removed(&child_path, v1));
                    }
                }
            }
            // map2에만 있는 키들 (added)
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
        // 두 Array 비교 (인덱스 기반)
        (Value::Array(arr1), Value::Array(arr2)) => {
            let max_len = arr1.len().max(arr2.len());
            for i in 0..max_len {
                let child_path = format!("{path_prefix}[{i}]");
                match (arr1.get(i), arr2.get(i)) {
                    (Some(v1), Some(v2)) => {
                        entries.extend(compute_diff(&child_path, v1, v2));
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
        }
        // 스칼라 값 비교
        _ => {
            if left == right {
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

/// diff 엔트리를 컬러로 출력
fn print_diff_entry(entry: &DiffEntry) {
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

    #[test]
    fn test_identical_values_no_diff() {
        let v = obj(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
        ]);
        let diffs = compute_diff("", &v, &v);
        assert!(diffs
            .iter()
            .all(|d| matches!(d, DiffEntry::Unchanged { .. })));
    }

    #[test]
    fn test_changed_scalar() {
        let v1 = obj(vec![("port", Value::Integer(5432))]);
        let v2 = obj(vec![("port", Value::Integer(3306))]);
        let diffs = compute_diff("", &v1, &v2);
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
        let diffs = compute_diff("", &v1, &v2);
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
        let diffs = compute_diff("", &v1, &v2);
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
        let diffs = compute_diff("", &v1, &v2);
        assert_eq!(diffs.len(), 2);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { path, .. } if path == "database.host"));
        assert!(matches!(&diffs[1], DiffEntry::Changed { path, .. } if path == "database.port"));
    }

    #[test]
    fn test_array_diff() {
        let v1 = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let v2 = Value::Array(vec![Value::Integer(1), Value::Integer(99)]);
        let diffs = compute_diff("items", &v1, &v2);
        assert_eq!(diffs.len(), 3);
        assert!(matches!(&diffs[0], DiffEntry::Unchanged { path, .. } if path == "items[0]"));
        assert!(matches!(&diffs[1], DiffEntry::Changed { path, .. } if path == "items[1]"));
        assert!(matches!(&diffs[2], DiffEntry::Removed { path, .. } if path == "items[2]"));
    }

    #[test]
    fn test_scalar_type_change() {
        let v1 = Value::String("hello".into());
        let v2 = Value::Integer(42);
        let diffs = compute_diff("field", &v1, &v2);
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
}
