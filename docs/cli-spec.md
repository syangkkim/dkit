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
| `--select <FIELDS>` | | 선택할 필드 목록 (쉼표 구분) | 전체 |
| `--group-by <FIELDS>` | | 그룹핑 기준 필드 (쉼표 구분) | |
| `--agg <EXPR>` | | 집계 함수 (`count()`, `sum(field)`, `avg(field)`, `min(field)`, `max(field)`) | |
| `--filter <EXPR>` | | 필터 표현식 | |
| `--sort-by <FIELD>` | | 정렬 기준 필드 | |
| `--sort-order <ORDER>` | | 정렬 방향 (`asc`, `desc`) | `asc` |
| `--head <N>` | | 처음 N개 레코드만 출력 | |
| `--tail <N>` | | 마지막 N개 레코드만 출력 | |
| `--unique` | | 전체 레코드 기준 중복 제거 | false |
| `--unique-by <FIELD>` | | 특정 필드 기준 중복 제거 (첫 번째 유지) | |
| `--add-field <EXPR>` | | 계산 필드 추가 (여러 번 사용 가능, e.g. `total = price * qty`) | |
| `--map <EXPR>` | | 기존 필드 값 변환 (여러 번 사용 가능, e.g. `name = upper(name)`) | |
| `--indent <INDENT>` | | JSON 들여쓰기 (숫자: 스페이스 수, `tab`: 탭 문자) | 2 |
| `--sort-keys` | | JSON 객체 키를 알파벳순으로 정렬 | false |
| `--explode <FIELD>` | | 배열 필드를 개별 행으로 펼침 (여러 번 사용 가능) | |
| `--unpivot <COLUMNS>` | | Wide → Long 변환: 언피벗할 컬럼 목록 (쉼표 구분) | |
| `--pivot` | | Long → Wide 변환 모드 활성화 | false |
| `--key <NAME>` | | 언피벗 시 key 컬럼명 | `variable` |
| `--value <NAME>` | | 언피벗/피벗 시 value 컬럼명 | `value` |
| `--index <FIELDS>` | | 피벗 시 유지할 인덱스 컬럼 (쉼표 구분) | |
| `--columns <FIELD>` | | 피벗 시 새 컬럼명이 될 필드 | |
| `--values <FIELD>` | | 피벗 시 새 컬럼 값이 될 필드 | |
| `--dry-run` | | 미리보기 모드 (파일 생성 없이 stdout으로 출력) | false |
| `--dry-run-limit <N>` | | 미리보기 시 출력 레코드 수 | 10 |

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

# 컬럼 선택 (--select)
dkit convert data.json --to csv --select 'name, email'   # name, email 컬럼만 출력
dkit convert data.csv --to json --select 'id, status'    # 특정 필드만 선택

# 그룹핑 및 집계 (--group-by + --agg)
dkit convert sales.csv --to json --group-by category --agg 'count(), sum(amount)'
dkit convert data.json --to csv --group-by status --agg 'count(), avg(score), min(score), max(score)'

# 필터 + 선택 + 정렬 조합
dkit convert data.json --to csv --select 'name, age' --filter 'age > 30' --sort-by age
dkit convert data.csv --to json --filter 'status == "active"' --head 10

# 미리보기 (--dry-run)
dkit convert huge.json --to csv -o output.csv --dry-run         # 파일 생성 없이 미리보기
dkit convert data.json --to csv --dry-run --dry-run-limit 5     # 처음 5개 레코드만 미리보기

# 중복 제거
dkit convert data.json --to csv --unique                 # 전체 레코드 기준 중복 제거
dkit convert data.json --to csv --unique-by email        # email 기준 중복 제거

# 계산 필드 추가 (--add-field)
dkit convert data.json --to json --add-field 'total = price * quantity'
dkit convert data.csv --to json --add-field 'greeting = name + " from " + city'
dkit convert data.json --to csv --add-field 'double_age = age * 2' --add-field 'label = upper(name)'

# 필드 값 변환 (--map)
dkit convert data.json --to json --map 'name = upper(name)'
dkit convert data.csv --to json --map 'email = lower(email)' --map 'name = trim(name)'

