/// CSV/TSV reader and writer.
pub mod csv;
/// .env file reader and writer.
pub mod env;
/// HTML table writer.
pub mod html;
/// INI/CFG configuration file reader and writer.
pub mod ini;
/// JSON reader, writer, and value conversion utilities.
pub mod json;
/// JSON Lines (NDJSON) reader and writer.
pub mod jsonl;
/// Log file reader (Apache, nginx, syslog, custom patterns).
pub mod log;
/// Markdown table writer.
pub mod markdown;
/// Java `.properties` file reader and writer.
pub mod properties;
/// TOML reader and writer.
pub mod toml;
/// YAML reader and writer.
pub mod yaml;

// --- Feature-gated format modules ---

/// MessagePack binary reader and writer.
#[cfg(feature = "msgpack")]
pub mod msgpack;
#[cfg(not(feature = "msgpack"))]
pub mod msgpack {
    //! Stub module — MessagePack feature not enabled.
    use super::{FormatReader, FormatWriter};
    use crate::value::Value;
    use std::io::{Read, Write};

    const MSG: &str = "MessagePack support requires the 'msgpack' feature.\n  Install with: cargo install dkit --features msgpack";

    pub struct MsgpackReader;
    impl MsgpackReader {
        pub fn read_from_bytes(&self, _bytes: &[u8]) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
    }
    impl FormatReader for MsgpackReader {
        fn read(&self, _: &str) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        fn read_from_reader(&self, _: impl Read) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
    }
    pub struct MsgpackWriter;
    impl MsgpackWriter {
        pub fn write_bytes(&self, _value: &Value) -> anyhow::Result<Vec<u8>> {
            anyhow::bail!(MSG)
        }
    }
    impl FormatWriter for MsgpackWriter {
        fn write(&self, _: &Value) -> anyhow::Result<String> {
            anyhow::bail!(MSG)
        }
        fn write_to_writer(&self, _: &Value, _: impl Write) -> anyhow::Result<()> {
            anyhow::bail!(MSG)
        }
    }
}

/// Apache Parquet columnar format reader and writer.
#[cfg(feature = "parquet")]
pub mod parquet;
#[cfg(not(feature = "parquet"))]
pub mod parquet {
    //! Stub module — Parquet feature not enabled.
    use crate::value::Value;

    const MSG: &str = "Parquet support requires the 'parquet' feature.\n  Install with: cargo install dkit --features parquet";

    #[derive(Debug, Clone, Default)]
    pub struct ParquetOptions {
        pub row_group: Option<usize>,
    }
    pub struct ParquetReader {
        _options: ParquetOptions,
    }
    impl ParquetReader {
        pub fn new(options: ParquetOptions) -> Self {
            Self { _options: options }
        }
        pub fn read_from_bytes(&self, _bytes: &[u8]) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        #[allow(dead_code)]
        pub fn read_metadata(_bytes: &[u8]) -> anyhow::Result<ParquetMetadata> {
            anyhow::bail!(MSG)
        }
    }
    #[allow(dead_code)]
    pub struct ParquetMetadata {
        pub num_rows: usize,
        pub num_row_groups: usize,
        pub columns: Vec<String>,
        pub column_types: Vec<String>,
    }
    #[derive(Debug, Clone, Default)]
    pub enum ParquetCompression {
        #[default]
        None,
        Snappy,
        Gzip,
        Zstd,
    }
    impl std::str::FromStr for ParquetCompression {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> anyhow::Result<Self> {
            match s.to_lowercase().as_str() {
                "none" | "uncompressed" => Ok(Self::None),
                "snappy" => Ok(Self::Snappy),
                "gzip" => Ok(Self::Gzip),
                "zstd" => Ok(Self::Zstd),
                _ => anyhow::bail!(
                    "Unknown Parquet compression '{}'. Valid options: none, snappy, gzip, zstd",
                    s
                ),
            }
        }
    }
    #[derive(Debug, Clone, Default)]
    pub struct ParquetWriteOptions {
        pub compression: ParquetCompression,
        pub row_group_size: Option<usize>,
    }
    pub struct ParquetWriter {
        _options: ParquetWriteOptions,
    }
    impl ParquetWriter {
        pub fn new(options: ParquetWriteOptions) -> Self {
            Self { _options: options }
        }
        pub fn write_to_bytes(&self, _value: &Value) -> anyhow::Result<Vec<u8>> {
            anyhow::bail!(MSG)
        }
    }
    /// Stub for arrow_value_to_value when parquet feature is disabled.
    pub fn arrow_value_to_value(_array: &dyn std::any::Any, _idx: usize) -> Value {
        Value::Null
    }
}

