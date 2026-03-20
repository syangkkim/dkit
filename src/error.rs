/// dkit 에러 타입 정의
///
/// 포맷 파싱, 쓰기, IO, 쿼리 등 카테고리별 에러를 구분하며,
/// `thiserror`로 `Display`와 `Error`를 자동 구현한다.
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

    #[allow(dead_code)]
    #[error("Invalid query: {0}")]
    QueryError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[allow(dead_code)]
    #[error("Path not found: {0}")]
    PathNotFound(String),
}

/// `anyhow` 통합을 위한 `Result` 타입 별칭.
/// 라이브러리 내부에서는 `DkitError`를 직접 사용하고,
/// 애플리케이션 레벨에서는 `anyhow::Result`로 변환하여 사용한다.
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, DkitError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_format_display() {
        let err = DkitError::UnknownFormat("xml".to_string());
        assert_eq!(err.to_string(), "Unknown format: xml");
    }

    #[test]
    fn test_parse_error_display() {
        let source: Box<dyn std::error::Error + Send + Sync> =
            "unexpected token".to_string().into();
        let err = DkitError::ParseError {
            format: "JSON".to_string(),
            source,
        };
        assert_eq!(err.to_string(), "Failed to parse JSON: unexpected token");
    }

    #[test]
    fn test_write_error_display() {
        let source: Box<dyn std::error::Error + Send + Sync> =
            "serialization failed".to_string().into();
        let err = DkitError::WriteError {
            format: "TOML".to_string(),
            source,
        };
        assert_eq!(
            err.to_string(),
            "Failed to write TOML: serialization failed"
        );
    }

    #[test]
    fn test_query_error_display() {
        let err = DkitError::QueryError("invalid syntax at position 5".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid query: invalid syntax at position 5"
        );
    }

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: DkitError = io_err.into();
        assert!(matches!(err, DkitError::IoError(_)));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_path_not_found_display() {
        let err = DkitError::PathNotFound(".users[0].name".to_string());
        assert_eq!(err.to_string(), "Path not found: .users[0].name");
    }

    #[test]
    fn test_anyhow_conversion() {
        // DkitError는 anyhow::Error로 변환 가능해야 한다
        let err = DkitError::UnknownFormat("bin".to_string());
        let anyhow_err: anyhow::Error = err.into();
        assert_eq!(anyhow_err.to_string(), "Unknown format: bin");
    }

    #[test]
    fn test_result_type_alias() {
        let ok: Result<i32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);

        let err: Result<i32> = Err(DkitError::UnknownFormat("x".to_string()));
        assert!(err.is_err());
    }
}
