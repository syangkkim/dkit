use std::io::{self, Read};
use std::path::Path;

use anyhow::{bail, Context as _, Result};

use super::{
    list_sqlite_tables, list_xlsx_sheets, read_file_bytes, read_file_with_encoding,
    read_parquet_from_bytes, read_sqlite_from_path, read_xlsx_from_bytes, EncodingOptions,
    ExcelOptions, SqliteOptions,
};
use crate::output::table::{render_table, TableOptions};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::csv::CsvWriter;
use dkit_core::format::env::{EnvReader, EnvWriter};
use dkit_core::format::hcl::{HclReader, HclWriter};
use dkit_core::format::html::HtmlWriter;
use dkit_core::format::ini::{IniReader, IniWriter};
use dkit_core::format::json::{JsonReader, JsonWriter};
use dkit_core::format::jsonl::{JsonlReader, JsonlWriter};
use dkit_core::format::markdown::MarkdownWriter;
use dkit_core::format::msgpack::{MsgpackReader, MsgpackWriter};
use dkit_core::format::properties::{PropertiesReader, PropertiesWriter};
use dkit_core::format::toml::{TomlReader, TomlWriter};
use dkit_core::format::xml::{XmlReader, XmlWriter};
use dkit_core::format::yaml::{YamlReader, YamlWriter};
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader, FormatWriter,
};
use dkit_core::value::Value;

pub struct ViewArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
    pub format: Option<&'a str>,
    pub path: Option<&'a str>,
    pub limit: Option<usize>,
    pub columns: Option<Vec<String>>,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub max_width: Option<u16>,
    pub hide_header: bool,
    pub row_numbers: bool,
    pub border: &'a str,
    pub color: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub list_sheets: bool,
    pub sqlite_opts: SqliteOptions,
    pub list_tables: bool,
    pub data_filter: super::DataFilterOptions,
}

/// view 서브커맨드 실행
pub fn run(args: &ViewArgs) -> Result<()> {
    // --list-sheets: Excel 시트 목록 출력
    if args.list_sheets {
        if args.input == "-" {
            bail!("--list-sheets requires a file path, not stdin");
        }
        let bytes = read_file_bytes(Path::new(args.input))?;
        let sheets = list_xlsx_sheets(&bytes)?;
        for (i, name) in sheets.iter().enumerate() {
            println!("{}: {}", i, name);
        }
        return Ok(());
    }

    // --list-tables: SQLite 테이블 목록 출력
    if args.list_tables {
        if args.input == "-" {
            bail!("--list-tables requires a file path, not stdin");
        }
        let tables = list_sqlite_tables(Path::new(args.input))?;
        for (i, name) in tables.iter().enumerate() {
            println!("{}: {}", i, name);
        }
        return Ok(());
    }

    let value = if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            MsgpackReader.read_from_bytes(&buf)?
        } else {
            let buf = read_stdin_with_encoding(&args.encoding_opts)?;
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

    // 데이터 필터/정렬 적용
    let target = super::apply_data_filters(target, &args.data_filter)?;

    // 출력 포맷 결정: --format 옵션 또는 기본 table
    let output_format = match args.format {
        Some(f) => Format::from_str(f)?,
        None => Format::Table,
    };

    if output_format == Format::Table {
        let table_opts = TableOptions {
            limit: args.limit,
            columns: args.columns.as_deref(),
            max_width: args.max_width,
            hide_header: args.hide_header,
            row_numbers: args.row_numbers,
            border: args.border,
            color: args.color,
        };
        let output = render_table(&target, &table_opts);
        println!("{output}");
    } else if output_format == Format::Msgpack {
        let bytes = MsgpackWriter.write_bytes(&target)?;
        use std::io::Write as _;
        std::io::stdout()
            .write_all(&bytes)
            .context("Failed to write to stdout")?;
    } else {
        let write_options = FormatOptions {
            pretty: true,
            ..Default::default()
        };
        let output = write_value(&target, output_format, &write_options)?;
        if output.ends_with('\n') {
            print!("{output}");
        } else {
            println!("{output}");
        }
    }

    Ok(())
}

/// stdin에서 인코딩을 고려하여 문자열을 읽는다.
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
            let bytes = read_file_bytes(Path::new(args.input))?;
            MsgpackReader.read_from_bytes(&bytes)
        }
    } else if format == Format::Xlsx {
        if args.input == "-" {
            bail!("Excel files cannot be read from stdin; provide a file path");
        }
        let bytes = read_file_bytes(Path::new(args.input))?;
        read_xlsx_from_bytes(&bytes, &args.excel_opts)
    } else if format == Format::Sqlite {
        if args.input == "-" {
            bail!("SQLite files cannot be read from stdin; provide a file path");
        }
        read_sqlite_from_path(Path::new(args.input), &args.sqlite_opts)
    } else if format == Format::Parquet {
        if args.input == "-" {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            read_parquet_from_bytes(&buf)
        } else {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_parquet_from_bytes(&bytes)
        }
    } else {
        let content = if args.input == "-" {
            read_stdin_with_encoding(&args.encoding_opts)?
        } else {
            read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?
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
        Format::Env => EnvReader.read(content),
        Format::Ini => IniReader.read(content),
        Format::Properties => PropertiesReader.read(content),
        Format::Hcl => HclReader.read(content),
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
        _ => bail!("Unsupported input format: {format}"),
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
        Format::Msgpack => MsgpackWriter.write(value),
        Format::Xlsx => bail!("Excel is an input-only format and cannot be used as output"),
        Format::Sqlite => bail!("SQLite is an input-only format and cannot be used as output"),
        Format::Parquet => bail!("Parquet is an input-only format and cannot be used as output"),
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => bail!("Table format is handled separately"),
        _ => bail!("Unsupported output format: {format}"),
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
