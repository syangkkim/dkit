pub mod csv;
pub mod json;
pub mod jsonl;
pub mod msgpack;
pub mod toml;
pub mod xml;
pub mod yaml;

use std::io::{Read, Write};
use std::path::Path;

use crate::error::DkitError;
use crate::value::Value;

/// 지원하는 데이터 포맷
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Json,
    Jsonl,
    Csv,
    Yaml,
    Toml,
    Xml,
    Msgpack,
}

impl Format {
    pub fn from_str(s: &str) -> Result<Self, DkitError> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "jsonl" | "jsonlines" | "ndjson" => Ok(Format::Jsonl),
            "csv" | "tsv" => Ok(Format::Csv),
            "yaml" | "yml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            "xml" => Ok(Format::Xml),
            "msgpack" | "messagepack" => Ok(Format::Msgpack),
            _ => Err(DkitError::UnknownFormat(s.to_string())),
        }
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Json => write!(f, "JSON"),
            Format::Jsonl => write!(f, "JSONL"),
            Format::Csv => write!(f, "CSV"),
            Format::Yaml => write!(f, "YAML"),
            Format::Toml => write!(f, "TOML"),
            Format::Xml => write!(f, "XML"),
            Format::Msgpack => write!(f, "MessagePack"),
        }
    }
}

/// 파일 확장자로 포맷을 자동 감지
pub fn detect_format(path: &Path) -> Result<Format, DkitError> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Ok(Format::Json),
        Some("jsonl" | "ndjson") => Ok(Format::Jsonl),
        Some("csv" | "tsv") => Ok(Format::Csv),
        Some("yaml" | "yml") => Ok(Format::Yaml),
        Some("toml") => Ok(Format::Toml),
        Some("xml") => Ok(Format::Xml),
        Some("msgpack") => Ok(Format::Msgpack),
        Some(ext) => Err(DkitError::UnknownFormat(ext.to_string())),
        None => Err(DkitError::UnknownFormat("(no extension)".to_string())),
    }
}

/// 파일 확장자에 따른 기본 delimiter 반환
/// `.tsv` 파일은 탭 구분자를 사용한다.
pub fn default_delimiter(path: &Path) -> Option<char> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("tsv") => Some('\t'),
        _ => None,
    }
}

/// `--to` 포맷 문자열에 따른 기본 delimiter 반환
pub fn default_delimiter_for_format(format_str: &str) -> Option<char> {
    match format_str.to_lowercase().as_str() {
        "tsv" => Some('\t'),
        _ => None,
    }
}

/// 포맷별 옵션
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// CSV delimiter (기본: ',')
    pub delimiter: Option<char>,
    /// CSV 헤더 없음 모드
    pub no_header: bool,
    /// Pretty-print 출력
    pub pretty: bool,
    /// Compact 출력 (JSON)
    pub compact: bool,
    /// YAML inline/flow 스타일
    pub flow_style: bool,
    /// XML 루트 엘리먼트 이름 (기본: "root")
    pub root_element: Option<String>,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            delimiter: None,
            no_header: false,
            pretty: true,
            compact: false,
            flow_style: false,
            root_element: None,
        }
    }
}

/// 데이터 포맷 읽기 트레이트
#[allow(dead_code)]
pub trait FormatReader {
    fn read(&self, input: &str) -> anyhow::Result<Value>;
    fn read_from_reader(&self, reader: impl Read) -> anyhow::Result<Value>;
}

