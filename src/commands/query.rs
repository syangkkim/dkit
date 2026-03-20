use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::{bail, Context, Result};

use crate::format::csv::CsvReader;
use crate::format::json::{JsonReader, JsonWriter};
use crate::format::toml::{TomlReader, TomlWriter};
use crate::format::yaml::{YamlReader, YamlWriter};
use crate::format::{detect_format, Format, FormatOptions, FormatReader, FormatWriter};
use crate::query::evaluator::evaluate_path;
use crate::query::parser::parse_query;
use crate::value::Value;

pub struct QueryArgs<'a> {
    pub input: &'a str,
    pub query: &'a str,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
}

/// query 서브커맨드 실행
pub fn run(args: &QueryArgs) -> Result<()> {
    // 입력 읽기
    let (content, source_format) = if args.input == "-" {
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => bail!("--from is required when reading from stdin"),
        };
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read from stdin")?;
        (buf, format)
    } else {
        let path = PathBuf::from(args.input);
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(&path)?,
        };
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        (content, format)
    };

    // 파싱
    let read_options = FormatOptions::default();
    let value = read_value(&content, source_format, &read_options)?;

    // 쿼리 파싱 및 실행
    let query = parse_query(args.query)?;
    let result = evaluate_path(&value, &query.path)?;

    // 출력 포맷 결정 (--to 지정 시 해당 포맷, 아니면 JSON 기본)
    let output_format = match args.to {
        Some(f) => Format::from_str(f)?,
        None => Format::Json,
    };

    let write_options = FormatOptions {
        pretty: true,
        ..Default::default()
    };
    let output = write_value(&result, output_format, &write_options)?;
    // 출력 끝에 줄바꿈이 없으면 추가
    if output.ends_with('\n') {
        print!("{output}");
    } else {
        println!("{output}");
    }

    Ok(())
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
    }
}

fn write_value(value: &Value, format: Format, options: &FormatOptions) -> Result<String> {
    match format {
        Format::Json => JsonWriter::new(options.clone()).write(value),
        Format::Csv => {
            // CSV 출력 시 배열 형태가 아닌 단일 값은 JSON으로 출력
            match value {
                Value::Array(_) => {
                    use crate::format::csv::CsvWriter;
                    CsvWriter::new(options.clone()).write(value)
                }
                _ => JsonWriter::new(options.clone()).write(value),
            }
        }
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
    }
}