/// SQLite database reader.
#[cfg(feature = "sqlite")]
pub mod sqlite;
#[cfg(not(feature = "sqlite"))]
pub mod sqlite {
    //! Stub module — SQLite feature not enabled.
    use crate::value::Value;
    use std::path::Path;

    const MSG: &str = "SQLite support requires the 'sqlite' feature.\n  Install with: cargo install dkit --features sqlite";

    #[derive(Debug, Clone, Default)]
    pub struct SqliteOptions {
        pub table: Option<String>,
        pub sql: Option<String>,
    }
    pub struct SqliteReader {
        _options: SqliteOptions,
    }
    impl SqliteReader {
        pub fn new(options: SqliteOptions) -> Self {
            Self { _options: options }
        }
        pub fn read_from_path(&self, _path: &Path) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        pub fn list_tables(_path: &Path) -> anyhow::Result<Vec<String>> {
            anyhow::bail!(MSG)
        }
    }
}

/// Excel (XLSX) reader.
#[cfg(feature = "excel")]
pub mod xlsx;
#[cfg(not(feature = "excel"))]
pub mod xlsx {
    //! Stub module — Excel feature not enabled.
    use crate::value::Value;

    const MSG: &str = "Excel support requires the 'excel' feature.\n  Install with: cargo install dkit --features excel";

    #[derive(Debug, Clone, Default)]
    pub struct XlsxOptions {
        pub sheet: Option<String>,
        pub header_row: usize,
    }
    pub struct XlsxReader {
        _options: XlsxOptions,
    }
    impl XlsxReader {
        pub fn new(options: XlsxOptions) -> Self {
            Self { _options: options }
        }
        pub fn read_from_bytes(&self, _bytes: &[u8]) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        pub fn list_sheets(_bytes: &[u8]) -> anyhow::Result<Vec<String>> {
            anyhow::bail!(MSG)
        }
    }
}

/// HCL (HashiCorp Configuration Language) reader and writer.
#[cfg(feature = "hcl")]
pub mod hcl;
#[cfg(not(feature = "hcl"))]
pub mod hcl {
    //! Stub module — HCL feature not enabled.
    use super::{FormatReader, FormatWriter};
    use crate::value::Value;
    use std::io::{Read, Write};

    const MSG: &str = "HCL support requires the 'hcl' feature.\n  Install with: cargo install dkit --features hcl";

    pub struct HclReader;
    impl FormatReader for HclReader {
        fn read(&self, _: &str) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        fn read_from_reader(&self, _: impl Read) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
    }
    pub struct HclWriter;
    impl FormatWriter for HclWriter {
        fn write(&self, _: &Value) -> anyhow::Result<String> {
            anyhow::bail!(MSG)
        }
        fn write_to_writer(&self, _: &Value, _: impl Write) -> anyhow::Result<()> {
            anyhow::bail!(MSG)
        }
    }
}

/// macOS Property List (plist) reader and writer.
#[cfg(feature = "plist")]
pub mod plist;
#[cfg(not(feature = "plist"))]
pub mod plist {
    //! Stub module — plist feature not enabled.
    use super::{FormatReader, FormatWriter};
    use crate::value::Value;
    use std::io::{Read, Write};

