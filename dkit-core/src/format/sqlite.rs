use std::path::Path;

use indexmap::IndexMap;
use rusqlite::{Connection, OpenFlags};

use crate::error::DkitError;
use crate::value::Value;

/// SQLite 읽기 옵션
#[derive(Debug, Clone, Default)]
pub struct SqliteOptions {
    /// 읽을 테이블 이름
    pub table: Option<String>,
    /// 실행할 SQL 쿼리
    pub sql: Option<String>,
}

pub struct SqliteReader {
    options: SqliteOptions,
}

impl SqliteReader {
    pub fn new(options: SqliteOptions) -> Self {
        Self { options }
    }

    /// SQLite 파일에서 데이터를 읽어 Value로 변환
    pub fn read_from_path(&self, path: &Path) -> anyhow::Result<Value> {
        let conn =
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|e| {
                DkitError::ParseError {
                    format: "SQLite".to_string(),
                    source: Box::new(e),
                }
            })?;

        let query = if let Some(ref sql) = self.options.sql {
            sql.clone()
        } else if let Some(ref table) = self.options.table {
            // Validate table name to prevent SQL injection
            validate_table_name(table)?;
            format!("SELECT * FROM \"{}\"", table)
        } else {
            // Default: read from the first table
            let tables = list_tables_from_conn(&conn)?;
            if tables.is_empty() {
                return Ok(Value::Array(vec![]));
            }
            format!("SELECT * FROM \"{}\"", tables[0])
        };

        execute_query(&conn, &query)
    }

    /// SQLite 파일의 테이블 목록을 반환
    pub fn list_tables(path: &Path) -> anyhow::Result<Vec<String>> {
        let conn =
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|e| {
                DkitError::ParseError {
                    format: "SQLite".to_string(),
                    source: Box::new(e),
                }
            })?;
        list_tables_from_conn(&conn)
    }
}

/// 테이블 이름을 검증한다 (SQL 인젝션 방지)
fn validate_table_name(name: &str) -> anyhow::Result<()> {
    if name.is_empty() {
        anyhow::bail!("Table name cannot be empty");
    }
    // Allow alphanumeric, underscore, and dot (for schema.table)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    {
        anyhow::bail!(
            "Invalid table name '{}': only alphanumeric characters, underscores, and dots are allowed",
            name
        );
    }
    Ok(())
}

/// 연결에서 테이블 목록을 조회한다
fn list_tables_from_conn(conn: &Connection) -> anyhow::Result<Vec<String>> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .map_err(|e| DkitError::ParseError {
            format: "SQLite".to_string(),
            source: Box::new(e),
        })?;

    let tables = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| DkitError::ParseError {
            format: "SQLite".to_string(),
            source: Box::new(e),
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DkitError::ParseError {
            format: "SQLite".to_string(),
            source: Box::new(e),
        })?;

    Ok(tables)
}

/// SQL 쿼리를 실행하고 결과를 Value::Array로 변환한다
fn execute_query(conn: &Connection, sql: &str) -> anyhow::Result<Value> {
    let mut stmt = conn.prepare(sql).map_err(|e| DkitError::ParseError {
        format: "SQLite".to_string(),
        source: Box::new(e),
    })?;

    let column_count = stmt.column_count();
    let column_names: Vec<String> = (0..column_count)
        .map(|i| stmt.column_name(i).unwrap_or("?").to_string())
        .collect();

    let rows = stmt
        .query_map([], |row| {
            let mut obj = IndexMap::new();
            for (i, name) in column_names.iter().enumerate() {
                let value = sqlite_value_to_value(row, i);
                obj.insert(name.clone(), value);
            }
            Ok(Value::Object(obj))
        })
        .map_err(|e| DkitError::ParseError {
            format: "SQLite".to_string(),
            source: Box::new(e),
        })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| DkitError::ParseError {
            format: "SQLite".to_string(),
            source: Box::new(e),
        })?);
    }

    Ok(Value::Array(result))
}

/// SQLite 값을 dkit Value로 변환한다
fn sqlite_value_to_value(row: &rusqlite::Row, idx: usize) -> Value {
    // Try to get the value as different types in order of specificity
    // rusqlite's ValueRef gives us the actual SQLite type
    let value_ref = row.get_ref(idx).unwrap_or(rusqlite::types::ValueRef::Null);

    match value_ref {
        rusqlite::types::ValueRef::Null => Value::Null,
        rusqlite::types::ValueRef::Integer(i) => Value::Integer(i),
        rusqlite::types::ValueRef::Real(f) => Value::Float(f),
        rusqlite::types::ValueRef::Text(bytes) => {
            Value::String(String::from_utf8_lossy(bytes).into_owned())
        }
        rusqlite::types::ValueRef::Blob(bytes) => {
            // Encode blob as hex string with prefix
            Value::String(format!("x'{}'", hex_encode(bytes)))
        }
    }
}

