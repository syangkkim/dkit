# dkit CLI Specification

## Command Structure

```
dkit <command> [options] [arguments]

Commands:
  convert   포맷 간 변환
  query     데이터 쿼리/필터
  view      데이터 미리보기 (테이블 출력)
  stats     기본 통계
  merge     여러 파일 합치기
  schema    데이터 구조(스키마) 출력
```

## convert

포맷 간 데이터 변환.

### Usage

```bash
dkit convert <INPUT> --to <FORMAT> [OPTIONS]
dkit convert --from <FORMAT> --to <FORMAT>  # stdin 사용 시
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--to <FORMAT>` | | 출력 포맷 (json, jsonl, csv, yaml, toml, xml, msgpack, md, html) | 필수 |
| `--from <FORMAT>` | | 입력 포맷 (stdin 사용 시 필수, 콘텐츠 스니핑 지원) | 확장자 자동 감지 |
| `--output <FILE>` | `-o` | 출력 파일 경로 | stdout |
| `--outdir <DIR>` | | 여러 파일 변환 시 출력 디렉토리 | |
| `--delimiter <CHAR>` | | CSV 구분자 | `,` |
| `--pretty` | | 포맷팅된 출력 | 기본 활성 |
| `--compact` | | 한 줄 출력 (JSON) | |
| `--no-header` | | CSV 헤더 없음 | |
| `--flow` | | YAML 인라인 스타일 | |
| `--root-element <NAME>` | | XML 루트 요소 이름 | `root` |
| `--styled` | | HTML 출력 시 인라인 CSS 스타일 포함 | false |
| `--full-html` | | HTML 출력 시 완전한 HTML 문서로 출력 | false |
| `--encoding <ENCODING>` | | 입력 파일 인코딩 (euc-kr, shift_jis, latin1 등) | UTF-8 |
| `--detect-encoding` | | 입력 파일 인코딩 자동 감지 | false |
| `--sheet <SHEET>` | | Excel 시트 이름 또는 0-based 인덱스 | 첫 번째 시트 |
| `--header-row <N>` | | Excel 헤더 행 번호 (1-based) | 1 |
| `--list-sheets` | | Excel 시트 목록 출력 | |
| `--table <TABLE>` | | SQLite 테이블 이름 | 첫 번째 테이블 |
| `--sql <SQL>` | | SQLite 커스텀 SQL 쿼리 | |
| `--list-tables` | | SQLite 테이블 목록 출력 | |
| `--outdir <DIR>` | | 일괄 변환 시 출력 디렉토리 | |
| `--rename <PATTERN>` | | 일괄 변환 시 파일명 패턴 (`{name}`, `{ext}`) | |
| `--continue-on-error` | | 일괄 변환 시 에러 발생해도 계속 진행 | false |

### Examples

