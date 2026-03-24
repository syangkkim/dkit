use std::io::{Read, Write};

use indexmap::IndexMap;
use toml::Table;

use crate::format::{FormatOptions, FormatReader, FormatWriter};
use crate::value::Value;

/// toml::Value → 내부 Value 변환
fn from_toml_value(v: toml::Value) -> Value {
    match v {
        toml::Value::Boolean(b) => Value::Bool(b),
        toml::Value::Integer(n) => Value::Integer(n),
        toml::Value::Float(f) => Value::Float(f),
        toml::Value::String(s) => Value::String(s),
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
        toml::Value::Array(arr) => Value::Array(arr.into_iter().map(from_toml_value).collect()),
        toml::Value::Table(table) => {
            let obj: IndexMap<String, Value> = table
                .into_iter()
                .map(|(k, v)| (k, from_toml_value(v)))
                .collect();
            Value::Object(obj)
        }
    }
}

/// 내부 Value → toml::Value 변환
///
/// TOML에는 Null 타입이 없으므로 Null → String("null")로 변환한다.
fn to_toml_value(v: &Value) -> toml::Value {
    match v {
        Value::Null => toml::Value::String("null".to_string()),
        Value::Bool(b) => toml::Value::Boolean(*b),
        Value::Integer(n) => toml::Value::Integer(*n),
        Value::Float(f) => {
            if f.is_nan() || f.is_infinite() {
                toml::Value::String(f.to_string())
            } else {
                toml::Value::Float(*f)
            }
        }
        Value::String(s) => toml::Value::String(s.clone()),
        Value::Array(arr) => toml::Value::Array(arr.iter().map(to_toml_value).collect()),
        Value::Object(map) => {
            let table: Table = map
                .iter()
                .map(|(k, v)| (k.clone(), to_toml_value(v)))
                .collect();
            toml::Value::Table(table)
        }
    }
}

/// TOML 포맷 Reader
pub struct TomlReader;

/// Convert a byte offset into a 1-indexed (line, column) pair.
fn byte_offset_to_line_col(s: &str, offset: usize) -> (usize, usize) {
    let before = &s[..offset.min(s.len())];
    let line = before.chars().filter(|&c| c == '\n').count() + 1;
    let column = before.rfind('\n').map(|p| offset - p).unwrap_or(offset + 1);
    (line, column)
}

impl FormatReader for TomlReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let toml_val: toml::Value = toml::from_str(input).map_err(|e: toml::de::Error| {
            if let Some(span) = e.span() {
                let (line, column) = byte_offset_to_line_col(input, span.start);
                let line_text = input
                    .lines()
                    .nth(line.saturating_sub(1))
                    .unwrap_or("")
                    .to_string();
                crate::error::DkitError::ParseErrorAt {
                    format: "TOML".to_string(),
                    source: Box::new(e),
                    line,
                    column,
                    line_text,
                }
            } else {
                crate::error::DkitError::ParseError {
                    format: "TOML".to_string(),
                    source: Box::new(e),
                }
            }
        })?;
        Ok(from_toml_value(toml_val))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| crate::error::DkitError::ParseError {
                format: "TOML".to_string(),
                source: Box::new(e),
            })?;
        self.read(&input)
    }
}

/// TOML 포맷 Writer
#[derive(Default)]
pub struct TomlWriter {
    options: FormatOptions,
}

impl TomlWriter {
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }
}

