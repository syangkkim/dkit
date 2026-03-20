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
| `--to <FORMAT>` | | 출력 포맷 (json, csv, yaml, toml) | 필수 |
| `--from <FORMAT>` | | 입력 포맷 (stdin 사용 시 필수) | 확장자 자동 감지 |
| `--output <FILE>` | `-o` | 출력 파일 경로 | stdout |
| `--outdir <DIR>` | | 여러 파일 변환 시 출력 디렉토리 | |
| `--delimiter <CHAR>` | | CSV 구분자 | `,` |
| `--pretty` | | 포맷팅된 출력 | 기본 활성 |
| `--compact` | | 한 줄 출력 (JSON) | |
| `--no-header` | | CSV 헤더 없음 | |
| `--flow` | | YAML 인라인 스타일 | |

### Examples

```bash
# 기본 변환
dkit convert data.json --to yaml
dkit convert users.csv --to json
dkit convert config.yaml --to toml

# 출력 파일 지정
dkit convert data.json --to csv -o output.csv

# 여러 파일
dkit convert *.csv --to json --outdir ./converted/

# stdin/stdout 파이프
cat data.json | dkit convert --from json --to csv
curl https://api.example.com/data | dkit convert --from json --to yaml

# 옵션
dkit convert data.tsv --to json --delimiter '\t'
dkit convert data.csv --to json --compact
dkit convert data.csv --to json --pretty
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

### Examples

```bash
dkit view users.csv
dkit view data.json --path '.users'
dkit view large_data.csv --limit 20
dkit view users.csv --columns name,email
```

## stats

데이터 기본 통계.

### Usage

```bash
dkit stats <INPUT> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--path <QUERY>` | 중첩 데이터 경로 |
| `--column <NAME>` | 특정 컬럼 통계 |

### Output

전체 통계:
```
rows: 1,234
columns: 5 (date, product, region, quantity, revenue)
```

컬럼 통계:
```
type: numeric
count: 1,234
sum: 45,678,900
avg: 37,017.34
min: 1,200
max: 892,000
median: 28,500
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
