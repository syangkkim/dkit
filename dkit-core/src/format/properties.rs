use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// Java `.properties` 포맷 Reader
///
/// `key=value`, `key: value`, `key value` 형식의 Java properties 파일을 파싱하여
/// flat Object(`{ "key": "value" }`)로 변환한다.
///
/// - `#` 또는 `!` 주석 지원
/// - 빈 줄 무시
/// - `\` 줄 연속 (multiline value)
/// - Unicode 이스케이프: `\uXXXX`
/// - 키의 `.` 구분자는 유지 (flat 모델)
pub struct PropertiesReader;

impl PropertiesReader {
    /// 이스케이프 시퀀스를 처리한다.
    fn unescape(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('=') => result.push('='),
                    Some(':') => result.push(':'),
                    Some(' ') => result.push(' '),
                    Some('#') => result.push('#'),
                    Some('!') => result.push('!'),
                    Some('u') => {
                        // Unicode escape: \uXXXX
                        let hex: String = chars.by_ref().take(4).collect();
                        if hex.len() == 4 {
                            if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(code) {
                                    result.push(ch);
                                    continue;
                                }
                            }
                        }
                        // 유효하지 않은 unicode escape는 원본 유지
                        result.push_str("\\u");
                        result.push_str(&hex);
                    }
                    Some(other) => {
                        // 알 수 없는 이스케이프는 문자만 유지
                        result.push(other);
                    }
                    None => {
                        // 문자열 끝의 \는 무시
                    }
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    /// 키-값 구분자 위치를 찾는다 (이스케이프되지 않은 `=`, `:`, 또는 공백).
    /// Java properties 사양에 따라: 먼저 이스케이프되지 않은 `=` 또는 `:`를 찾고,
    /// 없으면 첫 번째 공백을 구분자로 사용한다.
    fn find_separator(line: &str) -> Option<(usize, usize)> {
        let bytes = line.as_bytes();

        // 1단계: 이스케이프되지 않은 `=` 또는 `:` 찾기
        let mut escaped = false;
        for (i, &b) in bytes.iter().enumerate() {
            if escaped {
                escaped = false;
                continue;
            }
            if b == b'\\' {
                escaped = true;
                continue;
            }
            if b == b'=' || b == b':' {
                // 구분자 뒤의 선행 공백 건너뛰기
                let value_start = line[i + 1..]
                    .find(|c: char| c != ' ' && c != '\t')
                    .map_or(line.len(), |pos| i + 1 + pos);
                return Some((i, value_start));
            }
        }

        // 2단계: 이스케이프되지 않은 첫 공백 찾기
        escaped = false;
        for (i, &b) in bytes.iter().enumerate() {
            if escaped {
                escaped = false;
                continue;
            }
            if b == b'\\' {
                escaped = true;
                continue;
            }
            if b == b' ' || b == b'\t' {
                let value_start = line[i..]
                    .find(|c: char| c != ' ' && c != '\t')
                    .map_or(line.len(), |pos| i + pos);
                return Some((i, value_start));
            }
        }

        None
    }

    /// 논리적 줄을 결합한다 (줄 연속 `\` 처리).
    fn join_logical_lines(input: &str) -> Vec<String> {
        let mut logical_lines = Vec::new();
        let mut current = String::new();
        let mut continuation = false;

        for line in input.lines() {
            if continuation {
                // 연속 줄: 선행 공백 제거 후 이어붙임
                current.push_str(line.trim_start());
            } else {
                if !current.is_empty() {
                    logical_lines.push(std::mem::take(&mut current));
                }
                current = line.to_string();
            }

            // 줄 끝에 홀수 개의 `\`가 있으면 줄 연속
            let trailing_backslashes = current.bytes().rev().take_while(|&b| b == b'\\').count();
            if trailing_backslashes % 2 == 1 {
                // 마지막 `\` 제거
                current.truncate(current.len() - 1);
                continuation = true;
            } else {
                continuation = false;
            }
        }

        if !current.is_empty() {
            logical_lines.push(current);
        }

        logical_lines
    }
}

