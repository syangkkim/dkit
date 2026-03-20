pub mod convert;
pub mod merge;
pub mod query;
pub mod schema;
pub mod stats;
pub mod view;

use std::path::Path;

/// 파일 읽기 실패 시 도움말 힌트가 포함된 에러 메시지 생성
pub fn read_file(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).map_err(|e| {
        let hint = if e.kind() == std::io::ErrorKind::NotFound {
            format!(
                "\n  Hint: check that the file path '{}' is correct",
                path.display()
            )
        } else if e.kind() == std::io::ErrorKind::PermissionDenied {
            format!(
                "\n  Hint: permission denied for '{}' — check file permissions",
                path.display()
            )
        } else {
            String::new()
        };
        anyhow::anyhow!("Failed to read '{}': {e}{hint}", path.display())
    })
}