    const MSG: &str = "Plist support requires the 'plist' feature.\n  Install with: cargo install dkit --features plist";

    pub struct PlistReader;
    impl FormatReader for PlistReader {
        fn read(&self, _: &str) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        fn read_from_reader(&self, _: impl Read) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
    }
    pub struct PlistWriter;
    impl FormatWriter for PlistWriter {
        fn write(&self, _: &Value) -> anyhow::Result<String> {
            anyhow::bail!(MSG)
        }
        fn write_to_writer(&self, _: &Value, _: impl Write) -> anyhow::Result<()> {
            anyhow::bail!(MSG)
        }
    }
}

/// Template-based custom text output writer.
#[cfg(feature = "template")]
pub mod template;
#[cfg(not(feature = "template"))]
pub mod template {
    //! Stub module — Template feature not enabled.
    use super::FormatWriter;
    use crate::value::Value;
    use std::io::Write;

    const MSG: &str = "Template support requires the 'template' feature.\n  Install with: cargo install dkit --features template";

    pub struct TemplateWriter {
        _private: (),
    }
    impl TemplateWriter {
        pub fn new(_options: super::FormatOptions) -> Self {
            Self { _private: () }
        }
    }
    impl FormatWriter for TemplateWriter {
        fn write(&self, _: &Value) -> anyhow::Result<String> {
            anyhow::bail!(MSG)
        }
        fn write_to_writer(&self, _: &Value, _: impl Write) -> anyhow::Result<()> {
            anyhow::bail!(MSG)
        }
    }
}

/// XML reader and writer.
#[cfg(feature = "xml")]
pub mod xml;
#[cfg(not(feature = "xml"))]
pub mod xml {
    //! Stub module — XML feature not enabled.
    use super::{FormatReader, FormatWriter};
    use crate::value::Value;
    use std::io::{Read, Write};

    const MSG: &str = "XML support requires the 'xml' feature.\n  Install with: cargo install dkit --features xml";

    #[derive(Default)]
    pub struct XmlReader {
        _private: (),
    }
    impl XmlReader {
        #[allow(dead_code)]
        pub fn new(_strip_namespaces: bool) -> Self {
            Self { _private: () }
        }
    }
    impl FormatReader for XmlReader {
        fn read(&self, _: &str) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
        fn read_from_reader(&self, _: impl Read) -> anyhow::Result<Value> {
            anyhow::bail!(MSG)
        }
    }
    pub struct XmlWriter {
        _private: (),
    }
    impl XmlWriter {
        pub fn new(_pretty: bool, _root_element: Option<String>) -> Self {
            Self { _private: () }
        }
    }
    impl FormatWriter for XmlWriter {
        fn write(&self, _: &Value) -> anyhow::Result<String> {
            anyhow::bail!(MSG)
        }
        fn write_to_writer(&self, _: &Value, _: impl Write) -> anyhow::Result<()> {
            anyhow::bail!(MSG)
        }
    }
}

use std::io::{Read, Write};
use std::path::Path;

use crate::error::DkitError;
use crate::value::Value;

