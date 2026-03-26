use std::io::{self, Read};
use std::path::Path;

use anyhow::{bail, Result};
use colored::Colorize;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::env::EnvReader;
use dkit_core::format::json::JsonReader;
use dkit_core::format::jsonl::JsonlReader;
use dkit_core::format::msgpack::MsgpackReader;
use dkit_core::format::toml::TomlReader;
use dkit_core::format::xml::XmlReader;
use dkit_core::format::yaml::YamlReader;
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader,
};
use dkit_core::value::Value;

pub struct ValidateArgs<'a> {
    /// 검증할 데이터 파일 경로 (또는 '-' for stdin)
    pub input: &'a str,
    /// JSON Schema 파일 경로
    pub schema: &'a Path,
    /// 입력 포맷 (자동 감지 대신 명시)
    pub from: Option<&'a str>,
    /// 상세 에러 숨기기 (결과만 출력)
    pub quiet: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

/// validate 서브커맨드 실행.
///
/// 반환값: `true` = 검증 실패 (호출자가 exit code 1로 종료해야 함)
pub fn run(args: &ValidateArgs) -> Result<bool> {
    // 1. 스키마 파일 로드 및 파싱
    let schema_content = std::fs::read_to_string(args.schema).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read schema file '{}': {e}",
            args.schema.display()
        )
    })?;
    let schema_json: serde_json::Value = serde_json::from_str(&schema_content).map_err(|e| {
        anyhow::anyhow!(
            "Failed to parse schema file '{}': {e}\n  Hint: schema must be a valid JSON file",
            args.schema.display()
        )
    })?;

    // 2. 입력 데이터 로드
    let value = load_input(args)?;

    // 3. dkit_core::value::Value → serde_json::Value 변환
    let instance = value_to_json(value);

    // 4. JSON Schema 컴파일
    let compiled = jsonschema::JSONSchema::compile(&schema_json).map_err(|e| {
        anyhow::anyhow!("Failed to compile schema '{}': {e}", args.schema.display())
    })?;

    // 5. 검증 실행 (모든 에러 수집)
    let errors: Vec<_> = match compiled.validate(&instance) {
        Ok(_) => vec![],
        Err(errors) => errors
            .map(|e| {
                let path = e.instance_path.to_string();
                let path = if path.is_empty() {
                    "root".to_string()
                } else {
                    path
                };
                (path, e.to_string())
            })
            .collect(),
    };

    if errors.is_empty() {
        if args.quiet {
            println!("valid");
        } else {
            println!("{} Data is valid", "✓".green().bold());
        }
        Ok(false)
    } else {
        if args.quiet {
            println!("invalid: {} error(s)", errors.len());
        } else {
            println!(
                "{} Validation failed: {} error(s)",
                "✗".red().bold(),
                errors.len()
            );
            for (path, msg) in &errors {
                println!("  {} at {}: {}", "error:".red(), path.yellow(), msg);
            }
        }
        Ok(true)
    }
}

