use std::io::{self, Read};
use std::path::Path;

use crate::format::csv::CsvReader;
use crate::format::json::JsonReader;
use crate::format::jsonl::JsonlReader;
use crate::format::msgpack::MsgpackReader;
use crate::format::toml::TomlReader;
use crate::format::xml::XmlReader;
use crate::format::yaml::YamlReader;
use crate::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader,
};
use crate::value::Value;
use anyhow::{bail, Context, Result};

pub struct StatsArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
    pub path: Option<&'a str>,
    pub column: Option<&'a str>,
    pub delimiter: Option<char>,
    pub no_header: bool,
}

pub fn run(args: &StatsArgs) -> Result<()> {
    let (value, _source_format) = read_input_as_value(args)?;

    // --path 옵션으로 중첩 데이터 접근
    let target = match args.path {
        Some(path_expr) => resolve_path(&value, path_expr)?,
        None => value,
    };

    if let Some(col_name) = args.column {
        print_column_stats(&target, col_name)?;
    } else {
        print_overall_stats(&target)?;
    }

    Ok(())
}

/// 전체 통계 출력
fn print_overall_stats(value: &Value) -> Result<()> {
    match value {
        Value::Array(arr) => {
            let rows = arr.len();
            // 배열의 첫 번째 object에서 컬럼 목록 추출
            let columns = collect_columns(arr);
            println!("rows: {}", format_number(rows as f64));
            if !columns.is_empty() {
                println!("columns: {} ({})", columns.len(), columns.join(", "));
            }

            // 각 컬럼별 통계 출력
            if !columns.is_empty() {
                for col in &columns {
                    println!();
                    println!("--- {} ---", col);
                    let values = extract_column_values(arr, col);
                    print_values_stats(&values);
                }
            }
        }
        Value::Object(obj) => {
            println!("rows: 1");
            let columns: Vec<&String> = obj.keys().collect();
            println!(
                "columns: {} ({})",
                columns.len(),
                columns
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        _ => {
            println!("type: {}", value_type_name(value));
            println!("value: {}", value);
        }
    }
    Ok(())
}

/// 특정 컬럼의 통계 출력
fn print_column_stats(value: &Value, col_name: &str) -> Result<()> {
    let arr = match value {
        Value::Array(arr) => arr,
        _ => bail!("--column requires array data (rows of objects)"),
    };

    let columns = collect_columns(arr);
    if !columns.contains(&col_name.to_string()) {
        bail!(
            "Column '{}' not found\n  Available columns: {}",
            col_name,
            columns.join(", ")
        );
    }

    let values = extract_column_values(arr, col_name);
    print_values_stats(&values);
    Ok(())
}

/// 값 리스트에 대한 통계 출력
fn print_values_stats(values: &[Value]) {
    let numeric_values = extract_numeric_values(values);
    let non_null_count = values.iter().filter(|v| !v.is_null()).count();

    if numeric_values.len() == non_null_count && !numeric_values.is_empty() {
        // 숫자형 컬럼
        println!("type: numeric");
        println!("count: {}", format_number(values.len() as f64));
        let sum: f64 = numeric_values.iter().sum();
        let avg = sum / numeric_values.len() as f64;
        let min = numeric_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = numeric_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let median = compute_median(&numeric_values);

        println!("sum: {}", format_number(sum));
        println!("avg: {}", format_decimal(avg));
        println!("min: {}", format_number(min));
        println!("max: {}", format_number(max));
        println!("median: {}", format_number(median));
    } else {
        // 문자열형 컬럼
        println!("type: string");
        println!("count: {}", format_number(values.len() as f64));
        let null_count = values.iter().filter(|v| v.is_null()).count();
        if null_count > 0 {
            println!("null: {}", format_number(null_count as f64));
        }
        let unique = count_unique(values);
        println!("unique: {}", format_number(unique as f64));
    }
}

/// 배열의 object들에서 컬럼 이름 수집 (순서 유지)
fn collect_columns(arr: &[Value]) -> Vec<String> {
    let mut columns = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for item in arr {
        if let Value::Object(obj) = item {
            for key in obj.keys() {
                if seen.insert(key.clone()) {
                    columns.push(key.clone());
                }
            }
        }
    }
    columns
}

/// 특정 컬럼의 값들을 추출
fn extract_column_values(arr: &[Value], col: &str) -> Vec<Value> {
    arr.iter()
        .map(|item| {
            if let Value::Object(obj) = item {
                obj.get(col).cloned().unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        })
        .collect()
}

/// 숫자 값만 추출
fn extract_numeric_values(values: &[Value]) -> Vec<f64> {
    values
        .iter()
        .filter(|v| !v.is_null())
        .filter_map(|v| v.as_f64())
        .collect()
}

/// 중앙값 계산
fn compute_median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted.len();
    if len.is_multiple_of(2) {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    } else {
        sorted[len / 2]
    }
}

/// 고유 값 개수
fn count_unique(values: &[Value]) -> usize {
    let strs: std::collections::HashSet<String> = values.iter().map(|v| format!("{v}")).collect();
    strs.len()
}

/// Value 타입 이름
fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// 숫자 포맷 (천 단위 구분자)
fn format_number(n: f64) -> String {
    if n.fract() != 0.0 {
        return format_decimal(n);
    }
    let n = n as i64;
    if n < 0 {
        return format!("-{}", format_number((-n) as f64));
    }
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

/// 소수점 포맷
fn format_decimal(n: f64) -> String {
    let formatted = format!("{:.2}", n);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = format_number(parts[0].parse::<f64>().unwrap_or(0.0));
    format!("{}.{}", integer_part, parts[1])
}

fn read_input(args: &StatsArgs) -> Result<(String, Format)> {
    let path = Path::new(args.input);
    let format = match args.from {
        Some(f) => Format::from_str(f)?,
        None => detect_format(path)?,
    };
    let content = super::read_file(path)?;
    Ok((content, format))
}

/// MessagePack 바이너리 입력을 처리하여 Value를 반환
fn read_input_as_value(args: &StatsArgs) -> Result<(Value, Format)> {
    if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            let value = MsgpackReader.read_from_bytes(&buf)?;
            Ok((value, Format::Msgpack))
        } else {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read from stdin")?;
            let (format, sniffed_delimiter) = match args.from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&buf)?,
            };
            let auto_delimiter =
                sniffed_delimiter.or_else(|| args.from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: args.delimiter.or(auto_delimiter),
                no_header: args.no_header,
                ..Default::default()
            };
            let value = read_value(&buf, format, &read_options)?;
            Ok((value, format))
        }
    } else {
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(Path::new(args.input))?,
        };
        if format == Format::Msgpack {
            let bytes = super::read_file_bytes(Path::new(args.input))?;
            let value = MsgpackReader.read_from_bytes(&bytes)?;
            Ok((value, format))
        } else {
            let (content, format) = read_input(args)?;
            let auto_delimiter = default_delimiter(Path::new(args.input));
            let read_options = FormatOptions {
                delimiter: args.delimiter.or(auto_delimiter),
                no_header: args.no_header,
                ..Default::default()
            };
            let value = read_value(&content, format, &read_options)?;
            Ok((value, format))
        }
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

