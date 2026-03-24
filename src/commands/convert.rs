use std::fs;
use std::io::{self, IsTerminal, Read, Write as _};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::{
    read_file_bytes, read_file_with_encoding, read_sqlite_from_path, read_xlsx_from_bytes,
    EncodingOptions, ExcelOptions, SqliteOptions,
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

pub struct ConvertArgs<'a> {
    pub input: &'a [PathBuf],
    pub to: &'a str,
    pub from: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub outdir: Option<&'a Path>,
    pub delimiter: Option<char>,
    pub pretty: bool,
    pub compact: bool,
    pub no_header: bool,
    pub flow: bool,
    pub root_element: Option<String>,
    pub styled: bool,
    pub full_html: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

/// convert 서브커맨드 실행
pub fn run(args: &ConvertArgs) -> Result<()> {
    let target_format = Format::from_str(args.to)?;

    let write_delimiter = args
        .delimiter
        .or_else(|| default_delimiter_for_format(args.to));

    // Auto-detect pretty vs compact: if neither --pretty nor --compact is set,
    // use pretty when writing to a terminal, compact when piped.
    let (effective_pretty, effective_compact) = if args.pretty {
        (true, false)
    } else if args.compact {
        (false, true)
    } else if args.output.is_some() {
        // Writing to a file: default to pretty
        (true, false)
    } else {
        // Writing to stdout: detect terminal vs pipe
        let is_terminal = io::stdout().is_terminal();
        (is_terminal, !is_terminal)
    };

    let write_options = FormatOptions {
        delimiter: write_delimiter,
        no_header: args.no_header,
        pretty: effective_pretty,
        compact: effective_compact,
        flow_style: args.flow,
        root_element: args.root_element.clone(),
        styled: args.styled,
        full_html: args.full_html,
    };

    // stdin mode: no input files or explicit "-"
    let is_stdin =
        args.input.is_empty() || (args.input.len() == 1 && args.input[0] == Path::new("-"));
    if is_stdin {
        let value = if args.from == Some("msgpack") || args.from == Some("messagepack") {
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
            let read_delimiter = args
                .delimiter
                .or(sniffed_delimiter)
                .or_else(|| args.from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: read_delimiter,
                no_header: args.no_header,
                ..Default::default()
            };
            read_value(&buf, source_format, &read_options)?
        };

        write_output(&value, target_format, &write_options, args.output)?;
        return Ok(());
    }

    // Multiple files with --outdir
    if args.input.len() > 1 {
        let outdir = match args.outdir {
            Some(d) => d,
            None => bail!("--outdir is required when converting multiple files\n  Hint: specify an output directory, e.g. --outdir ./output"),
        };
        fs::create_dir_all(outdir)
            .with_context(|| format!("Failed to create directory {}", outdir.display()))?;

        for path in args.input {
            let source_format = match args.from {
                Some(f) => Format::from_str(f)?,
                None => detect_format(path)?,
            };

            let read_delimiter = args.delimiter.or_else(|| default_delimiter(path));
            let read_options = FormatOptions {
                delimiter: read_delimiter,
                no_header: args.no_header,
                ..Default::default()
            };

            let value = read_value_from_path(
                path,
                source_format,
                &read_options,
                &args.encoding_opts,
                &args.excel_opts,
                &args.sqlite_opts,
            )?;

            let out_name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                + "."
                + args.to;
            let out_path = outdir.join(out_name);
            write_output(&value, target_format, &write_options, Some(&out_path))?;
        }
        return Ok(());
    }

    // Single file
    let path = &args.input[0];
    let source_format = match args.from {
        Some(f) => Format::from_str(f)?,
        None => detect_format(path)?,
    };

    let read_delimiter = args.delimiter.or_else(|| default_delimiter(path));
    let read_options = FormatOptions {
        delimiter: read_delimiter,
        no_header: args.no_header,
        ..Default::default()
    };

    let value = read_value_from_path(
        path,
        source_format,
        &read_options,
        &args.encoding_opts,
        &args.excel_opts,
        &args.sqlite_opts,
    )?;

    let outdir_path = args.outdir.map(|d| {
        let name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
            + "."
            + args.to;
        d.join(name)
    });

    let out_path = args.output.or(outdir_path.as_deref());
    if let Some(out_path) = out_path {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {}", parent.display()))?;
            }
        }
    }
    write_output(&value, target_format, &write_options, out_path)?;

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

/// 파일 경로에서 Value를 읽는다 (바이너리 포맷 자동 처리)
fn read_value_from_path(
    path: &Path,
    format: Format,
    options: &FormatOptions,
    encoding_opts: &EncodingOptions,
    excel_opts: &ExcelOptions,
    sqlite_opts: &SqliteOptions,
) -> Result<Value> {
    if format == Format::Msgpack {
        let bytes = read_file_bytes(path)?;
        MsgpackReader.read_from_bytes(&bytes)
    } else if format == Format::Xlsx {
        let bytes = read_file_bytes(path)?;
        read_xlsx_from_bytes(&bytes, excel_opts)
    } else if format == Format::Sqlite {
        read_sqlite_from_path(path, sqlite_opts)
    } else {
        let content = read_file_with_encoding(path, encoding_opts)?;
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
        Format::Xlsx => {
            bail!("Excel files must be read as binary; use file path input instead of stdin")
        }
        Format::Sqlite => {
            bail!("SQLite files must be read from a file path, not from text input")
        }
        Format::Markdown => bail!("Markdown is an output-only format and cannot be used as input"),
        Format::Html => bail!("HTML is an output-only format and cannot be used as input"),
        Format::Table => bail!("Table is an output-only format and cannot be used as input"),
    }
}

/// Value를 출력한다 (바이너리 포맷 자동 처리)
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
    } else {
        let result = write_value(value, format, options)?;
        if let Some(out_path) = output {
            fs::write(out_path, &result)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            print!("{result}");
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
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
    }
}