/// dkit_core::value::Value를 serde_json::Value로 변환한다.
///
/// serde_json::to_value(&Value)는 enum variant를 tagged로 직렬화하므로
/// 직접 변환해야 한다.
fn value_to_json(v: Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::Integer(n) => serde_json::Value::Number(n.into()),
        Value::Float(f) => serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::String(s) => serde_json::Value::String(s),
        Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(value_to_json).collect()),
        Value::Object(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, v)| (k, value_to_json(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}

fn load_input(args: &ValidateArgs) -> Result<Value> {
    if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
            MsgpackReader.read_from_bytes(&buf)
        } else {
            let content = read_stdin_text(&args.encoding_opts)?;
            let (fmt, sniffed_delim) = match args.from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&content)?,
            };
            let delimiter =
                sniffed_delim.or_else(|| args.from.and_then(default_delimiter_for_format));
            let options = FormatOptions {
                delimiter,
                ..Default::default()
            };
            read_value(&content, fmt, &options)
        }
    } else {
        let source_format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(Path::new(args.input))?,
        };
        match source_format {
            Format::Msgpack => {
                let bytes = read_file_bytes(Path::new(args.input))?;
                MsgpackReader.read_from_bytes(&bytes)
            }
            Format::Xlsx => {
                let bytes = read_file_bytes(Path::new(args.input))?;
                read_xlsx_from_bytes(&bytes, &args.excel_opts)
            }
            Format::Sqlite => read_sqlite_from_path(Path::new(args.input), &args.sqlite_opts),
            Format::Parquet => {
                let bytes = read_file_bytes(Path::new(args.input))?;
                read_parquet_from_bytes(&bytes)
            }
            _ => {
                let content = read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?;
                let delimiter = default_delimiter(Path::new(args.input));
                let options = FormatOptions {
                    delimiter,
                    ..Default::default()
                };
                read_value(&content, source_format, &options)
            }
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
        Format::Env => EnvReader.read(content),
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

fn read_stdin_text(opts: &EncodingOptions) -> Result<String> {
    if opts.encoding.is_some() || opts.detect_encoding {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
        super::decode_bytes(&buf, opts)
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempPath};

    /// 임시 파일에 내용을 쓰고 파일 핸들을 닫는다.
    ///
    /// Windows에서는 파일 핸들이 열려 있는 동안 다른 프로세스가 파일을 여는데
    /// 제한이 생길 수 있다. into_temp_path()로 핸들을 닫으면서도
    /// 파일은 디스크에 유지한다.
    fn write_temp(content: &str) -> TempPath {
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        f.into_temp_path()
    }

    /// 주어진 확장자를 가진 임시 파일에 내용을 쓰고 파일 핸들을 닫는다.
    fn write_temp_with_suffix(content: &[u8], suffix: &str) -> TempPath {
        let mut f = tempfile::Builder::new().suffix(suffix).tempfile().unwrap();
        f.write_all(content).unwrap();
        f.flush().unwrap();
        f.into_temp_path()
    }

    #[test]
    fn test_valid_json_against_schema() {
        let schema_file = write_temp(
            r#"{
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "age": {"type": "integer"}
                },
                "required": ["name", "age"]
            }"#,
        );
        let json_file =
            write_temp_with_suffix(r#"{"name": "Alice", "age": 30}"#.as_bytes(), ".json");

        let args = ValidateArgs {
            input: json_file.to_str().unwrap(),
            schema: &schema_file,
            from: None,
            quiet: false,
            encoding_opts: EncodingOptions::default(),
            excel_opts: ExcelOptions::default(),
            sqlite_opts: SqliteOptions::default(),
        };
        let result = run(&args).unwrap();
        assert!(!result, "valid data should return false (no error)");
    }

    #[test]
    fn test_invalid_json_against_schema() {
        let schema_file = write_temp(
            r#"{
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "age": {"type": "integer"}
                },
                "required": ["name", "age"]
            }"#,
        );
        // age is a string, not integer → validation error
        let json_file =
            write_temp_with_suffix(r#"{"name": "Alice", "age": "thirty"}"#.as_bytes(), ".json");

        let args = ValidateArgs {
            input: json_file.to_str().unwrap(),
            schema: &schema_file,
            from: None,
            quiet: true,
            encoding_opts: EncodingOptions::default(),
            excel_opts: ExcelOptions::default(),
            sqlite_opts: SqliteOptions::default(),
        };
        let result = run(&args).unwrap();
        assert!(result, "invalid data should return true (has errors)");
    }

    #[test]
    fn test_missing_required_field() {
        let schema_file = write_temp(r#"{"type": "object", "required": ["name", "age"]}"#);
        let json_file = write_temp_with_suffix(r#"{"name": "Alice"}"#.as_bytes(), ".json");

        let args = ValidateArgs {
            input: json_file.to_str().unwrap(),
            schema: &schema_file,
            from: None,
            quiet: true,
            encoding_opts: EncodingOptions::default(),
            excel_opts: ExcelOptions::default(),
            sqlite_opts: SqliteOptions::default(),
        };
        let result = run(&args).unwrap();
        assert!(result, "missing required field should fail validation");
    }

    #[test]
    fn test_yaml_input_valid() {
        let schema_file =
            write_temp(r#"{"type": "object", "properties": {"host": {"type": "string"}}}"#);
        let yaml_file = write_temp_with_suffix(b"host: localhost\n", ".yaml");

        let args = ValidateArgs {
            input: yaml_file.to_str().unwrap(),
            schema: &schema_file,
            from: None,
            quiet: true,
            encoding_opts: EncodingOptions::default(),
            excel_opts: ExcelOptions::default(),
            sqlite_opts: SqliteOptions::default(),
        };
        let result = run(&args).unwrap();
        assert!(!result, "valid YAML should pass validation");
    }

    #[test]
    fn test_invalid_schema_file() {
        let schema_file = write_temp("not valid json");
        let json_file = write_temp_with_suffix(b"{}", ".json");

        let args = ValidateArgs {
            input: json_file.to_str().unwrap(),
            schema: &schema_file,
            from: None,
            quiet: true,
            encoding_opts: EncodingOptions::default(),
            excel_opts: ExcelOptions::default(),
            sqlite_opts: SqliteOptions::default(),
        };
        assert!(run(&args).is_err(), "invalid schema JSON should error");
    }
}
