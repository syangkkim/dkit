use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::format::csv::CsvReader;
use crate::format::json::{JsonReader, JsonWriter};
use crate::format::toml::{TomlReader, TomlWriter};
use crate::format::xml::{XmlReader, XmlWriter};
use crate::format::yaml::{YamlReader, YamlWriter};
use crate::format::{detect_format, Format, FormatOptions, FormatReader, FormatWriter};
use crate::query::evaluator::evaluate_path;
use crate::query::filter::apply_operations;
use crate::query::parser::parse_query;
use crate::value::Value;

pub struct QueryArgs<'a> {
    pub input: &'a str,
    pub query: &'a str,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub output: Option<&'a Path>,
}

/// query 서브커맨드 실행
pub fn run(args: &QueryArgs) -> Result<()> {
    // 입력 읽기
    let (content, source_format) = if args.input == "-" {
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => bail!("--from is required when reading from stdin\n  Hint: specify the input format, e.g. --from json"),
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
        let content = super::read_file(&path)?;
        (content, format)
    };

    // 파싱
    let read_options = FormatOptions::default();
    let value = read_value(&content, source_format, &read_options)?;

    // 쿼리 파싱 및 실행
    let query = parse_query(args.query)?;
    let path_result = evaluate_path(&value, &query.path)?;
    let result = apply_operations(path_result, &query.operations)?;

    // 출력 포맷 결정: -o 파일 확장자 → --to → 기본 JSON
    let output_format = match args.to {
        Some(f) => Format::from_str(f)?,
        None => match args.output {
            Some(p) => detect_format(p).unwrap_or(Format::Json),
            None => Format::Json,
        },
    };

    let write_options = FormatOptions {
        pretty: true,
        ..Default::default()
    };
    let output = write_value(&result, output_format, &write_options)?;

    // 출력: -o 지정 시 파일에 쓰기, 아니면 stdout
    match args.output {
        Some(path) => {
            let content = if output.ends_with('\n') {
                output
            } else {
                format!("{output}\n")
            };
            fs::write(path, &content)
                .with_context(|| format!("Failed to write to {}", path.display()))?;
        }
        None => {
            if output.ends_with('\n') {
                print!("{output}");
            } else {
                println!("{output}");
            }
        }
    }

    Ok(())
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
        Format::Xml => XmlReader.read(content),
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
        Format::Xml => XmlWriter::new(options.pretty).write(value),
    }
}