# JSON 출력 옵션
dkit convert data.csv --to json --indent 4               # 4-스페이스 들여쓰기
dkit convert data.csv --to json --indent tab              # 탭 들여쓰기
dkit convert data.csv --to json --sort-keys               # 키 알파벳순 정렬
dkit convert data.csv --to json --compact --sort-keys      # 한 줄 + 키 정렬

# .ini / .cfg 포맷 변환
dkit convert config.ini --to json                        # INI → JSON
dkit convert config.json --to ini -o config.ini          # JSON → INI
dkit convert config.cfg --to yaml                        # CFG → YAML

# .properties 포맷 변환
dkit convert app.properties --to json                    # properties → JSON
dkit convert config.json --to properties -o app.properties  # JSON → properties

# .env 포맷 변환
dkit convert .env --to json                              # .env → JSON
dkit convert config.json --to env -o .env                # JSON → .env
dkit convert .env --to yaml                              # .env → YAML

# HCL (Terraform) 포맷 변환
dkit convert main.tf --to json                           # HCL → JSON
dkit convert variables.json --to hcl -o vars.tf          # JSON → HCL
dkit convert main.tf --to yaml                           # HCL → YAML

# plist (macOS Property List) 포맷 변환
dkit convert Info.plist --to json                        # plist → JSON
dkit convert config.json --to plist -o Info.plist        # JSON → plist

# Explode (배열 필드를 개별 행으로 펼침)
dkit convert data.json --to csv --explode tags           # 배열 → 개별 행
dkit convert data.json --to json --explode tags --explode categories  # 복수 필드

# Unpivot (Wide → Long)
dkit convert wide.csv --to json --unpivot 'jan,feb,mar' --key month --value sales
dkit convert data.json --to csv --unpivot 'q1,q2,q3'    # 기본 키명: variable, value

# Pivot (Long → Wide)
dkit convert long.csv --to json --pivot --index name --columns month --values sales
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
| 멤버십 | `in (v1, v2, ...)`, `not in (v1, v2, ...)` |
| 정규식 | `matches "pattern"`, `not matches "pattern"` |
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

# 배열 슬라이싱 / 와일드카드
dkit query data.json '.[0:3]'                    # 처음 3개 요소
dkit query data.json '.[-2:]'                    # 마지막 2개 요소
dkit query data.json '.[::2]'                    # 짝수 인덱스 요소
dkit query data.json '.[*].name'                 # 모든 요소의 name 필드

# IN / NOT IN 연산자
dkit query data.json '.[] | where status in ("active", "pending")'
dkit query data.json '.[] | where category not in ("deleted", "archived")'

# matches 정규식 연산자
dkit query data.json '.[] | where name matches "^[A-C]"'
dkit query data.json '.[] | where email not matches "@test\\.com$"'
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
| `--select <FIELDS>` | 선택할 필드 목록 (쉼표 구분) | 전체 |
| `--group-by <FIELDS>` | 그룹핑 기준 필드 (쉼표 구분) | |
| `--agg <EXPR>` | 집계 함수 | |
| `--filter <EXPR>` | 필터 표현식 | |
| `--sort-by <FIELD>` | 정렬 기준 필드 | |
| `--head <N>` | 처음 N개 레코드만 출력 | |
| `--tail <N>` | 마지막 N개 레코드만 출력 | |
| `--unique` | 전체 레코드 기준 중복 제거 | false |
| `--unique-by <FIELD>` | 특정 필드 기준 중복 제거 | |
| `--add-field <EXPR>` | 계산 필드 추가 (여러 번 사용 가능) | |
| `--map <EXPR>` | 필드 값 변환 (여러 번 사용 가능) | |

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

# 컬럼 선택 및 집계
dkit view data.json --select 'name, email'       # 특정 필드만 표시
dkit view sales.csv --group-by category --agg 'count(), sum(amount)'  # 집계 테이블
dkit view data.csv --select 'name, score' --filter 'score > 80' --sort-by score

# 중복 제거
dkit view data.csv --unique-by email             # email 기준 유니크 행만 표시

# 필드 추가/변환
dkit view data.json --add-field 'total = price * qty'    # 계산 필드 추가
dkit view data.csv --map 'name = upper(name)'            # 필드 값 변환

# .env 파일 보기
dkit view .env                                   # .env 파일 테이블로 보기

