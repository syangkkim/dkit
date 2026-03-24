use std::io::{BufRead, Read, Write};

use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

use super::json::{from_json_value, to_json_value};

/// JSONL (JSON Lines) 포맷 Reader
///
/// 한 줄에 하나의 JSON 객체를 읽어 배열(Value::Array)로 변환한다.
/// 빈 줄은 무시하고, 파싱 실패 시 줄 번호를 포함한 에러를 반환한다.
pub struct JsonlReader;

impl JsonlReader {
    fn parse_lines(&self, input: &str) -> anyhow::Result<Value> {
        let mut items = Vec::new();
        for (line_num, line) in input.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let json_val: serde_json::Value =
                serde_json::from_str(trimmed).map_err(|e| crate::error::DkitError::ParseError {
                    format: "JSONL".to_string(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("line {}: {e}", line_num + 1),
                    )),
                })?;
            items.push(from_json_value(json_val));
        }
        Ok(Value::Array(items))
    }
}

impl FormatReader for JsonlReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        self.parse_lines(input)
    }

    fn read_from_reader(&self, reader: impl Read) -> anyhow::Result<Value> {
        let buf_reader = std::io::BufReader::new(reader);
        let mut items = Vec::new();
        for (line_num, line_result) in buf_reader.lines().enumerate() {
            let line = line_result.map_err(|e| crate::error::DkitError::ParseError {
                format: "JSONL".to_string(),
                source: Box::new(e),
            })?;
            let trimmed = line.trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let json_val: serde_json::Value = serde_json::from_str(&trimmed).map_err(|e| {
                crate::error::DkitError::ParseError {
                    format: "JSONL".to_string(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("line {}: {e}", line_num + 1),
                    )),
                }
            })?;
            items.push(from_json_value(json_val));
        }
        Ok(Value::Array(items))
    }
}

/// JSONL (JSON Lines) 포맷 Writer
///
/// Value::Array의 각 원소를 한 줄씩 JSON으로 직렬화한다.
/// Array가 아닌 값은 단일 줄로 출력한다.
pub struct JsonlWriter;

