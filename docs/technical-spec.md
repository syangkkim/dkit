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
    pub indent: Option<String>,       // JSON indentation ("2", "4", "tab")
    pub sort_keys: bool,              // Sort JSON object keys alphabetically
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

### Markdown → Value (출력 전용)

- GFM (GitHub Flavored Markdown) 테이블 형식
- Array<Object> → 컬럼 헤더 + 데이터 행 (숫자 컬럼 우측 정렬 `---:`)
- Single Object → key | value 2-컬럼 테이블
- Array<Primitive> → 단일 "value" 컬럼 테이블
- 파이프 문자 이스케이프 (`|` → `\|`)
- 중첩 값은 JSON 인라인 표시

### Excel (.xlsx) → Value (입력 전용)

- `calamine` 크레이트 사용 (`.xlsx` 형식 지원)
- 바이트 기반 읽기 (`read_from_bytes`) — 파일 경로가 아닌 바이트에서 파싱
- 시트 선택: 이름 또는 0-based 인덱스 (`--sheet`)
- 헤더 행 지정 (1-based, `--header-row`, 기본값: 1)
- 빈 헤더 셀은 `col1`, `col2`, ... 자동 생성
- 셀 타입 매핑:
  - `Empty` → `Value::Null`
  - `Bool` → `Value::Bool`
  - `Int` → `Value::Integer`
  - `Float` → 정수값이면 `Value::Integer`, 아니면 `Value::Float`
  - `String` → `Value::String`
  - `DateTime` → Excel 시리얼 날짜를 `"YYYY-MM-DD"` 또는 `"YYYY-MM-DD HH:MM:SS"` 문자열로 변환
  - `DateTimeIso`, `DurationIso` → `Value::String`
  - `Error` → `Value::String("#ERROR:...")` 형태
- 누락 컬럼은 `Value::Null`로 채움
- `list_sheets()`: 시트 이름 목록 반환
- **제한**: 입력 전용 (쓰기 불가), `.xls` 미지원

### SQLite (.db, .sqlite) → Value (입력 전용)

- `rusqlite` 크레이트 사용 (읽기 전용 모드: `SQLITE_OPEN_READ_ONLY`)
- 파일 경로 기반 읽기 (`read_from_path`)
- 테이블 선택: `--table` 옵션 (미지정 시 첫 번째 테이블)
- 커스텀 SQL: `--sql` 옵션 (SELECT, JOIN, GROUP BY, 집계 함수 등)
- 타입 매핑:
  - `NULL` → `Value::Null`
  - `INTEGER` → `Value::Integer`
  - `REAL` → `Value::Float`
  - `TEXT` → `Value::String`
  - `BLOB` → `Value::String("x'hex...'")` (16진수 인코딩)
- 테이블 이름 검증 (SQL 인젝션 방지: 영숫자, `_`, `.`만 허용)
- `list_tables()`: 테이블 이름 목록 반환
- **제한**: 입력 전용 (쓰기 불가)

### INI/CFG ↔ Value

- `ini` 크레이트 기반 파서 사용
- 섹션 기반 구조: `[section]` → 중첩 `Object`
- 데이터 모델: `Value::Object(IndexMap<String, Value::Object>)` (섹션 → 키-값)
- 섹션 없는 키는 최상위 Object에 포함
- 주석: `;` 또는 `#`으로 시작하는 줄 (무시)
- 읽기: 섹션명이 최상위 키, 각 섹션 내 키-값이 중첩 Object
- 쓰기: 중첩 Object → `[section]` + `key=value` 형식
- 포맷 자동 감지: `.ini`, `.cfg` 확장자

### .properties ↔ Value