# .ini / .properties 파일 보기
dkit view config.ini                             # INI 파일 보기
dkit view app.properties                         # Properties 파일 보기
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
| `--output-format <FORMAT>` / `-O` | 출력 포맷 (`table`, `json`, `yaml`) — 기본값: `table` |
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

# JSON/YAML 출력 (프로그래밍적 활용)
dkit stats data.csv --output-format json         # JSON 구조화 출력
dkit stats data.csv --output-format yaml         # YAML 출력
dkit stats data.csv --output-format json | dkit query - '.columns[].name'  # 파이프라인
```

## schema

데이터 구조(스키마)를 트리 형태로 출력.

### Usage

```bash
dkit schema <INPUT> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--output-format <FORMAT>` / `-O` | 출력 포맷 (`tree`, `json`, `yaml`) — 기본값: `tree` |

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

### Examples

```bash
dkit schema config.yaml
dkit schema data.json
dkit schema data.json --output-format json       # JSON Schema 형태로 출력
dkit schema data.json --output-format yaml       # YAML 출력
cat data.json | dkit schema - --from json
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

두 데이터 파일의 구조적/값 차이를 비교.

### Usage

```bash
dkit diff <FILE1> <FILE2> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--path <QUERY>` | 특정 경로만 비교 | root |
| `--quiet` | 결과 텍스트 없이 종료 코드만 반환 (0=동일, 1=다름) | false |
| `--mode <MODE>` | 비교 모드: `structural` (추가/삭제/변경), `value` (값 변경만), `key` (키 존재만) | structural |
| `--diff-format <FORMAT>` | 출력 포맷: `unified`, `side-by-side`, `json`, `summary` | unified |
| `--array-diff <STRATEGY>` | 배열 비교 전략: `index` (위치), `value` (값), `key=<field>` (키 필드) | index |
| `--ignore-order` | 배열 요소 순서 무시 | false |
| `--ignore-case` | 문자열 대소문자 무시 | false |
| `--encoding <ENCODING>` | 입력 파일 인코딩 | UTF-8 |
| `--detect-encoding` | 인코딩 자동 감지 | false |

### Examples

```bash
dkit diff old.json new.json
dkit diff config_dev.yaml config_prod.yaml
dkit diff data.json data.xml              # 크로스 포맷 비교
dkit diff a.json b.json --path '.database'
dkit diff a.json b.json --quiet && echo 'same' || echo 'different'

# 비교 모드
dkit diff a.json b.json --mode value          # 값 변경만 표시
dkit diff a.json b.json --mode key            # 키 존재 여부만 표시

# 출력 포맷
dkit diff a.json b.json --diff-format json           # JSON 출력
dkit diff a.json b.json --diff-format side-by-side    # 나란히 비교
dkit diff a.json b.json --diff-format summary         # 요약만 표시

# 배열 비교 전략
dkit diff a.json b.json --array-diff value            # 값 기반 비교
dkit diff a.json b.json --array-diff key=id           # id 필드 기준 매칭

# 비교 옵션
dkit diff a.json b.json --ignore-order                # 배열 순서 무시
dkit diff a.json b.json --ignore-case                 # 대소문자 무시
dkit diff a.json b.json --ignore-order --ignore-case  # 둘 다 무시
```

## validate

JSON Schema를 사용한 데이터 검증.

### Usage

```bash
dkit validate <INPUT> --schema <FILE> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--schema <FILE>` | JSON Schema 파일 경로 | 필수 |
| `--from <FORMAT>` | 입력 포맷 (stdin 사용 시) | 확장자 자동 감지 |
| `--quiet` | 상세 에러 숨기기 (valid/invalid만 출력) | false |
| `--encoding <ENCODING>` | 입력 파일 인코딩 | UTF-8 |
| `--detect-encoding` | 인코딩 자동 감지 | false |

### Output

검증 성공 시:
```
✓ Data is valid
```

검증 실패 시:
```
✗ Validation failed: 2 error(s)
  error: at /age: "thirty" is not of type "integer"
  error: at root: "email" is a required property
```

### Examples

```bash
dkit validate data.json --schema schema.json
dkit validate data.yaml --schema schema.json
dkit validate data.toml --schema schema.json
dkit validate - --schema schema.json --from json < data.json
dkit validate data.json --schema schema.json --quiet
```

