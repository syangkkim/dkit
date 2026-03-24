use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;

use anyhow::{bail, Context, Result};
use indexmap::IndexMap;

use dkit_core::format::json::from_json_value;
use dkit_core::format::{Format, FormatOptions};
use dkit_core::value::Value;

/// 스트리밍 변환 옵션
#[derive(Debug, Clone)]
pub struct StreamingOptions {
    /// 청크 크기 (한 번에 처리할 레코드 수)
    pub chunk_size: usize,
    /// 진행률 표시 여부
    pub progress: bool,
}

impl Default for StreamingOptions {
    fn default() -> Self {
        Self {
            chunk_size: 1000,
            progress: false,
        }
    }
}

/// 스트리밍 변환이 가능한 소스/타겟 포맷 조합인지 확인한다.
pub fn supports_streaming(source: Format, target: Format) -> bool {
    let readable = matches!(source, Format::Csv | Format::Jsonl)
        || (cfg!(feature = "parquet") && matches!(source, Format::Parquet));
    let writable = matches!(target, Format::Csv | Format::Jsonl);
    readable && writable
}

/// 스트리밍 변환 파이프라인을 실행한다.
///
/// 소스 파일에서 청크 단위로 레코드를 읽어 타겟 포맷으로 변환하여 출력한다.
/// 메모리에 전체 파일을 올리지 않고 청크 단위로 처리한다.
pub fn stream_convert(
    source_path: &Path,
    source_format: Format,
    target_format: Format,
    read_options: &FormatOptions,
    write_options: &FormatOptions,
    output: Option<&Path>,
    opts: &StreamingOptions,
) -> Result<()> {
    let file = File::open(source_path)
        .with_context(|| format!("Failed to open '{}'", source_path.display()))?;
    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
    let buf_reader = BufReader::new(file);

    let writer: Box<dyn Write> = if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {}", parent.display()))?;
            }
        }
        Box::new(BufWriter::new(File::create(out_path).with_context(
            || format!("Failed to create '{}'", out_path.display()),
        )?))
    } else {
        Box::new(BufWriter::new(io::stdout().lock()))
    };

    match source_format {
        Format::Jsonl => stream_from_jsonl(
            buf_reader,
            writer,
            target_format,
            write_options,
            opts,
            file_size,
        ),
        Format::Csv => stream_from_csv(
            buf_reader,
            writer,
            target_format,
            read_options,
            write_options,
            opts,
            file_size,
        ),
        #[cfg(feature = "parquet")]
        Format::Parquet => {
            // Parquet은 메모리 매핑 기반이므로 바이트를 읽어서 Row Group 단위로 처리
            let bytes = std::fs::read(source_path)
                .with_context(|| format!("Failed to read '{}'", source_path.display()))?;
            stream_from_parquet(&bytes, writer, target_format, write_options, opts)
        }
        _ => bail!("Streaming read is not supported for format: {source_format}"),
    }
}

/// stdin에서 스트리밍 변환을 수행한다.
pub fn stream_convert_stdin(
    stdin_reader: impl Read,
    source_format: Format,
    target_format: Format,
    read_options: &FormatOptions,
    write_options: &FormatOptions,
    output: Option<&Path>,
    opts: &StreamingOptions,
) -> Result<()> {
    let buf_reader = BufReader::new(stdin_reader);

    let writer: Box<dyn Write> = if let Some(out_path) = output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {}", parent.display()))?;
            }
        }
        Box::new(BufWriter::new(File::create(out_path).with_context(
            || format!("Failed to create '{}'", out_path.display()),
        )?))
    } else {
        Box::new(BufWriter::new(io::stdout().lock()))
    };

    match source_format {
        Format::Jsonl => {
            stream_from_jsonl(buf_reader, writer, target_format, write_options, opts, 0)
        }
        Format::Csv => stream_from_csv(
            buf_reader,
            writer,
            target_format,
            read_options,
            write_options,
            opts,
            0,
        ),
        _ => bail!("Streaming stdin is not supported for format: {source_format}"),
    }
}

// ── JSONL 스트리밍 리더 ──────────────────────────────────────────

