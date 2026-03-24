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
```

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

### Logical Operators

```bash
# AND
dkit query data.json '.users[] | where age > 25 and city == "Seoul"'

# OR
dkit query data.json '.users[] | where role == "admin" or role == "manager"'
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
path        = "." segment*
segment     = field_access | index_access | iterate
field_access = IDENTIFIER ( "." IDENTIFIER )*
index_access = "[" INTEGER "]"
iterate     = "[" "]"

operation   = where_op | select_op | sort_op | limit_op
            | count_op | sum_op | avg_op | min_op | max_op | distinct_op
where_op    = "where" condition
select_op   = "select" IDENTIFIER ( "," IDENTIFIER )*
sort_op     = "sort" IDENTIFIER [ "desc" ]
limit_op    = "limit" INTEGER
count_op    = "count" [ IDENTIFIER ]
sum_op      = "sum" IDENTIFIER
avg_op      = "avg" IDENTIFIER
min_op      = "min" IDENTIFIER
max_op      = "max" IDENTIFIER
distinct_op = "distinct" IDENTIFIER

condition   = comparison ( logic_op comparison )*
comparison  = IDENTIFIER compare_op value
            | IDENTIFIER string_op STRING
compare_op  = "==" | "!=" | ">" | "<" | ">=" | "<="
string_op   = "contains" | "starts_with" | "ends_with"
logic_op    = "and" | "or"

value       = STRING | NUMBER | "true" | "false" | "null"
```
