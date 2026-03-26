use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// .env 포맷 Reader
///
/// `KEY=VALUE` 형식의 환경 변수 파일을 파싱하여 flat한 Object로 변환한다.
/// - `#` 주석 지원
/// - 빈 줄 무시
/// - 큰따옴표/작은따옴표 값 지원
/// - `export` 접두사 무시
pub struct EnvReader;

impl EnvReader {
    /// 한 줄을 파싱하여 (key, value) 쌍을 반환한다.
    /// 주석이나 빈 줄은 None을 반환한다.
    fn parse_line(line: &str) -> Option<(String, Value)> {
        let trimmed = line.trim();

        // 빈 줄이나 주석은 무시
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }

        // export 접두사 제거
        let trimmed = trimmed
            .strip_prefix("export ")
            .or_else(|| trimmed.strip_prefix("export\t"))
            .unwrap_or(trimmed);

        // KEY=VALUE 분리 (첫 번째 '='를 기준으로)
        let eq_pos = trimmed.find('=')?;
        let key = trimmed[..eq_pos].trim().to_string();
        if key.is_empty() {
            return None;
        }

        let raw_value = trimmed[eq_pos + 1..].trim();
        let value = Self::parse_value(raw_value);

        Some((key, Value::String(value)))
    }

    /// 값 문자열을 파싱한다.
    /// 큰따옴표/작은따옴표로 감싸진 경우 따옴표를 벗긴다.
    fn parse_value(raw: &str) -> String {
        if raw.len() >= 2
            && ((raw.starts_with('"') && raw.ends_with('"'))
                || (raw.starts_with('\'') && raw.ends_with('\'')))
        {
            let inner = &raw[1..raw.len() - 1];
            // 큰따옴표 내부의 이스케이프 시퀀스 처리
            if raw.starts_with('"') {
                return inner
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\");
            }
            // 작은따옴표는 이스케이프 처리 없이 그대로 반환
            return inner.to_string();
        }

        // 따옴표 없는 값: 인라인 주석 제거
        if let Some(comment_pos) = raw.find(" #") {
            raw[..comment_pos].trim_end().to_string()
        } else {
            raw.to_string()
        }
    }
}

impl FormatReader for EnvReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let mut map = IndexMap::new();

        for (line_num, line) in input.lines().enumerate() {
            match Self::parse_line(line) {
                Some((key, value)) => {
                    map.insert(key, value);
                }
                None => {
                    // 주석/빈 줄이 아닌데 '='가 없는 줄은 무시하되,
                    // 내용이 있는 줄인데 파싱 불가능한 경우 에러
                    let trimmed = line.trim();
                    if !trimmed.is_empty()
                        && !trimmed.starts_with('#')
                        && !trimmed.starts_with("export ")
                        && !trimmed.contains('=')
                    {
                        return Err(crate::error::DkitError::ParseErrorAt {
                            format: "ENV".to_string(),
                            source: "invalid line: expected KEY=VALUE format".to_string().into(),
                            line: line_num + 1,
                            column: 1,
                            line_text: line.to_string(),
                        }
                        .into());
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
                format: "ENV".to_string(),
                source: Box::new(e),
            })?;
        self.read(&input)
    }
}

/// .env 포맷 Writer
///
/// flat한 Object를 `KEY=VALUE` 형식으로 출력한다.
/// 중첩 구조는 지원하지 않으며, 값은 문자열로 변환된다.
pub struct EnvWriter;

impl EnvWriter {
    /// 값을 .env 포맷 문자열로 변환한다.
    /// 특수 문자가 포함된 경우 큰따옴표로 감싼다.
    fn format_value(value: &Value) -> String {
        match value {
            Value::String(s) => Self::quote_if_needed(s),
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Array(_) | Value::Object(_) => {
                // 중첩 구조는 JSON 문자열로 직렬화
                let json = serde_json::to_string(value).unwrap_or_default();
                format!("'{json}'")
            }
        }
    }

    /// 특수 문자가 포함된 경우 큰따옴표로 감싸고, 이스케이프 처리한다.
    fn quote_if_needed(s: &str) -> String {
        if s.is_empty() {
            return "\"\"".to_string();
        }

        let needs_quoting = s.contains(' ')
            || s.contains('#')
            || s.contains('"')
            || s.contains('\'')
            || s.contains('\n')
            || s.contains('\r')
            || s.contains('\t')
            || s.contains('\\');

        if needs_quoting {
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            format!("\"{escaped}\"")
        } else {
            s.to_string()
        }
    }
}