## sample

데이터에서 레코드를 샘플링.

### Usage

```bash
dkit sample <INPUT> (-n <N> | --ratio <RATIO>) [OPTIONS]
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--count <N>` | `-n` | 샘플링할 레코드 수 | |
| `--ratio <RATIO>` | | 샘플링 비율 (0.0~1.0) | |
| `--seed <SEED>` | | 재현 가능한 랜덤 시드 | |
| `--method <METHOD>` | | 샘플링 방법: `random`, `systematic`, `stratified` | random |
| `--stratify-by <FIELD>` | | 층화 샘플링 기준 필드 (stratified 필수) | |
| `--from <FORMAT>` | | 입력 포맷 | 확장자 자동 감지 |
| `--format <FORMAT>` | `-f` | 출력 포맷 | 입력과 동일 |
| `--output <FILE>` | `-o` | 출력 파일 | stdout |
| `--pretty` | | 포맷팅된 출력 | false |

### Examples

```bash
dkit sample data.csv -n 100                              # 100개 랜덤 샘플
dkit sample data.json --ratio 0.1                        # 10% 샘플
dkit sample data.csv -n 50 --seed 42                     # 재현 가능한 샘플
dkit sample data.csv -n 100 --method systematic          # 체계적 샘플링
dkit sample data.csv -n 50 --method stratified --stratify-by category  # 층화 샘플링
dkit sample data.csv -n 100 -f json -o sample.json       # CSV → JSON 출력
cat data.json | dkit sample - --from json -n 50          # stdin 입력
```

## flatten

중첩 구조를 평탄화 (dot-notation 키로 변환).

### Usage

```bash
dkit flatten <INPUT> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--separator <SEP>` | 키 구분자 | `.` |
| `--array-format <FORMAT>` | 배열 인덱스 표기: `index` (items.0.name) 또는 `bracket` (items[0].name) | index |
| `--max-depth <N>` | 최대 평탄화 깊이 | 무제한 |
| `--from <FORMAT>` | 입력 포맷 | 확장자 자동 감지 |
| `--format <FORMAT>` | 출력 포맷 | 입력과 동일 |
| `--output <FILE>` | 출력 파일 | stdout |

### Examples

```bash
dkit flatten data.json                                   # 기본 평탄화
dkit flatten data.json --separator '/'                   # 구분자 변경
dkit flatten data.json --array-format bracket            # 배열 브래킷 표기
dkit flatten data.json --max-depth 2                     # 2단계까지만 평탄화
dkit flatten data.json -f yaml -o flat.yaml              # YAML 출력
cat data.json | dkit flatten - --from json               # stdin 입력
```

### Output

입력:
```json
{"server": {"host": "localhost", "port": 8080}, "users": [{"name": "Alice"}]}
```

출력 (index format):
```json
{"server.host": "localhost", "server.port": 8080, "users.0.name": "Alice"}
```

출력 (bracket format):
```json
{"server.host": "localhost", "server.port": 8080, "users[0].name": "Alice"}
```

## unflatten

평탄화된 키를 다시 중첩 구조로 복원.

### Usage

```bash
dkit unflatten <INPUT> [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--separator <SEP>` | 키 구분자 | `.` |
| `--from <FORMAT>` | 입력 포맷 | 확장자 자동 감지 |
| `--format <FORMAT>` | 출력 포맷 | 입력과 동일 |
| `--output <FILE>` | 출력 파일 | stdout |

### Examples

```bash
dkit unflatten flat.json                                 # 기본 복원
dkit unflatten flat.json --separator '/'                 # 구분자 지정
dkit unflatten flat.json -f yaml -o nested.yaml          # YAML 출력
cat flat.json | dkit unflatten - --from json              # stdin 입력

## config

설정 파일 관리.

### Usage

```bash
dkit config show              # 현재 설정 표시
dkit config init              # 사용자 설정 파일 생성
dkit config init --project    # 프로젝트 설정 파일 생성 (.dkit.toml)
```

### 설정 파일 우선순위

높은 우선순위 → 낮은 우선순위:

1. CLI 옵션 (최우선)
2. 프로젝트 설정 (`.dkit.toml` in 현재 디렉토리)
3. 사용자 설정 (`$XDG_CONFIG_HOME/dkit/config.toml` 또는 `~/.dkit.toml`)
4. 기본값

### 설정 파일 형식 (TOML)

```toml
# dkit configuration file

