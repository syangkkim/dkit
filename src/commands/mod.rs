pub mod convert;
pub mod diff;
pub mod merge;
pub mod query;
pub mod schema;
pub mod stats;
pub mod streaming;
pub mod view;

use std::path::Path;

use chardetng::EncodingDetector;
use encoding_rs::Encoding;

/// BOM(Byte Order Mark)을 감지하고 해당 인코딩과 BOM 크기를 반환한다.
fn detect_bom(bytes: &[u8]) -> Option<(&'static Encoding, usize)> {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        Some((encoding_rs::UTF_8, 3))
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        Some((encoding_rs::UTF_16LE, 2))
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        Some((encoding_rs::UTF_16BE, 2))
    } else {
        None
    }
}

/// 인코딩 라벨을 encoding_rs::Encoding으로 변환한다.
fn resolve_encoding(label: &str) -> anyhow::Result<&'static Encoding> {
    Encoding::for_label(label.as_bytes())
        .ok_or_else(|| anyhow::anyhow!("Unknown encoding: '{label}'\n  Hint: common encodings include utf-8, euc-kr, shift_jis, latin1, iso-8859-1, windows-1252"))
}

/// chardetng를 사용하여 인코딩을 자동 감지한다.
fn detect_encoding_from_bytes(bytes: &[u8]) -> &'static Encoding {
    let mut detector = EncodingDetector::new();
    detector.feed(bytes, true);
    detector.guess(None, true)
}

/// 바이너리 파일 읽기 (MessagePack 등 바이너리 포맷용)
pub fn read_file_bytes(path: &Path) -> anyhow::Result<Vec<u8>> {
    std::fs::read(path).map_err(|e| {
        let hint = if e.kind() == std::io::ErrorKind::NotFound {
            format!(
                "\n  Hint: check that the file path '{}' is correct",
                path.display()
            )
        } else if e.kind() == std::io::ErrorKind::PermissionDenied {
            format!(
                "\n  Hint: permission denied for '{}' — check file permissions",
                path.display()
            )
        } else {
            String::new()
        };
        anyhow::anyhow!("Failed to read '{}': {e}{hint}", path.display())
    })
}

/// 인코딩 옵션
#[derive(Debug, Clone, Default)]
pub struct EncodingOptions {
    /// 명시적 인코딩 라벨 (예: euc-kr, shift_jis)
    pub encoding: Option<String>,
    /// 인코딩 자동 감지 활성화
    pub detect_encoding: bool,
}

/// Excel 읽기 옵션
#[derive(Debug, Clone, Default)]
pub struct ExcelOptions {
    /// 시트 이름 또는 인덱스
    pub sheet: Option<String>,
    /// 헤더 행 번호 (1-based)
    pub header_row: Option<usize>,
}

/// Excel 파일을 바이트에서 Value로 읽는다.
pub fn read_xlsx_from_bytes(
    bytes: &[u8],
    excel_opts: &ExcelOptions,
) -> anyhow::Result<crate::value::Value> {
    use crate::format::xlsx::{XlsxOptions, XlsxReader};
    let opts = XlsxOptions {
        sheet: excel_opts.sheet.clone(),
        header_row: excel_opts.header_row.unwrap_or(1),
    };
    XlsxReader::new(opts).read_from_bytes(bytes)
}

/// Excel 파일의 시트 목록을 반환한다.
pub fn list_xlsx_sheets(bytes: &[u8]) -> anyhow::Result<Vec<String>> {
    crate::format::xlsx::XlsxReader::list_sheets(bytes)
}

/// SQLite 읽기 옵션
#[derive(Debug, Clone, Default)]
pub struct SqliteOptions {
    /// 테이블 이름
    pub table: Option<String>,
    /// 실행할 SQL 쿼리
    pub sql: Option<String>,
}

/// SQLite 파일을 경로에서 Value로 읽는다.
pub fn read_sqlite_from_path(
    path: &std::path::Path,
    sqlite_opts: &SqliteOptions,
) -> anyhow::Result<crate::value::Value> {
    use crate::format::sqlite::{SqliteOptions as ReaderOpts, SqliteReader};
    let opts = ReaderOpts {
        table: sqlite_opts.table.clone(),
        sql: sqlite_opts.sql.clone(),
    };
    SqliteReader::new(opts).read_from_path(path)
}

/// SQLite 파일의 테이블 목록을 반환한다.
pub fn list_sqlite_tables(path: &std::path::Path) -> anyhow::Result<Vec<String>> {
    crate::format::sqlite::SqliteReader::list_tables(path)
}