/// 간단한 경로 접근: ".field.subfield" 또는 ".array[0]" 형태
fn resolve_path(value: &Value, path_expr: &str) -> Result<Value> {
    let path_expr = path_expr.trim();
    if path_expr.is_empty() || path_expr == "." {
        return Ok(value.clone());
    }

    let path_expr = path_expr.strip_prefix('.').unwrap_or(path_expr);
    let mut current = value.clone();

    for segment in split_path_segments(path_expr) {
        if let Some((field, idx)) = parse_index_segment(&segment) {
            if !field.is_empty() {
                current = access_field(&current, &field)?;
            }
            current = access_index(&current, idx)?;
        } else {
            current = access_field(&current, &segment)?;
        }
    }

    Ok(current)
}

fn split_path_segments(path: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut in_bracket = false;

    for ch in path.chars() {
        match ch {
            '[' => {
                in_bracket = true;
                current.push(ch);
            }
            ']' => {
                in_bracket = false;
                current.push(ch);
            }
            '.' if !in_bracket => {
                if !current.is_empty() {
                    segments.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }

    segments
}

fn parse_index_segment(segment: &str) -> Option<(String, i64)> {
    let bracket_start = segment.find('[')?;
    let bracket_end = segment.find(']')?;
    let field = segment[..bracket_start].to_string();
    let idx_str = &segment[bracket_start + 1..bracket_end];
    let idx: i64 = idx_str.parse().ok()?;
    Some((field, idx))
}

fn access_field(value: &Value, field: &str) -> Result<Value> {
    match value {
        Value::Object(obj) => obj
            .get(field)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", field)),
        _ => bail!("Cannot access field '{}' on non-object value", field),
    }
}

fn access_index(value: &Value, idx: i64) -> Result<Value> {
    match value {
        Value::Array(arr) => {
            let actual_idx = if idx < 0 {
                (arr.len() as i64 + idx) as usize
            } else {
                idx as usize
            };
            arr.get(actual_idx).cloned().ok_or_else(|| {
                anyhow::anyhow!("Index {} out of bounds (length {})", idx, arr.len())
            })
        }
        _ => bail!("Cannot index into non-array value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(42.0), "42");
        assert_eq!(format_number(1234.0), "1,234");
        assert_eq!(format_number(1234567.0), "1,234,567");
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(3.14), "3.14");
        assert_eq!(format_decimal(37017.34), "37,017.34");
    }

    #[test]
    fn test_compute_median_odd() {
        assert_eq!(compute_median(&[1.0, 3.0, 5.0]), 3.0);
    }

    #[test]
    fn test_compute_median_even() {
        assert_eq!(compute_median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }

    #[test]
    fn test_compute_median_single() {
        assert_eq!(compute_median(&[42.0]), 42.0);
    }

    #[test]
    fn test_compute_median_empty() {
        assert_eq!(compute_median(&[]), 0.0);
    }

    #[test]
    fn test_collect_columns() {
        let mut obj1 = IndexMap::new();
        obj1.insert("name".to_string(), Value::String("Alice".to_string()));
        obj1.insert("age".to_string(), Value::Integer(30));
        let mut obj2 = IndexMap::new();
        obj2.insert("name".to_string(), Value::String("Bob".to_string()));
        obj2.insert("age".to_string(), Value::Integer(25));

        let arr = vec![Value::Object(obj1), Value::Object(obj2)];
        assert_eq!(collect_columns(&arr), vec!["name", "age"]);
    }

    #[test]
    fn test_extract_numeric_values() {
        let values = vec![
            Value::Integer(10),
            Value::Float(20.5),
            Value::Null,
            Value::Integer(30),
        ];
        let nums = extract_numeric_values(&values);
        assert_eq!(nums, vec![10.0, 20.5, 30.0]);
    }

    #[test]
    fn test_value_type_name() {
        assert_eq!(value_type_name(&Value::Null), "null");
        assert_eq!(value_type_name(&Value::Bool(true)), "boolean");
        assert_eq!(value_type_name(&Value::Integer(1)), "integer");
        assert_eq!(value_type_name(&Value::Float(1.0)), "float");
        assert_eq!(value_type_name(&Value::String("s".into())), "string");
    }

    #[test]
    fn test_count_unique() {
        let values = vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("a".to_string()),
            Value::Null,
        ];
        assert_eq!(count_unique(&values), 3); // "a", "b", "null"
    }
}