```bash
# 기본 변환
dkit convert data.json --to yaml
dkit convert users.csv --to json
dkit convert config.yaml --to toml

# XML 변환
dkit convert config.xml --to json
dkit convert data.json --to xml
dkit convert config.xml --to yaml
dkit convert data.json --to xml --root-element users

# JSONL (JSON Lines) 변환
dkit convert users.json --to jsonl        # JSON 배열 → 줄 단위 객체
dkit convert logs.jsonl --to json         # JSONL → JSON 배열
dkit convert logs.jsonl --to csv          # JSONL → CSV

# 출력 파일 지정
dkit convert data.json --to csv -o output.csv

# 여러 파일
dkit convert *.csv --to json --outdir ./converted/

# stdin/stdout 파이프
cat data.json | dkit convert --from json --to csv
cat logs.jsonl | dkit convert --from jsonl --to json

# 옵션
dkit convert data.tsv --to json --delimiter '\t'
dkit convert data.csv --to json --compact
dkit convert data.csv --to json --pretty

# Markdown/HTML 출력
dkit convert data.json --to md                           # GFM Markdown 테이블
dkit convert data.csv --to html                          # HTML 테이블
dkit convert data.json --to html --styled                # 인라인 CSS 스타일
dkit convert data.json --to html --full-html             # 완전한 HTML 문서
dkit convert data.json --to html --styled --full-html    # 스타일 포함 HTML 문서

# Excel (.xlsx) 입력
dkit convert data.xlsx --to json                         # Excel → JSON
dkit convert data.xlsx --to csv --sheet Products         # 시트 이름으로 선택
dkit convert data.xlsx --to yaml --sheet 1               # 시트 인덱스로 선택 (0-based)
dkit convert data.xlsx --to json --header-row 2          # 헤더 행 지정
dkit view data.xlsx --list-sheets                        # 시트 목록 출력

# SQLite (.db, .sqlite) 입력
dkit convert data.db --to json                           # SQLite → JSON
dkit convert data.db --to csv --table users              # 테이블 지정
dkit convert data.db --to json --sql "SELECT name, age FROM users WHERE age > 25"  # 커스텀 SQL
dkit view data.db --list-tables                          # 테이블 목록 출력

# 일괄 변환
dkit convert *.json --to csv --outdir ./out/             # glob 패턴
dkit convert ./data/ --to yaml --outdir ./out/           # 디렉토리 입력
dkit convert a.json b.csv --to yaml --outdir ./out/      # 여러 파일 명시
dkit convert *.json --to csv --outdir ./out/ --rename "{name}.converted.{ext}"  # 파일명 패턴
dkit convert ./data/ --to csv --outdir ./out/ --continue-on-error  # 에러 무시하고 계속

# 인코딩 변환
dkit convert data.csv --to json --encoding euc-kr        # EUC-KR 입력
dkit convert data.csv --to json --encoding shift_jis     # Shift-JIS 입력
dkit convert data.csv --to json --encoding latin1        # Latin1 입력
dkit convert data.csv --to json --detect-encoding        # 인코딩 자동 감지
```

## query

데이터 쿼리/필터링.

### Usage

```bash
dkit query <INPUT> '<QUERY>' [OPTIONS]
```

### Query Syntax

```
# 필드 접근
.field
.field.subfield

# 배열 접근
.array[0]
.array[-1]
.array[]          # 모든 요소 이터레이션

# 필터링
.items[] | where <condition>
.items[] | where field > value
.items[] | where field == "string"
.items[] | where field contains "substr"

# 선택
.items[] | select field1, field2

# 정렬/제한
.items[] | sort field [desc]
.items[] | limit N

# 조합
.items[] | where age > 25 and city == "Seoul" | select name, email | sort name | limit 10
```

### Operators

| Type | Operators |
|------|-----------|
| 비교 | `==`, `!=`, `>`, `<`, `>=`, `<=` |
| 문자열 | `contains`, `starts_with`, `ends_with` |
| 논리 | `and`, `or` |

### Options

| Option | Description |
|--------|-------------|
| `--to <FORMAT>` | 결과를 다른 포맷으로 출력 |
| `-o <FILE>` | 출력 파일 |

### Examples

```bash
dkit query config.yaml '.database.host'
dkit query data.json '.users[0].name'
dkit query data.json '.users[] | where age > 30'
dkit query data.json '.users[] | where age > 30 | select name, email'
dkit query users.csv '.rows[] | where age > 30' --to json -o filtered.json
```

## view

데이터를 테이블 형태로 미리보기.

### Usage

```bash
dkit view <INPUT> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--path <QUERY>` | 중첩 데이터 경로 | root |
| `--limit <N>` | 표시할 행 수 | 전체 |
| `--columns <COLS>` | 표시할 컬럼 (쉼표 구분) | 전체 |
| `--max-width <N>` | 컬럼 최대 너비 (긴 값 잘라내기) | 제한 없음 |
| `--hide-header` | 헤더 행 숨기기 | false |
| `--row-numbers` | 행 번호 표시 | false |
| `--border <STYLE>` | 테이블 테두리 스타일 (none, simple, rounded, heavy) | simple |
| `--color` | 데이터 타입별 색상 출력 (숫자=청색, null=회색, 불리언=노란색) | false |
| `--format <FORMAT>` | 출력 포맷 (table, json, csv, yaml, md, html 등) | table |
| `--encoding <ENCODING>` | 입력 파일 인코딩 (euc-kr, shift_jis, latin1 등) | UTF-8 |
| `--detect-encoding` | 입력 파일 인코딩 자동 감지 | false |
| `--sheet <SHEET>` | Excel 시트 이름 또는 인덱스 | 첫 번째 시트 |
| `--list-sheets` | Excel 시트 목록 출력 | |
| `--table <TABLE>` | SQLite 테이블 이름 | 첫 번째 테이블 |
| `--list-tables` | SQLite 테이블 목록 출력 | |

