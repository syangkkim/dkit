pub mod csv;
pub mod html;
pub mod json;
pub mod jsonl;
pub mod markdown;
pub mod msgpack;
pub mod parquet;
pub mod sqlite;
pub mod toml;
pub mod xlsx;
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
    Xlsx,
    Sqlite,
    Parquet,
    Markdown,
    Html,
    Table,
}

impl Format {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, DkitError> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "jsonl" | "jsonlines" | "ndjson" => Ok(Format::Jsonl),
            "csv" | "tsv" => Ok(Format::Csv),
            "yaml" | "yml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            "xml" => Ok(Format::Xml),
            "msgpack" | "messagepack" => Ok(Format::Msgpack),
            "xlsx" | "excel" | "xls" => Ok(Format::Xlsx),
            "sqlite" | "sqlite3" | "db" => Ok(Format::Sqlite),
            "parquet" | "pq" => Ok(Format::Parquet),
            "md" | "markdown" => Ok(Format::Markdown),
            "html" => Ok(Format::Html),
            "table" => Ok(Format::Table),
            _ => Err(DkitError::UnknownFormat(s.to_string())),
        }
    }

    /// 사용 가능한 출력 포맷 목록을 반환한다
    pub fn list_output_formats() -> &'static [(&'static str, &'static str)] {
        &[
            ("json", "JSON format"),
            ("csv", "Comma-separated values"),
            ("tsv", "Tab-separated values (CSV variant)"),
            ("yaml", "YAML format"),
            ("toml", "TOML format"),
            ("xml", "XML format"),
            ("jsonl", "JSON Lines (one JSON object per line)"),
            ("msgpack", "MessagePack binary format"),
            ("xlsx", "Excel spreadsheet (input only)"),
            ("sqlite", "SQLite database (input only)"),
            ("parquet", "Apache Parquet columnar format"),
            ("md", "Markdown table"),
            ("html", "HTML table"),
            ("table", "Terminal table (default for view)"),
        ]
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
            Format::Xlsx => write!(f, "Excel"),
            Format::Sqlite => write!(f, "SQLite"),
            Format::Parquet => write!(f, "Parquet"),
            Format::Markdown => write!(f, "Markdown"),
            Format::Html => write!(f, "HTML"),
            Format::Table => write!(f, "Table"),
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
        Some("xlsx" | "xls" | "xlsm" | "xlsb" | "ods") => Ok(Format::Xlsx),
        Some("db" | "sqlite" | "sqlite3") => Ok(Format::Sqlite),
        Some("parquet" | "pq") => Ok(Format::Parquet),
        Some("md") => Ok(Format::Markdown),
        Some("html") => Ok(Format::Html),
        Some(ext) => Err(DkitError::UnknownFormat(ext.to_string())),
        None => Err(DkitError::UnknownFormat("(no extension)".to_string())),
    }
}