# 기본 출력 포맷 (json, csv, yaml, toml, xml, md, html, table)
# default_format = "json"

# 컬러 출력: "auto", "always", "never"
# color = "auto"

# 기본 입력 인코딩 (예: "utf-8", "euc-kr", "shift_jis")
# encoding = "utf-8"

[table]
# 기본 테이블 테두리 스타일 (simple, rounded, heavy, none, double, ascii)
# border_style = "simple"

# 기본 최대 컬럼 너비
# max_width = 40

[aliases]
# 사용자 정의 별칭
# mytool = "convert --from json --to yaml"
```

### Examples

```bash
dkit config show                    # 현재 유효 설정 표시 (소스 포함)
dkit config init                    # 사용자 설정 파일 생성
dkit config init --project          # 프로젝트 설정 파일 (.dkit.toml) 생성
```

## alias

커맨드 별칭 관리.

### Usage

```bash
dkit alias list                     # 모든 별칭 목록 (내장 + 사용자)
dkit alias set <NAME> <COMMAND>     # 사용자 별칭 등록/수정
dkit alias remove <NAME>            # 사용자 별칭 삭제
```

### 내장 별칭 (Built-in Aliases)

| 별칭 | 확장 커맨드 |
|------|------------|
| `j2c` | `convert --from json --to csv` |
| `c2j` | `convert --from csv --to json` |
| `j2y` | `convert --from json --to yaml` |
| `y2j` | `convert --from yaml --to json` |
| `j2t` | `convert --from json --to toml` |
| `t2j` | `convert --from toml --to json` |
| `c2y` | `convert --from csv --to yaml` |
| `y2c` | `convert --from yaml --to csv` |

### Examples

```bash
dkit j2c data.json                  # JSON → CSV (내장 별칭)
dkit c2j data.csv                   # CSV → JSON (내장 별칭)
dkit alias list                     # 모든 별칭 목록
dkit alias set j2t2c "convert --from json --to csv"  # 사용자 별칭 등록
dkit alias remove j2t2c             # 사용자 별칭 삭제
```

## completions

쉘 자동완성 스크립트 생성.

### Usage

```bash
dkit completions <SHELL>
```

지원 쉘: `bash`, `zsh`, `fish`, `powershell`

### Examples

```bash
# Bash
dkit completions bash > ~/.bash_completion.d/dkit
source ~/.bash_completion.d/dkit

# Zsh
dkit completions zsh > ~/.zfunc/_dkit
# ~/.zshrc에 fpath=(~/.zfunc $fpath) 추가 후 compinit 재실행

# Fish
dkit completions fish > ~/.config/fish/completions/dkit.fish

# PowerShell
dkit completions powershell > dkit.ps1
. ./dkit.ps1
```

## watch 모드

`convert` 및 `view` 커맨드에서 파일 변경을 감지하여 자동으로 재실행.

### Options

| Option | Description |
|--------|-------------|
| `--watch` | 파일 변경 감지 모드 활성화 |
| `--watch-path <PATH>` | 추가 감시 경로 지정 (여러 번 사용 가능) |

### Examples

```bash
dkit convert data.json -f csv --watch                   # 파일 변경 시 자동 변환
dkit view data.csv --watch                              # 파일 변경 시 자동 새로고침
dkit convert data.json -f yaml --watch --watch-path ./templates/  # 추가 경로 감시
```

## 에러 메시지 출력

dkit는 에러 발생 시 색상 강조와 컨텍스트 정보를 포함한 메시지를 출력.

| 에러 종류 | 출력 내용 |
|----------|----------|
| 알 수 없는 포맷 | 지원 포맷 목록 + "Did you mean?" 제안 |
| 파싱 에러 | 줄/열 번호 + 코드 스니펫 + 화살표 |
| 포맷 감지 실패 | `--from` 옵션 사용 힌트 |
| 일반 에러 | `error:` 접두사 + 메시지 |

### 전역 옵션

| Option | Description |
|--------|-------------|
| `--verbose` | 상세 에러 출력 (전체 에러 체인 + 백트레이스) |
```
