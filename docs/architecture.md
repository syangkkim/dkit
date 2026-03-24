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
│   ├── mod.rs              # FormatReader/Writer 트레이트, 포맷 감지
│   ├── json.rs
│   ├── jsonl.rs            # JSON Lines (JSONL/NDJSON)
│   ├── csv.rs
│   ├── yaml.rs
│   ├── toml.rs
│   ├── xml.rs              # XML (quick-xml 기반)
│   ├── msgpack.rs          # MessagePack
│   ├── markdown.rs         # Markdown 테이블 (GFM, 출력 전용)
│   ├── html.rs             # HTML 테이블 (출력 전용)
│   ├── xlsx.rs             # Excel (.xlsx, 입력 전용, calamine)
│   └── sqlite.rs           # SQLite (.db/.sqlite, 입력 전용, rusqlite)
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
│   ├── schema.rs
│   ├── merge.rs
│   └── diff.rs
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

포맷 감지는 두 가지 전략을 사용한다:

1. **파일 확장자**: `.json`, `.jsonl`/`.ndjson`, `.csv`/`.tsv`, `.yaml`/`.yml`, `.toml`, `.xml`, `.msgpack`, `.xlsx`, `.db`/`.sqlite`, `.md` (출력 전용)
2. **콘텐츠 스니핑**: stdin 입력 시 내용 기반으로 포맷을 추론 (XML → JSONL → JSON → TOML → YAML → CSV 순)
3. **인코딩 감지**: BOM 우선 → `--encoding` 명시 → `--detect-encoding` 자동 감지 → UTF-8 기본
4. **바이너리 포맷**: Excel(.xlsx)은 바이트 기반 읽기, SQLite는 파일 경로 기반 읽기

## Dependencies

| 용도 | 크레이트 | 버전 | 이유 |
|------|---------|------|------|
| CLI 파싱 | clap (derive) | 4.x | Rust CLI 표준, 서브커맨드 지원 |
| JSON | serde_json | 1.x | serde 생태계 표준 |
| YAML | serde_yaml | 0.9.x | serde 통합 |
| CSV | csv | 1.x | BurntSushi 제작, 최고 성능 |
| TOML | toml | 0.8.x | serde 통합 |
| XML | quick-xml | 0.37.x | 고성능 XML 파서/시리얼라이저 |
| MessagePack | rmp-serde | 1.x | MessagePack serde 통합 |
| 순서 보존 Map | indexmap | 2.x | JSON/YAML 키 순서 유지 |
| 테이블 출력 | comfy-table | 7.x | 예쁜 터미널 테이블 |
| 색상 출력 | colored | 2.x | 터미널 하이라이팅 |
| 에러 처리 | thiserror + anyhow | 1.x / 1.x | 라이브러리 + 애플리케이션 에러 |
| 인코딩 | encoding_rs | 0.8.x | 다중 인코딩 지원 (EUC-KR, Shift-JIS, Latin1 등) |
| 인코딩 자동 감지 | chardetng | 0.1.x | BOM 없는 파일의 인코딩 휴리스틱 감지 |
| Excel 읽기 | calamine | 0.26.x | .xlsx 파일 파싱 (입력 전용) |
| SQLite 읽기 | rusqlite | 0.32.x | SQLite 데이터베이스 읽기 (입력 전용) |

## Conversion Matrix (v0.6)

| FROM \ TO | JSON | JSONL | CSV | YAML | TOML | XML | MsgPack | MD | HTML |
|-----------|------|-------|-----|------|------|-----|---------|----|------|
| JSON      | -    | O     | O   | O    | O    | O   | O       | O  | O    |
| JSONL     | O    | -     | O   | O    | O    | O   | O       | O  | O    |
| CSV       | O    | O     | -   | O    | O    | O   | O       | O  | O    |
| YAML      | O    | O     | O   | -    | O    | O   | O       | O  | O    |
| TOML      | O    | O     | O   | O    | -    | O   | O       | O  | O    |
| XML       | O    | O     | O*  | O    | O    | -   | O       | O  | O    |
| MsgPack   | O    | O     | O   | O    | O    | O   | -       | O  | O    |
| Excel     | O    | O     | O   | O    | O    | O   | O       | O  | O    |
| SQLite    | O    | O     | O   | O    | O    | O   | O       | O  | O    |

*XML → CSV는 데이터가 Array of Objects 구조인 경우에만 가능
**MD, HTML은 출력 전용 포맷 (Write-only)
***Excel, SQLite은 입력 전용 포맷 (Read-only)