/// バイト列を16進数文字列にエンコードする
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(all(test, not(miri)))]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER, score REAL);
             INSERT INTO users VALUES (1, 'Alice', 30, 95.5);
             INSERT INTO users VALUES (2, 'Bob', 25, 88.0);
             INSERT INTO users VALUES (3, 'Charlie', 35, NULL);",
        )
        .unwrap();
        file
    }

    fn create_multi_table_db() -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
             INSERT INTO users VALUES (1, 'Alice');
             CREATE TABLE products (id INTEGER PRIMARY KEY, title TEXT, price REAL);
             INSERT INTO products VALUES (1, 'Widget', 9.99);
             INSERT INTO products VALUES (2, 'Gadget', 24.95);",
        )
        .unwrap();
        file
    }

    #[test]
    fn test_list_tables() {
        let db = create_test_db();
        let tables = SqliteReader::list_tables(db.path()).unwrap();
        assert_eq!(tables, vec!["users"]);
    }

    #[test]
    fn test_list_tables_multiple() {
        let db = create_multi_table_db();
        let tables = SqliteReader::list_tables(db.path()).unwrap();
        assert_eq!(tables, vec!["products", "users"]);
    }

    #[test]
    fn test_read_default_table() {
        let db = create_test_db();
        let reader = SqliteReader::new(SqliteOptions::default());
        let value = reader.read_from_path(db.path()).unwrap();

        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 3);

        let first = arr[0].as_object().unwrap();
        assert_eq!(first.get("id"), Some(&Value::Integer(1)));
        assert_eq!(first.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(first.get("age"), Some(&Value::Integer(30)));
        assert_eq!(first.get("score"), Some(&Value::Float(95.5)));
    }

    #[test]
    fn test_read_specific_table() {
        let db = create_multi_table_db();
        let reader = SqliteReader::new(SqliteOptions {
            table: Some("products".to_string()),
            sql: None,
        });
        let value = reader.read_from_path(db.path()).unwrap();

        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        let first = arr[0].as_object().unwrap();
        assert_eq!(
            first.get("title"),
            Some(&Value::String("Widget".to_string()))
        );
        assert_eq!(first.get("price"), Some(&Value::Float(9.99)));
    }

    #[test]
    fn test_read_with_sql_query() {
        let db = create_test_db();
        let reader = SqliteReader::new(SqliteOptions {
            table: None,
            sql: Some("SELECT name, age FROM users WHERE age > 25 ORDER BY age".to_string()),
        });
        let value = reader.read_from_path(db.path()).unwrap();

        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        let first = arr[0].as_object().unwrap();
        assert_eq!(first.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(first.get("age"), Some(&Value::Integer(30)));

        let second = arr[1].as_object().unwrap();
        assert_eq!(
            second.get("name"),
            Some(&Value::String("Charlie".to_string()))
        );
        assert_eq!(second.get("age"), Some(&Value::Integer(35)));
    }

    #[test]
    fn test_null_value_handling() {
        let db = create_test_db();
        let reader = SqliteReader::new(SqliteOptions {
            table: None,
            sql: Some("SELECT score FROM users WHERE name = 'Charlie'".to_string()),
        });
        let value = reader.read_from_path(db.path()).unwrap();

        let arr = value.as_array().unwrap();
        let first = arr[0].as_object().unwrap();
        assert_eq!(first.get("score"), Some(&Value::Null));
    }

    #[test]
    fn test_blob_value_handling() {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();
        conn.execute_batch(
            "CREATE TABLE blobs (id INTEGER, data BLOB);
             INSERT INTO blobs VALUES (1, x'DEADBEEF');",
        )
        .unwrap();

        let reader = SqliteReader::new(SqliteOptions {
            table: Some("blobs".to_string()),
            sql: None,
        });
        let value = reader.read_from_path(file.path()).unwrap();

        let arr = value.as_array().unwrap();
        let first = arr[0].as_object().unwrap();
        assert_eq!(
            first.get("data"),
            Some(&Value::String("x'deadbeef'".to_string()))
        );
    }

    #[test]
    fn test_empty_table() {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();
        conn.execute_batch("CREATE TABLE empty_table (id INTEGER, name TEXT);")
            .unwrap();

        let reader = SqliteReader::new(SqliteOptions {
            table: Some("empty_table".to_string()),
            sql: None,
        });
        let value = reader.read_from_path(file.path()).unwrap();

        let arr = value.as_array().unwrap();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_no_tables() {
        let file = NamedTempFile::new().unwrap();
        let _conn = Connection::open(file.path()).unwrap();

        let reader = SqliteReader::new(SqliteOptions::default());
        let value = reader.read_from_path(file.path()).unwrap();
        assert_eq!(value, Value::Array(vec![]));
    }

    #[test]
    fn test_validate_table_name_valid() {
        assert!(validate_table_name("users").is_ok());
        assert!(validate_table_name("my_table").is_ok());
        assert!(validate_table_name("schema.table").is_ok());
        assert!(validate_table_name("Table123").is_ok());
    }

    #[test]
    fn test_validate_table_name_invalid() {
        assert!(validate_table_name("").is_err());
        assert!(validate_table_name("users; DROP TABLE").is_err());
        assert!(validate_table_name("table name").is_err());
    }

    #[test]
    fn test_float_as_integer_preservation() {
        let file = NamedTempFile::new().unwrap();
        let conn = Connection::open(file.path()).unwrap();
        conn.execute_batch(
            "CREATE TABLE nums (int_val INTEGER, real_val REAL);
             INSERT INTO nums VALUES (42, 3.14);",
        )
        .unwrap();

        let reader = SqliteReader::new(SqliteOptions {
            table: Some("nums".to_string()),
            sql: None,
        });
        let value = reader.read_from_path(file.path()).unwrap();

        let arr = value.as_array().unwrap();
        let first = arr[0].as_object().unwrap();
        assert_eq!(first.get("int_val"), Some(&Value::Integer(42)));
        assert_eq!(first.get("real_val"), Some(&Value::Float(3.14)));
    }
}
