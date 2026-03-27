# dkit Query Syntax

dkit의 쿼리 문법은 jq에서 영감을 받았지만, 더 직관적이고 배우기 쉽게 설계되었다.

## Path Access

데이터의 특정 위치에 접근한다.

```bash
# 필드 접근
dkit query data.json '.name'
dkit query data.json '.database.host'

# 배열 인덱스
dkit query data.json '.users[0]'
dkit query data.json '.users[-1]'        # 마지막 요소
dkit query data.json '.users[0].name'    # 배열 요소의 필드

# 배열 이터레이션 (모든 요소)
dkit query data.json '.users[]'
dkit query data.json '.users[].name'     # 모든 요소의 name 필드

# 배열 와일드카드
dkit query data.json '.[*].name'         # [*]는 []와 동일

# 배열 슬라이싱
dkit query data.json '.[0:3]'            # 처음 3개 요소 (인덱스 0, 1, 2)
dkit query data.json '.[2:]'             # 인덱스 2부터 끝까지
dkit query data.json '.[:5]'             # 처음 5개 요소
dkit query data.json '.[-2:]'            # 마지막 2개 요소
dkit query data.json '.[::2]'            # 짝수 인덱스 요소 (step=2)
dkit query data.json '.[1:5:2]'          # 인덱스 1~4 중 step=2
```

## Recursive Descent (`..`)

깊이에 관계없이 특정 키를 재귀적으로 찾는다. jq의 `..` 연산자에 해당.

```bash
# 모든 depth의 "email" 필드 찾기
dkit query nested.json '..email'

# 특정 경로 아래에서만 재귀 탐색
dkit query data.json '.config..host'

# 모든 "id" 필드 수집
dkit query complex.json '..id'

# 재귀 탐색 후 파이프라인 연결
dkit query data.json '..name | where name != "admin"'
```

결과는 배열로 반환된다. depth limit이 적용되어 순환 참조를 방지한다.

## Pipeline

`|` 로 여러 연산을 체이닝한다. 앞 연산의 결과가 다음 연산의 입력이 된다.

```bash
dkit query data.json '.users[] | where age > 30 | select name, email | sort name'
```

## where — Filtering

조건에 맞는 요소만 필터링한다.

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | 같음 | `where name == "Alice"` |
| `!=` | 다름 | `where status != "inactive"` |
| `>` | 초과 | `where age > 30` |
| `<` | 미만 | `where age < 25` |
| `>=` | 이상 | `where score >= 80` |
| `<=` | 이하 | `where price <= 1000` |

### String Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `contains` | 포함 | `where email contains "@gmail"` |
| `starts_with` | ~로 시작 | `where name starts_with "A"` |
| `ends_with` | ~로 끝남 | `where file ends_with ".json"` |

### Membership Operators (IN / NOT IN)

값이 목록에 포함되는지 확인한다.

```bash
# IN — 값이 목록에 포함
dkit query data.json '.users[] | where city in ("Seoul", "Busan")'
dkit query data.json '.[] | where status in ("active", "pending", "done")'

# NOT IN — 값이 목록에 미포함
dkit query data.json '.users[] | where status not in ("deleted", "archived")'
```

### Regex Operator (matches)

정규식 패턴으로 문자열을 매칭한다.

```bash
# matches — 정규식 매칭
dkit query data.json '.users[] | where name matches "^[A-C]"'
dkit query data.json '.[] | where email matches "@gmail\\.com$"'

# not matches — 정규식 비매칭
dkit query data.json '.users[] | where name not matches "^test"'
```

### Logical Operators

```bash
# AND
dkit query data.json '.users[] | where age > 25 and city == "Seoul"'

# OR
dkit query data.json '.users[] | where role == "admin" or role == "manager"'

# 복합 조합
dkit query data.json '.users[] | where city in ("Seoul", "Busan") and age > 25'
dkit query data.json '.users[] | where name matches "^A" and role in ("engineer", "manager")'
```

## select — Column Selection

특정 필드만 추출한다.

```bash
dkit query data.json '.users[] | select name, email'
dkit query data.json '.users[] | select name, age, email'
```

## sort — Sorting

결과를 정렬한다.

```bash
# 오름차순 (기본)
dkit query data.json '.users[] | sort age'

# 내림차순
dkit query data.json '.users[] | sort age desc'
```