- Java `.properties` 파일 포맷 지원
- `key=value` 또는 `key: value` 또는 `key value` 형식
- 주석: `#` 또는 `!`로 시작하는 줄 (무시)
- `\` 줄 연속 (multiline value) 지원
- 데이터 모델: flat `Value::Object` (모든 값은 `Value::String`)
- 키의 `.` 구분자는 그대로 유지 (flat key)
- 읽기: `Value::Object(IndexMap<String, Value>)` 반환
- 쓰기: `key=value` 형식 출력, 특수문자 포함 시 이스케이프
- 포맷 자동 감지: `.properties` 확장자
- **참고**: 키에 `.`이 포함된 경우 쿼리 시 중첩 구조로 해석되므로, `.` 없는 단순 키 사용 권장

### .env ↔ Value

- `KEY=VALUE` 라인 기반 포맷 (환경 변수 설정 파일)
- 주석: `#` 으로 시작하는 줄 (무시)
- 빈 줄 무시
- `export` 접두사 자동 제거 (`export KEY=VALUE` → `KEY=VALUE`)
- 큰따옴표/작은따옴표 값 지원 (`KEY="value with spaces"`, `KEY='literal'`)
- 큰따옴표 내 이스케이프 시퀀스: `\n`, `\r`, `\t`, `\"`, `\\`
- 인라인 주석 지원 (따옴표 밖 `#` 이후)
- 데이터 모델: flat `Value::Object` (중첩 구조 없음)
- 읽기: 항상 `Value::Object(IndexMap<String, Value>)` 반환 (값은 `Value::String`)
- 쓰기: 특수문자 포함 값은 자동으로 큰따옴표 감싸기
- 빈 값: `KEY=` → `Value::String("")`
- Null 값: `KEY=` 로 출력
- 배열/객체 값: JSON 직렬화 후 작은따옴표로 감싸기
- 포맷 자동 감지: `.env` 확장자
- **제한**: 변수 확장 (`$VAR`, `${VAR}`) 미지원

### HTML → Value (출력 전용)

- HTML 테이블 생성 (Array<Object>, Single Object, Array<Primitive>)
- `--styled`: 인라인 CSS 스타일 (border-collapse, 헤더 다크 배경, 줄무늬 행, 호버 효과)
- `--full-html`: 완전한 HTML 문서 (DOCTYPE, charset, 선택적 style 블록)
- HTML 엔티티 이스케이프 (`&`, `<`, `>`, `"`, `'`)

### HCL (HashiCorp Configuration Language) ↔ Value

- `hcl-rs` 크레이트 사용
- HCL v2 문법 지원 (Terraform 0.12+)
- 블록 구조 → 중첩 Object로 매핑
- 읽기: HCL 파싱 후 `Value::Object` 트리로 변환
- 쓰기: `Value::Object` → HCL 블록/속성 형식 출력
- 포맷 자동 감지: `.hcl`, `.tf`, `.tfvars` 확장자
- feature flag: `hcl` (선택적 의존성)
- **참고**: HCL은 블록 기반 언어로, 단순 key=value가 아닌 복잡한 구조 지원

### plist (macOS Property List) ↔ Value

- `plist` 크레이트 사용
- XML plist 형식 지원
- 데이터 타입 매핑:
  - `dict` → `Value::Object`
  - `array` → `Value::Array`
  - `string` → `Value::String`
  - `integer` → `Value::Integer`
  - `real` → `Value::Float`
  - `true`/`false` → `Value::Bool`
  - `date` → `Value::String` (ISO 8601)
  - `data` → `Value::String` (Base64 인코딩)
- 읽기: XML plist 파싱 후 `Value` 트리로 변환
- 쓰기: `Value` → XML plist 형식 출력
- 포맷 자동 감지: `.plist` 확장자
- feature flag: `plist` (선택적 의존성)

### Log Format Parser

- `dkit-core/src/format/log.rs`에서 구현
- `LogReader`가 `FormatReader` 트레이트를 구현하여 기존 파이프라인과 통합
- 패턴 기반 파서: 포맷 문자열 → 정규식(regex) 컴파일 → 라인별 매칭

#### 사전 정의 포맷