/// 데이터 포맷 쓰기 트레이트
#[allow(dead_code)]
pub trait FormatWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String>;
    fn write_to_writer(&self, value: &Value, writer: impl Write) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // --- Format::from_str ---

    #[test]
    fn test_format_from_str() {
        assert_eq!(Format::from_str("json").unwrap(), Format::Json);
        assert_eq!(Format::from_str("JSON").unwrap(), Format::Json);
        assert_eq!(Format::from_str("csv").unwrap(), Format::Csv);
        assert_eq!(Format::from_str("tsv").unwrap(), Format::Csv);
        assert_eq!(Format::from_str("TSV").unwrap(), Format::Csv);
        assert_eq!(Format::from_str("yaml").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("yml").unwrap(), Format::Yaml);
        assert_eq!(Format::from_str("toml").unwrap(), Format::Toml);
    }

    #[test]
    fn test_format_from_str_jsonl() {
        assert_eq!(Format::from_str("jsonl").unwrap(), Format::Jsonl);
        assert_eq!(Format::from_str("jsonlines").unwrap(), Format::Jsonl);
        assert_eq!(Format::from_str("ndjson").unwrap(), Format::Jsonl);
        assert_eq!(Format::from_str("JSONL").unwrap(), Format::Jsonl);
    }

    #[test]
    fn test_format_from_str_xml() {
        assert_eq!(Format::from_str("xml").unwrap(), Format::Xml);
    }

    #[test]
    fn test_format_from_str_msgpack() {
        assert_eq!(Format::from_str("msgpack").unwrap(), Format::Msgpack);
        assert_eq!(Format::from_str("messagepack").unwrap(), Format::Msgpack);
    }

    #[test]
    fn test_format_from_str_unknown() {
        let err = Format::from_str("bin").unwrap_err();
        assert!(matches!(err, DkitError::UnknownFormat(s) if s == "bin"));
    }

    // --- Format::Display ---

    #[test]
    fn test_format_display() {
        assert_eq!(Format::Json.to_string(), "JSON");
        assert_eq!(Format::Csv.to_string(), "CSV");
        assert_eq!(Format::Yaml.to_string(), "YAML");
        assert_eq!(Format::Toml.to_string(), "TOML");
        assert_eq!(Format::Jsonl.to_string(), "JSONL");
        assert_eq!(Format::Xml.to_string(), "XML");
        assert_eq!(Format::Msgpack.to_string(), "MessagePack");
    }

    // --- detect_format ---

    #[test]
    fn test_detect_format_json() {
        assert_eq!(
            detect_format(&PathBuf::from("data.json")).unwrap(),
            Format::Json
        );
    }

    #[test]
    fn test_detect_format_csv_tsv() {
        assert_eq!(
            detect_format(&PathBuf::from("data.csv")).unwrap(),
            Format::Csv
        );
        assert_eq!(
            detect_format(&PathBuf::from("data.tsv")).unwrap(),
            Format::Csv
        );
    }

    #[test]
    fn test_detect_format_yaml() {
        assert_eq!(
            detect_format(&PathBuf::from("data.yaml")).unwrap(),
            Format::Yaml
        );
        assert_eq!(
            detect_format(&PathBuf::from("data.yml")).unwrap(),
            Format::Yaml
        );
    }

    #[test]
    fn test_detect_format_toml() {
        assert_eq!(
            detect_format(&PathBuf::from("config.toml")).unwrap(),
            Format::Toml
        );
    }

    #[test]
    fn test_detect_format_jsonl() {
        assert_eq!(
            detect_format(&PathBuf::from("data.jsonl")).unwrap(),
            Format::Jsonl
        );
        assert_eq!(
            detect_format(&PathBuf::from("data.ndjson")).unwrap(),
            Format::Jsonl
        );
    }

    #[test]
    fn test_detect_format_xml() {
        assert_eq!(
            detect_format(&PathBuf::from("data.xml")).unwrap(),
            Format::Xml
        );
    }

    #[test]
    fn test_detect_format_msgpack() {
        assert_eq!(
            detect_format(&PathBuf::from("data.msgpack")).unwrap(),
            Format::Msgpack
        );
    }

    #[test]
    fn test_detect_format_unknown_ext() {
        let err = detect_format(&PathBuf::from("data.bin")).unwrap_err();
        assert!(matches!(err, DkitError::UnknownFormat(s) if s == "bin"));
    }

    #[test]
    fn test_detect_format_no_extension() {
        let err = detect_format(&PathBuf::from("Makefile")).unwrap_err();
        assert!(matches!(err, DkitError::UnknownFormat(s) if s == "(no extension)"));
    }

    // --- FormatOptions ---

    // --- default_delimiter ---

    #[test]
    fn test_default_delimiter_tsv() {
        assert_eq!(default_delimiter(&PathBuf::from("data.tsv")), Some('\t'));
    }

    #[test]
    fn test_default_delimiter_csv() {
        assert_eq!(default_delimiter(&PathBuf::from("data.csv")), None);
    }

    #[test]
    fn test_default_delimiter_json() {
        assert_eq!(default_delimiter(&PathBuf::from("data.json")), None);
    }

    #[test]
    fn test_default_delimiter_for_format_tsv() {
        assert_eq!(default_delimiter_for_format("tsv"), Some('\t'));
        assert_eq!(default_delimiter_for_format("TSV"), Some('\t'));
    }

    #[test]
    fn test_default_delimiter_for_format_csv() {
        assert_eq!(default_delimiter_for_format("csv"), None);
    }

    // --- FormatOptions ---

    #[test]
    fn test_format_options_default() {
        let opts = FormatOptions::default();
        assert_eq!(opts.delimiter, None);
        assert!(!opts.no_header);
        assert!(opts.pretty);
        assert!(!opts.compact);
        assert!(!opts.flow_style);
        assert_eq!(opts.root_element, None);
    }
}
