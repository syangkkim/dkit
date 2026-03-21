use std::io::{Read, Write};

use crate::format::json::{from_json_value, to_json_value};
use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// MessagePack 포맷 Reader
pub struct MsgpackReader;

impl FormatReader for MsgpackReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        self.read_from_reader(input.as_bytes())
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        self.read_from_bytes(&buf)
    }
}

impl MsgpackReader {
    /// 바이트 슬라이스에서 Value를 읽는다
    pub fn read_from_bytes(&self, bytes: &[u8]) -> anyhow::Result<Value> {
        let json_val: serde_json::Value =
            rmp_serde::from_slice(bytes).map_err(|e| crate::error::DkitError::ParseError {
                format: "MessagePack".to_string(),
                source: Box::new(e),
            })?;
        Ok(from_json_value(json_val))
    }
}

/// MessagePack 포맷 Writer
pub struct MsgpackWriter;

impl FormatWriter for MsgpackWriter {
    fn write(&self, _value: &Value) -> anyhow::Result<String> {
        anyhow::bail!("MessagePack is a binary format. Use write_to_writer or write_bytes instead.")
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let bytes = self.write_bytes(value)?;
        writer.write_all(&bytes)?;
        Ok(())
    }
}

impl MsgpackWriter {
    /// Value를 MessagePack 바이너리로 직렬화
    pub fn write_bytes(&self, value: &Value) -> anyhow::Result<Vec<u8>> {
        let json_val = to_json_value(value);
        rmp_serde::to_vec(&json_val).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "MessagePack".to_string(),
                source: Box::new(e),
            }
            .into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn roundtrip(val: &Value) -> Value {
        let bytes = MsgpackWriter.write_bytes(val).unwrap();
        MsgpackReader.read_from_bytes(&bytes).unwrap()
    }

    // --- 기본 타입 라운드트립 ---

    #[test]
    fn test_null() {
        assert_eq!(roundtrip(&Value::Null), Value::Null);
    }

    #[test]
    fn test_bool_true() {
        assert_eq!(roundtrip(&Value::Bool(true)), Value::Bool(true));
    }

    #[test]
    fn test_bool_false() {
        assert_eq!(roundtrip(&Value::Bool(false)), Value::Bool(false));
    }

    #[test]
    fn test_integer_positive() {
        assert_eq!(roundtrip(&Value::Integer(42)), Value::Integer(42));
    }

    #[test]
    fn test_integer_negative() {
        assert_eq!(roundtrip(&Value::Integer(-100)), Value::Integer(-100));
    }

    #[test]
    fn test_integer_zero() {
        assert_eq!(roundtrip(&Value::Integer(0)), Value::Integer(0));
    }

    #[test]
    fn test_float() {
        assert_eq!(roundtrip(&Value::Float(3.14)), Value::Float(3.14));
    }

    #[test]
    fn test_float_negative() {
        assert_eq!(roundtrip(&Value::Float(-2.5)), Value::Float(-2.5));
    }

    #[test]
    fn test_string() {
        let val = Value::String("hello".to_string());
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_string_empty() {
        let val = Value::String(String::new());
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_string_unicode() {
        let val = Value::String("안녕하세요 🌍".to_string());
        assert_eq!(roundtrip(&val), val);
    }

    // --- 컬렉션 타입 ---

    #[test]
    fn test_array_empty() {
        let val = Value::Array(vec![]);
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_array_mixed() {
        let val = Value::Array(vec![
            Value::Integer(1),
            Value::String("two".to_string()),
            Value::Bool(true),
            Value::Null,
        ]);
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_object_simple() {
        let mut map = IndexMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Integer(30));
        let val = Value::Object(map);
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_object_empty() {
        let val = Value::Object(IndexMap::new());
        assert_eq!(roundtrip(&val), val);
    }

    // --- 중첩 구조 ---

    #[test]
    fn test_nested_object() {
        let mut inner = IndexMap::new();
        inner.insert("host".to_string(), Value::String("localhost".to_string()));
        inner.insert("port".to_string(), Value::Integer(5432));

        let mut outer = IndexMap::new();
        outer.insert("database".to_string(), Value::Object(inner));
        outer.insert("debug".to_string(), Value::Bool(false));

        let val = Value::Object(outer);
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_array_of_objects() {
        let mut obj1 = IndexMap::new();
        obj1.insert("id".to_string(), Value::Integer(1));
        obj1.insert("name".to_string(), Value::String("Alice".to_string()));

        let mut obj2 = IndexMap::new();
        obj2.insert("id".to_string(), Value::Integer(2));
        obj2.insert("name".to_string(), Value::String("Bob".to_string()));

        let val = Value::Array(vec![Value::Object(obj1), Value::Object(obj2)]);
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_deeply_nested() {
        let val = Value::Array(vec![Value::Array(vec![Value::Array(vec![
            Value::Integer(42),
        ])])]);
        assert_eq!(roundtrip(&val), val);
    }

    // --- 경계값 ---

    #[test]
    fn test_large_integer() {
        let val = Value::Integer(i64::MAX);
        assert_eq!(roundtrip(&val), val);
    }

    #[test]
    fn test_large_negative_integer() {
        let val = Value::Integer(i64::MIN);
        assert_eq!(roundtrip(&val), val);
    }

    // --- Reader/Writer 인터페이스 ---

    #[test]
    fn test_read_from_reader() {
        let val = Value::String("test".to_string());
        let bytes = MsgpackWriter.write_bytes(&val).unwrap();
        let cursor = std::io::Cursor::new(bytes);
        let result = MsgpackReader.read_from_reader(cursor).unwrap();
        assert_eq!(result, val);
    }

    #[test]
    fn test_write_to_writer() {
        let val = Value::Integer(42);
        let mut buf = Vec::new();
        MsgpackWriter.write_to_writer(&val, &mut buf).unwrap();
        let result = MsgpackReader.read_from_bytes(&buf).unwrap();
        assert_eq!(result, val);
    }

    // --- 에러 케이스 ---

    #[test]
    fn test_invalid_msgpack_data() {
        let invalid = vec![0xc1]; // MessagePack reserved byte
        let result = MsgpackReader.read_from_bytes(&invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_returns_error_for_string() {
        let val = Value::Null;
        let result = MsgpackWriter.write(&val);
        assert!(result.is_err());
    }
}
