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

## Format-specific Notes

### JSON ↔ Value

- `serde_json::Value` → `Value` 직접 매핑
- `Number`가 정수이면 `Integer`, 아니면 `Float`

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
