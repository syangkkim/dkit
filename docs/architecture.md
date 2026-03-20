# dkit Architecture

## Overview

dkit은 모든 데이터 포맷을 하나의 CLI로 변환하고 쿼리하는 도구이다.

```
입력 파일 → Reader(포맷별) → Value → Writer(포맷별) → 출력
                                ↑
                          Query 엔진이 여기서 처리
```

## Internal Data Model

모든 포맷은 내부적으로 통합 `Value` 타입으로 변환된 뒤 처리된다.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),  // 키 순서 보존
}
```

## Module Structure

```
src/
├── main.rs                 # CLI 엔트리포인트
├── cli.rs                  # clap 서브커맨드 정의
├── value.rs                # 통합 Value 타입
├── error.rs                # 에러 타입 정의
│
├── format/                 # 포맷별 Reader/Writer
│   ├── mod.rs              # FormatReader/Writer 트레이트
│   ├── json.rs
│   ├── csv.rs
│   ├── yaml.rs
│   └── toml.rs
│
├── query/                  # 쿼리 엔진
│   ├── mod.rs
│   ├── parser.rs           # 쿼리 문법 파서
│   ├── evaluator.rs        # 쿼리 실행기
│   └── filter.rs           # where/sort/limit 처리
│
├── commands/               # 서브커맨드 구현
│   ├── mod.rs
│   ├── convert.rs
│   ├── query.rs
│   ├── view.rs
│   ├── stats.rs
│   └── schema.rs
│
└── output/                 # 출력 포맷터
    ├── mod.rs
    ├── table.rs            # 테이블 출력
    └── pretty.rs           # 색상 하이라이팅
```

## Core Traits

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

## Format Detection

```rust
pub fn detect_format(path: &Path) -> Result<Format> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Ok(Format::Json),
        Some("csv" | "tsv") => Ok(Format::Csv),
        Some("yaml" | "yml") => Ok(Format::Yaml),
        Some("toml") => Ok(Format::Toml),
        _ => Err(Error::UnknownFormat),
    }
}
```

## Dependencies

| 용도 | 크레이트 | 버전 | 이유 |
|------|---------|------|------|
| CLI 파싱 | clap (derive) | 4.x | Rust CLI 표준, 서브커맨드 지원 |
| JSON | serde_json | 1.x | serde 생태계 표준 |
| YAML | serde_yaml | 0.9.x | serde 통합 |
| CSV | csv | 1.x | BurntSushi 제작, 최고 성능 |
| TOML | toml | 0.8.x | serde 통합 |
| 순서 보존 Map | indexmap | 2.x | JSON/YAML 키 순서 유지 |
| 테이블 출력 | comfy-table | 7.x | 예쁜 터미널 테이블 |
| 색상 출력 | colored | 2.x | 터미널 하이라이팅 |
| 에러 처리 | thiserror + anyhow | 1.x / 1.x | 라이브러리 + 애플리케이션 에러 |

## Conversion Matrix (v0.1)

| FROM \ TO | JSON | CSV | YAML | TOML |
|-----------|------|-----|------|------|
| JSON | - | O | O | O |
| CSV | O | - | O | O |
| YAML | O | O | - | O |
| TOML | O | O | O | - |