impl FormatReader for PropertiesReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let mut map = IndexMap::new();
        let logical_lines = Self::join_logical_lines(input);

        for (line_num, line) in logical_lines.iter().enumerate() {
            let trimmed = line.trim_start();

            // 빈 줄 무시
            if trimmed.is_empty() {
                continue;
            }

            // 주석 무시 (`#` 또는 `!`)
            if trimmed.starts_with('#') || trimmed.starts_with('!') {
                continue;
            }

            // 키-값 분리
            match Self::find_separator(trimmed) {
                Some((key_end, value_start)) => {
                    let raw_key = &trimmed[..key_end];
                    let raw_value = if value_start <= trimmed.len() {
                        &trimmed[value_start..]
                    } else {
                        ""
                    };
                    let key = Self::unescape(raw_key.trim_end());
                    let value = Self::unescape(raw_value);

                    if key.is_empty() {
                        return Err(crate::error::DkitError::ParseErrorAt {
                            format: "Properties".to_string(),
                            source: "empty key".to_string().into(),
                            line: line_num + 1,
                            column: 1,
                            line_text: line.to_string(),
                        }
                        .into());
                    }

                    map.insert(key, Value::String(value));
                }
                None => {
                    // 구분자 없음 = 빈 값의 키
                    let key = Self::unescape(trimmed);
                    if !key.is_empty() {
                        map.insert(key, Value::String(String::new()));
                    }
                }
            }
        }

        Ok(Value::Object(map))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| crate::error::DkitError::ParseError {
                format: "Properties".to_string(),
                source: Box::new(e),
            })?;
        self.read(&input)
    }
}

/// Java `.properties` 포맷 Writer
///
/// flat Object를 `key=value` 형식의 Java properties 파일로 출력한다.
pub struct PropertiesWriter;

impl PropertiesWriter {
    /// 키를 이스케이프한다.
    fn escape_key(key: &str) -> String {
        let mut result = String::with_capacity(key.len());
        for (i, c) in key.chars().enumerate() {
            match c {
                ' ' => {
                    if i == 0 {
                        result.push_str("\\ ");
                    } else {
                        result.push(' ');
                    }
                }
                '=' => result.push_str("\\="),
                ':' => result.push_str("\\:"),
                '\\' => result.push_str("\\\\"),
                '#' => result.push_str("\\#"),
                '!' => result.push_str("\\!"),
                '\t' => result.push_str("\\t"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                c if !(' '..='~').contains(&c) => {
                    // Non-ASCII → \uXXXX
                    for unit in c.encode_utf16(&mut [0u16; 2]) {
                        result.push_str(&format!("\\u{:04X}", unit));
                    }
                }
                _ => result.push(c),
            }
        }
        result
    }

    /// 값을 이스케이프한다.
    fn escape_value(value: &str) -> String {
        let mut result = String::with_capacity(value.len());
        for (i, c) in value.chars().enumerate() {
            match c {
                ' ' if i == 0 => result.push_str("\\ "),
                '\\' => result.push_str("\\\\"),
                '\t' => result.push_str("\\t"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                c if !(' '..='~').contains(&c) => {
                    for unit in c.encode_utf16(&mut [0u16; 2]) {
                        result.push_str(&format!("\\u{:04X}", unit));
                    }
                }
                _ => result.push(c),
            }
        }
        result
    }

    /// Value를 문자열로 변환한다.
    fn value_to_string(value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Array(_) | Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
        }
    }
}

impl FormatWriter for PropertiesWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        match value {
            Value::Object(map) => {
                let mut output = String::new();
                for (key, val) in map {
                    let escaped_key = Self::escape_key(key);
                    let raw_value = Self::value_to_string(val);
                    let escaped_value = Self::escape_value(&raw_value);
                    output.push_str(&format!("{}={}\n", escaped_key, escaped_value));
                }
                Ok(output)
            }
            _ => {
                anyhow::bail!(
                    "Properties format requires an Object value (flat key-value pairs). Got: {}",
                    match value {
                        Value::Null => "null",
                        Value::Bool(_) => "boolean",
                        Value::Integer(_) => "integer",
                        Value::Float(_) => "float",
                        Value::String(_) => "string",
                        Value::Array(_) => "array",
                        _ => "unknown",
                    }
                );
            }
        }
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let output = self.write(value)?;
        writer
            .write_all(output.as_bytes())
            .map_err(|e| crate::error::DkitError::WriteError {
                format: "Properties".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- PropertiesReader 테스트 ---

    #[test]
    fn test_reader_simple_key_value() {
        let reader = PropertiesReader;
        let input = "name=Alice\nage=30\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::String("30".to_string())));
    }

    #[test]
    fn test_reader_colon_separator() {
        let reader = PropertiesReader;
        let input = "name: Alice\nage: 30\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::String("30".to_string())));
    }

    #[test]
    fn test_reader_space_separator() {
        let reader = PropertiesReader;
        let input = "name Alice\nage 30\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::String("30".to_string())));
    }

    #[test]
    fn test_reader_comments_hash() {
        let reader = PropertiesReader;
        let input = "# This is a comment\nkey=value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.get("key"), Some(&Value::String("value".to_string())));
    }

    #[test]
    fn test_reader_comments_exclamation() {
        let reader = PropertiesReader;
        let input = "! This is a comment\nkey=value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_reader_empty_lines() {
        let reader = PropertiesReader;
        let input = "key1=val1\n\n\nkey2=val2\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_reader_empty_value() {
        let reader = PropertiesReader;
        let input = "key=\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("key"), Some(&Value::String(String::new())));
    }

    #[test]
    fn test_reader_key_without_separator() {
        let reader = PropertiesReader;
        // key 뒤에 구분자가 없으면 빈 값
        let input = "lonely_key\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("lonely_key"), Some(&Value::String(String::new())));
    }

    #[test]
    fn test_reader_multiline_value() {
        let reader = PropertiesReader;
        let input = "message=Hello \\\n    World\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("message"),
            Some(&Value::String("Hello World".to_string()))
        );
    }

    #[test]
    fn test_reader_multiline_multiple_continuations() {
        let reader = PropertiesReader;
        let input = "long=line1 \\\nline2 \\\nline3\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("long"),
            Some(&Value::String("line1 line2 line3".to_string()))
        );
    }