/// 콘텐츠 스니핑으로 포맷을 자동 감지
///
/// 감지 우선순위:
/// 1. `<?xml` → XML
/// 2. 첫 줄이 JSON 객체 + 둘째 줄도 JSON 객체 → JSONL
/// 3. `{` 또는 `[` 시작 → JSON
/// 4. 탭 구분자가 포함된 구조적 데이터 → CSV (TSV)
/// 5. TOML 패턴 (키 = 값, [섹션])
/// 6. YAML 패턴 (키: 값, ---)
pub fn detect_format_from_content(content: &str) -> Result<(Format, Option<char>), DkitError> {
    let trimmed = content.trim_start();

    if trimmed.is_empty() {
        return Err(DkitError::FormatDetectionFailed(
            "input is empty".to_string(),
        ));
    }

    // XML: <?xml 또는 루트 태그로 시작
    if trimmed.starts_with("<?xml") || trimmed.starts_with("<!DOCTYPE") {
        return Ok((Format::Xml, None));
    }

    // JSONL: 첫째 줄과 둘째 줄 모두 JSON 객체
    let mut lines = trimmed.lines().filter(|l| !l.trim().is_empty());
    if let Some(first_line) = lines.next() {
        if let Some(second_line) = lines.next() {
            let first_trimmed = first_line.trim();
            let second_trimmed = second_line.trim();
            if first_trimmed.starts_with('{')
                && first_trimmed.ends_with('}')
                && second_trimmed.starts_with('{')
                && second_trimmed.ends_with('}')
            {
                return Ok((Format::Jsonl, None));
            }
        }
    }

    // JSON: { 로 시작 (단일 객체)
    if trimmed.starts_with('{') {
        return Ok((Format::Json, None));
    }

    // [ 로 시작: JSON 배열 vs TOML 섹션 헤더 구분
    // TOML 섹션: [word] 형태 (내부가 알파벳/밑줄/점/하이픈)
    // JSON 배열: [값, ...] 또는 여러 줄에 걸친 배열
    if trimmed.starts_with('[') {
        let first_line = trimmed.lines().next().unwrap_or("").trim();
        // TOML 섹션 헤더: [section] 또는 [[array]]
        let is_toml_section = first_line.starts_with("[[")
            || (first_line.starts_with('[')
                && first_line.ends_with(']')
                && !first_line.contains(',')
                && first_line[1..first_line.len() - 1].chars().all(|c| {
                    c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == ' ' || c == '"'
                }));
        if is_toml_section {
            return Ok((Format::Toml, None));
        }
        return Ok((Format::Json, None));
    }

    // XML: < 로 시작하는 태그 (<?xml 없이 바로 태그로 시작하는 경우)
    if trimmed.starts_with('<') {
        return Ok((Format::Xml, None));
    }

    // TSV: 첫째 줄에 탭이 포함되어 있으면 TSV로 간주
    if let Some(first_line) = trimmed.lines().next() {
        if first_line.contains('\t') {
            return Ok((Format::Csv, Some('\t')));
        }
    }

    // TOML: key = value 패턴 (섹션 헤더는 위에서 처리됨)
    let first_line = trimmed.lines().next().unwrap_or("");
    let ft = first_line.trim();
    if ft.contains(" = ") {
        return Ok((Format::Toml, None));
    }

    // YAML: --- 또는 key: value 패턴
    if ft.starts_with("---") || ft.contains(": ") || ft.ends_with(':') {
        return Ok((Format::Yaml, None));
    }

    // CSV: 콤마가 포함된 구조적 데이터
    if ft.contains(',') {
        return Ok((Format::Csv, None));
    }

    Err(DkitError::FormatDetectionFailed(
        "could not determine format from content".to_string(),
    ))
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
    /// HTML 인라인 CSS 스타일 포함
    pub styled: bool,
    /// HTML 완전한 문서 출력
    pub full_html: bool,
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
            styled: false,
            full_html: false,
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
    fn test_format_from_str_markdown() {
        assert_eq!(Format::from_str("md").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
        assert_eq!(Format::from_str("MD").unwrap(), Format::Markdown);
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
        assert_eq!(Format::Markdown.to_string(), "Markdown");
        assert_eq!(Format::Table.to_string(), "Table");
    }

    #[test]
    fn test_format_from_str_table() {
        assert_eq!(Format::from_str("table").unwrap(), Format::Table);
        assert_eq!(Format::from_str("TABLE").unwrap(), Format::Table);
    }

    #[test]
    fn test_list_output_formats() {
        let formats = Format::list_output_formats();
        assert!(formats.len() >= 10);
        assert!(formats.iter().any(|(name, _)| *name == "table"));
        assert!(formats.iter().any(|(name, _)| *name == "json"));
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
    fn test_detect_format_markdown() {
        assert_eq!(
            detect_format(&PathBuf::from("output.md")).unwrap(),
            Format::Markdown
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

    // --- detect_format_from_content ---

    #[test]
    fn test_sniff_xml_declaration() {
        let (fmt, delim) = detect_format_from_content("<?xml version=\"1.0\"?>\n<root/>").unwrap();
        assert_eq!(fmt, Format::Xml);
        assert_eq!(delim, None);
    }

    #[test]
    fn test_sniff_xml_tag() {
        let (fmt, _) = detect_format_from_content("<root><item>hello</item></root>").unwrap();
        assert_eq!(fmt, Format::Xml);
    }

    #[test]
    fn test_sniff_json_object() {
        let (fmt, _) = detect_format_from_content("{\"name\": \"Alice\"}").unwrap();
        assert_eq!(fmt, Format::Json);
    }

    #[test]
    fn test_sniff_json_array() {
        let (fmt, _) = detect_format_from_content("[1, 2, 3]").unwrap();
        assert_eq!(fmt, Format::Json);
    }

    #[test]
    fn test_sniff_jsonl() {
        let content = "{\"name\": \"Alice\"}\n{\"name\": \"Bob\"}\n";
        let (fmt, _) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Jsonl);
    }

    #[test]
    fn test_sniff_tsv() {
        let content = "name\tage\tcity\nAlice\t30\tSeoul\n";
        let (fmt, delim) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Csv);
        assert_eq!(delim, Some('\t'));
    }

    #[test]
    fn test_sniff_toml_section() {
        let content = "[database]\nhost = \"localhost\"\nport = 5432\n";
        let (fmt, _) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Toml);
    }

    #[test]
    fn test_sniff_toml_key_value() {
        let content = "title = \"My App\"\nversion = \"1.0\"\n";
        let (fmt, _) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Toml);
    }

    #[test]
    fn test_sniff_yaml_document() {
        let content = "---\nname: Alice\nage: 30\n";
        let (fmt, _) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Yaml);
    }

    #[test]
    fn test_sniff_yaml_key_value() {
        let content = "name: Alice\nage: 30\n";
        let (fmt, _) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Yaml);
    }

    #[test]
    fn test_sniff_csv() {
        let content = "name,age,city\nAlice,30,Seoul\n";
        let (fmt, delim) = detect_format_from_content(content).unwrap();
        assert_eq!(fmt, Format::Csv);
        assert_eq!(delim, None);
    }

    #[test]
    fn test_sniff_empty_content() {
        let err = detect_format_from_content("").unwrap_err();
        assert!(matches!(err, DkitError::FormatDetectionFailed(_)));
    }

    #[test]
    fn test_sniff_whitespace_only() {
        let err = detect_format_from_content("   \n  \n").unwrap_err();
        assert!(matches!(err, DkitError::FormatDetectionFailed(_)));
    }
}