| 포맷명 | 별칭 | 설명 |
|--------|------|------|
| `apache-combined` | `apache`, `combined` | Apache Combined Log Format |
| `apache-common` | `common` | Apache Common Log Format |
| `nginx` | | nginx 기본 access log |
| `syslog` | | RFC 3164 syslog |

#### 사용자 정의 패턴

- `{field_name}` 플레이스홀더를 사용하여 커스텀 패턴 정의
- 예: `{timestamp} [{level}] {message}`
- 각 `{field}`는 정규식 캡처 그룹으로 변환
- 필드 다음 문자에 따라 매칭 전략 결정:
  - 공백: `\S+` (비공백 매칭)
  - `[` 또는 `"`: 해당 구분자까지 매칭
  - 패턴 끝: `.+` (나머지 전체 매칭)

#### 타입 추론

- `"-"` → `Value::Null` (Apache 로그의 빈 필드 표현)
- 정수 패턴 → `Value::Integer`
- 소수점 포함 실수 패턴 → `Value::Float`
- 기타 → `Value::String`

#### 에러 처리 (LogParseErrorMode)

```rust
pub enum LogParseErrorMode {
    Skip,  // 파싱 실패 행 건너뛰기 (기본값)
    Raw,   // 실패 행을 { "_raw": "원본 라인" } 객체로 포함
}
```

## Encoding Support

### 인코딩 감지 우선순위

1. **BOM 감지** (최우선): UTF-8 BOM (`EF BB BF`), UTF-16LE BOM (`FF FE`), UTF-16BE BOM (`FE FF`)
2. **`--encoding <label>`**: 사용자 명시 인코딩 (encoding_rs 지원 레이블)
3. **`--detect-encoding`**: chardetng 휴리스틱 자동 감지
4. **UTF-8 기본값**: 위 3가지 모두 해당 없으면 UTF-8로 디코딩

### 지원 인코딩

- UTF-8 (기본), UTF-16LE, UTF-16BE
- EUC-KR, Shift-JIS
- Latin1 (ISO-8859-1), Windows-1252
- encoding_rs가 지원하는 모든 인코딩

### EncodingOptions

```rust
pub struct EncodingOptions {
    pub encoding: Option<String>,    // 명시 인코딩 레이블
    pub detect_encoding: bool,       // 자동 감지 플래그
}
```

모든 서브커맨드(convert, view, query, stats, schema, merge, diff)에서 인코딩 옵션 지원.

## Query Engine

### Query Grammar (EBNF)

