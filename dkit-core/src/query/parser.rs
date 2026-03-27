use crate::error::DkitError;

/// 쿼리 AST (Abstract Syntax Tree)
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    /// 경로 접근 (`.users[0].name`)
    pub path: Path,
    /// 파이프라인 연산 (`| where ...`)
    pub operations: Vec<Operation>,
}

/// 경로: `.` + 세그먼트들
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Segment>,
}

/// A single segment of a navigation path.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Segment {
    /// 필드 접근 (`.name`)
    Field(String),
    /// 배열 인덱스 접근 (`[0]`, `[-1]`)
    Index(i64),
    /// 배열 이터레이션 (`[]`)
    Iterate,
    /// 배열 슬라이싱 (`[0:3]`, `[-2:]`, `[::2]`)
    Slice {
        start: Option<i64>,
        end: Option<i64>,
        step: Option<i64>,
    },
    /// 배열 와일드카드 (`[*]`) — Iterate와 동일 의미
    Wildcard,
}

/// Pipeline operation applied after path navigation (e.g., `| where ...`, `| sort ...`).
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Operation {
    /// `where` 필터링
    Where(Condition),
    /// `select` 컬럼 선택: `select name, upper(name), round(price, 2)`
    Select(Vec<SelectExpr>),
    /// `sort` 정렬: `sort age` (오름차순) / `sort age desc` (내림차순)
    Sort { field: String, descending: bool },
    /// `limit` 결과 제한: `limit 10`
    Limit(usize),
    /// `count` 전체 카운트 / `count field` 비null 카운트
    Count { field: Option<String> },
    /// `sum field` 숫자 필드 합계
    Sum { field: String },
    /// `avg field` 숫자 필드 평균
    Avg { field: String },
    /// `min field` 최솟값
    Min { field: String },
    /// `max field` 최댓값
    Max { field: String },
    /// `distinct field` 고유값 목록
    Distinct { field: String },
    /// `group_by field1, field2` 그룹별 집계
    /// 집계 연산: `group_by category | select category, count, sum_price`
    GroupBy {
        fields: Vec<String>,
        having: Option<Condition>,
        aggregates: Vec<GroupAggregate>,
    },
    /// 전체 레코드 동일성 기준 중복 제거
    Unique,
    /// 특정 필드 기준 중복 제거 (첫 번째 등장 레코드 유지)
    UniqueBy { field: String },
    /// 새 필드 추가 (computed column): `--add-field 'total = amount * quantity'`
    AddField { name: String, expr: Expr },
    /// 기존 필드 값 변환: `--map 'name = upper(name)'`
    MapField { name: String, expr: Expr },
}

/// GROUP BY 집계 연산 정의
#[derive(Debug, Clone, PartialEq)]
pub struct GroupAggregate {
    pub func: AggregateFunc,
    pub field: Option<String>,
    pub alias: String,
}

/// Aggregate function used in `group_by` operations.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

/// Boolean condition used in `where` clauses.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Condition {
    /// 단일 비교: `field op value`
    Comparison(Comparison),
    /// 논리 AND: `condition and condition`
    And(Box<Condition>, Box<Condition>),
    /// 논리 OR: `condition or condition`
    Or(Box<Condition>, Box<Condition>),
}

/// 비교식: `IDENTIFIER compare_op value`
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub field: String,
    pub op: CompareOp,
    pub value: LiteralValue,
}

/// Comparison operator used in `where` conditions.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CompareOp {
    Eq,         // ==
    Ne,         // !=
    Gt,         // >
    Lt,         // <
    Ge,         // >=
    Le,         // <=
    Contains,   // contains
    StartsWith, // starts_with
    EndsWith,   // ends_with
    In,         // in
    NotIn,      // not in
}

/// Literal value used as a comparison operand or in expressions.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum LiteralValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
    List(Vec<LiteralValue>),
}

/// Arithmetic binary operator.
#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