    #[test]
    fn test_reader_unicode_escape() {
        let reader = PropertiesReader;
        let input = "greeting=\\u0048\\u0065\\u006C\\u006C\\u006F\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("greeting"),
            Some(&Value::String("Hello".to_string()))
        );
    }

    #[test]
    fn test_reader_escaped_separator_in_key() {
        let reader = PropertiesReader;
        let input = "key\\=with\\=equals=value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("key=with=equals"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_escaped_special_chars() {
        let reader = PropertiesReader;
        let input = "path=C\\:\\\\Users\\\\test\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("path"),
            Some(&Value::String("C:\\Users\\test".to_string()))
        );
    }

    #[test]
    fn test_reader_dotted_keys() {
        let reader = PropertiesReader;
        let input = "app.db.host=localhost\napp.db.port=5432\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("app.db.host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(
            obj.get("app.db.port"),
            Some(&Value::String("5432".to_string()))
        );
    }

    #[test]
    fn test_reader_whitespace_around_separator() {
        let reader = PropertiesReader;
        let input = "key = value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("key"), Some(&Value::String("value".to_string())));
    }

    #[test]
    fn test_reader_duplicate_keys_last_wins() {
        let reader = PropertiesReader;
        let input = "key=first\nkey=second\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("key"), Some(&Value::String("second".to_string())));
    }

    #[test]
    fn test_reader_empty_input() {
        let reader = PropertiesReader;
        let v = reader.read("").unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_only_comments() {
        let reader = PropertiesReader;
        let input = "# comment 1\n! comment 2\n";
        let v = reader.read(input).unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = PropertiesReader;
        let input = b"key=value" as &[u8];
        let v = reader.read_from_reader(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("key"), Some(&Value::String("value".to_string())));
    }

    #[test]
    fn test_reader_newline_escape_in_value() {
        let reader = PropertiesReader;
        let input = "msg=line1\\nline2\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("msg"),
            Some(&Value::String("line1\nline2".to_string()))
        );
    }

    #[test]
    fn test_reader_tab_escape_in_value() {
        let reader = PropertiesReader;
        let input = "data=col1\\tcol2\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("data"),
            Some(&Value::String("col1\tcol2".to_string()))
        );
    }

    #[test]
    fn test_reader_realistic_i18n() {
        let reader = PropertiesReader;
        let input = r#"# Application messages
app.title=My Application
app.greeting=Welcome, {0}!
app.error.notfound=Page not found
app.error.server=Internal server error
"#;
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 4);
        assert_eq!(
            obj.get("app.title"),
            Some(&Value::String("My Application".to_string()))
        );
        assert_eq!(
            obj.get("app.greeting"),
            Some(&Value::String("Welcome, {0}!".to_string()))
        );
    }

    // --- PropertiesWriter 테스트 ---

    #[test]
    fn test_writer_simple() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m.insert("age".to_string(), Value::String("30".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("name=Alice\n"));
        assert!(output.contains("age=30\n"));
    }

    #[test]
    fn test_writer_special_chars_in_key() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("key=with".to_string(), Value::String("value".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("key\\=with=value\n"));
    }

    #[test]
    fn test_writer_non_ascii() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "greeting".to_string(),
                Value::String("こんにちは".to_string()),
            );
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("greeting="));
        assert!(output.contains("\\u"));
    }

    #[test]
    fn test_writer_newline_in_value() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("msg".to_string(), Value::String("line1\nline2".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("msg=line1\\nline2\n"));
    }

    #[test]
    fn test_writer_non_string_values() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("count".to_string(), Value::Integer(42));
            m.insert("rate".to_string(), Value::Float(3.14));
            m.insert("enabled".to_string(), Value::Bool(true));
            m.insert("empty".to_string(), Value::Null);
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("count=42\n"));
        assert!(output.contains("rate=3.14\n"));
        assert!(output.contains("enabled=true\n"));
        assert!(output.contains("empty=\n"));
    }

    #[test]
    fn test_writer_non_object_error() {
        let writer = PropertiesWriter;
        let result = writer.write(&Value::String("hello".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("key".to_string(), Value::String("value".to_string()));
            m
        });
        let mut buf = Vec::new();
        writer.write_to_writer(&v, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "key=value\n");
    }

    #[test]
    fn test_writer_leading_space_in_value() {
        let writer = PropertiesWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("key".to_string(), Value::String(" leading".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("key=\\ leading\n"));
    }

    // --- 왕복 변환 테스트 ---

    #[test]
    fn test_roundtrip_simple() {
        let input = "name=Alice\nage=30\n";
        let reader = PropertiesReader;
        let writer = PropertiesWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_dotted_keys() {
        let input = "app.db.host=localhost\napp.db.port=5432\n";
        let reader = PropertiesReader;
        let writer = PropertiesWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_special_chars() {
        let reader = PropertiesReader;
        let writer = PropertiesWriter;

        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("key=eq".to_string(), Value::String("val:colon".to_string()));
            m.insert("path".to_string(), Value::String("C:\\Users".to_string()));
            m
        });

        let output = writer.write(&v).unwrap();
        let value2 = reader.read(&output).unwrap();
        assert_eq!(v, value2);
    }

    #[test]
    fn test_roundtrip_unicode() {
        let reader = PropertiesReader;
        let writer = PropertiesWriter;

        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("msg".to_string(), Value::String("Hello 세계".to_string()));
            m
        });

        let output = writer.write(&v).unwrap();
        let value2 = reader.read(&output).unwrap();
        assert_eq!(v, value2);
    }

    #[test]
    fn test_roundtrip_newlines() {
        let reader = PropertiesReader;
        let writer = PropertiesWriter;

        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "multiline".to_string(),
                Value::String("line1\nline2\nline3".to_string()),
            );
            m
        });

        let output = writer.write(&v).unwrap();
        let value2 = reader.read(&output).unwrap();
        assert_eq!(v, value2);
    }
}
