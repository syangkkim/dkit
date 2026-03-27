use std::io::{Read, Write};

use indexmap::IndexMap;
use serde::Serialize;

use crate::format::{FormatOptions, FormatReader, FormatWriter};
use crate::value::Value;

/// Recursively sort all object keys alphabetically
fn sort_value_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted: IndexMap<String, Value> = IndexMap::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted.insert(key.clone(), sort_value_keys(&map[key]));
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_value_keys).collect()),
        other => other.clone(),
    }
}

/// serde_json::Value → 내부 Value 변환
pub fn from_json_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                // u64 that doesn't fit in i64
                Value::Float(n.as_f64().unwrap_or(f64::NAN))
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(from_json_value).collect())
        }
        serde_json::Value::Object(map) => {
            let obj: IndexMap<String, Value> = map
                .into_iter()
                .map(|(k, v)| (k, from_json_value(v)))
                .collect();
            Value::Object(obj)
        }
    }
}

/// 내부 Value → serde_json::Value 변환
pub fn to_json_value(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Integer(n) => serde_json::Value::Number((*n).into()),
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(to_json_value).collect()),
        Value::Object(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), to_json_value(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
    }
}

/// JSON 포맷 Reader
pub struct JsonReader;

impl FormatReader for JsonReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let json_val: serde_json::Value = serde_json::from_str(input).map_err(|e| {
            let line = e.line();
            let column = e.column();
            let line_text = input
                .lines()
                .nth(line.saturating_sub(1))
                .unwrap_or("")
                .to_string();
            crate::error::DkitError::ParseErrorAt {
                format: "JSON".to_string(),
                source: Box::new(e),
                line,
                column,
                line_text,
            }
        })?;
        Ok(from_json_value(json_val))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let json_val: serde_json::Value = serde_json::from_reader(&mut reader).map_err(|e| {
            crate::error::DkitError::ParseError {
                format: "JSON".to_string(),
                source: Box::new(e),
            }
        })?;
        Ok(from_json_value(json_val))
    }
}

/// JSON 포맷 Writer
#[derive(Default)]
pub struct JsonWriter {
    options: FormatOptions,
}

impl JsonWriter {
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }
}

impl JsonWriter {
    /// Serialize a serde_json::Value with custom indent string
    fn serialize_with_indent(json_val: &serde_json::Value, indent: &str) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        serde_json::Value::serialize(json_val, &mut ser).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "JSON".to_string(),
                source: Box::new(e),
            }
        })?;
        Ok(String::from_utf8(buf)?)
    }

    fn serialize_with_indent_to_writer(
        json_val: &serde_json::Value,
        indent: &str,
        writer: impl Write,
    ) -> anyhow::Result<()> {
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
        let mut ser = serde_json::Serializer::with_formatter(writer, formatter);
        serde_json::Value::serialize(json_val, &mut ser).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "JSON".to_string(),
                source: Box::new(e),
            }
        })?;
        Ok(())
    }

    /// Resolve the indent string from options
    fn resolve_indent(&self) -> Option<String> {
        self.options.indent.as_ref().map(|v| {
            if v.eq_ignore_ascii_case("tab") {
                "\t".to_string()
            } else if let Ok(n) = v.parse::<usize>() {
                " ".repeat(n)
            } else {
                // fallback: default 2 spaces
                "  ".to_string()
            }
        })
    }

    fn prepare_value(&self, value: &Value) -> serde_json::Value {
        let value = if self.options.sort_keys {
            sort_value_keys(value)
        } else {
            value.clone()
        };
        to_json_value(&value)
    }
}