/// Parquet 파일을 바이트에서 Value로 읽는다.
pub fn read_parquet_from_bytes(bytes: &[u8]) -> anyhow::Result<crate::value::Value> {
    use crate::format::parquet::{ParquetOptions, ParquetReader};
    ParquetReader::new(ParquetOptions::default()).read_from_bytes(bytes)
}

/// Parquet 쓰기 옵션
#[derive(Debug, Clone, Default)]
pub struct ParquetWriteOptions {
    /// 압축 방식 문자열 (none, snappy, gzip, zstd)
    pub compression: String,
    /// Row Group 최대 크기
    pub row_group_size: Option<usize>,
}

/// Value를 Parquet 바이트로 직렬화한다.
pub fn write_parquet_to_bytes(
    value: &crate::value::Value,
    opts: &ParquetWriteOptions,
) -> anyhow::Result<Vec<u8>> {
    use crate::format::parquet::{
        ParquetCompression, ParquetWriteOptions as FmtOpts, ParquetWriter,
    };
    let compression: ParquetCompression = opts.compression.parse()?;
    let write_opts = FmtOpts {
        compression,
        row_group_size: opts.row_group_size,
    };
    ParquetWriter::new(write_opts).write_to_bytes(value)
}

/// 인코딩을 고려하여 파일을 읽는다.
///
/// 동작 우선순위:
/// 1. BOM이 있으면 BOM에 따른 인코딩 사용 (BOM 제거)
/// 2. `--encoding` 옵션이 있으면 해당 인코딩 사용
/// 3. `--detect-encoding` 옵션이 있으면 chardetng로 자동 감지
/// 4. 기본: UTF-8로 읽기
pub fn read_file_with_encoding(path: &Path, opts: &EncodingOptions) -> anyhow::Result<String> {
    if opts.encoding.is_none() && !opts.detect_encoding {
        // 최적화: 인코딩 옵션이 없으면 기존 방식으로 읽되 BOM만 처리
        let bytes = read_file_bytes(path)?;
        return decode_bytes(&bytes, opts);
    }

    let bytes = read_file_bytes(path)?;
    decode_bytes(&bytes, opts)
}

/// stdin에서 읽은 바이트를 인코딩 옵션에 따라 디코딩한다.
pub fn decode_bytes(bytes: &[u8], opts: &EncodingOptions) -> anyhow::Result<String> {
    // 1. BOM 감지
    if let Some((bom_encoding, bom_len)) = detect_bom(bytes) {
        let content_bytes = &bytes[bom_len..];
        let (result, _, had_errors) = bom_encoding.decode(content_bytes);
        if had_errors {
            anyhow::bail!(
                "Failed to decode with BOM-detected encoding ({}): input contains invalid bytes\n  Hint: try specifying the correct encoding with --encoding",
                bom_encoding.name()
            );
        }
        return Ok(result.into_owned());
    }

    // 2. 명시적 인코딩 지정
    if let Some(ref label) = opts.encoding {
        let encoding = resolve_encoding(label)?;
        let (result, _, had_errors) = encoding.decode(bytes);
        if had_errors {
            anyhow::bail!(
                "Failed to decode with encoding '{}': input contains invalid bytes\n  Hint: check that the encoding is correct, or try --detect-encoding",
                label
            );
        }
        return Ok(result.into_owned());
    }

    // 3. 자동 감지
    if opts.detect_encoding {
        let encoding = detect_encoding_from_bytes(bytes);
        let (result, _, had_errors) = encoding.decode(bytes);
        if had_errors {
            anyhow::bail!(
                "Failed to decode with detected encoding ({}): input contains invalid bytes",
                encoding.name()
            );
        }
        return Ok(result.into_owned());
    }

    // 4. 기본: UTF-8
    String::from_utf8(bytes.to_vec()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read as UTF-8: {e}\n  Hint: try --encoding <encoding> or --detect-encoding for non-UTF-8 files"
        )
    })
}

/// 데이터 정렬 및 필터링 옵션 (convert/view 공통)
#[derive(Debug, Clone, Default)]
pub struct DataFilterOptions {
    /// 정렬 기준 필드
    pub sort_by: Option<String>,
    /// 내림차순 정렬 여부
    pub descending: bool,
    /// 상위 N개 레코드
    pub head: Option<usize>,
    /// 하위 N개 레코드
    pub tail: Option<usize>,
    /// 필터 표현식 문자열
    pub filter: Option<String>,
}

