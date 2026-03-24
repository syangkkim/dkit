use std::fs;
use std::io::{self, IsTerminal, Read, Write as _};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, write_parquet_to_bytes, EncodingOptions, ExcelOptions,
    ParquetWriteOptions, SqliteOptions,
};
use dkit_core::format::csv::{CsvReader, CsvWriter};
use dkit_core::format::html::HtmlWriter;
use dkit_core::format::json::{JsonReader, JsonWriter};
use dkit_core::format::jsonl::{JsonlReader, JsonlWriter};
use dkit_core::format::markdown::MarkdownWriter;
use dkit_core::format::msgpack::{MsgpackReader, MsgpackWriter};
use dkit_core::format::toml::{TomlReader, TomlWriter};
use dkit_core::format::xml::{XmlReader, XmlWriter};
use dkit_core::format::yaml::{YamlReader, YamlWriter};
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader, FormatWriter,
};
use dkit_core::value::Value;

/// 지원되는 입력 파일 확장자 목록
const SUPPORTED_EXTENSIONS: &[&str] = &[
    "json", "jsonl", "ndjson", "csv", "tsv", "yaml", "yml", "toml", "xml", "msgpack", "xlsx",
    "xls", "xlsm", "xlsb", "ods", "db", "sqlite", "sqlite3", "parquet", "pq",
];

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
    pub rename: Option<&'a str>,
    pub continue_on_error: bool,
    pub data_filter: super::DataFilterOptions,
    pub parquet_opts: ParquetWriteOptions,
    pub streaming_opts: Option<super::streaming::StreamingOptions>,
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

    // 스트리밍 모드: --chunk-size가 지정된 경우 스트리밍 파이프라인 시도
    if let Some(ref streaming_opts) = args.streaming_opts {
        if is_stdin {
            // stdin 스트리밍
            let source_format = match args.from {
                Some(f) => Format::from_str(f)?,
                None => bail!("--from is required for streaming from stdin"),
            };
            if super::streaming::supports_streaming(source_format, target_format) {
                let read_options = FormatOptions {
                    delimiter: args
                        .delimiter
                        .or_else(|| args.from.and_then(default_delimiter_for_format)),
                    no_header: args.no_header,
                    ..Default::default()
                };
                return super::streaming::stream_convert_stdin(
                    io::stdin().lock(),
                    source_format,
                    target_format,
                    &read_options,
                    &write_options,
                    args.output,
                    streaming_opts,
                );
            }
            // 스트리밍 미지원 조합이면 일반 모드로 fallback
        } else {
            let resolved_files = expand_inputs(args.input)?;
            if resolved_files.len() == 1 {
                let path = &resolved_files[0];
                let source_format = match args.from {
                    Some(f) => Format::from_str(f)?,
                    None => detect_format(path)?,
                };
                if super::streaming::supports_streaming(source_format, target_format) {
                    let read_delimiter = args.delimiter.or_else(|| default_delimiter(path));
                    let read_options = FormatOptions {
                        delimiter: read_delimiter,
                        no_header: args.no_header,
                        ..Default::default()
                    };

                    let outdir_path = args.outdir.map(|d| {
                        let name = make_output_name(path, args.to, args.rename);
                        d.join(name)
                    });
                    let out_path = args.output.or(outdir_path.as_deref());

                    return super::streaming::stream_convert(
                        path,
                        source_format,
                        target_format,
                        &read_options,
                        &write_options,
                        out_path,
                        streaming_opts,
                    );
                }
                // 스트리밍 미지원 조합이면 일반 모드로 fallback
            }
            // 멀티 파일 배치에서의 스트리밍은 일반 모드로 fallback
        }
    }

    if is_stdin {
        let value = if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            MsgpackReader.read_from_bytes(&buf)?
        } else if args.from == Some("parquet") || args.from == Some("pq") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            read_parquet_from_bytes(&buf)?
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

        let value = super::apply_data_filters(value, &args.data_filter)?;
        write_output(
            &value,
            target_format,
            &write_options,
            args.output,
            &args.parquet_opts,
        )?;
        return Ok(());
    }

    // Expand inputs: resolve glob patterns and directories
    let resolved_files = expand_inputs(args.input)?;

    if resolved_files.is_empty() {
        bail!("No matching files found");
    }

    // Multiple files with --outdir (batch mode)
    if resolved_files.len() > 1 {
        let outdir = match args.outdir {
            Some(d) => d,
            None => bail!("--outdir is required when converting multiple files\n  Hint: specify an output directory, e.g. --outdir ./output"),
        };
        fs::create_dir_all(outdir)
            .with_context(|| format!("Failed to create directory {}", outdir.display()))?;

        let total = resolved_files.len();
        let mut success_count = 0usize;
        let mut error_count = 0usize;
        let mut errors: Vec<(PathBuf, String)> = Vec::new();

        for (idx, path) in resolved_files.iter().enumerate() {
            eprint!("Converting ({}/{}) {} ... ", idx + 1, total, path.display());

            match convert_single_file(path, args, target_format, &write_options, outdir) {
                Ok(()) => {
                    success_count += 1;
                    eprintln!("ok");
                }
                Err(e) => {
                    error_count += 1;
                    let msg = format!("{e:#}");
                    eprintln!("FAILED: {msg}");
                    errors.push((path.clone(), msg));
                    if !args.continue_on_error {
                        bail!(
                            "Conversion failed for {}\n  Use --continue-on-error to skip failed files",
                            path.display()
                        );
                    }
                }
            }
        }

        // Print summary
        eprintln!();
        eprintln!("Batch conversion complete: {success_count} succeeded, {error_count} failed out of {total} files");

        if !errors.is_empty() {
            eprintln!();
            eprintln!("Failed files:");
            for (path, msg) in &errors {
                eprintln!("  {}: {msg}", path.display());
            }
            bail!("{error_count} file(s) failed to convert");
        }

        return Ok(());
    }

    // Single file
    let path = &resolved_files[0];
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
    let value = super::apply_data_filters(value, &args.data_filter)?;

    let outdir_path = args.outdir.map(|d| {
        let name = make_output_name(path, args.to, args.rename);
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
    write_output(
        &value,
        target_format,
        &write_options,
        out_path,
        &args.parquet_opts,
    )?;

    Ok(())
}

/// 단일 파일을 변환하여 outdir에 저장한다 (배치 모드용)
fn convert_single_file(
    path: &Path,
    args: &ConvertArgs,
    target_format: Format,
    write_options: &FormatOptions,
    outdir: &Path,
) -> Result<()> {
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
    let value = super::apply_data_filters(value, &args.data_filter)?;

    let out_name = make_output_name(path, args.to, args.rename);
    let out_path = outdir.join(out_name);
    write_output(
        &value,
        target_format,
        write_options,
        Some(&out_path),
        &args.parquet_opts,
    )
}

/// 입력 경로를 확장한다: 글롭 패턴, 디렉토리, 일반 파일을 처리
fn expand_inputs(inputs: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for input in inputs {
        let input_str = input.to_string_lossy();

        // Check if the input contains glob metacharacters
        if contains_glob_chars(&input_str) {
            let matches: Vec<_> = glob::glob(&input_str)
                .with_context(|| format!("Invalid glob pattern: {input_str}"))?
                .collect();

            if matches.is_empty() {
                bail!("No files matched pattern: {input_str}");
            }

            for entry in matches {
                let path =
                    entry.with_context(|| format!("Error reading glob match for: {input_str}"))?;
                if path.is_file() {
                    files.push(path);
                }
            }
        } else if input.is_dir() {
            // Scan directory for supported files
            let mut dir_files = collect_supported_files(input)?;
            if dir_files.is_empty() {
                bail!("No supported files found in directory: {}", input.display());
            }
            dir_files.sort();
            files.extend(dir_files);
        } else {
            // Regular file path
            files.push(input.clone());
        }
    }

    Ok(files)
}

/// 문자열에 글롭 메타문자가 포함되어 있는지 확인
fn contains_glob_chars(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

/// 디렉토리에서 지원되는 확장자를 가진 파일을 수집한다
fn collect_supported_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| format!("Failed to read entry in: {}", dir.display()))?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    files.push(path);
                }
            }
        }
    }

    Ok(files)
}

