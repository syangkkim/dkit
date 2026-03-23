use std::io::{self, Read};
use std::path::Path;

use anyhow::{bail, Context as _, Result};

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
use crate::output::table::render_table;
use crate::value::Value;

pub struct ViewArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
    pub path: Option<&'a str>,
    pub limit: Option<usize>,
    pub columns: Option<Vec<String>>,
    pub delimiter: Option<char>,
    pub no_header: bool,
}

/// view 서브커맨드 실행
pub fn run(args: &ViewArgs) -> Result<()> {
    let value = if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            MsgpackReader.read_from_bytes(&buf)?
        } else {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read from stdin")?;
            let (source_format, sniffed_delimiter) = match args.from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&buf)?,
            };
            let auto_delimiter = args
                .delimiter
                .or(sniffed_delimiter)
                .or_else(|| args.from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: auto_delimiter,
                no_header: args.no_header,
                ..Default::default()
            };
            read_value(&buf, source_format, &read_options)?
        }
    } else {
        let source_format = determine_format(args)?;
        let auto_delimiter = default_delimiter(Path::new(args.input));
        let read_options = FormatOptions {
            delimiter: args.delimiter.or(auto_delimiter),
            no_header: args.no_header,
            ..Default::default()
        };
        read_input_value(args, source_format, &read_options)?
    };

    // --path 옵션으로 중첩 데이터 접근
    let target = match args.path {
        Some(path_expr) => resolve_path(&value, path_expr)?,
        None => value,
    };

    let output = render_table(&target, args.limit, args.columns.as_deref());

    println!("{output}");
    Ok(())
}

/// 입력 포맷을 결정한다
fn determine_format(args: &ViewArgs) -> Result<Format> {
    if args.input == "-" {
        match args.from {
            Some(f) => Format::from_str(f),
            None => bail!("--from is required when reading from stdin\n  Hint: specify the input format, e.g. --from json"),
        }
    } else {
        match args.from {
            Some(f) => Format::from_str(f),
            None => detect_format(Path::new(args.input)),
        }
    }.map_err(Into::into)
}

/// 입력 소스에서 Value를 읽어온다 (바이너리 포맷 자동 처리)
fn read_input_value(args: &ViewArgs, format: Format, options: &FormatOptions) -> Result<Value> {
    if format == Format::Msgpack {
        if args.input == "-" {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            MsgpackReader.read_from_bytes(&buf)
        } else {
            let bytes = super::read_file_bytes(Path::new(args.input))?;
            MsgpackReader.read_from_bytes(&bytes)
        }
    } else {
        let content = if args.input == "-" {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read from stdin")?;
            buf
        } else {
            super::read_file(Path::new(args.input))?
        };
        read_value(&content, format, options)
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
    }
}

/// 간단한 경로 접근: ".field.subfield" 또는 ".array[0]" 형태
fn resolve_path(value: &Value, path_expr: &str) -> Result<Value> {
    let path_expr = path_expr.trim();
    if path_expr.is_empty() || path_expr == "." {
        return Ok(value.clone());
    }

    // 선행 dot 제거
    let path_expr = path_expr.strip_prefix('.').unwrap_or(path_expr);

    let mut current = value.clone();

    for segment in split_path_segments(path_expr) {
        // 배열 인덱싱 확인: "field[0]" 또는 "[0]"
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

/// 경로를 dot으로 분할 (대괄호 내부의 dot은 무시)
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

/// "field[N]" → (field, N) 파싱. 음수 인덱스 지원.
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
    fn test_resolve_path_root() {
        let v = Value::Integer(42);
        assert_eq!(resolve_path(&v, ".").unwrap(), Value::Integer(42));
        assert_eq!(resolve_path(&v, "").unwrap(), Value::Integer(42));
    }

    #[test]
    fn test_resolve_path_field() {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String("Alice".to_string()));
        let v = Value::Object(m);
        assert_eq!(
            resolve_path(&v, ".name").unwrap(),
            Value::String("Alice".to_string())
        );
    }

    #[test]
    fn test_resolve_path_nested() {
        let mut inner = IndexMap::new();
        inner.insert("host".to_string(), Value::String("localhost".to_string()));
        let mut outer = IndexMap::new();
        outer.insert("db".to_string(), Value::Object(inner));
        let v = Value::Object(outer);
        assert_eq!(
            resolve_path(&v, ".db.host").unwrap(),
            Value::String("localhost".to_string())
        );
    }

    #[test]
    fn test_resolve_path_array_index() {
        let mut m = IndexMap::new();
        m.insert(
            "items".to_string(),
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
            ]),
        );
        let v = Value::Object(m);
        assert_eq!(
            resolve_path(&v, ".items[0]").unwrap(),
            Value::String("a".to_string())
        );
        assert_eq!(
            resolve_path(&v, ".items[-1]").unwrap(),
            Value::String("b".to_string())
        );
    }

    #[test]
    fn test_resolve_path_not_found() {
        let mut m = IndexMap::new();
        m.insert("x".to_string(), Value::Integer(1));
        let v = Value::Object(m);
        assert!(resolve_path(&v, ".y").is_err());
    }

    #[test]
    fn test_split_path_segments() {
        let segs = split_path_segments("users[0].name");
        assert_eq!(segs, vec!["users[0]", "name"]);
    }

    #[test]
    fn test_split_path_segments_nested_brackets() {
        let segs = split_path_segments("a.b[1].c");
        assert_eq!(segs, vec!["a", "b[1]", "c"]);
    }
}
