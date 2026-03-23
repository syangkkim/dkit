# dkit Technical Specification

## Value Type

모든 데이터 포맷의 통합 내부 표현.

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}
```

### Accessor Methods

```rust
impl Value {
    pub fn as_bool(&self) -> Option<bool>;
    pub fn as_i64(&self) -> Option<i64>;
    pub fn as_f64(&self) -> Option<f64>;
    pub fn as_str(&self) -> Option<&str>;
    pub fn as_array(&self) -> Option<&Vec<Value>>;
    pub fn as_object(&self) -> Option<&IndexMap<String, Value>>;
    pub fn is_null(&self) -> bool;
}
```

### Key Design Decisions

- `IndexMap` 사용: JSON/YAML 키 순서 보존을 위해 `HashMap` 대신 `IndexMap` 사용
- `Integer` vs `Float` 분리: TOML은 정수/실수를 구분하므로 별도 variant 필요
- `Null` variant: JSON의 null, YAML의 ~, 빈 값 표현

## FormatReader / FormatWriter Traits

```rust
pub trait FormatReader {
    fn read(&self, input: &str) -> Result<Value>;
    fn read_from_reader(&self, reader: impl Read) -> Result<Value>;
}

pub trait FormatWriter {
    fn write(&self, value: &Value) -> Result<String>;
    fn write_to_writer(&self, value: &Value, writer: impl Write) -> Result<()>;
}
```

### Format Options

```rust
pub struct FormatOptions {
    pub delimiter: Option<char>,      // CSV delimiter (default: ',')
    pub no_header: bool,              // CSV without header
    pub pretty: bool,                 // Pretty-print output
    pub compact: bool,                // Compact output (JSON)
    pub flow_style: bool,             // YAML inline style
}
```

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum DkitError {
    #[error("Unknown format: {0}")]
    UnknownFormat(String),

    #[error("Failed to parse {format}: {source}")]
    ParseError {
        format: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Failed to write {format}: {source}")]
    WriteError {
        format: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Invalid query: {0}")]
    QueryError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Path not found: {0}")]
    PathNotFound(String),
}
```

### Format Options

```rust
pub struct FormatOptions {
    pub delimiter: Option<char>,      // CSV delimiter (default: ',')
    pub no_header: bool,              // CSV without header
    pub pretty: bool,                 // Pretty-print output
    pub compact: bool,                // Compact output (JSON)
    pub flow_style: bool,             // YAML inline style
    pub root_element: Option<String>, // XML root element name
}
```

## Format-specific Notes

### JSON ↔ Value

- `serde_json::Value` → `Value` 직접 매핑
- `Number`가 정수이면 `Integer`, 아니면 `Float`

### JSONL (JSON Lines) ↔ Value

- 각 줄을 독립적인 JSON 객체로 파싱
- 빈 줄은 건너뜀
- 읽기: 항상 `Value::Array(Vec<Value>)` 반환
- 쓰기: `Array`이면 요소당 한 줄, 비배열이면 단일 줄 출력
- 에러 메시지에 줄 번호 포함
- 포맷 확장자: `.jsonl`, `.ndjson`

### CSV ↔ Value

- CSV는 항상 `Array(Vec<Object>)` 형태로 변환
- 모든 CSV 값은 기본적으로 `String`
- 숫자 자동 추론: 정수/실수 패턴이면 `Integer`/`Float`로 변환 시도
- `--no-header` 시 컬럼명은 `col0`, `col1`, ... 자동 생성
- **주의**: CSV → JSON → CSV 왕복 시 타입 정보 차이 발생 가능

### YAML ↔ Value

- `serde_yaml::Value` → `Value` 직접 매핑
- YAML의 `~`는 `Null`
- YAML의 `true`/`false`는 `Bool`

### TOML ↔ Value

- TOML은 top-level이 반드시 table(object)
- 배열 데이터를 TOML로 변환 시 `data` 키로 감싸기
- TOML의 `Datetime`은 `String`으로 변환

### XML ↔ Value

- `quick-xml` 이벤트 기반 파서 사용
- XML 속성: `@attr_name` 키로 매핑
- 텍스트 콘텐츠: `#text` 키로 매핑
- 자식 요소: 중첩 Object 또는 Array
- 네임스페이스 접두사 제거 지원
- 텍스트 값 타입 추론: null, true/false, 정수, 실수, 문자열
- `--root-element` 옵션으로 루트 요소 이름 지정 가능
- **주의**: XML → JSON 변환 시 루트 요소가 최상위 키로 포함됨

### MessagePack ↔ Value

- `rmp-serde` 크레이트 사용
- 바이너리 포맷이므로 `read_from_reader`/`write_to_writer` 사용
- JSON과 유사한 타입 매핑

## Query Engine

### Query Grammar (EBNF)

```
query       = path ("|" operation)*
path        = "." segment*
segment     = field | index | iterate
field       = identifier
index       = "[" integer "]"
iterate     = "[]"
operation   = where | select | sort | limit
where       = "where" condition
condition   = expr compare_op expr (logic_op expr compare_op expr)*
select      = "select" field ("," field)*
sort        = "sort" field ["desc"]
limit       = "limit" integer
compare_op  = "==" | "!=" | ">" | "<" | ">=" | "<="
logic_op    = "and" | "or"
expr        = path | string_literal | number_literal
```

### Parser Implementation

재귀 하강 파서(recursive descent parser) 방식으로 구현.

```rust
pub struct QueryParser {
    tokens: Vec<Token>,
    pos: usize,
}

pub enum QueryNode {
    Path(Vec<PathSegment>),
    Where(Condition),
    Select(Vec<String>),
    Sort(String, SortOrder),
    Limit(usize),
    Pipeline(Vec<QueryNode>),
}
```

## Testing Strategy

### Unit Tests

각 모듈별 단위 테스트. `#[cfg(test)]` 모듈 내부에 작성.

### Integration Tests

`tests/` 디렉토리에 CLI 레벨 통합 테스트.

```rust
use assert_cmd::Command;

#[test]
fn convert_json_to_csv() {
    Command::cargo_bin("dkit")
        .unwrap()
        .args(&["convert", "tests/fixtures/users.json", "--to", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,email"));
}
```

### Test Fixtures

`tests/fixtures/` 에 다양한 테스트 데이터 파일 배치. 자세한 목록은 Issue #12 참조.