impl FormatWriter for JsonWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let json_val = self.prepare_value(value);
        let output = if self.options.compact {
            serde_json::to_string(&json_val).map_err(|e| crate::error::DkitError::WriteError {
                format: "JSON".to_string(),
                source: Box::new(e),
            })?
        } else if let Some(indent) = self.resolve_indent() {
            Self::serialize_with_indent(&json_val, &indent)?
        } else if self.options.pretty {
            serde_json::to_string_pretty(&json_val).map_err(|e| {
                crate::error::DkitError::WriteError {
                    format: "JSON".to_string(),
                    source: Box::new(e),
                }
            })?
        } else {
            serde_json::to_string(&json_val).map_err(|e| crate::error::DkitError::WriteError {
                format: "JSON".to_string(),
                source: Box::new(e),
            })?
        };
        Ok(output)
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let json_val = self.prepare_value(value);
        if self.options.compact {
            serde_json::to_writer(&mut writer, &json_val)
        } else if let Some(indent) = self.resolve_indent() {
            return Self::serialize_with_indent_to_writer(&json_val, &indent, &mut writer);
        } else if self.options.pretty {
            serde_json::to_writer_pretty(&mut writer, &json_val)
        } else {
            serde_json::to_writer(&mut writer, &json_val)
        }
        .map_err(|e| crate::error::DkitError::WriteError {
            format: "JSON".to_string(),
            source: Box::new(e),
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- from_json_value 변환 테스트 ---

    #[test]
    fn test_convert_null() {
        let v = from_json_value(serde_json::Value::Null);
        assert_eq!(v, Value::Null);
    }

    #[test]
    fn test_convert_bool() {
        assert_eq!(
            from_json_value(serde_json::Value::Bool(true)),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_convert_integer() {
        let v = from_json_value(serde_json::json!(42));
        assert_eq!(v, Value::Integer(42));
    }

    #[test]
    fn test_convert_float() {
        let v = from_json_value(serde_json::json!(3.14));
        assert_eq!(v, Value::Float(3.14));
    }

    #[test]
    fn test_convert_string() {
        let v = from_json_value(serde_json::json!("hello"));
        assert_eq!(v, Value::String("hello".to_string()));
    }

    #[test]
    fn test_convert_array() {
        let v = from_json_value(serde_json::json!([1, "two", null]));
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::Integer(1));
        assert_eq!(arr[1], Value::String("two".to_string()));
        assert_eq!(arr[2], Value::Null);
    }

    #[test]
    fn test_convert_object() {
        let v = from_json_value(serde_json::json!({"name": "dkit", "version": 1}));
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("version"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_convert_nested() {
        let v = from_json_value(serde_json::json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ]
        }));
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
    }

    // --- to_json_value 왕복 변환 테스트 ---

    #[test]
    fn test_roundtrip_primitives() {
        let values = vec![
            Value::Null,
            Value::Bool(false),
            Value::Integer(100),
            Value::Float(2.718),
            Value::String("test".to_string()),
        ];
        for v in values {
            let json = to_json_value(&v);
            let back = from_json_value(json);
            assert_eq!(back, v);
        }
    }

    #[test]
    fn test_roundtrip_complex() {
        let mut map = IndexMap::new();
        map.insert(
            "key".to_string(),
            Value::Array(vec![Value::Integer(1), Value::Null]),
        );
        let original = Value::Object(map);
        let json = to_json_value(&original);
        let back = from_json_value(json);
        assert_eq!(back, original);
    }

    // --- JsonReader 테스트 ---

    #[test]
    fn test_reader_simple_object() {
        let reader = JsonReader;
        let v = reader.read(r#"{"name": "dkit", "count": 42}"#).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("count"), Some(&Value::Integer(42)));
    }

    #[test]
    fn test_reader_array() {
        let reader = JsonReader;
        let v = reader.read("[1, 2, 3]").unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_reader_invalid_json() {
        let reader = JsonReader;
        let result = reader.read("{invalid}");
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_empty_object() {
        let reader = JsonReader;
        let v = reader.read("{}").unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_empty_array() {
        let reader = JsonReader;
        let v = reader.read("[]").unwrap();
        assert!(v.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = JsonReader;
        let input = r#"{"x": 1}"#.as_bytes();
        let v = reader.read_from_reader(input).unwrap();
        assert_eq!(v.as_object().unwrap().get("x"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_reader_from_reader_invalid() {
        let reader = JsonReader;
        let input = b"not json" as &[u8];
        assert!(reader.read_from_reader(input).is_err());
    }

    // --- JsonWriter 테스트 ---

    #[test]
    fn test_writer_pretty() {
        let writer = JsonWriter::default(); // pretty: true by default
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains('\n'));
        assert!(output.contains("  ")); // indentation
    }

    #[test]
    fn test_writer_compact() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            pretty: false,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, r#"{"a":1}"#);
    }

    #[test]
    fn test_writer_null() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            ..Default::default()
        });
        assert_eq!(writer.write(&Value::Null).unwrap(), "null");
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            ..Default::default()
        });
        let mut buf = Vec::new();
        writer
            .write_to_writer(&Value::Integer(42), &mut buf)
            .unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "42");
    }

    #[test]
    fn test_writer_nan_becomes_null() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            ..Default::default()
        });
        let output = writer.write(&Value::Float(f64::NAN)).unwrap();
        assert_eq!(output, "null");
    }

    // --- 대규모 데이터 테스트 ---

    #[test]
    fn test_large_array() {
        let reader = JsonReader;
        let arr: Vec<String> = (0..1000).map(|i| format!("{i}")).collect();
        let json = serde_json::to_string(&arr).unwrap();
        let v = reader.read(&json).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 1000);
    }

    // --- 특수 케이스 ---

    #[test]
    fn test_unicode_string() {
        let reader = JsonReader;
        let v = reader.read(r#"{"emoji": "🎉", "korean": "한글"}"#).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("emoji"), Some(&Value::String("🎉".to_string())));
        assert_eq!(obj.get("korean"), Some(&Value::String("한글".to_string())));
    }

    #[test]
    fn test_negative_numbers() {
        let reader = JsonReader;
        let v = reader
            .read(r#"{"neg_int": -42, "neg_float": -3.14}"#)
            .unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("neg_int"), Some(&Value::Integer(-42)));
        assert_eq!(obj.get("neg_float"), Some(&Value::Float(-3.14)));
    }

    #[test]
    fn test_deeply_nested() {
        let reader = JsonReader;
        let v = reader.read(r#"{"a": {"b": {"c": {"d": 1}}}}"#).unwrap();
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

    // --- --indent, --sort-keys, --compact 테스트 ---

    #[test]
    fn test_writer_indent_2_spaces() {
        let writer = JsonWriter::new(FormatOptions {
            indent: Some("2".to_string()),
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("\n  \"a\""));
        assert!(!output.contains("    \"a\"")); // not 4 spaces
    }

    #[test]
    fn test_writer_indent_4_spaces() {
        let writer = JsonWriter::new(FormatOptions {
            indent: Some("4".to_string()),
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("x".to_string(), Value::Integer(42));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("\n    \"x\""));
    }

    #[test]
    fn test_writer_indent_tab() {
        let writer = JsonWriter::new(FormatOptions {
            indent: Some("tab".to_string()),
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("\n\t\"a\""));
    }

    #[test]
    fn test_writer_sort_keys() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            sort_keys: true,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("zebra".to_string(), Value::Integer(1));
            m.insert("apple".to_string(), Value::Integer(2));
            m.insert("mango".to_string(), Value::Integer(3));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, r#"{"apple":2,"mango":3,"zebra":1}"#);
    }

    #[test]
    fn test_writer_sort_keys_nested() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            sort_keys: true,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "z".to_string(),
                Value::Object({
                    let mut inner = IndexMap::new();
                    inner.insert("b".to_string(), Value::Integer(2));
                    inner.insert("a".to_string(), Value::Integer(1));
                    inner
                }),
            );
            m.insert("a".to_string(), Value::Integer(0));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, r#"{"a":0,"z":{"a":1,"b":2}}"#);
    }

    #[test]
    fn test_writer_indent_and_sort_keys() {
        let writer = JsonWriter::new(FormatOptions {
            indent: Some("2".to_string()),
            sort_keys: true,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("c".to_string(), Value::Integer(3));
            m.insert("a".to_string(), Value::Integer(1));
            m.insert("b".to_string(), Value::Integer(2));
            m
        });
        let output = writer.write(&v).unwrap();
        // Keys should be sorted and indent should be 2 spaces
        assert!(output.contains("\"a\": 1"));
        let keys_pos_a = output.find("\"a\"").unwrap();
        let keys_pos_b = output.find("\"b\"").unwrap();
        let keys_pos_c = output.find("\"c\"").unwrap();
        assert!(keys_pos_a < keys_pos_b);
        assert!(keys_pos_b < keys_pos_c);
    }

    #[test]
    fn test_writer_compact_overrides_indent() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            indent: Some("4".to_string()),
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, r#"{"a":1}"#);
    }

    #[test]
    fn test_writer_to_writer_with_indent() {
        let writer = JsonWriter::new(FormatOptions {
            indent: Some("2".to_string()),
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("key".to_string(), Value::String("val".to_string()));
            m
        });
        let mut buf = Vec::new();
        writer.write_to_writer(&v, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("\n  \"key\""));
    }

    #[test]
    fn test_writer_to_writer_with_sort_keys() {
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            sort_keys: true,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("z".to_string(), Value::Integer(1));
            m.insert("a".to_string(), Value::Integer(2));
            m
        });
        let mut buf = Vec::new();
        writer.write_to_writer(&v, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, r#"{"a":2,"z":1}"#);
    }
}