fn stream_from_jsonl(
    reader: impl BufRead,
    writer: impl Write,
    target_format: Format,
    write_options: &FormatOptions,
    opts: &StreamingOptions,
    file_size: u64,
) -> Result<()> {
    let mut chunk = Vec::with_capacity(opts.chunk_size);
    let mut total_records: u64 = 0;
    let mut bytes_read: u64 = 0;
    let mut is_first_chunk = true;

    let mut stream_writer = StreamChunkWriter::new(writer, target_format, write_options)?;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.with_context(|| format!("Failed to read line {}", line_num + 1))?;
        bytes_read += line.len() as u64 + 1; // +1 for newline

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let json_val: serde_json::Value = serde_json::from_str(trimmed).with_context(|| {
            format!(
                "Invalid JSON at line {}: {}",
                line_num + 1,
                trimmed.chars().take(50).collect::<String>()
            )
        })?;
        chunk.push(from_json_value(json_val));

        if chunk.len() >= opts.chunk_size {
            total_records += chunk.len() as u64;
            stream_writer.write_chunk(&chunk, is_first_chunk)?;
            is_first_chunk = false;

            if opts.progress {
                print_progress(total_records, bytes_read, file_size);
            }

            chunk.clear();
        }
    }

    // 남은 레코드 처리
    if !chunk.is_empty() {
        total_records += chunk.len() as u64;
        stream_writer.write_chunk(&chunk, is_first_chunk)?;
    }

    stream_writer.finish()?;

    if opts.progress {
        print_progress_done(total_records, bytes_read);
    }

    Ok(())
}

// ── CSV 스트리밍 리더 ───────────────────────────────────────────

fn stream_from_csv(
    reader: impl BufRead,
    writer: impl Write,
    target_format: Format,
    read_options: &FormatOptions,
    write_options: &FormatOptions,
    opts: &StreamingOptions,
    file_size: u64,
) -> Result<()> {
    let delimiter = read_options.delimiter.unwrap_or(',') as u8;
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(!read_options.no_header)
        .delimiter(delimiter)
        .from_reader(reader);

    let headers: Vec<String> = if read_options.no_header {
        Vec::new()
    } else {
        csv_reader
            .headers()
            .map_err(|e| dkit_core::error::DkitError::ParseError {
                format: "CSV".to_string(),
                source: Box::new(e),
            })?
            .iter()
            .map(|h| h.to_string())
            .collect()
    };

    let mut chunk = Vec::with_capacity(opts.chunk_size);
    let mut total_records: u64 = 0;
    let mut bytes_read: u64 = 0;
    let mut is_first_chunk = true;

    let mut stream_writer = StreamChunkWriter::new(writer, target_format, write_options)?;

    for result in csv_reader.records() {
        let record = result.map_err(|e| dkit_core::error::DkitError::ParseError {
            format: "CSV".to_string(),
            source: Box::new(e),
        })?;

        // 대략적인 바이트 추적
        bytes_read += record.as_slice().len() as u64 + 1;

        let col_names: Vec<String> = if read_options.no_header {
            (0..record.len()).map(|i| format!("col{i}")).collect()
        } else {
            headers.clone()
        };

        let mut obj = IndexMap::new();
        for (i, field) in record.iter().enumerate() {
            let key = col_names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("col{i}"));
            obj.insert(key, infer_csv_value(field));
        }
        chunk.push(Value::Object(obj));

        if chunk.len() >= opts.chunk_size {
            total_records += chunk.len() as u64;
            stream_writer.write_chunk(&chunk, is_first_chunk)?;
            is_first_chunk = false;

            if opts.progress {
                print_progress(total_records, bytes_read, file_size);
            }

            chunk.clear();
        }
    }

    // 남은 레코드 처리
    if !chunk.is_empty() {
        total_records += chunk.len() as u64;
        stream_writer.write_chunk(&chunk, is_first_chunk)?;
    }

    stream_writer.finish()?;

    if opts.progress {
        print_progress_done(total_records, bytes_read);
    }

    Ok(())
}

// ── Parquet 스트리밍 리더 (Row Group 단위) ──────────────────────