/// Supported data formats for reading and writing.
///
/// Each variant represents a data serialization format that dkit can
/// convert to or from the unified [`Value`] model.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Format {
    /// JSON (`*.json`)
    Json,
    /// JSON Lines / NDJSON (`*.jsonl`, `*.ndjson`)
    Jsonl,
    /// Comma/Tab-separated values (`*.csv`, `*.tsv`)
    Csv,
    /// YAML (`*.yaml`, `*.yml`)
    Yaml,
    /// TOML (`*.toml`)
    Toml,
    /// XML (`*.xml`)
    Xml,
    /// MessagePack binary format (`*.msgpack`)
    Msgpack,
    /// Excel spreadsheet (`*.xlsx`, read-only)
    Xlsx,
    /// SQLite database (`*.sqlite`, read-only)
    Sqlite,
    /// Apache Parquet columnar format (`*.parquet`)
    Parquet,
    /// Markdown table (write-only)
    Markdown,
    /// HTML table (write-only)
    Html,
    /// Terminal table (write-only, used by `dkit view`)
    Table,
    /// .env file format (`*.env`, `.env.*`)
    Env,
    /// INI/CFG configuration file format (`*.ini`, `*.cfg`)
    Ini,
    /// Java `.properties` file format (`*.properties`)
    Properties,
    /// HCL (HashiCorp Configuration Language) (`*.hcl`, `*.tf`, `*.tfvars`)
    Hcl,
    /// macOS Property List (`*.plist`)
    Plist,
    /// Template-based custom text output (write-only)
    Template,
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
            "env" | "dotenv" => Ok(Format::Env),
            "ini" | "cfg" | "conf" | "config" => Ok(Format::Ini),
            "properties" => Ok(Format::Properties),
            "hcl" | "tf" | "tfvars" => Ok(Format::Hcl),
            "plist" => Ok(Format::Plist),
            "template" | "tpl" => Ok(Format::Template),
            _ => Err(DkitError::UnknownFormat(s.to_string())),
        }
    }

    /// 사용 가능한 출력 포맷 목록을 반환한다
    pub fn list_output_formats() -> Vec<(&'static str, &'static str)> {
        let mut formats = vec![
            ("json", "JSON format"),
            ("csv", "Comma-separated values"),
            ("tsv", "Tab-separated values (CSV variant)"),
            ("yaml", "YAML format"),
            ("toml", "TOML format"),
            ("jsonl", "JSON Lines (one JSON object per line)"),
        ];

        if cfg!(feature = "xml") {
            formats.push(("xml", "XML format"));
        } else {
            formats.push(("xml", "XML format (requires --features xml)"));
        }
        if cfg!(feature = "msgpack") {
            formats.push(("msgpack", "MessagePack binary format"));
        } else {
            formats.push((
                "msgpack",
                "MessagePack binary format (requires --features msgpack)",
            ));
        }
        if cfg!(feature = "excel") {
            formats.push(("xlsx", "Excel spreadsheet (input only)"));
        } else {
            formats.push(("xlsx", "Excel spreadsheet (requires --features excel)"));
        }
        if cfg!(feature = "sqlite") {
            formats.push(("sqlite", "SQLite database (input only)"));
        } else {
            formats.push(("sqlite", "SQLite database (requires --features sqlite)"));
        }
        if cfg!(feature = "parquet") {
            formats.push(("parquet", "Apache Parquet columnar format"));
        } else {
            formats.push((
                "parquet",
                "Apache Parquet columnar format (requires --features parquet)",
            ));
        }

        if cfg!(feature = "hcl") {
            formats.push(("hcl", "HCL (HashiCorp Configuration Language)"));
        } else {
            formats.push((
                "hcl",
                "HCL (HashiCorp Configuration Language) (requires --features hcl)",
            ));
        }

        if cfg!(feature = "plist") {
            formats.push(("plist", "macOS Property List format"));
        } else {
            formats.push((
                "plist",
                "macOS Property List format (requires --features plist)",
            ));
        }

        if cfg!(feature = "template") {
            formats.push(("template", "Custom text output via Tera templates"));
        } else {
            formats.push((
                "template",
                "Custom text output via Tera templates (requires --features template)",
            ));
        }

        formats.push(("env", "Environment variables (.env) format"));
        formats.push(("ini", "INI/CFG configuration file format"));
        formats.push(("properties", "Java .properties file format"));
        formats.push(("md", "Markdown table"));
        formats.push(("html", "HTML table"));
        formats.push(("table", "Terminal table (default for view)"));

        formats
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
            Format::Env => write!(f, "ENV"),
            Format::Ini => write!(f, "INI"),
            Format::Properties => write!(f, "Properties"),
            Format::Hcl => write!(f, "HCL"),
            Format::Plist => write!(f, "Plist"),
            Format::Template => write!(f, "Template"),
        }
    }
}