/// Expression used in `select` clauses and function arguments.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Expr {
    /// 필드 참조: `name`
    Field(String),
    /// 리터럴 값: `42`, `"hello"`, `true`
    Literal(LiteralValue),
    /// 함수 호출: `upper(name)`, `round(price, 2)`, `upper(trim(name))`
    FuncCall { name: String, args: Vec<Expr> },
    /// 이항 산술 연산: `amount * quantity`, `first_name + " " + last_name`
    BinaryOp {
        op: ArithmeticOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// SELECT 절의 컬럼 표현식
#[derive(Debug, Clone, PartialEq)]
pub struct SelectExpr {
    pub expr: Expr,
    /// 출력 키 별칭 (`upper(name) as name_upper` 에서 `name_upper`)
    pub alias: Option<String>,
}

/// Internal query string parser.
///
/// Use the public [`parse_query`] function instead of constructing this directly.
pub(crate) struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// Parse the query string into a [`Query`] AST.
    pub(crate) fn parse(&mut self) -> Result<Query, DkitError> {
        self.skip_whitespace();
        let path = self.parse_path()?;
        self.skip_whitespace();

        // 파이프라인 연산 파싱: `| where ...`
        let mut operations = Vec::new();
        while self.peek() == Some('|') {
            self.advance(); // consume '|'
            self.skip_whitespace();
            operations.push(self.parse_operation()?);
            self.skip_whitespace();
        }

        if self.pos != self.input.len() {
            return Err(DkitError::QueryError(format!(
                "unexpected character '{}' at position {}",
                self.input[self.pos], self.pos
            )));
        }

        Ok(Query { path, operations })
    }

    /// 경로를 파싱: `.` 으로 시작
    fn parse_path(&mut self) -> Result<Path, DkitError> {
        if !self.consume_char('.') {
            return Err(DkitError::QueryError(
                "query must start with '.'".to_string(),
            ));
        }

        let mut segments = Vec::new();

        // `.` 만 있으면 루트 경로 (세그먼트 없음)
        if self.is_at_end() {
            return Ok(Path { segments });
        }

        // 첫 번째 세그먼트: `[` 이면 인덱스/이터레이터, 아니면 필드
        if self.peek() == Some('[') {
            segments.push(self.parse_bracket()?);
        } else if self.peek_is_identifier_start() {
            segments.push(self.parse_field()?);
        }

        // 나머지 세그먼트
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.peek() == Some('.') {
                self.advance(); // consume '.'
                if self.peek() == Some('[') {
                    segments.push(self.parse_bracket()?);
                } else {
                    segments.push(self.parse_field()?);
                }
            } else if self.peek() == Some('[') {
                segments.push(self.parse_bracket()?);
            } else {
                break;
            }
        }

        Ok(Path { segments })
    }

    /// 필드 이름 파싱
    fn parse_field(&mut self) -> Result<Segment, DkitError> {
        let start = self.pos;
        while !self.is_at_end() {
            let c = self.input[self.pos];
            if c.is_alphanumeric() || c == '_' || c == '-' {
                self.pos += 1;
            } else {
                break;
            }
        }

        if self.pos == start {
            return Err(DkitError::QueryError(format!(
                "expected field name at position {}",
                self.pos
            )));
        }

        let name: String = self.input[start..self.pos].iter().collect();
        Ok(Segment::Field(name))
    }

    /// `[...]` 파싱: 인덱스, 이터레이션, 슬라이스, 와일드카드
    fn parse_bracket(&mut self) -> Result<Segment, DkitError> {
        if !self.consume_char('[') {
            return Err(DkitError::QueryError(format!(
                "expected '[' at position {}",
                self.pos
            )));
        }

        self.skip_whitespace();

        // `[]` — 이터레이션
        if self.peek() == Some(']') {
            self.advance();
            return Ok(Segment::Iterate);
        }

        // `[*]` — 와일드카드
        if self.peek() == Some('*') {
            self.advance();
            self.skip_whitespace();
            if !self.consume_char(']') {
                return Err(DkitError::QueryError(format!(
                    "expected ']' after '*' at position {}",
                    self.pos
                )));
            }
            return Ok(Segment::Wildcard);
        }

        // `[:]` — 슬라이스 (콜론으로 시작)
        if self.peek() == Some(':') {
            return self.parse_slice(None);
        }

        // 숫자 파싱 (인덱스 또는 슬라이스의 start)
        let negative = self.consume_char('-');
        let start = self.pos;
        while !self.is_at_end() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(DkitError::QueryError(format!(
                "expected integer index at position {}",
                self.pos
            )));
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        let num: i64 = num_str.parse().map_err(|_| {
            DkitError::QueryError(format!("invalid index '{}' at position {}", num_str, start))
        })?;
        let num = if negative { -num } else { num };

        self.skip_whitespace();

        // `:` 이 나오면 슬라이스
        if self.peek() == Some(':') {
            return self.parse_slice(Some(num));
        }

        // `]` 이면 단일 인덱스
        if !self.consume_char(']') {
            return Err(DkitError::QueryError(format!(
                "expected ']' or ':' at position {}",
                self.pos
            )));
        }

        Ok(Segment::Index(num))
    }

    /// 슬라이스 나머지 파싱: start는 이미 파싱됨, `:` 부터 시작
    fn parse_slice(&mut self, start: Option<i64>) -> Result<Segment, DkitError> {
        // consume first ':'
        if !self.consume_char(':') {
            return Err(DkitError::QueryError(format!(
                "expected ':' at position {}",
                self.pos
            )));
        }

        self.skip_whitespace();

        // end 파싱
        let end = if self.peek() == Some(']') || self.peek() == Some(':') {
            None
        } else {
            Some(self.parse_signed_integer()?)
        };

        self.skip_whitespace();

        // step 파싱 (optional second ':')
        let step = if self.peek() == Some(':') {
            self.advance();
            self.skip_whitespace();
            if self.peek() == Some(']') {
                None
            } else {
                Some(self.parse_signed_integer()?)
            }
        } else {
            None
        };

        self.skip_whitespace();
        if !self.consume_char(']') {
            return Err(DkitError::QueryError(format!(
                "expected ']' at position {}",
                self.pos
            )));
        }

        Ok(Segment::Slice { start, end, step })
    }

    /// 부호 있는 정수 파싱
    fn parse_signed_integer(&mut self) -> Result<i64, DkitError> {
        let negative = self.consume_char('-');
        let start = self.pos;
        while !self.is_at_end() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(DkitError::QueryError(format!(
                "expected integer at position {}",
                self.pos
            )));
        }
        let num_str: String = self.input[start..self.pos].iter().collect();
        let num: i64 = num_str.parse().map_err(|_| {
            DkitError::QueryError(format!(
                "invalid integer '{}' at position {}",
                num_str, start
            ))
        })?;
        Ok(if negative { -num } else { num })
    }

    // --- 파이프라인 연산 파싱 ---

    /// 연산 파싱: `where ...`, `select ...`
    fn parse_operation(&mut self) -> Result<Operation, DkitError> {
        let keyword = self.parse_keyword()?;
        match keyword.as_str() {
            "where" => {
                self.skip_whitespace();
                let condition = self.parse_condition()?;
                Ok(Operation::Where(condition))
            }
            "select" => {
                self.skip_whitespace();
                let exprs = self.parse_select_expr_list()?;
                Ok(Operation::Select(exprs))
            }
            "sort" => {
                self.skip_whitespace();
                let field = self.parse_identifier()?;
                self.skip_whitespace();
                let descending = self.try_consume_keyword("desc");
                Ok(Operation::Sort { field, descending })
            }
            "limit" => {
                self.skip_whitespace();
                let n = self.parse_positive_integer()?;
                Ok(Operation::Limit(n))
            }
            "count" => {
                self.skip_whitespace();
                let field = self.try_parse_identifier();
                Ok(Operation::Count { field })
            }
            "sum" => {
                self.skip_whitespace();
                let field = self.parse_identifier()?;
                Ok(Operation::Sum { field })
            }
            "avg" => {
                self.skip_whitespace();
                let field = self.parse_identifier()?;
                Ok(Operation::Avg { field })
            }
            "min" => {
                self.skip_whitespace();
                let field = self.parse_identifier()?;
                Ok(Operation::Min { field })
            }
            "max" => {
                self.skip_whitespace();
                let field = self.parse_identifier()?;
                Ok(Operation::Max { field })
            }
            "distinct" => {
                self.skip_whitespace();
                let field = self.parse_identifier()?;
                Ok(Operation::Distinct { field })
            }
            "group_by" => {
                self.skip_whitespace();
                let fields = self.parse_identifier_list()?;
                self.skip_whitespace();

                // Parse optional aggregate functions
                let aggregates = self.parse_group_aggregates()?;

                // Parse optional HAVING clause
                let having = if self.try_consume_keyword("having") {
                    self.skip_whitespace();
                    Some(self.parse_condition()?)
                } else {
                    None
                };

                Ok(Operation::GroupBy {
                    fields,
                    having,
                    aggregates,
                })
            }
            _ => Err(DkitError::QueryError(format!(
                "unknown operation '{}' at position {}",
                keyword,
                self.pos - keyword.chars().count()
            ))),
        }
    }

    /// GROUP BY 집계 함수 목록 파싱: `count(), sum(field), avg(field), ...`
    fn parse_group_aggregates(&mut self) -> Result<Vec<GroupAggregate>, DkitError> {
        let mut aggregates = Vec::new();

        loop {
            let saved_pos = self.pos;
            if let Some(agg) = self.try_parse_single_aggregate()? {
                aggregates.push(agg);
                self.skip_whitespace();
                if !self.consume_char(',') {
                    // No comma, check if next is "having" or end
                    break;
                }
                self.skip_whitespace();
            } else {
                self.pos = saved_pos;
                break;
            }
        }

        Ok(aggregates)
    }

    /// 단일 집계 함수 파싱: `count()`, `sum(field)`, `avg(field)` 등
    fn try_parse_single_aggregate(&mut self) -> Result<Option<GroupAggregate>, DkitError> {
        let saved_pos = self.pos;

        // Try to read a keyword
        let func_name = match self.parse_keyword() {
            Ok(name) => name,
            Err(_) => {
                self.pos = saved_pos;
                return Ok(None);
            }
        };

        let func = match func_name.as_str() {
            "count" => AggregateFunc::Count,
            "sum" => AggregateFunc::Sum,
            "avg" => AggregateFunc::Avg,
            "min" => AggregateFunc::Min,
            "max" => AggregateFunc::Max,
            _ => {
                // Not an aggregate function, restore position
                self.pos = saved_pos;
                return Ok(None);
            }
        };

        self.skip_whitespace();

        // Must have '('
        if !self.consume_char('(') {
            self.pos = saved_pos;
            return Ok(None);
        }

        self.skip_whitespace();

        // Parse optional field name
        let field = if self.peek() == Some(')') {
            None
        } else {
            Some(self.parse_identifier()?)
        };

        self.skip_whitespace();

        if !self.consume_char(')') {
            return Err(DkitError::QueryError(format!(
                "expected ')' at position {}",
                self.pos
            )));
        }

        // Generate alias
        let alias = match &field {
            Some(f) => format!("{}_{}", func_name, f),
            None => func_name.clone(),
        };

        Ok(Some(GroupAggregate { func, field, alias }))
    }

    /// SELECT 절의 표현식 목록 파싱: `expr [as alias] ( "," expr [as alias] )*`
    fn parse_select_expr_list(&mut self) -> Result<Vec<SelectExpr>, DkitError> {
        let mut exprs = vec![self.parse_select_expr()?];
        loop {
            self.skip_whitespace();
            if self.consume_char(',') {
                self.skip_whitespace();
                exprs.push(self.parse_select_expr()?);
            } else {
                break;
            }
        }
        Ok(exprs)
    }

    /// 단일 SELECT 표현식 파싱: `expr [as alias]`
    fn parse_select_expr(&mut self) -> Result<SelectExpr, DkitError> {
        let expr = self.parse_expr()?;
        self.skip_whitespace();
        // Optional alias: `as alias_name`
        let alias = {
            let saved = self.pos;
            if let Ok(keyword) = self.parse_keyword() {
                if keyword == "as" {
                    self.skip_whitespace();
                    Some(self.parse_identifier()?)
                } else {
                    self.pos = saved;
                    None
                }
            } else {
                self.pos = saved;
                None
            }
        };
        Ok(SelectExpr { expr, alias })
    }

    /// 표현식 파싱: 산술 연산자 포함 (우선순위: +- < */)
    fn parse_expr(&mut self) -> Result<Expr, DkitError> {
        self.parse_additive_expr()
    }

    /// 덧셈/뺄셈 수준 표현식: `term (('+' | '-') term)*`
    fn parse_additive_expr(&mut self) -> Result<Expr, DkitError> {
        let mut left = self.parse_multiplicative_expr()?;

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('+') => {
                    self.advance();
                    self.skip_whitespace();
                    let right = self.parse_multiplicative_expr()?;
                    left = Expr::BinaryOp {
                        op: ArithmeticOp::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some('-') => {
                    // Distinguish subtraction from negative number literal at end of expression
                    // Only treat as subtraction if we've already parsed a left operand
                    self.advance();
                    self.skip_whitespace();
                    let right = self.parse_multiplicative_expr()?;
                    left = Expr::BinaryOp {
                        op: ArithmeticOp::Sub,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// 곱셈/나눗셈 수준 표현식: `atom (('*' | '/') atom)*`
    fn parse_multiplicative_expr(&mut self) -> Result<Expr, DkitError> {
        let mut left = self.parse_atom_expr()?;

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('*') => {
                    self.advance();
                    self.skip_whitespace();
                    let right = self.parse_atom_expr()?;
                    left = Expr::BinaryOp {
                        op: ArithmeticOp::Mul,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some('/') => {
                    self.advance();
                    self.skip_whitespace();
                    let right = self.parse_atom_expr()?;
                    left = Expr::BinaryOp {
                        op: ArithmeticOp::Div,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// 원자 표현식: 리터럴, 필드 참조, 함수 호출, 괄호
    fn parse_atom_expr(&mut self) -> Result<Expr, DkitError> {
        match self.peek() {
            Some('(') => {
                self.advance(); // consume '('
                self.skip_whitespace();
                let expr = self.parse_expr()?;
                self.skip_whitespace();
                if !self.consume_char(')') {
                    return Err(DkitError::QueryError(format!(
                        "expected ')' at position {}",
                        self.pos
                    )));
                }
                Ok(expr)
            }
            Some('"') => {
                let lit = self.parse_string_literal()?;
                Ok(Expr::Literal(lit))
            }
            Some(c) if c.is_ascii_digit() => {
                let lit = self.parse_number_literal()?;
                Ok(Expr::Literal(lit))
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let name = self.parse_identifier()?;
                // Check for bool/null literals
                match name.as_str() {
                    "true" => return Ok(Expr::Literal(LiteralValue::Bool(true))),
                    "false" => return Ok(Expr::Literal(LiteralValue::Bool(false))),
                    "null" => return Ok(Expr::Literal(LiteralValue::Null)),
                    _ => {}
                }
                // Check for function call: name(...)
                if self.peek() == Some('(') {
                    self.advance(); // consume '('
                    self.skip_whitespace();
                    let mut args = Vec::new();
                    if self.peek() != Some(')') {
                        args.push(self.parse_expr()?);
                        loop {
                            self.skip_whitespace();
                            if self.consume_char(',') {
                                self.skip_whitespace();
                                args.push(self.parse_expr()?);
                            } else {
                                break;
                            }
                        }
                    }
                    self.skip_whitespace();
                    if !self.consume_char(')') {
                        return Err(DkitError::QueryError(format!(
                            "expected ')' at position {}",
                            self.pos
                        )));
                    }
                    Ok(Expr::FuncCall { name, args })
                } else {
                    Ok(Expr::Field(name))
                }
            }
            Some(c) => Err(DkitError::QueryError(format!(
                "expected expression at position {}, found '{}'",
                self.pos, c
            ))),
            None => Err(DkitError::QueryError(format!(
                "expected expression at position {}",
                self.pos
            ))),
        }
    }

    /// 쉼표로 구분된 식별자 목록 파싱: `IDENTIFIER ( "," IDENTIFIER )*`
    fn parse_identifier_list(&mut self) -> Result<Vec<String>, DkitError> {
        let mut fields = vec![self.parse_identifier()?];
        loop {
            self.skip_whitespace();
            if self.consume_char(',') {
                self.skip_whitespace();
                fields.push(self.parse_identifier()?);
            } else {
                break;
            }
        }
        Ok(fields)
    }

    /// 키워드 파싱 (알파벳 + 언더스코어)
    fn parse_keyword(&mut self) -> Result<String, DkitError> {
        let start = self.pos;
        while !self.is_at_end() {
            let c = self.input[self.pos];
            if c.is_alphabetic() || c == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err(DkitError::QueryError(format!(
                "expected operation keyword at position {}",
                self.pos
            )));
        }
        Ok(self.input[start..self.pos].iter().collect())
    }

    /// 조건식 파싱: `comparison (and|or comparison)*`
    fn parse_condition(&mut self) -> Result<Condition, DkitError> {
        let mut left = Condition::Comparison(self.parse_comparison()?);

        loop {
            self.skip_whitespace();
            let saved_pos = self.pos;
            if let Ok(keyword) = self.parse_keyword() {
                match keyword.as_str() {
                    "and" => {
                        self.skip_whitespace();
                        let right = Condition::Comparison(self.parse_comparison()?);
                        left = Condition::And(Box::new(left), Box::new(right));
                    }
                    "or" => {
                        self.skip_whitespace();
                        let right = Condition::Comparison(self.parse_comparison()?);
                        left = Condition::Or(Box::new(left), Box::new(right));
                    }
                    _ => {
                        // Not a logical operator, restore position
                        self.pos = saved_pos;
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// 비교식 파싱: `IDENTIFIER compare_op literal_value`
    /// 또는 `IDENTIFIER in (value1, value2, ...)` / `IDENTIFIER not in (value1, value2, ...)`
    fn parse_comparison(&mut self) -> Result<Comparison, DkitError> {
        // 필드 이름
        let field = self.parse_identifier()?;
        self.skip_whitespace();

        // Check for `in` / `not in` operators
        let saved_pos = self.pos;
        if let Ok(keyword) = self.parse_keyword() {
            match keyword.as_str() {
                "in" => {
                    self.skip_whitespace();
                    let list = self.parse_literal_list()?;
                    return Ok(Comparison {
                        field,
                        op: CompareOp::In,
                        value: LiteralValue::List(list),
                    });
                }
                "not" => {
                    self.skip_whitespace();
                    let saved_pos2 = self.pos;
                    if let Ok(kw2) = self.parse_keyword() {
                        if kw2 == "in" {
                            self.skip_whitespace();
                            let list = self.parse_literal_list()?;
                            return Ok(Comparison {
                                field,
                                op: CompareOp::NotIn,
                                value: LiteralValue::List(list),
                            });
                        }
                    }
                    self.pos = saved_pos2;
                    self.pos = saved_pos;
                }
                _ => {
                    self.pos = saved_pos;
                }
            }
        } else {
            self.pos = saved_pos;
        }

        // 비교 연산자
        let op = self.parse_compare_op()?;
        self.skip_whitespace();

        // 리터럴 값
        let value = self.parse_literal_value()?;

        Ok(Comparison { field, op, value })
    }

    /// 식별자 파싱 (필드 이름)
    fn parse_identifier(&mut self) -> Result<String, DkitError> {
        let start = self.pos;
        while !self.is_at_end() {
            let c = self.input[self.pos];
            if c.is_alphanumeric() || c == '_' || c == '-' {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err(DkitError::QueryError(format!(
                "expected field name at position {}",
                self.pos
            )));
        }
        Ok(self.input[start..self.pos].iter().collect())
    }

    /// 비교 연산자 파싱: ==, !=, >=, <=, >, <, contains, starts_with, ends_with
    fn parse_compare_op(&mut self) -> Result<CompareOp, DkitError> {
        let c1 = self.peek().ok_or_else(|| {
            DkitError::QueryError(format!(
                "expected comparison operator at position {}",
                self.pos
            ))
        })?;

        match c1 {
            '=' => {
                self.advance();
                if self.consume_char('=') {
                    Ok(CompareOp::Eq)
                } else {
                    Err(DkitError::QueryError(format!(
                        "expected '==' at position {}",
                        self.pos - 1
                    )))
                }
            }
            '!' => {
                self.advance();
                if self.consume_char('=') {
                    Ok(CompareOp::Ne)
                } else {
                    Err(DkitError::QueryError(format!(
                        "expected '!=' at position {}",
                        self.pos - 1
                    )))
                }
            }
            '>' => {
                self.advance();
                if self.consume_char('=') {
                    Ok(CompareOp::Ge)
                } else {
                    Ok(CompareOp::Gt)
                }
            }
            '<' => {
                self.advance();
                if self.consume_char('=') {
                    Ok(CompareOp::Le)
                } else {
                    Ok(CompareOp::Lt)
                }
            }
            c if c.is_alphabetic() => {
                let saved_pos = self.pos;
                let keyword = self.parse_keyword()?;
                match keyword.as_str() {
                    "contains" => Ok(CompareOp::Contains),
                    "starts_with" => Ok(CompareOp::StartsWith),
                    "ends_with" => Ok(CompareOp::EndsWith),
                    _ => {
                        self.pos = saved_pos;
                        Err(DkitError::QueryError(format!(
                            "expected comparison operator at position {}, found '{}'",
                            saved_pos, keyword
                        )))
                    }
                }
            }
            _ => Err(DkitError::QueryError(format!(
                "expected comparison operator at position {}, found '{}'",
                self.pos, c1
            ))),
        }
    }

    /// 리터럴 값 파싱: 문자열, 숫자, bool, null
    fn parse_literal_value(&mut self) -> Result<LiteralValue, DkitError> {
        match self.peek() {
            Some('"') => self.parse_string_literal(),
            Some(c) if c.is_ascii_digit() || c == '-' => self.parse_number_literal(),
            Some(c) if c.is_alphabetic() => {
                let word = self.parse_keyword()?;
                match word.as_str() {
                    "true" => Ok(LiteralValue::Bool(true)),
                    "false" => Ok(LiteralValue::Bool(false)),
                    "null" => Ok(LiteralValue::Null),
                    _ => Err(DkitError::QueryError(format!(
                        "unexpected value '{}' at position {}",
                        word,
                        self.pos - word.len()
                    ))),
                }
            }
            Some(c) => Err(DkitError::QueryError(format!(
                "unexpected character '{}' at position {}",
                c, self.pos
            ))),
            None => Err(DkitError::QueryError(format!(
                "expected value at position {}",
                self.pos
            ))),
        }
    }

    /// 리터럴 리스트 파싱: `(value1, value2, ...)`
    fn parse_literal_list(&mut self) -> Result<Vec<LiteralValue>, DkitError> {
        if !self.consume_char('(') {
            return Err(DkitError::QueryError(format!(
                "expected '(' at position {}",
                self.pos
            )));
        }

        let mut values = Vec::new();
        self.skip_whitespace();

        // Handle empty list
        if self.peek() == Some(')') {
            self.advance();
            return Ok(values);
        }

        // Parse first value
        values.push(self.parse_literal_value()?);

        loop {
            self.skip_whitespace();
            if self.consume_char(')') {
                break;
            }
            if !self.consume_char(',') {
                return Err(DkitError::QueryError(format!(
                    "expected ',' or ')' at position {}",
                    self.pos
                )));
            }
            self.skip_whitespace();
            values.push(self.parse_literal_value()?);
        }

        Ok(values)
    }

    /// 문자열 리터럴 파싱: `"..."`
    fn parse_string_literal(&mut self) -> Result<LiteralValue, DkitError> {
        if !self.consume_char('"') {
            return Err(DkitError::QueryError(format!(
                "expected '\"' at position {}",
                self.pos
            )));
        }
        let start = self.pos;
        while !self.is_at_end() && self.input[self.pos] != '"' {
            self.pos += 1;
        }
        if self.is_at_end() {
            return Err(DkitError::QueryError(format!(
                "unterminated string starting at position {}",
                start - 1
            )));
        }
        let s: String = self.input[start..self.pos].iter().collect();
        self.advance(); // consume closing '"'
        Ok(LiteralValue::String(s))
    }

    /// 숫자 리터럴 파싱: 정수 또는 부동소수점
    fn parse_number_literal(&mut self) -> Result<LiteralValue, DkitError> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.advance();
        }
        while !self.is_at_end() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let mut is_float = false;
        if self.peek() == Some('.') {
            is_float = true;
            self.advance();
            while !self.is_at_end() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }
        if self.pos == start || (self.pos == start + 1 && self.input[start] == '-') {
            return Err(DkitError::QueryError(format!(
                "expected number at position {}",
                start
            )));
        }
        let num_str: String = self.input[start..self.pos].iter().collect();
        if is_float {
            let f: f64 = num_str.parse().map_err(|_| {
                DkitError::QueryError(format!(
                    "invalid number '{}' at position {}",
                    num_str, start
                ))
            })?;
            Ok(LiteralValue::Float(f))
        } else {
            let n: i64 = num_str.parse().map_err(|_| {
                DkitError::QueryError(format!(
                    "invalid number '{}' at position {}",
                    num_str, start
                ))
            })?;
            Ok(LiteralValue::Integer(n))
        }
    }

    // --- 유틸리티 ---

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek_is_identifier_start(&self) -> bool {
        self.peek().is_some_and(|c| c.is_alphabetic() || c == '_')
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn consume_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// 식별자를 시도적으로 파싱: 식별자가 없으면 None 반환 (위치 복원)
    fn try_parse_identifier(&mut self) -> Option<String> {
        if !self.peek_is_identifier_start() {
            return None;
        }
        let saved_pos = self.pos;
        match self.parse_identifier() {
            Ok(id) => Some(id),
            Err(_) => {
                self.pos = saved_pos;
                None
            }
        }
    }

    /// 키워드를 시도적으로 소비: 매치하면 true, 아니면 위치를 복원하고 false
    fn try_consume_keyword(&mut self, keyword: &str) -> bool {
        let saved_pos = self.pos;
        if let Ok(word) = self.parse_keyword() {
            if word == keyword {
                return true;
            }
        }
        self.pos = saved_pos;
        false
    }

    /// 양의 정수 파싱 (limit 절용)
    fn parse_positive_integer(&mut self) -> Result<usize, DkitError> {
        let start = self.pos;
        while !self.is_at_end() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(DkitError::QueryError(format!(
                "expected positive integer at position {}",
                self.pos
            )));
        }
        let num_str: String = self.input[start..self.pos].iter().collect();
        num_str.parse().map_err(|_| {
            DkitError::QueryError(format!(
                "invalid integer '{}' at position {}",
                num_str, start
            ))
        })
    }
}

/// 편의 함수: 쿼리 문자열 → Query
pub fn parse_query(input: &str) -> Result<Query, DkitError> {
    Parser::new(input).parse()
}

/// `--add-field` 표현식 파싱: `"name = expression"`
/// 예: "total = amount * quantity", "full_name = first_name + \" \" + last_name"
pub fn parse_add_field_expr(input: &str) -> Result<(String, Expr), DkitError> {
    let mut parser = Parser::new(input);
    parser.skip_whitespace();
    let name = parser.parse_identifier().map_err(|_| {
        DkitError::QueryError(format!(
            "expected field name in --add-field expression: '{input}'"
        ))
    })?;
    parser.skip_whitespace();
    if !parser.consume_char('=') {
        return Err(DkitError::QueryError(format!(
            "expected '=' after field name in --add-field expression: '{input}'"
        )));
    }
    parser.skip_whitespace();
    let expr = parser.parse_expr()?;
    parser.skip_whitespace();
    if parser.pos != parser.input.len() {
        return Err(DkitError::QueryError(format!(
            "unexpected character '{}' at position {} in --add-field expression",
            parser.input[parser.pos], parser.pos
        )));
    }
    Ok((name, expr))
}

/// where 절의 조건식만 파싱하는 편의 함수
/// 예: "age > 30 and city == \"Seoul\""
pub fn parse_condition_expr(input: &str) -> Result<Condition, DkitError> {
    let mut parser = Parser::new(input);
    parser.skip_whitespace();
    let condition = parser.parse_condition()?;
    parser.skip_whitespace();
    if parser.pos != parser.input.len() {
        return Err(DkitError::QueryError(format!(
            "unexpected character '{}' at position {} in where expression",
            parser.input[parser.pos], parser.pos
        )));
    }
    Ok(condition)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- 기본 경로 파싱 ---

    #[test]
    fn test_root_path() {
        let q = parse_query(".").unwrap();
        assert!(q.path.segments.is_empty());
    }

    #[test]
    fn test_single_field() {
        let q = parse_query(".name").unwrap();
        assert_eq!(q.path.segments, vec![Segment::Field("name".to_string())]);
    }

    #[test]
    fn test_nested_fields() {
        let q = parse_query(".database.host").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("database".to_string()),
                Segment::Field("host".to_string()),
            ]
        );
    }

    #[test]
    fn test_deeply_nested_fields() {
        let q = parse_query(".a.b.c.d").unwrap();
        assert_eq!(q.path.segments.len(), 4);
        assert_eq!(q.path.segments[3], Segment::Field("d".to_string()));
    }

    // --- 배열 인덱싱 ---

    #[test]
    fn test_array_index() {
        let q = parse_query(".users[0]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("users".to_string()), Segment::Index(0),]
        );
    }

    #[test]
    fn test_array_negative_index() {
        let q = parse_query(".users[-1]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("users".to_string()), Segment::Index(-1),]
        );
    }

    #[test]
    fn test_array_index_with_field_after() {
        let q = parse_query(".users[0].name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("users".to_string()),
                Segment::Index(0),
                Segment::Field("name".to_string()),
            ]
        );
    }

    #[test]
    fn test_large_index() {
        let q = parse_query(".items[999]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("items".to_string()), Segment::Index(999),]
        );
    }

    // --- 배열 이터레이션 ---

    #[test]
    fn test_array_iterate() {
        let q = parse_query(".users[]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("users".to_string()), Segment::Iterate,]
        );
    }

    #[test]
    fn test_array_iterate_with_field() {
        let q = parse_query(".users[].name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("users".to_string()),
                Segment::Iterate,
                Segment::Field("name".to_string()),
            ]
        );
    }

    #[test]
    fn test_array_iterate_nested() {
        let q = parse_query(".data[].items[].name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("data".to_string()),
                Segment::Iterate,
                Segment::Field("items".to_string()),
                Segment::Iterate,
                Segment::Field("name".to_string()),
            ]
        );
    }

    // --- 배열 와일드카드 ---

    #[test]
    fn test_array_wildcard() {
        let q = parse_query(".[*]").unwrap();
        assert_eq!(q.path.segments, vec![Segment::Wildcard]);
    }

    #[test]
    fn test_array_wildcard_with_field() {
        let q = parse_query(".users[*].name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("users".to_string()),
                Segment::Wildcard,
                Segment::Field("name".to_string()),
            ]
        );
    }

    // --- 배열 슬라이싱 ---

    #[test]
    fn test_array_slice_basic() {
        let q = parse_query(".[0:3]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: Some(0),
                end: Some(3),
                step: None
            }]
        );
    }

    #[test]
    fn test_array_slice_open_end() {
        let q = parse_query(".[1:]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: Some(1),
                end: None,
                step: None
            }]
        );
    }

    #[test]
    fn test_array_slice_open_start() {
        let q = parse_query(".[:3]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: None,
                end: Some(3),
                step: None
            }]
        );
    }

    #[test]
    fn test_array_slice_negative() {
        let q = parse_query(".[-2:]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: Some(-2),
                end: None,
                step: None
            }]
        );
    }

    #[test]
    fn test_array_slice_with_step() {
        let q = parse_query(".[1:5:2]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: Some(1),
                end: Some(5),
                step: Some(2)
            }]
        );
    }

    #[test]
    fn test_array_slice_full_open() {
        let q = parse_query(".[:]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: None,
                end: None,
                step: None
            }]
        );
    }

    #[test]
    fn test_array_slice_with_field() {
        let q = parse_query(".users[0:3].name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("users".to_string()),
                Segment::Slice {
                    start: Some(0),
                    end: Some(3),
                    step: None
                },
                Segment::Field("name".to_string()),
            ]
        );
    }

    #[test]
    fn test_array_slice_reverse_step() {
        let q = parse_query(".[::-1]").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Slice {
                start: None,
                end: None,
                step: Some(-1)
            }]
        );
    }

    // --- 복합 경로 ---

    #[test]
    fn test_complex_path() {
        let q = parse_query(".data.users[0].address.city").unwrap();
        assert_eq!(
            q.path.segments,
            vec![
                Segment::Field("data".to_string()),
                Segment::Field("users".to_string()),
                Segment::Index(0),
                Segment::Field("address".to_string()),
                Segment::Field("city".to_string()),
            ]
        );
    }

    // --- 필드 이름에 특수 문자 ---

    #[test]
    fn test_field_with_underscore() {
        let q = parse_query(".user_name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("user_name".to_string())]
        );
    }

    #[test]
    fn test_field_with_hyphen() {
        let q = parse_query(".content-type").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("content-type".to_string())]
        );
    }

    #[test]
    fn test_field_with_digits() {
        let q = parse_query(".field1").unwrap();
        assert_eq!(q.path.segments, vec![Segment::Field("field1".to_string())]);
    }

    // --- 에러 케이스 ---

    #[test]
    fn test_error_no_dot() {
        let err = parse_query("name").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_empty() {
        let err = parse_query("").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_unclosed_bracket() {
        let err = parse_query(".users[0").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_invalid_index() {
        let err = parse_query(".users[abc]").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_trailing_garbage() {
        let err = parse_query(".name xyz").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    // --- 공백 처리 ---

    #[test]
    fn test_whitespace_around() {
        let q = parse_query("  .name  ").unwrap();
        assert_eq!(q.path.segments, vec![Segment::Field("name".to_string())]);
    }

    // --- 루트 배열 접근 ---

    #[test]
    fn test_root_array_index() {
        let q = parse_query(".[0]").unwrap();
        assert_eq!(q.path.segments, vec![Segment::Index(0)]);
    }

    #[test]
    fn test_root_array_iterate() {
        let q = parse_query(".[]").unwrap();
        assert_eq!(q.path.segments, vec![Segment::Iterate]);
    }

    #[test]
    fn test_root_iterate_with_field() {
        let q = parse_query(".[].name").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Iterate, Segment::Field("name".to_string()),]
        );
    }

    // --- where 절 파싱 ---

    #[test]
    fn test_where_eq_integer() {
        let q = parse_query(".users[] | where age == 30").unwrap();
        assert_eq!(
            q.path.segments,
            vec![Segment::Field("users".to_string()), Segment::Iterate]
        );
        assert_eq!(q.operations.len(), 1);
        assert_eq!(
            q.operations[0],
            Operation::Where(Condition::Comparison(Comparison {
                field: "age".to_string(),
                op: CompareOp::Eq,
                value: LiteralValue::Integer(30),
            }))
        );
    }

    #[test]
    fn test_where_ne_string() {
        let q = parse_query(".items[] | where status != \"inactive\"").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Where(Condition::Comparison(Comparison {
                field: "status".to_string(),
                op: CompareOp::Ne,
                value: LiteralValue::String("inactive".to_string()),
            }))
        );
    }

    #[test]
    fn test_where_gt() {
        let q = parse_query(".[] | where age > 25").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.field, "age");
        assert_eq!(cmp.op, CompareOp::Gt);
        assert_eq!(cmp.value, LiteralValue::Integer(25));
    }

    #[test]
    fn test_where_lt() {
        let q = parse_query(".[] | where price < 100").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.op, CompareOp::Lt);
        assert_eq!(cmp.value, LiteralValue::Integer(100));
    }

    #[test]
    fn test_where_ge() {
        let q = parse_query(".[] | where score >= 80").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.op, CompareOp::Ge);
        assert_eq!(cmp.value, LiteralValue::Integer(80));
    }

    #[test]
    fn test_where_le() {
        let q = parse_query(".[] | where price <= 1000").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.op, CompareOp::Le);
        assert_eq!(cmp.value, LiteralValue::Integer(1000));
    }

    #[test]
    fn test_where_float_literal() {
        let q = parse_query(".[] | where score > 3.14").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.value, LiteralValue::Float(3.14));
    }

    #[test]
    fn test_where_negative_number() {
        let q = parse_query(".[] | where temp > -10").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.value, LiteralValue::Integer(-10));
    }

    #[test]
    fn test_where_bool_literal() {
        let q = parse_query(".[] | where active == true").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.value, LiteralValue::Bool(true));
    }

    #[test]
    fn test_where_null_literal() {
        let q = parse_query(".[] | where value == null").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.value, LiteralValue::Null);
    }

    #[test]
    fn test_where_no_operations_for_path_only() {
        let q = parse_query(".users[0].name").unwrap();
        assert!(q.operations.is_empty());
    }

    #[test]
    fn test_where_with_extra_whitespace() {
        let q = parse_query(".[]  |  where  age  >  30  ").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.field, "age");
        assert_eq!(cmp.op, CompareOp::Gt);
        assert_eq!(cmp.value, LiteralValue::Integer(30));
    }

    // --- where 파싱 에러 ---

    #[test]
    fn test_error_where_missing_field() {
        let err = parse_query(".[] | where == 30").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_where_missing_operator() {
        let err = parse_query(".[] | where age 30").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_where_missing_value() {
        let err = parse_query(".[] | where age >").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_where_unterminated_string() {
        let err = parse_query(".[] | where name == \"hello").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_unknown_operation() {
        let err = parse_query(".[] | foobar age > 30").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    // --- 문자열 연산자 파싱 ---

    #[test]
    fn test_where_contains() {
        let q = parse_query(".[] | where email contains \"@gmail\"").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.field, "email");
        assert_eq!(cmp.op, CompareOp::Contains);
        assert_eq!(cmp.value, LiteralValue::String("@gmail".to_string()));
    }

    #[test]
    fn test_where_starts_with() {
        let q = parse_query(".[] | where name starts_with \"A\"").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.field, "name");
        assert_eq!(cmp.op, CompareOp::StartsWith);
        assert_eq!(cmp.value, LiteralValue::String("A".to_string()));
    }

    #[test]
    fn test_where_ends_with() {
        let q = parse_query(".[] | where file ends_with \".json\"").unwrap();
        let Operation::Where(Condition::Comparison(cmp)) = &q.operations[0] else {
            panic!("expected Comparison");
        };
        assert_eq!(cmp.field, "file");
        assert_eq!(cmp.op, CompareOp::EndsWith);
        assert_eq!(cmp.value, LiteralValue::String(".json".to_string()));
    }

    // --- 논리 연산자 파싱 ---

    #[test]
    fn test_where_and() {
        let q = parse_query(".[] | where age > 25 and city == \"Seoul\"").unwrap();
        let Operation::Where(cond) = &q.operations[0] else {
            panic!("expected Where operation");
        };
        match cond {
            Condition::And(left, right) => {
                let Condition::Comparison(l) = left.as_ref() else {
                    panic!("expected left Comparison");
                };
                assert_eq!(l.field, "age");
                assert_eq!(l.op, CompareOp::Gt);
                assert_eq!(l.value, LiteralValue::Integer(25));
                let Condition::Comparison(r) = right.as_ref() else {
                    panic!("expected right Comparison");
                };
                assert_eq!(r.field, "city");
                assert_eq!(r.op, CompareOp::Eq);
                assert_eq!(r.value, LiteralValue::String("Seoul".to_string()));
            }
            _ => panic!("expected And condition"),
        }
    }

    #[test]
    fn test_where_or() {
        let q = parse_query(".[] | where role == \"admin\" or role == \"manager\"").unwrap();
        let Operation::Where(cond) = &q.operations[0] else {
            panic!("expected Where operation");
        };
        match cond {
            Condition::Or(left, right) => {
                let Condition::Comparison(l) = left.as_ref() else {
                    panic!("expected left Comparison");
                };
                assert_eq!(l.field, "role");
                assert_eq!(l.value, LiteralValue::String("admin".to_string()));
                let Condition::Comparison(r) = right.as_ref() else {
                    panic!("expected right Comparison");
                };
                assert_eq!(r.field, "role");
                assert_eq!(r.value, LiteralValue::String("manager".to_string()));
            }
            _ => panic!("expected Or condition"),
        }
    }

    #[test]
    fn test_where_and_with_string_op() {
        let q = parse_query(".[] | where name starts_with \"A\" and age > 20").unwrap();
        let Operation::Where(cond) = &q.operations[0] else {
            panic!("expected Where operation");
        };
        assert!(matches!(cond, Condition::And(_, _)));
    }

    #[test]
    fn test_where_chained_and() {
        let q = parse_query(".[] | where a == 1 and b == 2 and c == 3").unwrap();
        let Operation::Where(cond) = &q.operations[0] else {
            panic!("expected Where operation");
        };
        // Left-associative: ((a==1 and b==2) and c==3)
        match cond {
            Condition::And(left, right) => {
                assert!(matches!(left.as_ref(), Condition::And(_, _)));
                assert!(matches!(right.as_ref(), Condition::Comparison(_)));
            }
            _ => panic!("expected And condition"),
        }
    }

    // --- select 절 파싱 ---

    fn field(name: &str) -> SelectExpr {
        SelectExpr {
            expr: Expr::Field(name.to_string()),
            alias: None,
        }
    }

    fn fields(names: &[&str]) -> Operation {
        Operation::Select(names.iter().map(|n| field(n)).collect())
    }

    #[test]
    fn test_select_single_field() {
        let q = parse_query(".users[] | select name").unwrap();
        assert_eq!(q.operations.len(), 1);
        assert_eq!(q.operations[0], fields(&["name"]));
    }

    #[test]
    fn test_select_multiple_fields() {
        let q = parse_query(".users[] | select name, email").unwrap();
        assert_eq!(q.operations[0], fields(&["name", "email"]));
    }

    #[test]
    fn test_select_three_fields() {
        let q = parse_query(".users[] | select name, age, email").unwrap();
        assert_eq!(q.operations[0], fields(&["name", "age", "email"]));
    }

    #[test]
    fn test_select_with_extra_whitespace() {
        let q = parse_query(".[]  |  select  name ,  email  ").unwrap();
        assert_eq!(q.operations[0], fields(&["name", "email"]));
    }

    #[test]
    fn test_select_field_with_underscore() {
        let q = parse_query(".[] | select user_name, created_at").unwrap();
        assert_eq!(q.operations[0], fields(&["user_name", "created_at"]));
    }

    #[test]
    fn test_select_field_with_hyphen() {
        let q = parse_query(".[] | select content-type").unwrap();
        assert_eq!(q.operations[0], fields(&["content-type"]));
    }

    #[test]
    fn test_where_then_select() {
        let q = parse_query(".users[] | where age > 30 | select name, email").unwrap();
        assert_eq!(q.operations.len(), 2);
        assert!(matches!(&q.operations[0], Operation::Where(_)));
        assert_eq!(q.operations[1], fields(&["name", "email"]));
    }

    #[test]
    fn test_error_select_missing_fields() {
        let err = parse_query(".[] | select").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_select_func_single() {
        let q = parse_query(".[] | select upper(name)").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Select(vec![SelectExpr {
                expr: Expr::FuncCall {
                    name: "upper".to_string(),
                    args: vec![Expr::Field("name".to_string())],
                },
                alias: None,
            }])
        );
    }

    #[test]
    fn test_select_func_with_alias() {
        let q = parse_query(".[] | select upper(name) as NAME").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Select(vec![SelectExpr {
                expr: Expr::FuncCall {
                    name: "upper".to_string(),
                    args: vec![Expr::Field("name".to_string())],
                },
                alias: Some("NAME".to_string()),
            }])
        );
    }

    #[test]
    fn test_select_func_nested() {
        let q = parse_query(".[] | select upper(trim(name))").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Select(vec![SelectExpr {
                expr: Expr::FuncCall {
                    name: "upper".to_string(),
                    args: vec![Expr::FuncCall {
                        name: "trim".to_string(),
                        args: vec![Expr::Field("name".to_string())],
                    }],
                },
                alias: None,
            }])
        );
    }

    #[test]
    fn test_select_func_with_literal_arg() {
        let q = parse_query(".[] | select round(price, 2)").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Select(vec![SelectExpr {
                expr: Expr::FuncCall {
                    name: "round".to_string(),
                    args: vec![
                        Expr::Field("price".to_string()),
                        Expr::Literal(LiteralValue::Integer(2)),
                    ],
                },
                alias: None,
            }])
        );
    }

    #[test]
    fn test_select_mixed_fields_and_funcs() {
        let q = parse_query(".[] | select name, upper(city)").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Select(vec![
                field("name"),
                SelectExpr {
                    expr: Expr::FuncCall {
                        name: "upper".to_string(),
                        args: vec![Expr::Field("city".to_string())],
                    },
                    alias: None,
                }
            ])
        );
    }

    // --- sort 절 파싱 ---

    #[test]
    fn test_sort_asc() {
        let q = parse_query(".users[] | sort age").unwrap();
        assert_eq!(q.operations.len(), 1);
        assert_eq!(
            q.operations[0],
            Operation::Sort {
                field: "age".to_string(),
                descending: false,
            }
        );
    }

    #[test]
    fn test_sort_desc() {
        let q = parse_query(".users[] | sort age desc").unwrap();
        assert_eq!(q.operations.len(), 1);
        assert_eq!(
            q.operations[0],
            Operation::Sort {
                field: "age".to_string(),
                descending: true,
            }
        );
    }

    #[test]
    fn test_sort_with_extra_whitespace() {
        let q = parse_query(".[]  |  sort  name  ").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Sort {
                field: "name".to_string(),
                descending: false,
            }
        );
    }

    #[test]
    fn test_sort_desc_with_extra_whitespace() {
        let q = parse_query(".[]  |  sort  name  desc  ").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Sort {
                field: "name".to_string(),
                descending: true,
            }
        );
    }

    #[test]
    fn test_sort_field_with_underscore() {
        let q = parse_query(".[] | sort created_at").unwrap();
        assert_eq!(
            q.operations[0],
            Operation::Sort {
                field: "created_at".to_string(),
                descending: false,
            }
        );
    }

    #[test]
    fn test_error_sort_missing_field() {
        let err = parse_query(".[] | sort").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    // --- limit 절 파싱 ---

    #[test]
    fn test_limit() {
        let q = parse_query(".users[] | limit 10").unwrap();
        assert_eq!(q.operations.len(), 1);
        assert_eq!(q.operations[0], Operation::Limit(10));
    }

    #[test]
    fn test_limit_one() {
        let q = parse_query(".[] | limit 1").unwrap();
        assert_eq!(q.operations[0], Operation::Limit(1));
    }

    #[test]
    fn test_limit_with_extra_whitespace() {
        let q = parse_query(".[]  |  limit  5  ").unwrap();
        assert_eq!(q.operations[0], Operation::Limit(5));
    }

    #[test]
    fn test_error_limit_missing_number() {
        let err = parse_query(".[] | limit").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    #[test]
    fn test_error_limit_negative() {
        let err = parse_query(".[] | limit -5").unwrap_err();
        assert!(matches!(err, DkitError::QueryError(_)));
    }

    // --- 복합 파이프라인 ---

    #[test]
    fn test_where_sort_limit() {
        let q = parse_query(".users[] | where age > 20 | sort age desc | limit 5").unwrap();
        assert_eq!(q.operations.len(), 3);
        assert!(matches!(&q.operations[0], Operation::Where(_)));
        assert_eq!(
            q.operations[1],
            Operation::Sort {
                field: "age".to_string(),
                descending: true,
            }
        );
        assert_eq!(q.operations[2], Operation::Limit(5));
    }

    #[test]
    fn test_where_select_sort() {
        let q = parse_query(".users[] | where age > 30 | select name, email | sort name").unwrap();
        assert_eq!(q.operations.len(), 3);
        assert!(matches!(&q.operations[0], Operation::Where(_)));
        assert_eq!(q.operations[1], fields(&["name", "email"]));
        assert_eq!(
            q.operations[2],
            Operation::Sort {
                field: "name".to_string(),
                descending: false,
            }
        );
    }

    #[test]
    fn test_group_by_single_field() {
        let q = parse_query(".[] | group_by category").unwrap();
        assert_eq!(q.operations.len(), 1);
        match &q.operations[0] {
            Operation::GroupBy {
                fields,
                having,
                aggregates,
            } => {
                assert_eq!(fields, &vec!["category".to_string()]);
                assert!(having.is_none());
                assert!(aggregates.is_empty());
            }
            _ => panic!("expected GroupBy"),
        }
    }

    #[test]
    fn test_group_by_multiple_fields() {
        let q = parse_query(".[] | group_by region, category").unwrap();
        match &q.operations[0] {
            Operation::GroupBy { fields, .. } => {
                assert_eq!(fields, &vec!["region".to_string(), "category".to_string()]);
            }
            _ => panic!("expected GroupBy"),
        }
    }

    #[test]
    fn test_group_by_with_aggregates() {
        let q = parse_query(".[] | group_by category count(), sum(price), avg(score)").unwrap();
        match &q.operations[0] {
            Operation::GroupBy { aggregates, .. } => {
                assert_eq!(aggregates.len(), 3);
                assert_eq!(aggregates[0].func, AggregateFunc::Count);
                assert_eq!(aggregates[0].field, None);
                assert_eq!(aggregates[0].alias, "count");
                assert_eq!(aggregates[1].func, AggregateFunc::Sum);
                assert_eq!(aggregates[1].field, Some("price".to_string()));
                assert_eq!(aggregates[1].alias, "sum_price");
                assert_eq!(aggregates[2].func, AggregateFunc::Avg);
                assert_eq!(aggregates[2].field, Some("score".to_string()));
                assert_eq!(aggregates[2].alias, "avg_score");
            }
            _ => panic!("expected GroupBy"),
        }
    }

    #[test]
    fn test_group_by_with_having() {
        let q = parse_query(".[] | group_by category count() having count > 5").unwrap();
        match &q.operations[0] {
            Operation::GroupBy {
                fields,
                having,
                aggregates,
            } => {
                assert_eq!(fields, &vec!["category".to_string()]);
                assert!(having.is_some());
                assert_eq!(aggregates.len(), 1);
            }
            _ => panic!("expected GroupBy"),
        }
    }

    #[test]
    fn test_group_by_with_min_max() {
        let q = parse_query(".[] | group_by category min(price), max(price)").unwrap();
        match &q.operations[0] {
            Operation::GroupBy { aggregates, .. } => {
                assert_eq!(aggregates.len(), 2);
                assert_eq!(aggregates[0].func, AggregateFunc::Min);
                assert_eq!(aggregates[1].func, AggregateFunc::Max);
            }
            _ => panic!("expected GroupBy"),
        }
    }

    #[test]
    fn test_group_by_pipeline() {
        let q = parse_query(".[] | group_by category count() | sort count desc | limit 5").unwrap();
        assert_eq!(q.operations.len(), 3);
        assert!(matches!(&q.operations[0], Operation::GroupBy { .. }));
        assert!(matches!(
            &q.operations[1],
            Operation::Sort {
                descending: true,
                ..
            }
        ));
        assert_eq!(q.operations[2], Operation::Limit(5));
    }

    // --- parse_add_field_expr tests ---

    #[test]
    fn test_add_field_simple_arithmetic() {
        let (name, expr) = parse_add_field_expr("total = amount * quantity").unwrap();
        assert_eq!(name, "total");
        assert!(matches!(
            expr,
            Expr::BinaryOp {
                op: ArithmeticOp::Mul,
                ..
            }
        ));
    }

    #[test]
    fn test_add_field_string_concat() {
        let (name, expr) =
            parse_add_field_expr("full_name = first_name + \" \" + last_name").unwrap();
        assert_eq!(name, "full_name");
        // Should be (first_name + " ") + last_name
        assert!(matches!(
            expr,
            Expr::BinaryOp {
                op: ArithmeticOp::Add,
                ..
            }
        ));
    }

    #[test]
    fn test_add_field_with_literal() {
        let (name, expr) = parse_add_field_expr("tax = price * 0.1").unwrap();
        assert_eq!(name, "tax");
        assert!(matches!(
            expr,
            Expr::BinaryOp {
                op: ArithmeticOp::Mul,
                ..
            }
        ));
    }

    #[test]
    fn test_add_field_complex_expr() {
        let (name, expr) = parse_add_field_expr("total = price + price * 0.1").unwrap();
        assert_eq!(name, "total");
        // price + (price * 0.1) due to precedence
        if let Expr::BinaryOp { op, left, right } = &expr {
            assert_eq!(*op, ArithmeticOp::Add);
            assert!(matches!(left.as_ref(), Expr::Field(f) if f == "price"));
            assert!(matches!(
                right.as_ref(),
                Expr::BinaryOp {
                    op: ArithmeticOp::Mul,
                    ..
                }
            ));
        } else {
            panic!("expected BinaryOp");
        }
    }

    #[test]
    fn test_add_field_with_parens() {
        let (name, expr) = parse_add_field_expr("total = (price + tax) * quantity").unwrap();
        assert_eq!(name, "total");
        if let Expr::BinaryOp { op, left, right } = &expr {
            assert_eq!(*op, ArithmeticOp::Mul);
            assert!(matches!(
                left.as_ref(),
                Expr::BinaryOp {
                    op: ArithmeticOp::Add,
                    ..
                }
            ));
            assert!(matches!(right.as_ref(), Expr::Field(f) if f == "quantity"));
        } else {
            panic!("expected BinaryOp");
        }
    }

    #[test]
    fn test_add_field_with_function() {
        let (name, expr) = parse_add_field_expr("name_upper = upper(name)").unwrap();
        assert_eq!(name, "name_upper");
        assert!(matches!(expr, Expr::FuncCall { .. }));
    }

    #[test]
    fn test_add_field_missing_equals() {
        let result = parse_add_field_expr("total amount * quantity");
        assert!(result.is_err());
    }

    // --- Expr arithmetic tests ---

    #[test]
    fn test_expr_division() {
        let q = parse_query(".items[] | select total / count as avg").unwrap();
        assert_eq!(q.operations.len(), 1);
    }

    #[test]
    fn test_expr_subtraction() {
        let q = parse_query(".items[] | select price - discount as net").unwrap();
        assert_eq!(q.operations.len(), 1);
    }
}