## limit — Result Limiting

결과 개수를 제한한다.

```bash
dkit query data.json '.users[] | limit 10'
dkit query data.json '.users[] | sort age desc | limit 5'
```

## Aggregate Functions

배열 데이터에서 집계 값을 계산한다. 결과는 단일 값으로 반환된다.

### count — 요소 개수

```bash
# 전체 요소 수
dkit query data.csv '.[] | count'

# 특정 필드의 비null 요소 수
dkit query data.csv '.[] | count email'
```

### sum — 합계

```bash
dkit query data.csv '.[] | sum price'
dkit query data.json '.users[] | sum age'
```

### avg — 평균

```bash
dkit query data.csv '.[] | avg price'
dkit query data.json '.users[] | avg score'
```

### min / max — 최솟값 / 최댓값

숫자 및 문자열 필드 모두 지원한다.

```bash
dkit query data.csv '.[] | min price'
dkit query data.csv '.[] | max price'
dkit query data.json '.users[] | min name'
dkit query data.json '.users[] | max name'
```

### distinct — 고유값 목록

```bash
dkit query data.csv '.[] | distinct category'
dkit query data.json '.users[] | distinct country'
```

### 집계 + 필터 조합

```bash
# 특정 조건 후 집계
dkit query data.csv '.[] | where region == "KR" | sum revenue'
dkit query data.json '.users[] | where age > 30 | count'
dkit query data.json '.users[] | where active == true | avg score'
```

## group_by — Grouping and Aggregation

배열 데이터를 지정된 필드 기준으로 그룹화하고, 각 그룹에 대해 집계를 수행한다.

### 기본 사용법

```bash
# 단일 필드 그룹화 (기본: count 포함)
dkit query data.csv '.[] | group_by category'

# 다중 필드 그룹화
dkit query data.csv '.[] | group_by region, category'
```

### 집계 함수 조합

`group_by` 뒤에 집계 함수를 지정한다. 함수 형식: `func()` 또는 `func(field)`.

```bash
# 카테고리별 개수, 합계, 평균
dkit query data.csv '.[] | group_by category count(), sum(price), avg(price)'

# 지역별 최솟값, 최댓값
dkit query data.csv '.[] | group_by region min(price), max(price)'
```

지원되는 집계 함수:

| 함수 | 설명 | 예시 |
|------|------|------|
| `count()` | 그룹 내 요소 수 | `count()` |
| `count(field)` | 비null 필드 수 | `count(email)` |
| `sum(field)` | 숫자 필드 합계 | `sum(price)` |
| `avg(field)` | 숫자 필드 평균 | `avg(score)` |
| `min(field)` | 최솟값 | `min(price)` |
| `max(field)` | 최댓값 | `max(price)` |

### HAVING — 그룹 필터링

집계 결과를 기준으로 그룹을 필터링한다.

```bash
# 2개 이상인 그룹만 표시
dkit query data.csv '.[] | group_by category count() having count > 1'

# 평균 가격이 100 이상인 그룹만
dkit query data.csv '.[] | group_by category count(), avg(price) having avg_price >= 100'
```

### 후속 파이프라인 조합

GROUP BY 결과에 sort, limit 등을 추가로 적용할 수 있다.

```bash
# 카테고리별 개수를 내림차순 정렬, 상위 5개
dkit query data.csv '.[] | group_by category count() | sort count desc | limit 5'

# 그룹 결과에서 특정 필드만 선택
dkit query data.csv '.[] | group_by category count(), sum(price) | select category, count'
```

## Built-in Functions

`select` 절에서 내장 함수를 사용하여 데이터를 변환할 수 있다. 함수는 중첩 호출이 가능하다.

```bash
dkit query data.csv '.[] | select upper(name), round(price, 2)'
dkit query data.json '.users[] | select upper(trim(name)), to_string(age)'
```

### 별칭 (as)

`as` 키워드로 출력 필드명을 지정한다.

```bash
dkit query data.csv '.[] | select upper(name) as NAME, round(price, 2) as price_rounded'
```

### 문자열 함수

