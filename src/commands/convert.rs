use std::fs;
use std::io::{self, Read, Write as _};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::{read_file, read_file_bytes};
use crate::format::csv::{CsvReader, CsvWriter};
use crate::format::json::{JsonReader, JsonWriter};
use crate::format::msgpack::{MsgpackReader, MsgpackWriter};
use crate::format::toml::{TomlReader, TomlWriter};
use crate::format::xml::{XmlReader, XmlWriter};
use crate::format::yaml::{YamlReader, YamlWriter};
use crate::format::{
    default_delimiter, default_delimiter_for_format, detect_format, Format, FormatOptions,
    FormatReader, FormatWriter,
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
}

/// convert 서브커맨드 실행
pub fn run(args: &ConvertArgs) -> Result<()> {
    let target_format = Format::from_str(args.to)?;

    let write_delimiter = args
        .delimiter
        .or_else(|| default_delimiter_for_format(args.to));
    let write_options = FormatOptions {
        delimiter: write_delimiter,
        no_header: args.no_header,
        pretty: if args.compact {
            false
        } else {
            args.pretty || !args.compact
        },
        compact: args.compact,
        flow_style: args.flow,
    };

    // stdin mode: no input files
    if args.input.is_empty() {
        let source_format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => bail!("--from is required when reading from stdin\n  Hint: specify the input format, e.g. --from json"),
        };

        let value = if source_format == Format::Msgpack {
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
            let read_delimiter = args
                .delimiter
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

            let value = read_value_from_path(path, source_format, &read_options)?;

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

    let value = read_value_from_path(path, source_format, &read_options)?;

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

/// 파일 경로에서 Value를 읽는다 (바이너리 포맷 자동 처리)
fn read_value_from_path(path: &Path, format: Format, options: &FormatOptions) -> Result<Value> {
    if format == Format::Msgpack {
        let bytes = read_file_bytes(path)?;
        MsgpackReader.read_from_bytes(&bytes)
    } else {
        let content = read_file(path)?;
        read_value(&content, format, options)
    }
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
        Format::Xml => XmlReader.read(content),
        Format::Msgpack => MsgpackReader.read(content),
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
        Format::Csv => CsvWriter::new(options.clone()).write(value),
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty).write(value),
        Format::Msgpack => MsgpackWriter.write(value),
    }
}