impl FormatWriter for TomlWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        // TOML 최상위는 반드시 테이블이어야 한다.
        // 배열이나 프리미티브가 오면 "data" 키로 감싼다.
        let toml_val = match value {
            Value::Object(_) => to_toml_value(value),
            _ => {
                let mut table = Table::new();
                table.insert("data".to_string(), to_toml_value(value));
                toml::Value::Table(table)
            }
        };

        let output = if self.options.pretty {
            toml::to_string_pretty(&toml_val)
        } else {
            toml::to_string(&toml_val)
        };

        output.map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "TOML".to_string(),
                source: Box::new(e),
            }
            .into()
        })
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let output = self.write(value)?;
        writer
            .write_all(output.as_bytes())
            .map_err(|e| crate::error::DkitError::WriteError {
                format: "TOML".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- from_toml_value 변환 테스트 ---

    #[test]
    fn test_convert_bool() {
        assert_eq!(
            from_toml_value(toml::Value::Boolean(true)),
            Value::Bool(true)
        );
        assert_eq!(
            from_toml_value(toml::Value::Boolean(false)),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_convert_integer() {
        assert_eq!(
            from_toml_value(toml::Value::Integer(42)),
            Value::Integer(42)
        );
    }

    #[test]
    fn test_convert_float() {
        assert_eq!(
            from_toml_value(toml::Value::Float(3.14)),
            Value::Float(3.14)
        );
    }

    #[test]
    fn test_convert_string() {
        assert_eq!(
            from_toml_value(toml::Value::String("hello".to_string())),
            Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_convert_datetime() {
        let dt_str = "2024-01-15T10:30:00";
        let toml_input = format!("dt = {dt_str}");
        let table: Table = toml::from_str(&toml_input).unwrap();
        let val = from_toml_value(table["dt"].clone());
        assert_eq!(val, Value::String(dt_str.to_string()));
    }

    #[test]
    fn test_convert_array() {
        let arr = toml::Value::Array(vec![
            toml::Value::Integer(1),
            toml::Value::String("two".to_string()),
            toml::Value::Boolean(true),
        ]);
        let v = from_toml_value(arr);
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::Integer(1));
        assert_eq!(arr[1], Value::String("two".to_string()));
        assert_eq!(arr[2], Value::Bool(true));
    }

    #[test]
    fn test_convert_table() {
        let mut table = Table::new();
        table.insert("name".to_string(), toml::Value::String("dkit".to_string()));
        table.insert("version".to_string(), toml::Value::Integer(1));
        let v = from_toml_value(toml::Value::Table(table));
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("version"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_convert_nested() {
        let toml_input = r#"
[server]
host = "localhost"
port = 8080

[server.tls]
enabled = true
"#;
        let toml_val: toml::Value = toml::from_str(toml_input).unwrap();
        let v = from_toml_value(toml_val);
        let server = v.as_object().unwrap().get("server").unwrap();
        assert_eq!(
            server.as_object().unwrap().get("host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(
            server.as_object().unwrap().get("port"),
            Some(&Value::Integer(8080))
        );
        let tls = server.as_object().unwrap().get("tls").unwrap();
        assert_eq!(
            tls.as_object().unwrap().get("enabled"),
            Some(&Value::Bool(true))
        );
    }

    // --- to_toml_value 왕복 변환 테스트 ---

    #[test]
    fn test_roundtrip_primitives() {
        let values = vec![
            Value::Bool(false),
            Value::Integer(100),
            Value::Float(2.718),
            Value::String("test".to_string()),
        ];
        for v in values {
            let toml_v = to_toml_value(&v);
            let back = from_toml_value(toml_v);
            assert_eq!(back, v);
        }
    }

    #[test]
    fn test_roundtrip_complex() {
        let mut map = IndexMap::new();
        map.insert(
            "key".to_string(),
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
        );
        let original = Value::Object(map);
        let toml_v = to_toml_value(&original);
        let back = from_toml_value(toml_v);
        assert_eq!(back, original);
    }

    #[test]
    fn test_null_converts_to_string() {
        let toml_v = to_toml_value(&Value::Null);
        assert_eq!(toml_v, toml::Value::String("null".to_string()));
    }

    #[test]
    fn test_nan_converts_to_string() {
        let toml_v = to_toml_value(&Value::Float(f64::NAN));
        assert_eq!(toml_v, toml::Value::String("NaN".to_string()));
    }

    #[test]
    fn test_infinity_converts_to_string() {
        let toml_v = to_toml_value(&Value::Float(f64::INFINITY));
        assert_eq!(toml_v, toml::Value::String("inf".to_string()));
    }

    // --- TomlReader 테스트 ---

    #[test]
    fn test_reader_simple_table() {
        let reader = TomlReader;
        let v = reader.read("name = \"dkit\"\ncount = 42").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("count"), Some(&Value::Integer(42)));
    }

    #[test]
    fn test_reader_nested_table() {
        let reader = TomlReader;
        let toml_input = r#"
[package]
name = "dkit"
version = "0.1.0"
"#;
        let v = reader.read(toml_input).unwrap();
        let pkg = v.as_object().unwrap().get("package").unwrap();
        assert_eq!(
            pkg.as_object().unwrap().get("name"),
            Some(&Value::String("dkit".to_string()))
        );
    }

    #[test]
    fn test_reader_array_of_tables() {
        let reader = TomlReader;
        let toml_input = r#"
[[users]]
name = "Alice"
age = 30

[[users]]
name = "Bob"
age = 25
"#;
        let v = reader.read(toml_input).unwrap();
        let users = v
            .as_object()
            .unwrap()
            .get("users")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(
            users[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(
            users[1].as_object().unwrap().get("age"),
            Some(&Value::Integer(25))
        );
    }

    #[test]
    fn test_reader_invalid_toml() {
        let reader = TomlReader;
        let result = reader.read("[invalid\nkey = ");
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_empty_table() {
        let reader = TomlReader;
        let v = reader.read("").unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = TomlReader;
        let input = "x = 1".as_bytes();
        let v = reader.read_from_reader(input).unwrap();
        assert_eq!(v.as_object().unwrap().get("x"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_reader_from_reader_invalid() {
        let reader = TomlReader;
        let input = b"[bad" as &[u8];
        assert!(reader.read_from_reader(input).is_err());
    }

    #[test]
    fn test_reader_inline_table() {
        let reader = TomlReader;
        let v = reader.read("point = { x = 1, y = 2 }").unwrap();
        let point = v.as_object().unwrap().get("point").unwrap();
        assert_eq!(
            point.as_object().unwrap().get("x"),
            Some(&Value::Integer(1))
        );
        assert_eq!(
            point.as_object().unwrap().get("y"),
            Some(&Value::Integer(2))
        );
    }

    #[test]
    fn test_reader_multiline_string() {
        let reader = TomlReader;
        let toml_input = "desc = \"\"\"\nline1\nline2\"\"\"";
        let v = reader.read(toml_input).unwrap();
        let desc = v
            .as_object()
            .unwrap()
            .get("desc")
            .unwrap()
            .as_str()
            .unwrap();
        assert!(desc.contains("line1"));
        assert!(desc.contains("line2"));
    }

    #[test]
    fn test_reader_datetime() {
        let reader = TomlReader;
        let v = reader.read("created = 2024-01-15T10:30:00").unwrap();
        let created = v.as_object().unwrap().get("created").unwrap();
        assert_eq!(created, &Value::String("2024-01-15T10:30:00".to_string()));
    }

    // --- TomlWriter 테스트 ---

    #[test]
    fn test_writer_default() {
        let writer = TomlWriter::default();
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("a"));
        assert!(output.contains('1'));
    }

    #[test]
    fn test_writer_pretty() {
        let writer = TomlWriter::new(FormatOptions {
            pretty: true,
            ..Default::default()
        });
        let mut inner = IndexMap::new();
        inner.insert("host".to_string(), Value::String("localhost".to_string()));
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("server".to_string(), Value::Object(inner));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("[server]"));
    }

    #[test]
    fn test_writer_compact() {
        let writer = TomlWriter::new(FormatOptions {
            pretty: false,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("a = 1"));
    }

    #[test]
    fn test_writer_wraps_non_table() {
        let writer = TomlWriter::default();
        let output = writer.write(&Value::Integer(42)).unwrap();
        assert!(output.contains("data = 42"));
    }

    #[test]
    fn test_writer_wraps_array() {
        let writer = TomlWriter::default();
        let v = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let output = writer.write(&v).unwrap();
        assert!(output.contains("data"));
    }

    #[test]
    fn test_writer_null_as_string() {
        let writer = TomlWriter::default();
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("val".to_string(), Value::Null);
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("\"null\""));
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = TomlWriter::default();
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("x".to_string(), Value::Integer(42));
            m
        });
        let mut buf = Vec::new();
        writer.write_to_writer(&v, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("x"));
        assert!(output.contains("42"));
    }

    // --- 왕복 변환 테스트 (Reader → Writer → Reader) ---

    #[test]
    fn test_full_roundtrip() {
        let toml_input = r#"
name = "dkit"
version = 1

[settings]
debug = false
threshold = 0.5

[settings.nested]
key = "value"
"#;
        let reader = TomlReader;
        let writer = TomlWriter::new(FormatOptions {
            pretty: true,
            ..Default::default()
        });

        let value = reader.read(toml_input).unwrap();
        let toml_output = writer.write(&value).unwrap();
        let value2 = reader.read(&toml_output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_array_of_tables() {
        let toml_input = r#"
[[items]]
name = "a"
count = 1

[[items]]
name = "b"
count = 2
"#;
        let reader = TomlReader;
        let writer = TomlWriter::new(FormatOptions {
            pretty: true,
            ..Default::default()
        });

        let value = reader.read(toml_input).unwrap();
        let toml_output = writer.write(&value).unwrap();
        let value2 = reader.read(&toml_output).unwrap();

        assert_eq!(value, value2);
    }

    // --- 특수 케이스 ---

    #[test]
    fn test_unicode_string() {
        let reader = TomlReader;
        let v = reader.read("emoji = \"🎉\"\nkorean = \"한글\"").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("emoji"), Some(&Value::String("🎉".to_string())));
        assert_eq!(obj.get("korean"), Some(&Value::String("한글".to_string())));
    }

    #[test]
    fn test_negative_numbers() {
        let reader = TomlReader;
        let v = reader.read("neg_int = -42\nneg_float = -3.14").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("neg_int"), Some(&Value::Integer(-42)));
        assert_eq!(obj.get("neg_float"), Some(&Value::Float(-3.14)));
    }

    #[test]
    fn test_deeply_nested() {
        let toml_input = r#"
[a.b.c]
d = 1
"#;
        let reader = TomlReader;
        let v = reader.read(toml_input).unwrap();
        let d = v
            .as_object()
            .unwrap()
            .get("a")
            .unwrap()
            .as_object()
            .unwrap()
            .get("b")
            .unwrap()
            .as_object()
            .unwrap()
            .get("c")
            .unwrap()
            .as_object()
            .unwrap()
            .get("d")
            .unwrap();
        assert_eq!(d, &Value::Integer(1));
    }

    #[test]
    fn test_boolean_values() {
        let reader = TomlReader;
        let v = reader.read("yes_val = true\nno_val = false").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("yes_val"), Some(&Value::Bool(true)));
        assert_eq!(obj.get("no_val"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_array_of_mixed_types_string() {
        // TOML arrays must be homogeneous; mixed types are not valid TOML
        let reader = TomlReader;
        let v = reader.read("tags = [\"rust\", \"cli\", \"tool\"]").unwrap();
        let tags = v
            .as_object()
            .unwrap()
            .get("tags")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], Value::String("rust".to_string()));
    }

    #[test]
    fn test_large_integer() {
        let reader = TomlReader;
        let v = reader.read("big = 9223372036854775807").unwrap();
        assert_eq!(
            v.as_object().unwrap().get("big"),
            Some(&Value::Integer(i64::MAX))
        );
    }
}