| 함수 | 설명 | 예시 |
|------|------|------|
| `upper(s)` | 대문자 변환 | `upper(name)` |
| `lower(s)` | 소문자 변환 | `lower(email)` |
| `trim(s)` | 앞뒤 공백 제거 | `trim(name)` |
| `ltrim(s)` | 앞쪽 공백 제거 | `ltrim(name)` |
| `rtrim(s)` | 뒤쪽 공백 제거 | `rtrim(name)` |
| `length(s)` | 문자열 길이 | `length(name)` |
| `substr(s, start)` | 부분 문자열 (시작 위치~끝) | `substr(name, 2)` |
| `substr(s, start, len)` | 부분 문자열 (시작 위치, 길이) | `substr(name, 0, 5)` |
| `concat(a, b, ...)` | 문자열 합치기 | `concat(first, " ", last)` |
| `replace(s, from, to)` | 문자열 치환 | `replace(name, "old", "new")` |
| `split(s, sep)` | 문자열 분리 (배열 반환) | `split(tags, ",")` |
| `index_of(s, substr)` | 부분 문자열 위치 반환 (-1 if not found) | `index_of(email, "@")` |
| `rindex_of(s, substr)` | 마지막 위치 반환 | `rindex_of(path, "/")` |
| `starts_with(s, prefix)` | 접두사 확인 (Boolean) | `starts_with(name, "Dr.")` |
| `ends_with(s, suffix)` | 접미사 확인 (Boolean) | `ends_with(file, ".json")` |
| `reverse(s)` | 문자열 뒤집기 | `reverse(name)` |
| `repeat(s, n)` | 문자열 반복 | `repeat("*", 5)` |
| `pad_left(s, width, char)` | 왼쪽 패딩 | `pad_left(to_string(id), 5, "0")` |
| `pad_right(s, width, char)` | 오른쪽 패딩 | `pad_right(name, 10, ".")` |

### 수학 함수

| 함수 | 설명 | 예시 |
|------|------|------|
| `round(n)` | 반올림 (정수 반환) | `round(price)` |
| `round(n, d)` | d자리까지 반올림 | `round(price, 2)` |
| `ceil(n)` | 올림 | `ceil(price)` |
| `floor(n)` | 내림 | `floor(price)` |
| `abs(n)` | 절댓값 | `abs(diff)` |
| `sqrt(n)` | 제곱근 | `sqrt(area)` |
| `pow(base, exp)` | 거듭제곱 | `pow(x, 2)` |

### 날짜 함수

날짜 문자열은 ISO 8601 형식(`yyyy-MM-dd` 또는 `yyyy-MM-ddTHH:mm:ssZ`)을 기대한다.

| 함수 | 설명 | 예시 |
|------|------|------|
| `now()` | 현재 UTC 시각 (ISO 8601) | `now()` |
| `date(s)` | 날짜 정규화 (yyyy-MM-dd) | `date(created_at)` |
| `year(s)` | 연도 추출 | `year(created_at)` |
| `month(s)` | 월 추출 (1-12) | `month(created_at)` |
| `day(s)` | 일 추출 (1-31) | `day(created_at)` |

### 타입 변환

| 함수 | 설명 | 예시 |
|------|------|------|
| `to_int(v)` / `int(v)` | 정수 변환 | `to_int(score)` |
| `to_float(v)` / `float(v)` | 부동소수점 변환 | `to_float(price)` |
| `to_string(v)` / `str(v)` | 문자열 변환 | `to_string(id)` |
| `to_bool(v)` / `bool(v)` | 불리언 변환 | `to_bool(active)` |

### 유틸 함수

| 함수 | 설명 | 예시 |
|------|------|------|
| `coalesce(a, b, ...)` | 첫 번째 non-null 값 반환 | `coalesce(name, "unknown")` |
| `if_null(v, default)` | null이면 기본값 반환 | `if_null(email, "N/A")` |

### 조건부 표현식

#### if(condition, then, else)

간단한 조건부 값 할당. 중첩 가능.

```bash
# 단순 조건
dkit query data.json '.[] | select name, if(age < 18, "minor", "adult") as category'

# 중첩 if
dkit query data.json '.[] | select name, if(age < 18, "minor", if(age < 65, "adult", "senior")) as category'

# 문자열 비교
dkit query data.json '.[] | select name, if(role == "engineer", "tech", "non-tech") as dept'
```

#### case when ... then ... else ... end

SQL 스타일의 다중 조건 분기. 복잡한 조건에 적합.