/// 출력 파일명을 생성한다 (rename 패턴 또는 기본 확장자 교체)
fn make_output_name(path: &Path, target_ext: &str, rename: Option<&str>) -> String {
    let stem = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    match rename {
        Some(pattern) => pattern
            .replace("{name}", &stem)
            .replace("{ext}", target_ext),
        None => format!("{stem}.{target_ext}"),
    }
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
    } else if format == Format::Parquet {
        let bytes = read_file_bytes(path)?;
        read_parquet_from_bytes(&bytes)
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
        Format::Parquet => {
            bail!("Parquet files must be read from a file path, not from text input")
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
    parquet_opts: &ParquetWriteOptions,
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
    } else if format == Format::Parquet {
        let bytes = write_parquet_to_bytes(value, parquet_opts)?;
        if let Some(out_path) = output {
            fs::write(out_path, &bytes)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            io::stdout()
                .write_all(&bytes)
                .context("Failed to write Parquet to stdout")?;
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
        Format::Parquet => {
            bail!("Internal error: Parquet output should be handled via write_output")
        }
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_glob_chars() {
        assert!(contains_glob_chars("*.json"));
        assert!(contains_glob_chars("data?.csv"));
        assert!(contains_glob_chars("data[0-9].json"));
        assert!(!contains_glob_chars("data.json"));
        assert!(!contains_glob_chars("path/to/file.csv"));
    }

    #[test]
    fn test_make_output_name_default() {
        let path = Path::new("data.json");
        assert_eq!(make_output_name(path, "csv", None), "data.csv");
    }

    #[test]
    fn test_make_output_name_with_rename_pattern() {
        let path = Path::new("data.json");
        assert_eq!(
            make_output_name(path, "csv", Some("{name}.converted.{ext}")),
            "data.converted.csv"
        );
    }

    #[test]
    fn test_make_output_name_custom_pattern() {
        let path = Path::new("/some/dir/users.json");
        assert_eq!(
            make_output_name(path, "yaml", Some("output_{name}.{ext}")),
            "output_users.yaml"
        );
    }
}
