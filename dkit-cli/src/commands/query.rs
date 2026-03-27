use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::env::{EnvReader, EnvWriter};
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
use dkit_core::query::evaluator::evaluate_path;
use dkit_core::query::filter::apply_operations;
use dkit_core::query::parser::parse_query;
use dkit_core::value::Value;

pub struct QueryArgs<'a> {
    pub input: &'a str,
    pub query: &'a str,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

/// query 서브커맨드 실행
pub fn run(args: &QueryArgs) -> Result<()> {
    // 입력 읽기 (바이너리 포맷 자동 처리)
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
            let auto_delimiter =
                sniffed_delimiter.or_else(|| args.from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: auto_delimiter,
                ..Default::default()
            };
            read_value(&buf, source_format, &read_options)?
        }
    } else {
        let source_format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(&PathBuf::from(args.input))?,
        };
        if source_format == Format::Msgpack {
            let bytes = read_file_bytes(Path::new(args.input))?;
            MsgpackReader.read_from_bytes(&bytes)?
        } else if source_format == Format::Xlsx {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_xlsx_from_bytes(&bytes, &args.excel_opts)?
        } else if source_format == Format::Sqlite {
            read_sqlite_from_path(Path::new(args.input), &args.sqlite_opts)?
        } else if source_format == Format::Parquet {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_parquet_from_bytes(&bytes)?
        } else {
            let content = read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?;
            let auto_delimiter = default_delimiter(Path::new(args.input));
            let read_options = FormatOptions {
                delimiter: auto_delimiter,
                ..Default::default()
            };
            read_value(&content, source_format, &read_options)?
        }
    };

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

    // 출력
    if output_format == Format::Msgpack {
        let bytes = MsgpackWriter.write_bytes(&result)?;
        match args.output {
            Some(path) => {
                fs::write(path, &bytes)
                    .with_context(|| format!("Failed to write to {}", path.display()))?;
            }
            None => {
                use std::io::Write as _;
                std::io::stdout()
                    .write_all(&bytes)
                    .context("Failed to write to stdout")?;
            }
        }
    } else {
        // Auto-detect: pretty when writing to terminal or file, compact when piped
        let effective_pretty = match args.output {
            Some(_) => true,
            None => io::stdout().is_terminal(),
        };
        let write_options = FormatOptions {
            pretty: effective_pretty,
            compact: !effective_pretty,
            ..Default::default()
        };
        let output = write_value(&result, output_format, &write_options)?;

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
        Format::Csv => {
            // CSV 출력 시 배열 형태가 아닌 단일 값은 JSON으로 출력
            match value {
                Value::Array(_) => {
                    use dkit_core::format::csv::CsvWriter;
                    CsvWriter::new(options.clone()).write(value)
                }
                _ => JsonWriter::new(options.clone()).write(value),
            }
        }
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty, options.root_element.clone()).write(value),
        Format::Env => EnvWriter.write(value),
        Format::Ini => IniWriter.write(value),
        Format::Properties => PropertiesWriter.write(value),
        Format::Msgpack => MsgpackWriter.write(value),
        Format::Xlsx => bail!("Excel is an input-only format and cannot be used as output"),
        Format::Sqlite => bail!("SQLite is an input-only format and cannot be used as output"),
        Format::Parquet => bail!("Parquet is an input-only format and cannot be used as output"),
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
        _ => bail!("Unsupported output format: {format}"),
    }
}