```bash
# 점수 등급 분류
dkit query data.json '.[] | select name, case when score >= 90 then "A" when score >= 70 then "B" else "C" end as grade'

# 다중 조건
dkit query data.json '.[] | select name, case when age < 18 then "minor" when age < 65 then "adult" else "senior" end as category'
```

### Statistical Aggregate Functions

기존 집계 함수(count, sum, avg, min, max, distinct) 외에 통계 분석용 함수를 지원한다.

| 함수 | 설명 | 예시 |
|------|------|------|
| `median(field)` | 중앙값 | `.[] \| median salary` |
| `percentile(field, p)` | p번째 백분위수 (p: 0.0~1.0) | `.[] \| percentile latency 0.95` |
| `stddev(field)` | 표준편차 (모집단) | `.[] \| stddev score` |
| `variance(field)` | 분산 | `.[] \| variance score` |
| `mode(field)` | 최빈값 | `.[] \| mode color` |
| `group_concat(field, sep)` | 그룹 내 문자열 연결 | `group_by category group_concat(name, ", ")` |

```bash
# 급여 중앙값
dkit query employees.json '.[] | median salary'

# 응답 시간 p95, p99
dkit query metrics.json '.[] | percentile latency 0.95'

# 부서별 통계
dkit query employees.json '.[] | group_by department median(salary), stddev(salary)'

# 카테고리별 이름 합치기
dkit query products.json '.[] | group_by category group_concat(name, ", ")'
```

### 함수 조합 예시

```bash
# 이름 대문자 변환 + 공백 제거
dkit query data.csv '.[] | select upper(trim(name))'

# 가격 소수점 2자리 반올림, 별칭 사용
dkit query data.csv '.[] | select name, round(price, 2) as price'

# 날짜에서 연도 추출
dkit query data.json '.[] | select upper(name), year(created_at) as year'

# 타입 변환 + where 필터 후 함수 적용
dkit query data.csv '.[] | where score > 80 | select name, to_string(score) as score_str'
```

## Window Functions

윈도우 함수를 사용하여 행 순서 기반 분석을 수행한다. `OVER` 절로 파티션과 정렬을 지정한다.

### 기본 문법

```
window_func() OVER ([PARTITION BY field, ...] ORDER BY field [ASC|DESC])
```

### 지원 함수

| 함수 | 설명 | 예시 |
|------|------|------|
| `row_number()` | 행 번호 (1부터) | `row_number() over (order by name)` |
| `rank()` | 순위 (동점 시 같은 순위, 다음 순위 건너뜀) | `rank() over (order by score desc)` |
| `dense_rank()` | 순위 (동점 시 같은 순위, 다음 순위 연속) | `dense_rank() over (order by score desc)` |
| `lag(expr, offset)` | 이전 행 참조 | `lag(value, 1) over (order by date)` |
| `lead(expr, offset)` | 다음 행 참조 | `lead(value, 1) over (order by date)` |
| `first_value(expr)` | 윈도우 내 첫 번째 값 | `first_value(name) over (order by score desc)` |
| `last_value(expr)` | 윈도우 내 마지막 값 | `last_value(name) over (order by score desc)` |
| `sum/avg/count/min/max(field)` | 윈도우 집계 | `sum(amount) over (order by date)` |

### 사용 예시

```bash
# 순위 매기기
dkit query sales.json '.[] | select name, revenue, rank() over (order by revenue desc) as rank'

# 파티션별 순위
dkit query sales.json '.[] | select department, name, revenue, row_number() over (partition by department order by revenue desc) as dept_rank'

# 이전/다음 행 참조
dkit query timeseries.json '.[] | select date, value, lag(value, 1) over (order by date) as prev_value'
dkit query timeseries.json '.[] | select date, value, lead(value, 1) over (order by date) as next_value'

# 누적 합계 (Running Total)
dkit query transactions.json '.[] | select date, amount, sum(amount) over (order by date) as running_total'

# 첫 번째/마지막 값
dkit query sales.json '.[] | select name, revenue, first_value(name) over (order by revenue desc) as top_earner'
```

### 주의사항

- 윈도우 함수는 `select` 절에서만 사용 가능
- 입력은 배열이어야 함
- `PARTITION BY`와 `ORDER BY` 모두 선택적이지만, 대부분 `ORDER BY`가 필요
- 데이터를 메모리에 로드하여 처리하므로 매우 큰 데이터셋에서는 주의

## Combined Examples