```
query       = path ("|" operation)*
path        = "." segment* | ".." identifier      (* recursive descent *)
segment     = field | index | iterate | wildcard | slice
            | ".." identifier                      (* recursive descent mid-path *)
field       = identifier
index       = "[" integer "]"
iterate     = "[]"
wildcard    = "[" "*" "]"
slice       = "[" [integer] ":" [integer] [":" integer] "]"
operation   = where | select | sort | limit
            | count | sum | avg | min | max | distinct
            | median | percentile | stddev | variance | mode | group_concat
            | group_by
where       = "where" condition
condition   = expr compare_op expr (logic_op expr compare_op expr)*
            | expr "in" "(" value_list ")"
            | expr "not" "in" "(" value_list ")"
            | expr "matches" string_literal
            | expr "not" "matches" string_literal
select      = "select" select_expr ("," select_expr)*
select_expr = expr ["as" identifier]
sort        = "sort" field ["desc"]
limit       = "limit" integer
median      = "median" identifier
percentile  = "percentile" identifier number_literal
stddev      = "stddev" identifier
variance    = "variance" identifier
mode        = "mode" identifier
group_concat = "group_concat" identifier string_literal
compare_op  = "==" | "!=" | ">" | "<" | ">=" | "<="
logic_op    = "and" | "or"
value_list  = value ("," value)*
expr        = path | string_literal | number_literal
            | func_call | if_expr | case_expr
func_call   = identifier "(" [expr ("," expr)*] ")"
if_expr     = "if" "(" condition "," expr "," expr ")"
case_expr   = "case" ("when" condition "then" expr)+ ["else" expr] "end"
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

## Template Output Format

- `tera` 크레이트 사용 (Jinja2 호환 템플릿 엔진)
- feature flag: `template` (선택적 의존성)
- `--template <STRING>`: 인라인 템플릿 문자열
- `--template-file <PATH>`: 파일 기반 템플릿
- 각 레코드마다 템플릿 렌더링, Array일 경우 요소별로 줄바꿈 출력
- Object 값은 키-값 쌍으로 Tera 컨텍스트에 삽입
- 비-Object 값은 `value` 키로 컨텍스트에 삽입
- 내장 필터: `upper`, `lower`, `default` (Tera 내장)
- 파일: `dkit-core/src/format/template.rs`

### FormatOptions 확장

```rust
pub struct FormatOptions {
    // ... 기존 필드 ...
    pub template: Option<String>,       // 인라인 템플릿 문자열
    pub template_file: Option<String>,  // 템플릿 파일 경로
}
```

## JOIN Engine

두 데이터 소스를 공통 키로 결합하는 해시 기반 JOIN 구현.

### Join Types

```rust
pub enum JoinType {
    Inner,  // 양쪽 매칭만
    Left,   // 왼쪽 전체 + 매칭
    Right,  // 오른쪽 전체 + 매칭
    Full,   // 양쪽 전체
}
```

### 알고리즘

- Hash Join: 작은 쪽 테이블을 HashMap에 로드, 큰 쪽을 스트리밍
- 키 명세: `field` (양쪽 동일 키) 또는 `left_field=right_field` (다른 키)
- 입력: Array<Object> (표형 데이터) 필수
- 다양한 입력 포맷 지원 (JSON, CSV, YAML, Excel, SQLite 등)
- 파일: `dkit-cli/src/commands/join.rs`

## Window Functions

쿼리 엔진에 윈도우 함수를 추가하여 행 순서 기반 분석을 지원.

### WindowFunc Enum

```rust
pub enum WindowFunc {
    RowNumber,
    Rank,
    DenseRank,
    Lag { expr: Box<Expr>, offset: Option<i64> },
    Lead { expr: Box<Expr>, offset: Option<i64> },
    FirstValue { expr: Box<Expr>, ignore_nulls: bool },
    LastValue { expr: Box<Expr>, ignore_nulls: bool },
    Aggregate { func: String, field: Option<String> },
}

pub struct WindowSpec {
    pub partition_by: Vec<String>,
    pub order_by: Vec<(String, SortOrder)>,
    pub frame: Option<WindowFrame>,
}
```

### 구현 방식

- 파서: `OVER (PARTITION BY ... ORDER BY ...)` 절 파싱
- 평가기: 데이터를 메모리에 로드 후 파티션별 처리
- `select` 절에서만 사용 가능, 배열 입력 필수
- 파일: `dkit-core/src/query/parser.rs`, `dkit-core/src/query/functions.rs`

## Profile Command

데이터셋의 구조와 품질을 자동으로 프로파일링.

### 프로파일 결과 구조

```rust
struct DatasetProfile {
    total_records: usize,
    total_fields: usize,
    duplicate_rows: usize,
    file_format: String,
    fields: Vec<FieldProfile>,
}

struct FieldProfile {
    name: String,
    inferred_type: &'static str,  // "str", "int", "float", "bool"
    null_percent: f64,
    unique_count: usize,
    top_value: String,
    pattern: String,               // "enum", "text(~N)", "*@*.com", "N-M" 등
    numeric: Option<NumericProfile>,
    string: Option<StringProfile>,
}
```

### 출력 포맷

- `text`/`table`: 테이블 형태 (기본값)
- `json`: JSON 구조화 출력
- `yaml`: YAML 출력
- `md`/`markdown`: Markdown 테이블

### 파일

- `dkit-cli/src/commands/profile.rs`

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