/// 파일 확장자로 포맷을 자동 감지
pub fn detect_format(path: &Path) -> Result<Format, DkitError> {
    // .env 파일 감지: .env, .env.local, .env.development 등
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if name == ".env" || name.starts_with(".env.") {
            return Ok(Format::Env);
        }
    }

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
        Some("env") => Ok(Format::Env),
        Some("ini" | "cfg") => Ok(Format::Ini),
        Some("properties") => Ok(Format::Properties),
        Some("hcl" | "tf" | "tfvars") => Ok(Format::Hcl),
        Some("plist") => Ok(Format::Plist),
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

    // Plist: <?xml followed by <!DOCTYPE plist or <plist
    if trimmed.starts_with("<?xml") || trimmed.starts_with("<!DOCTYPE") {
        if trimmed.contains("<!DOCTYPE plist") || trimmed.contains("<plist") {
            return Ok((Format::Plist, None));
        }
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

    // ENV: KEY=VALUE 패턴 (대문자 키, = 주변에 공백 없음)
    // TOML과 구별: TOML은 " = " (공백 포함), ENV는 "KEY=value" (공백 없음, 대문자)
    let first_line = trimmed.lines().next().unwrap_or("");
    let ft = first_line.trim();
    let env_line = ft.strip_prefix("export ").unwrap_or(ft);
    if let Some(eq_pos) = env_line.find('=') {
        let key_part = env_line[..eq_pos].trim();
        if !key_part.is_empty()
            && !key_part.contains(' ')
            && key_part
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        {
            // 여러 줄이 모두 ENV 패턴인지 확인
            let env_lines = trimmed
                .lines()
                .filter(|l| {
                    let t = l.trim();
                    !t.is_empty() && !t.starts_with('#')
                })
                .take(5);
            let all_env = env_lines.clone().all(|l| {
                let l = l.trim().strip_prefix("export ").unwrap_or(l.trim());
                if let Some(p) = l.find('=') {
                    let k = l[..p].trim();
                    !k.is_empty()
                        && !k.contains(' ')
                        && k.chars()
                            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                } else {
                    false
                }
            });
            if all_env {
                return Ok((Format::Env, None));
            }
        }
    }

    // TOML: key = value 패턴 (섹션 헤더는 위에서 처리됨)
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

/// Format-specific options controlling how data is read or written.
///
/// Use [`Default::default()`] to get sensible defaults.
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
    /// JSON 들여쓰기 설정 (숫자: 스페이스 수, "tab": 탭 문자)
    pub indent: Option<String>,
    /// JSON 오브젝트 키를 알파벳순으로 정렬
    pub sort_keys: bool,
    /// Inline template string for template output
    pub template: Option<String>,
    /// File path for template output
    pub template_file: Option<String>,
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
            indent: None,
            sort_keys: false,
            template: None,
            template_file: None,
        }
    }
}

/// Trait for reading a data format into a [`Value`].
///
/// Implement this trait to add support for reading a new data format.
#[allow(dead_code)]
pub trait FormatReader {
    /// Parse the given string content and return a [`Value`].
    fn read(&self, input: &str) -> anyhow::Result<Value>;

    /// Parse data from an [`io::Read`](std::io::Read) source and return a [`Value`].
    fn read_from_reader(&self, reader: impl Read) -> anyhow::Result<Value>;
}

/// Trait for writing a [`Value`] to a data format.
///
/// Implement this trait to add support for writing a new data format.
#[allow(dead_code)]
pub trait FormatWriter {
    /// Serialize the given [`Value`] and return the formatted string.
    fn write(&self, value: &Value) -> anyhow::Result<String>;

    /// Serialize the given [`Value`] and write to an [`io::Write`](std::io::Write) destination.
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
