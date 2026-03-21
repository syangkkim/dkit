use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::read_file;
use crate::format::csv::{CsvReader, CsvWriter};
use crate::format::json::{JsonReader, JsonWriter};
use crate::format::toml::{TomlReader, TomlWriter};
use crate::format::xml::{XmlReader, XmlWriter};
use crate::format::yaml::{YamlReader, YamlWriter};
use crate::format::{detect_format, Format, FormatOptions, FormatReader, FormatWriter};
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

    let write_options = FormatOptions {
        delimiter: args.delimiter,
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
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read from stdin")?;

        let read_options = FormatOptions {
            delimiter: args.delimiter,
            no_header: args.no_header,
            ..Default::default()
        };
        let value = read_value(&buf, source_format, &read_options)?;
        let result = write_value(&value, target_format, &write_options)?;

        if let Some(out_path) = args.output {
            fs::write(out_path, &result)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            print!("{result}");
        }
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

            let read_options = FormatOptions {
                delimiter: args.delimiter,
                no_header: args.no_header,
                ..Default::default()
            };

            let content = read_file(path)?;
            let value = read_value(&content, source_format, &read_options)?;
            let result = write_value(&value, target_format, &write_options)?;

            let out_name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                + "."
                + args.to;
            let out_path = outdir.join(out_name);
            fs::write(&out_path, &result)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        }
        return Ok(());
    }

    // Single file
    let path = &args.input[0];
    let source_format = match args.from {
        Some(f) => Format::from_str(f)?,
        None => detect_format(path)?,
    };

    let read_options = FormatOptions {
        delimiter: args.delimiter,
        no_header: args.no_header,
        ..Default::default()
    };

    let content = read_file(path)?;
    let value = read_value(&content, source_format, &read_options)?;
    let result = write_value(&value, target_format, &write_options)?;

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

    if let Some(out_path) = args.output.or(outdir_path.as_deref()) {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {}", parent.display()))?;
            }
        }
        fs::write(out_path, &result)
            .with_context(|| format!("Failed to write to {}", out_path.display()))?;
    } else {
        print!("{result}");
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
        Format::Csv => CsvWriter::new(options.clone()).write(value),
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty).write(value),
    }
}