#[cfg(feature = "parquet")]
fn stream_from_parquet(
    bytes: &[u8],
    writer: impl Write,
    target_format: Format,
    write_options: &FormatOptions,
    opts: &StreamingOptions,
) -> Result<()> {
    use bytes::Bytes;
    use parquet_impl::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    let bytes = Bytes::copy_from_slice(bytes);
    let builder = ParquetRecordBatchReaderBuilder::try_new(bytes).map_err(|e| {
        dkit_core::error::DkitError::ParseError {
            format: "Parquet".to_string(),
            source: Box::new(e),
        }
    })?;

    let metadata = builder.metadata().clone();
    let num_row_groups = metadata.num_row_groups();

    let reader = builder
        .with_batch_size(opts.chunk_size)
        .build()
        .map_err(|e| dkit_core::error::DkitError::ParseError {
            format: "Parquet".to_string(),
            source: Box::new(e),
        })?;

    let mut stream_writer = StreamChunkWriter::new(writer, target_format, write_options)?;
    let mut total_records: u64 = 0;
    let mut is_first_chunk = true;

    for batch_result in reader {
        let batch = batch_result.map_err(|e| dkit_core::error::DkitError::ParseError {
            format: "Parquet".to_string(),
            source: Box::new(e),
        })?;

        let schema = batch.schema();
        let num_rows = batch.num_rows();
        let mut chunk = Vec::with_capacity(num_rows);

        for row_idx in 0..num_rows {
            let mut obj = IndexMap::new();
            for (col_idx, field) in schema.fields().iter().enumerate() {
                let col = batch.column(col_idx);
                let value = dkit_core::format::parquet::arrow_value_to_value(col.as_ref(), row_idx);
                obj.insert(field.name().clone(), value);
            }
            chunk.push(Value::Object(obj));
        }

        if !chunk.is_empty() {
            total_records += chunk.len() as u64;
            stream_writer.write_chunk(&chunk, is_first_chunk)?;
            is_first_chunk = false;

            if opts.progress {
                eprint!(
                    "\r  Streaming: {} records processed ({} row groups total)  ",
                    total_records, num_row_groups
                );
            }
        }
    }

    stream_writer.finish()?;

    if opts.progress {
        eprintln!(
            "\r  Done: {} records processed from {} row groups            ",
            total_records, num_row_groups
        );
    }

    Ok(())
}

// ── 스트리밍 청크 Writer ────────────────────────────────────────

/// 출력 포맷별로 청크 단위 쓰기를 담당하는 구조체
struct StreamChunkWriter<W: Write> {
    writer: W,
    target_format: Format,
    write_options: FormatOptions,
    /// CSV 스트리밍 시 헤더가 이미 작성되었는지 여부
    csv_headers_written: bool,
    /// CSV 스트리밍 시 수집된 헤더
    csv_headers: Vec<String>,
}

impl<W: Write> StreamChunkWriter<W> {
    fn new(writer: W, target_format: Format, write_options: &FormatOptions) -> Result<Self> {
        if !matches!(target_format, Format::Csv | Format::Jsonl) {
            bail!("Streaming write is not supported for format: {target_format}");
        }
        Ok(Self {
            writer,
            target_format,
            write_options: write_options.clone(),
            csv_headers_written: false,
            csv_headers: Vec::new(),
        })
    }

    /// 청크를 출력 포맷으로 쓴다.
    fn write_chunk(&mut self, records: &[Value], _is_first: bool) -> Result<()> {
        match self.target_format {
            Format::Jsonl => self.write_jsonl_chunk(records),
            Format::Csv => self.write_csv_chunk(records),
            _ => bail!(
                "Streaming write is not supported for format: {}",
                self.target_format
            ),
        }
    }

    fn write_jsonl_chunk(&mut self, records: &[Value]) -> Result<()> {
        for record in records {
            let json_val = dkit_core::format::json::to_json_value(record);
            serde_json::to_writer(&mut self.writer, &json_val)
                .context("Failed to write JSONL record")?;
            self.writer
                .write_all(b"\n")
                .context("Failed to write newline")?;
        }
        Ok(())
    }

    fn write_csv_chunk(&mut self, records: &[Value]) -> Result<()> {
        let delimiter = self.write_options.delimiter.unwrap_or(',') as u8;

        // 첫 청크에서 헤더 수집
        if !self.csv_headers_written {
            let mut header_map = IndexMap::new();
            for record in records {
                if let Value::Object(obj) = record {
                    for key in obj.keys() {
                        header_map.entry(key.clone()).or_insert(());
                    }
                }
            }
            self.csv_headers = header_map.into_keys().collect();

            // 헤더 쓰기
            if !self.write_options.no_header {
                let header_line: Vec<u8> = self
                    .csv_headers
                    .iter()
                    .map(|h| csv_escape_field(h, delimiter))
                    .collect::<Vec<_>>()
                    .join(&[delimiter][..]);
                self.writer.write_all(&header_line)?;
                self.writer.write_all(b"\n")?;
            }
            self.csv_headers_written = true;
        }

        // 데이터 행 쓰기
        for record in records {
            let fields: Vec<String> = if let Value::Object(obj) = record {
                self.csv_headers
                    .iter()
                    .map(|h| obj.get(h).map(value_to_csv_field).unwrap_or_default())
                    .collect()
            } else {
                vec![value_to_csv_field(record)]
            };

            let line: Vec<u8> = fields
                .iter()
                .map(|f| csv_escape_field(f, delimiter))
                .collect::<Vec<_>>()
                .join(&[delimiter][..]);
            self.writer.write_all(&line)?;
            self.writer.write_all(b"\n")?;
        }

        Ok(())
    }