impl FormatWriter for JsonlWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let mut output = String::new();
        match value {
            Value::Array(items) => {
                for item in items {
                    let json_val = to_json_value(item);
                    let line = serde_json::to_string(&json_val).map_err(|e| {
                        crate::error::DkitError::WriteError {
                            format: "JSONL".to_string(),
                            source: Box::new(e),
                        }
                    })?;
                    output.push_str(&line);
                    output.push('\n');
                }
            }
            other => {
                let json_val = to_json_value(other);
                let line = serde_json::to_string(&json_val).map_err(|e| {
                    crate::error::DkitError::WriteError {
                        format: "JSONL".to_string(),
                        source: Box::new(e),
                    }
                })?;
                output.push_str(&line);
                output.push('\n');
            }
        }
        Ok(output)
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        match value {
            Value::Array(items) => {
                for item in items {
                    let json_val = to_json_value(item);
                    serde_json::to_writer(&mut writer, &json_val).map_err(|e| {
                        crate::error::DkitError::WriteError {
                            format: "JSONL".to_string(),
                            source: Box::new(e),
                        }
                    })?;
                    writer
                        .write_all(b"\n")
                        .map_err(|e| crate::error::DkitError::WriteError {
                            format: "JSONL".to_string(),
                            source: Box::new(e),
                        })?;
                }
            }
            other => {
                let json_val = to_json_value(other);
                serde_json::to_writer(&mut writer, &json_val).map_err(|e| {
                    crate::error::DkitError::WriteError {
                        format: "JSONL".to_string(),
                        source: Box::new(e),
                    }
                })?;
                writer
                    .write_all(b"\n")
                    .map_err(|e| crate::error::DkitError::WriteError {
                        format: "JSONL".to_string(),
                        source: Box::new(e),
                    })?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    // --- JsonlReader 테스트 ---

    #[test]
    fn test_read_basic() {
        let reader = JsonlReader;
        let input = r#"{"name":"Alice","age":30}
{"name":"Bob","age":25}"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("age"),
            Some(&Value::Integer(25))
        );
    }

    #[test]
    fn test_read_skip_empty_lines() {
        let reader = JsonlReader;
        let input = r#"{"a":1}

{"b":2}

"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_read_single_line() {
        let reader = JsonlReader;
        let input = r#"{"key":"value"}"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn test_read_empty_input() {
        let reader = JsonlReader;
        let result = reader.read("").unwrap();
        let arr = result.as_array().unwrap();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_read_only_empty_lines() {
        let reader = JsonlReader;
        let result = reader.read("\n\n\n").unwrap();
        let arr = result.as_array().unwrap();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_read_various_json_types() {
        let reader = JsonlReader;
        let input = "42\n\"hello\"\ntrue\nnull\n[1,2,3]";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], Value::Integer(42));
        assert_eq!(arr[1], Value::String("hello".to_string()));
        assert_eq!(arr[2], Value::Bool(true));
        assert_eq!(arr[3], Value::Null);
        assert_eq!(arr[4].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_read_malformed_line_error_with_line_number() {
        let reader = JsonlReader;
        let input = r#"{"a":1}
{invalid json}
{"b":2}"#;
        let err = reader.read(input).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("JSONL"));
        assert!(msg.contains("line 2"));
    }

    #[test]
    fn test_read_from_reader() {
        let reader = JsonlReader;
        let input = b"{\"x\":1}\n{\"x\":2}\n";
        let result = reader.read_from_reader(&input[..]).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_read_whitespace_trimmed() {
        let reader = JsonlReader;
        let input = "  {\"a\":1}  \n  {\"b\":2}  ";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_read_unicode() {
        let reader = JsonlReader;
        let input = r#"{"emoji":"🎉","korean":"한글"}"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        let obj = arr[0].as_object().unwrap();
        assert_eq!(obj.get("emoji"), Some(&Value::String("🎉".to_string())));
        assert_eq!(obj.get("korean"), Some(&Value::String("한글".to_string())));
    }

    // --- JsonlWriter 테스트 ---

    #[test]
    fn test_write_array() {
        let writer = JsonlWriter;
        let value = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert("age".to_string(), Value::Integer(30));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert("age".to_string(), Value::Integer(25));
                m
            }),
        ]);
        let output = writer.write(&value).unwrap();
        let lines: Vec<&str> = output.trim_end().split('\n').collect();
        assert_eq!(lines.len(), 2);
        // Each line should be valid JSON containing the expected fields
        let parsed0: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(parsed0["name"], "Alice");
        assert_eq!(parsed0["age"], 30);
        let parsed1: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
        assert_eq!(parsed1["name"], "Bob");
        assert_eq!(parsed1["age"], 25);
    }

    #[test]
    fn test_write_empty_array() {
        let writer = JsonlWriter;
        let output = writer.write(&Value::Array(vec![])).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn test_write_non_array() {
        let writer = JsonlWriter;
        let output = writer.write(&Value::Integer(42)).unwrap();
        assert_eq!(output, "42\n");
    }

    #[test]
    fn test_write_to_writer() {
        let writer = JsonlWriter;
        let value = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let mut buf = Vec::new();
        writer.write_to_writer(&value, &mut buf).unwrap();
        assert_eq!(String::from_utf8(buf).unwrap(), "1\n2\n");
    }

    // --- 라운드트립 테스트 ---

    #[test]
    fn test_roundtrip() {
        let original = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("id".to_string(), Value::Integer(1));
                m.insert("name".to_string(), Value::String("test".to_string()));
                m.insert("active".to_string(), Value::Bool(true));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("id".to_string(), Value::Integer(2));
                m.insert("name".to_string(), Value::String("other".to_string()));
                m.insert("active".to_string(), Value::Bool(false));
                m
            }),
        ]);

        let writer = JsonlWriter;
        let written = writer.write(&original).unwrap();

        let reader = JsonlReader;
        let parsed = reader.read(&written).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_roundtrip_nested() {
        let original = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "data".to_string(),
                Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
            );
            m.insert(
                "nested".to_string(),
                Value::Object({
                    let mut inner = IndexMap::new();
                    inner.insert("key".to_string(), Value::String("val".to_string()));
                    inner
                }),
            );
            m
        })]);

        let writer = JsonlWriter;
        let written = writer.write(&original).unwrap();
        let reader = JsonlReader;
        let parsed = reader.read(&written).unwrap();
        assert_eq!(original, parsed);
    }

    // --- 대용량 테스트 ---

    #[test]
    fn test_large_input() {
        let lines: Vec<String> = (0..1000)
            .map(|i| format!(r#"{{"id":{i},"value":"item_{i}"}}"#))
            .collect();
        let input = lines.join("\n");

        let reader = JsonlReader;
        let result = reader.read(&input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1000);
        assert_eq!(
            arr[999].as_object().unwrap().get("id"),
            Some(&Value::Integer(999))
        );
    }
}