```bash
# 30세 이상 사용자의 이름과 이메일을 이름순으로 정렬
dkit query users.json '.users[] | where age > 30 | select name, email | sort name'

# 서울 거주자 중 상위 5명
dkit query users.json '.users[] | where city == "Seoul" | sort score desc | limit 5'

# CSV에서도 동일 문법 (CSV의 경우 .rows[] 사용)
dkit query sales.csv '.rows[] | where region == "KR" | select product, revenue | sort revenue desc'

# 쿼리 결과를 다른 포맷으로 저장
dkit query sales.csv '.rows[] | where region == "KR"' --to json -o korea.json
```

## Grammar (EBNF)

```
query       = path ( "|" operation )*
path        = "." segment* | ".." IDENTIFIER    (* recursive descent *)
segment     = field_access | index_access | iterate | wildcard | slice
            | ".." IDENTIFIER                    (* recursive descent mid-path *)
field_access = IDENTIFIER ( "." IDENTIFIER )*
index_access = "[" INTEGER "]"
iterate     = "[" "]"
wildcard    = "[" "*" "]"
slice       = "[" [ INTEGER ] ":" [ INTEGER ] [ ":" INTEGER ] "]"

operation   = where_op | select_op | sort_op | limit_op
            | count_op | sum_op | avg_op | min_op | max_op | distinct_op
            | median_op | percentile_op | stddev_op | variance_op | mode_op
            | group_concat_op | group_by_op
where_op    = "where" condition
select_op   = "select" select_expr ( "," select_expr )*
select_expr = expr [ "as" IDENTIFIER ]
expr        = IDENTIFIER | literal | func_call | if_expr | case_expr | window_expr
func_call   = IDENTIFIER "(" [ expr ( "," expr )* ] ")"
window_expr = window_func "over" "(" [ partition_clause ] [ order_clause ] ")"
window_func = "row_number" "(" ")"
            | "rank" "(" ")"
            | "dense_rank" "(" ")"
            | "lag" "(" expr [ "," INTEGER ] ")"
            | "lead" "(" expr [ "," INTEGER ] ")"
            | "first_value" "(" expr ")"
            | "last_value" "(" expr ")"
            | agg_func "(" [ IDENTIFIER ] ")"
partition_clause = "partition" "by" IDENTIFIER ( "," IDENTIFIER )*
order_clause     = "order" "by" IDENTIFIER [ "asc" | "desc" ] ( "," IDENTIFIER [ "asc" | "desc" ] )*
if_expr     = "if" "(" condition "," expr "," expr ")"
case_expr   = "case" ( "when" condition "then" expr )+ [ "else" expr ] "end"
sort_op     = "sort" IDENTIFIER [ "desc" ]
limit_op    = "limit" INTEGER
count_op    = "count" [ IDENTIFIER ]
sum_op      = "sum" IDENTIFIER
avg_op      = "avg" IDENTIFIER
min_op      = "min" IDENTIFIER
max_op      = "max" IDENTIFIER
distinct_op = "distinct" IDENTIFIER
median_op   = "median" IDENTIFIER
percentile_op = "percentile" IDENTIFIER NUMBER
stddev_op   = "stddev" IDENTIFIER
variance_op = "variance" IDENTIFIER
mode_op     = "mode" IDENTIFIER
group_concat_op = "group_concat" IDENTIFIER STRING
group_by_op = "group_by" IDENTIFIER ( "," IDENTIFIER )* aggregate* [ "having" condition ]
aggregate   = agg_func "(" [ IDENTIFIER [ "," expr ] ] ")"
agg_func    = "count" | "sum" | "avg" | "min" | "max"
            | "median" | "percentile" | "stddev" | "variance" | "mode" | "group_concat"

condition   = comparison ( logic_op comparison )*
comparison  = IDENTIFIER compare_op value
            | IDENTIFIER string_op STRING
            | IDENTIFIER "in" "(" value_list ")"
            | IDENTIFIER "not" "in" "(" value_list ")"
            | IDENTIFIER "matches" STRING
            | IDENTIFIER "not" "matches" STRING
compare_op  = "==" | "!=" | ">" | "<" | ">=" | "<="
string_op   = "contains" | "starts_with" | "ends_with"
logic_op    = "and" | "or"
value_list  = value ( "," value )*

value       = STRING | NUMBER | "true" | "false" | "null"
```