    fn finish(mut self) -> Result<()> {
        self.writer.flush().context("Failed to flush output")?;
        Ok(())
    }
}

// ── 유틸리티 함수 ───────────────────────────────────────────────

/// CSV 값 타입 추론 (csv.rs의 infer_value와 동일)
fn infer_csv_value(s: &str) -> Value {
    if s.is_empty() {
        return Value::Null;
    }
    if let Ok(i) = s.parse::<i64>() {
        return Value::Integer(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        if f.is_finite() {
            return Value::Float(f);
        }
    }
    Value::String(s.to_string())
}

/// Value를 CSV 필드 문자열로 변환
fn value_to_csv_field(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(a) => {
            let parts: Vec<String> = a.iter().map(|v| format!("{v}")).collect();
            format!("[{}]", parts.join(", "))
        }
        Value::Object(o) => {
            let parts: Vec<String> = o.iter().map(|(k, v)| format!("\"{k}\": {v}")).collect();
            format!("{{{}}}", parts.join(", "))
        }
        _ => format!("{v}"),
    }
}

/// CSV 필드를 필요 시 따옴표로 감싸는 함수
fn csv_escape_field(field: &str, delimiter: u8) -> Vec<u8> {
    let delim_char = delimiter as char;
    if field.contains(delim_char)
        || field.contains('"')
        || field.contains('\n')
        || field.contains('\r')
    {
        let mut escaped = String::with_capacity(field.len() + 2);
        escaped.push('"');
        for c in field.chars() {
            if c == '"' {
                escaped.push('"');
            }
            escaped.push(c);
        }
        escaped.push('"');
        escaped.into_bytes()
    } else {
        field.as_bytes().to_vec()
    }
}

/// 진행률 표시 (stderr)
fn print_progress(records: u64, bytes_read: u64, file_size: u64) {
    if file_size > 0 {
        let pct = (bytes_read as f64 / file_size as f64 * 100.0).min(100.0);
        eprint!(
            "\r  Streaming: {} records ({:.1}%, {})  ",
            records,
            pct,
            format_bytes(bytes_read)
        );
    } else {
        eprint!(
            "\r  Streaming: {} records ({})  ",
            records,
            format_bytes(bytes_read)
        );
    }
}

/// 완료 메시지 (stderr)
fn print_progress_done(records: u64, bytes_read: u64) {
    eprintln!(
        "\r  Done: {} records processed ({})            ",
        records,
        format_bytes(bytes_read)
    );
}

