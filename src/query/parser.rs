use crate::error::DkitError;

/// 쿼리 AST (Abstract Syntax Tree)
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    /// 경로 접근 (`.users[0].name`)
    pub path: Path,
}

/// 경로: `.` + 세그먼트들
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Segment>,
}

/// 경로 세그먼트
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    /// 필드 접근 (`.name`)
    Field(String),
    /// 배열 인덱스 접근 (`[0]`, `[-1]`)
    Index(i64),
    /// 배열 이터레이션 (`[]`)
    Iterate,
}

/// 쿼리 문자열을 파싱하는 파서
pub struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    /// 쿼리 문자열을 파싱하여 Query AST를 반환
    pub fn parse(&mut self) -> Result<Query, DkitError> {
        self.skip_whitespace();
        let path = self.parse_path()?;
        self.skip_whitespace();

        if self.pos != self.input.len() {
            return Err(DkitError::QueryError(format!(
                "unexpected character '{}' at position {}",
                self.input[self.pos], self.pos
            )));
        }

        Ok(Query { path })
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

    /// `[...]` 파싱: 인덱스 또는 이터레이션
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

        // `[N]` or `[-N]` — 인덱스
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
        let index: i64 = num_str.parse().map_err(|_| {
            DkitError::QueryError(format!("invalid index '{}' at position {}", num_str, start))
        })?;

        self.skip_whitespace();
        if !self.consume_char(']') {
            return Err(DkitError::QueryError(format!(
                "expected ']' at position {}",
                self.pos
            )));
        }

        Ok(Segment::Index(if negative { -index } else { index }))
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
}

/// 편의 함수: 쿼리 문자열 → Query
pub fn parse_query(input: &str) -> Result<Query, DkitError> {
    Parser::new(input).parse()
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
}