impl FormatWriter for EnvWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        match value {
            Value::Object(map) => {
                let mut lines: Vec<String> = Vec::with_capacity(map.len());
                for (key, val) in map {
                    let formatted = Self::format_value(val);
                    lines.push(format!("{key}={formatted}"));
                }
                let mut output = lines.join("\n");
                if !output.is_empty() {
                    output.push('\n');
                }
                Ok(output)
            }
            Value::Array(arr) => {
                // Array of Objects → 첫 번째 요소만 사용하거나 모든 키를 병합
                if arr.is_empty() {
                    return Ok(String::new());
                }
                // 각 요소가 Object인 경우 병합
                let mut map = IndexMap::new();
                for item in arr {
                    if let Value::Object(obj) = item {
                        for (k, v) in obj {
                            map.insert(k.clone(), v.clone());
                        }
                    } else {
                        anyhow::bail!(
                            "ENV format only supports flat Object data. \
                             Got array with non-object elements."
                        );
                    }
                }
                self.write(&Value::Object(map))
            }
            _ => {
                anyhow::bail!(
                    "ENV format only supports Object (key-value pairs). \
                     Got: {}",
                    match value {
                        Value::Null => "null",
                        Value::Bool(_) => "boolean",
                        Value::Integer(_) => "integer",
                        Value::Float(_) => "float",
                        Value::String(_) => "string",
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
                format: "ENV".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- EnvReader 테스트 ---

    #[test]
    fn test_reader_simple() {
        let reader = EnvReader;
        let input = "DB_HOST=localhost\nDB_PORT=5432\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("DB_HOST"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(obj.get("DB_PORT"), Some(&Value::String("5432".to_string())));
    }

    #[test]
    fn test_reader_comments() {
        let reader = EnvReader;
        let input = "# This is a comment\nKEY=value\n# Another comment\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.get("KEY"), Some(&Value::String("value".to_string())));
    }

    #[test]
    fn test_reader_empty_lines() {
        let reader = EnvReader;
        let input = "A=1\n\n\nB=2\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 2);
    }

    #[test]
    fn test_reader_double_quoted() {
        let reader = EnvReader;
        let input = "MSG=\"hello world\"\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("MSG"),
            Some(&Value::String("hello world".to_string()))
        );
    }

    #[test]
    fn test_reader_single_quoted() {
        let reader = EnvReader;
        let input = "MSG='hello world'\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("MSG"),
            Some(&Value::String("hello world".to_string()))
        );
    }

    #[test]
    fn test_reader_export_prefix() {
        let reader = EnvReader;
        let input = "export DB_HOST=localhost\nexport DB_PORT=5432\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("DB_HOST"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(obj.get("DB_PORT"), Some(&Value::String("5432".to_string())));
    }

    #[test]
    fn test_reader_escape_sequences() {
        let reader = EnvReader;
        let input = "MSG=\"line1\\nline2\"\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("MSG"),
            Some(&Value::String("line1\nline2".to_string()))
        );
    }

    #[test]
    fn test_reader_single_quote_no_escape() {
        let reader = EnvReader;
        let input = "MSG='line1\\nline2'\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("MSG"),
            Some(&Value::String("line1\\nline2".to_string()))
        );
    }

    #[test]
    fn test_reader_empty_value() {
        let reader = EnvReader;
        let input = "EMPTY=\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("EMPTY"),
            Some(&Value::String(String::new()))
        );
    }

    #[test]
    fn test_reader_value_with_equals() {
        let reader = EnvReader;
        let input = "URL=https://example.com?key=value\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("URL"),
            Some(&Value::String("https://example.com?key=value".to_string()))
        );
    }

    #[test]
    fn test_reader_inline_comment() {
        let reader = EnvReader;
        let input = "KEY=value # this is a comment\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("KEY"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_quoted_value_preserves_hash() {
        let reader = EnvReader;
        let input = "KEY=\"value # not a comment\"\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("KEY"),
            Some(&Value::String("value # not a comment".to_string()))
        );
    }

    #[test]
    fn test_reader_empty_input() {
        let reader = EnvReader;
        let v = reader.read("").unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_whitespace_around_key() {
        let reader = EnvReader;
        let input = "  KEY  = value\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("KEY"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = EnvReader;
        let input = b"KEY=value" as &[u8];
        let v = reader.read_from_reader(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("KEY"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_duplicate_keys_last_wins() {
        let reader = EnvReader;
        let input = "KEY=first\nKEY=second\n";
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("KEY"),
            Some(&Value::String("second".to_string()))
        );
    }

    // --- EnvWriter 테스트 ---

    #[test]
    fn test_writer_simple() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("KEY".to_string(), Value::String("value".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "KEY=value\n");
    }

    #[test]
    fn test_writer_multiple_keys() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("A".to_string(), Value::String("1".to_string()));
            m.insert("B".to_string(), Value::String("2".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("A=1\n"));
        assert!(output.contains("B=2\n"));
    }

    #[test]
    fn test_writer_quotes_spaces() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("MSG".to_string(), Value::String("hello world".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "MSG=\"hello world\"\n");
    }

    #[test]
    fn test_writer_empty_value() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("EMPTY".to_string(), Value::String(String::new()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "EMPTY=\"\"\n");
    }

    #[test]
    fn test_writer_null_value() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("VAL".to_string(), Value::Null);
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "VAL=\n");
    }

    #[test]
    fn test_writer_boolean() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("DEBUG".to_string(), Value::Bool(true));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "DEBUG=true\n");
    }

    #[test]
    fn test_writer_integer() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("PORT".to_string(), Value::Integer(5432));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "PORT=5432\n");
    }

    #[test]
    fn test_writer_escapes_newline() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("MSG".to_string(), Value::String("line1\nline2".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "MSG=\"line1\\nline2\"\n");
    }

    #[test]
    fn test_writer_non_object_error() {
        let writer = EnvWriter;
        let result = writer.write(&Value::String("hello".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("KEY".to_string(), Value::String("value".to_string()));
            m
        });
        let mut buf = Vec::new();
        writer.write_to_writer(&v, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "KEY=value\n");
    }

    // --- 왕복 변환 테스트 ---

    #[test]
    fn test_roundtrip() {
        let input = "DB_HOST=localhost\nDB_PORT=5432\nDEBUG=true\n";
        let reader = EnvReader;
        let writer = EnvWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_quoted() {
        let input = "MSG=\"hello world\"\nPATH=\"/usr/bin:/usr/local/bin\"\n";
        let reader = EnvReader;
        let writer = EnvWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_reader_escaped_double_quote() {
        let reader = EnvReader;
        let input = r#"MSG="say \"hello\"""#;
        let v = reader.read(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("MSG"),
            Some(&Value::String("say \"hello\"".to_string()))
        );
    }

    #[test]
    fn test_writer_escapes_double_quote() {
        let writer = EnvWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "MSG".to_string(),
                Value::String("say \"hello\"".to_string()),
            );
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "MSG=\"say \\\"hello\\\"\"\n");
    }
}