/// 바이트 크기를 사람이 읽기 좋은 형식으로 변환
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tempfile::NamedTempFile;

    #[test]
    fn test_supports_streaming() {
        assert!(supports_streaming(Format::Jsonl, Format::Jsonl));
        assert!(supports_streaming(Format::Jsonl, Format::Csv));
        assert!(supports_streaming(Format::Csv, Format::Jsonl));
        assert!(supports_streaming(Format::Csv, Format::Csv));
        #[cfg(feature = "parquet")]
        {
            assert!(supports_streaming(Format::Parquet, Format::Jsonl));
            assert!(supports_streaming(Format::Parquet, Format::Csv));
        }

        assert!(!supports_streaming(Format::Json, Format::Jsonl));
        assert!(!supports_streaming(Format::Jsonl, Format::Json));
        assert!(!supports_streaming(Format::Yaml, Format::Csv));
    }

    #[test]
    fn test_infer_csv_value() {
        assert_eq!(infer_csv_value(""), Value::Null);
        assert_eq!(infer_csv_value("42"), Value::Integer(42));
        assert_eq!(infer_csv_value("3.14"), Value::Float(3.14));
        assert_eq!(infer_csv_value("hello"), Value::String("hello".to_string()));
    }

    #[test]
    fn test_csv_escape_field() {
        assert_eq!(csv_escape_field("hello", b','), b"hello");
        assert_eq!(csv_escape_field("he,llo", b','), b"\"he,llo\"");
        assert_eq!(csv_escape_field("he\"llo", b','), b"\"he\"\"llo\"");
        assert_eq!(csv_escape_field("he\nllo", b','), b"\"he\nllo\"");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_stream_jsonl_to_jsonl() {
        let input = r#"{"name":"Alice","age":30}
{"name":"Bob","age":25}
{"name":"Charlie","age":35}
"#;
        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(input.as_bytes()).unwrap();

        let mut output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_path_buf();

        let opts = StreamingOptions {
            chunk_size: 2,
            progress: false,
        };

        stream_convert(
            input_file.path(),
            Format::Jsonl,
            Format::Jsonl,
            &FormatOptions::default(),
            &FormatOptions::default(),
            Some(&output_path),
            &opts,
        )
        .unwrap();

        let result = std::fs::read_to_string(&output_path).unwrap();
        let lines: Vec<&str> = result.trim().lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("Alice"));
        assert!(lines[1].contains("Bob"));
        assert!(lines[2].contains("Charlie"));
    }

    #[test]
    fn test_stream_jsonl_to_csv() {
        let input = r#"{"name":"Alice","age":30}
{"name":"Bob","age":25}
"#;
        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(input.as_bytes()).unwrap();

        let mut output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_path_buf();

        let opts = StreamingOptions {
            chunk_size: 10,
            progress: false,
        };

        stream_convert(
            input_file.path(),
            Format::Jsonl,
            Format::Csv,
            &FormatOptions::default(),
            &FormatOptions::default(),
            Some(&output_path),
            &opts,
        )
        .unwrap();

        let result = std::fs::read_to_string(&output_path).unwrap();
        let lines: Vec<&str> = result.trim().lines().collect();
        assert_eq!(lines.len(), 3); // header + 2 rows
        assert!(lines[0].contains("name"));
        assert!(lines[0].contains("age"));
    }

    #[test]
    fn test_stream_csv_to_jsonl() {
        let input = "name,age\nAlice,30\nBob,25\nCharlie,35\n";
        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(input.as_bytes()).unwrap();

        let mut output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_path_buf();

        let opts = StreamingOptions {
            chunk_size: 2,
            progress: false,
        };

        stream_convert(
            input_file.path(),
            Format::Csv,
            Format::Jsonl,
            &FormatOptions::default(),
            &FormatOptions::default(),
            Some(&output_path),
            &opts,
        )
        .unwrap();

        let result = std::fs::read_to_string(&output_path).unwrap();
        let lines: Vec<&str> = result.trim().lines().collect();
        assert_eq!(lines.len(), 3);

        // Verify each line is valid JSON
        for line in &lines {
            let _: serde_json::Value = serde_json::from_str(line).unwrap();
        }
        assert!(lines[0].contains("Alice"));
    }

    #[test]
    fn test_stream_csv_to_csv() {
        let input = "name,age\nAlice,30\nBob,25\n";
        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(input.as_bytes()).unwrap();

        let mut output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_path_buf();

        let opts = StreamingOptions {
            chunk_size: 1,
            progress: false,
        };

        stream_convert(
            input_file.path(),
            Format::Csv,
            Format::Csv,
            &FormatOptions::default(),
            &FormatOptions::default(),
            Some(&output_path),
            &opts,
        )
        .unwrap();

        let result = std::fs::read_to_string(&output_path).unwrap();
        let lines: Vec<&str> = result.trim().lines().collect();
        assert_eq!(lines.len(), 3); // header + 2 rows
        assert!(lines[0].contains("name"));
        assert!(lines[1].contains("Alice"));
    }

    #[test]
    fn test_stream_empty_file() {
        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(b"").unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_path_buf();

        let opts = StreamingOptions {
            chunk_size: 100,
            progress: false,
        };

        stream_convert(
            input_file.path(),
            Format::Jsonl,
            Format::Jsonl,
            &FormatOptions::default(),
            &FormatOptions::default(),
            Some(&output_path),
            &opts,
        )
        .unwrap();

        let result = std::fs::read_to_string(&output_path).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_stream_large_dataset() {
        // 10,000 레코드 스트리밍 테스트
        let lines: Vec<String> = (0..10_000)
            .map(|i| format!(r#"{{"id":{},"value":"item_{}"}}"#, i, i))
            .collect();
        let input = lines.join("\n") + "\n";

        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(input.as_bytes()).unwrap();

        let output_file = NamedTempFile::new().unwrap();
        let output_path = output_file.path().to_path_buf();

        let opts = StreamingOptions {
            chunk_size: 500,
            progress: false,
        };

        stream_convert(
            input_file.path(),
            Format::Jsonl,
            Format::Jsonl,
            &FormatOptions::default(),
            &FormatOptions::default(),
            Some(&output_path),
            &opts,
        )
        .unwrap();

        let result = std::fs::read_to_string(&output_path).unwrap();
        let output_lines: Vec<&str> = result.trim().lines().collect();
        assert_eq!(output_lines.len(), 10_000);
    }
}