/// 데이터 필터/정렬 옵션을 Value에 적용한다.
/// 적용 순서: where → sort → head/tail
pub fn apply_data_filters(
    value: crate::value::Value,
    opts: &DataFilterOptions,
) -> anyhow::Result<crate::value::Value> {
    use crate::query::filter::apply_operations;
    use crate::query::parser::{parse_condition_expr, Operation};

    let mut operations = Vec::new();

    // 1. where 필터
    if let Some(ref expr) = opts.filter {
        let condition = parse_condition_expr(expr).map_err(|e| {
            anyhow::anyhow!(
                "Invalid --filter expression: {e}\n  Hint: use format like 'age > 30' or 'name == \"Alice\"'"
            )
        })?;
        operations.push(Operation::Where(condition));
    }

    // 2. sort
    if let Some(ref field) = opts.sort_by {
        operations.push(Operation::Sort {
            field: field.clone(),
            descending: opts.descending,
        });
    }

    // 3. head (= limit)
    if let Some(n) = opts.head {
        operations.push(Operation::Limit(n));
    }

    // head와 tail이 동시에 지정된 경우 head를 먼저 적용하고 tail을 적용하므로
    // 별도 처리가 필요
    if operations.is_empty() && opts.tail.is_none() {
        return Ok(value);
    }

    let mut result = if operations.is_empty() {
        value
    } else {
        apply_operations(value, &operations)?
    };

    // 4. tail: 배열의 마지막 N개 요소 추출
    if let Some(n) = opts.tail {
        if let crate::value::Value::Array(ref arr) = result {
            let start = arr.len().saturating_sub(n);
            result = crate::value::Value::Array(arr[start..].to_vec());
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bom_utf8() {
        let bytes = [0xEF, 0xBB, 0xBF, b'h', b'i'];
        let (enc, len) = detect_bom(&bytes).unwrap();
        assert_eq!(enc, encoding_rs::UTF_8);
        assert_eq!(len, 3);
    }

    #[test]
    fn test_detect_bom_utf16le() {
        let bytes = [0xFF, 0xFE, b'h', 0x00];
        let (enc, len) = detect_bom(&bytes).unwrap();
        assert_eq!(enc, encoding_rs::UTF_16LE);
        assert_eq!(len, 2);
    }

    #[test]
    fn test_detect_bom_utf16be() {
        let bytes = [0xFE, 0xFF, 0x00, b'h'];
        let (enc, len) = detect_bom(&bytes).unwrap();
        assert_eq!(enc, encoding_rs::UTF_16BE);
        assert_eq!(len, 2);
    }

    #[test]
    fn test_detect_bom_none() {
        let bytes = b"hello world";
        assert!(detect_bom(bytes).is_none());
    }

    #[test]
    fn test_resolve_encoding_valid() {
        assert!(resolve_encoding("utf-8").is_ok());
        assert!(resolve_encoding("euc-kr").is_ok());
        assert!(resolve_encoding("shift_jis").is_ok());
        assert!(resolve_encoding("latin1").is_ok());
        assert!(resolve_encoding("iso-8859-1").is_ok());
        assert!(resolve_encoding("windows-1252").is_ok());
    }

    #[test]
    fn test_resolve_encoding_invalid() {
        assert!(resolve_encoding("invalid-encoding-xyz").is_err());
    }

    #[test]
    fn test_decode_bytes_utf8() {
        let bytes = b"hello world";
        let opts = EncodingOptions::default();
        let result = decode_bytes(bytes, &opts).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_decode_bytes_utf8_bom() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(b"hello");
        let opts = EncodingOptions::default();
        let result = decode_bytes(&bytes, &opts).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_decode_bytes_explicit_encoding() {
        // "hello" in latin1 is just ASCII, so it's the same bytes
        let bytes = b"hello";
        let opts = EncodingOptions {
            encoding: Some("latin1".to_string()),
            detect_encoding: false,
        };
        let result = decode_bytes(bytes, &opts).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_decode_bytes_euc_kr() {
        // "한글" in EUC-KR
        let bytes: &[u8] = &[0xC7, 0xD1, 0xB1, 0xDB];
        let opts = EncodingOptions {
            encoding: Some("euc-kr".to_string()),
            detect_encoding: false,
        };
        let result = decode_bytes(bytes, &opts).unwrap();
        assert_eq!(result, "한글");
    }

    #[test]
    fn test_decode_bytes_detect_encoding() {
        // Pure ASCII is detected as UTF-8
        let bytes = b"hello world";
        let opts = EncodingOptions {
            encoding: None,
            detect_encoding: true,
        };
        let result = decode_bytes(bytes, &opts).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_decode_bytes_utf16le_bom() {
        // UTF-16LE BOM + "hi"
        let bytes: &[u8] = &[0xFF, 0xFE, b'h', 0x00, b'i', 0x00];
        let opts = EncodingOptions::default();
        let result = decode_bytes(bytes, &opts).unwrap();
        assert_eq!(result, "hi");
    }

    // --- apply_data_filters tests ---

    use crate::value::Value;
    use indexmap::IndexMap;

    fn make_record(name: &str, age: i64) -> Value {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String(name.to_string()));
        m.insert("age".to_string(), Value::Integer(age));
        Value::Object(m)
    }

    fn sample_data() -> Value {
        Value::Array(vec![
            make_record("Alice", 30),
            make_record("Bob", 25),
            make_record("Charlie", 35),
            make_record("Diana", 28),
            make_record("Eve", 22),
        ])
    }

    #[test]
    fn test_data_filter_no_ops() {
        let data = sample_data();
        let opts = DataFilterOptions::default();
        let result = apply_data_filters(data.clone(), &opts).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_data_filter_sort_asc() {
        let data = sample_data();
        let opts = DataFilterOptions {
            sort_by: Some("age".to_string()),
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr[0].as_object().unwrap()["name"].as_str().unwrap(), "Eve");
        assert_eq!(
            arr[4].as_object().unwrap()["name"].as_str().unwrap(),
            "Charlie"
        );
    }

    #[test]
    fn test_data_filter_sort_desc() {
        let data = sample_data();
        let opts = DataFilterOptions {
            sort_by: Some("age".to_string()),
            descending: true,
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap()["name"].as_str().unwrap(),
            "Charlie"
        );
        assert_eq!(arr[4].as_object().unwrap()["name"].as_str().unwrap(), "Eve");
    }

    #[test]
    fn test_data_filter_head() {
        let data = sample_data();
        let opts = DataFilterOptions {
            head: Some(2),
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(
            arr[0].as_object().unwrap()["name"].as_str().unwrap(),
            "Alice"
        );
        assert_eq!(arr[1].as_object().unwrap()["name"].as_str().unwrap(), "Bob");
    }

    #[test]
    fn test_data_filter_tail() {
        let data = sample_data();
        let opts = DataFilterOptions {
            tail: Some(2),
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(
            arr[0].as_object().unwrap()["name"].as_str().unwrap(),
            "Diana"
        );
        assert_eq!(arr[1].as_object().unwrap()["name"].as_str().unwrap(), "Eve");
    }

    #[test]
    fn test_data_filter_where() {
        let data = sample_data();
        let opts = DataFilterOptions {
            filter: Some("age > 27".to_string()),
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3); // Alice(30), Charlie(35), Diana(28)
    }

    #[test]
    fn test_data_filter_where_and_sort() {
        let data = sample_data();
        let opts = DataFilterOptions {
            filter: Some("age > 27".to_string()),
            sort_by: Some("age".to_string()),
            descending: true,
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(
            arr[0].as_object().unwrap()["name"].as_str().unwrap(),
            "Charlie"
        );
        assert_eq!(
            arr[2].as_object().unwrap()["name"].as_str().unwrap(),
            "Diana"
        );
    }

    #[test]
    fn test_data_filter_sort_and_head() {
        let data = sample_data();
        let opts = DataFilterOptions {
            sort_by: Some("age".to_string()),
            head: Some(3),
            ..Default::default()
        };
        let result = apply_data_filters(data, &opts).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        // youngest 3
        assert_eq!(arr[0].as_object().unwrap()["name"].as_str().unwrap(), "Eve");
        assert_eq!(arr[1].as_object().unwrap()["name"].as_str().unwrap(), "Bob");
        assert_eq!(
            arr[2].as_object().unwrap()["name"].as_str().unwrap(),
            "Diana"
        );
    }

    #[test]
    fn test_data_filter_invalid_expr() {
        let data = sample_data();
        let opts = DataFilterOptions {
            filter: Some("invalid!!!".to_string()),
            ..Default::default()
        };
        assert!(apply_data_filters(data, &opts).is_err());
    }

    #[test]
    fn test_data_filter_non_array() {
        // Non-array data should pass through for no-ops
        let data = Value::Integer(42);
        let opts = DataFilterOptions::default();
        let result = apply_data_filters(data, &opts).unwrap();
        assert_eq!(result, Value::Integer(42));
    }
}