### Examples

```bash
dkit view users.csv
dkit view data.json --path '.users'
dkit view large_data.csv --limit 20
dkit view users.csv --columns name,email
dkit view data.csv --border rounded --color
dkit view data.json --row-numbers --max-width 30
dkit view data.json --hide-header --border none

# 출력 포맷 변경
dkit view data.json --format json
dkit view data.json --format md
dkit view data.json --format html

# 인코딩
dkit view korean.csv --encoding euc-kr
dkit view data.csv --detect-encoding

# Excel
dkit view data.xlsx                              # Excel 파일 미리보기
dkit view data.xlsx --sheet Products             # 특정 시트 보기
dkit view data.xlsx --list-sheets                # 시트 목록

# SQLite
dkit view data.db                                # SQLite 테이블 미리보기
dkit view data.db --table users                  # 특정 테이블 보기
dkit view data.db --list-tables                  # 테이블 목록
```

## stats

데이터 통계 (필드별 상세 통계 포함).

### Usage

```bash
dkit stats <INPUT> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--path <QUERY>` | 중첩 데이터 경로 |
| `--column <NAME>` | 특정 컬럼 통계 |
| `--field <NAME>` | 특정 필드 상세 분석 (`--column` 별칭) |
| `-f, --format <FORMAT>` | 출력 포맷 (`json`, `table`, `md`) |
| `--histogram` | 숫자 필드에 텍스트 히스토그램 출력 |

### Output

전체 통계:
```
rows: 1,234
columns: 5 (date, product, region, quantity, revenue)
```

숫자형 컬럼 통계:
```
type: numeric
count: 1,234
sum: 45,678,900
avg: 37,017.34
std: 12,345.67
min: 1,200
p25: 18,000
median: 28,500
p75: 52,000
max: 892,000
```

문자열형 컬럼 통계:
```
type: string
count: 1,234
unique: 42
min_length: 3
max_length: 50
avg_length: 12.30
top_values:
  Seoul (234)
  Busan (189)
  Incheon (156)
```

null 비율 (missing이 있을 때):
```
missing: 15 (1.2%)
```

타입 혼재 감지:
```
⚠ mixed types: integer(100), string(20)
```

### Examples

```bash
dkit stats data.csv
dkit stats data.json --path .users
dkit stats data.csv --column revenue
dkit stats data.csv --field revenue --format json
dkit stats data.csv --format md
dkit stats data.csv --column age --histogram
```

## schema

데이터 구조(스키마)를 트리 형태로 출력.

### Usage

```bash
dkit schema <INPUT>
```

### Output

```
root: object
├─ database: object
│  ├─ host: string
│  ├─ port: integer
│  └─ name: string
├─ server: object
│  ├─ port: integer
│  └─ debug: boolean
└─ users: array[object]
   ├─ name: string
   └─ email: string
```

## merge

여러 파일을 하나로 합치기.

### Usage

```bash
dkit merge <INPUT...> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--to <FORMAT>` | 출력 포맷 |
| `-o <FILE>` | 출력 파일 |

## diff

두 데이터 파일 비교.

### Usage

```bash
dkit diff <FILE1> <FILE2> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--path <QUERY>` | 특정 경로만 비교 |
| `--quiet` | 결과 텍스트 없이 종료 코드만 반환 (0=동일, 1=다름) |

### Examples

```bash
dkit diff old.json new.json
dkit diff config_dev.yaml config_prod.yaml
dkit diff data.json data.xml              # 크로스 포맷 비교
dkit diff a.json b.json --path '.database'
dkit diff a.json b.json --quiet && echo 'same' || echo 'different'
```
