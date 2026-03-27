use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// INI/CFG 포맷 Reader
///
/// `[section]` 헤더와 `key = value` 형식의 설정 파일을 파싱하여
/// 2-depth Object(`{ section: { key: value } }`)로 변환한다.
///
/// - `#` 또는 `;` 주석 지원
/// - 빈 줄 무시
/// - 구분자: `=` 또는 `:`
/// - 섹션 없는 키는 최상위 오브젝트에 배치
/// - `.cfg` 확장자도 동일 포맷으로 처리
pub struct IniReader;

impl IniReader {
    /// 값 문자열을 적절한 Value 타입으로 변환한다.
    fn parse_value(raw: &str) -> Value {
        let trimmed = raw.trim();

        // 빈 값
        if trimmed.is_empty() {
            return Value::String(String::new());
        }

        // 따옴표로 감싸진 문자열
        if trimmed.len() >= 2
            && ((trimmed.starts_with('"') && trimmed.ends_with('"'))
                || (trimmed.starts_with('\'') && trimmed.ends_with('\'')))
        {
            return Value::String(trimmed[1..trimmed.len() - 1].to_string());
        }

        // Boolean
        match trimmed.to_lowercase().as_str() {
            "true" | "yes" | "on" => return Value::Bool(true),
            "false" | "no" | "off" => return Value::Bool(false),
            _ => {}
        }

        // Integer
        if let Ok(n) = trimmed.parse::<i64>() {
            return Value::Integer(n);
        }

        // Float
        if let Ok(f) = trimmed.parse::<f64>() {
            return Value::Float(f);
        }

        Value::String(trimmed.to_string())
    }

    /// 인라인 주석을 제거한다.
    /// 따옴표 내부의 `;`이나 `#`은 주석으로 처리하지 않는다.
    fn strip_inline_comment(value_str: &str) -> &str {
        let mut in_single_quote = false;
        let mut in_double_quote = false;

        for (i, c) in value_str.char_indices() {
            match c {
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                ';' | '#' if !in_single_quote && !in_double_quote => {
                    // 주석 앞에 공백이 있는 경우만 인라인 주석으로 처리
                    if i > 0 && value_str.as_bytes()[i - 1] == b' ' {
                        return &value_str[..i - 1];
                    }
                    // 값의 시작이 ; 또는 #이면 그 자체가 주석이 아닌 빈 값 + 주석
                    if i == 0 {
                        return "";
                    }
                }
                _ => {}
            }
        }

        value_str
    }
}

impl FormatReader for IniReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let mut root = IndexMap::new();
        let mut current_section: Option<String> = None;

        for (line_num, line) in input.lines().enumerate() {
            let trimmed = line.trim();

            // 빈 줄 무시
            if trimmed.is_empty() {
                continue;
            }

            // 주석 무시
            if trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }

            // 섹션 헤더: [section]
            if trimmed.starts_with('[') {
                if let Some(end) = trimmed.find(']') {
                    let section_name = trimmed[1..end].trim().to_string();
                    if section_name.is_empty() {
                        return Err(crate::error::DkitError::ParseErrorAt {
                            format: "INI".to_string(),
                            source: "empty section name".to_string().into(),
                            line: line_num + 1,
                            column: 1,
                            line_text: line.to_string(),
                        }
                        .into());
                    }
                    current_section = Some(section_name.clone());
                    // 섹션이 아직 없으면 빈 Object로 초기화
                    root.entry(section_name)
                        .or_insert_with(|| Value::Object(IndexMap::new()));
                    continue;
                } else {
                    return Err(crate::error::DkitError::ParseErrorAt {
                        format: "INI".to_string(),
                        source: "unclosed section header (missing ']')".to_string().into(),
                        line: line_num + 1,
                        column: 1,
                        line_text: line.to_string(),
                    }
                    .into());
                }
            }

            // key=value 또는 key:value 파싱
            // 첫 번째 '=' 또는 ':' 를 구분자로 사용
            let sep_pos = trimmed
                .find('=')
                .or_else(|| trimmed.find(':'))
                .ok_or_else(|| crate::error::DkitError::ParseErrorAt {
                    format: "INI".to_string(),
                    source: "expected key=value or key:value format".to_string().into(),
                    line: line_num + 1,
                    column: 1,
                    line_text: line.to_string(),
                })?;

            let key = trimmed[..sep_pos].trim().to_string();
            if key.is_empty() {
                return Err(crate::error::DkitError::ParseErrorAt {
                    format: "INI".to_string(),
                    source: "empty key".to_string().into(),
                    line: line_num + 1,
                    column: 1,
                    line_text: line.to_string(),
                }
                .into());
            }

            let raw_value = &trimmed[sep_pos + 1..];
            let value_str = Self::strip_inline_comment(raw_value);
            let value = Self::parse_value(value_str);

            match &current_section {
                Some(section) => {
                    // 섹션 내부의 키-값
                    if let Some(Value::Object(section_map)) = root.get_mut(section) {
                        section_map.insert(key, value);
                    }
                }
                None => {
                    // 섹션 없는 최상위 키-값
                    root.insert(key, value);
                }
            }
        }

        Ok(Value::Object(root))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| crate::error::DkitError::ParseError {
                format: "INI".to_string(),
                source: Box::new(e),
            })?;
        self.read(&input)
    }
}

/// INI/CFG 포맷 Writer
///
/// 2-depth Object를 `[section]` + `key = value` 형식으로 출력한다.
/// 최상위 프리미티브 키는 섹션 없이 파일 상단에 출력한다.
pub struct IniWriter;

impl IniWriter {
    /// Value를 INI 값 문자열로 변환한다.
    fn format_value(value: &Value) -> String {
        match value {
            Value::String(s) => {
                // 특수 문자가 포함되면 따옴표로 감싼다
                if s.is_empty()
                    || s.contains(';')
                    || s.contains('#')
                    || s.contains('=')
                    || s.contains(':')
                    || s.starts_with(' ')
                    || s.ends_with(' ')
                    || s.starts_with('"')
                    || s.starts_with('\'')
                {
                    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                } else {
                    s.clone()
                }
            }
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Array(_) | Value::Object(_) => {
                // 중첩 구조는 JSON 문자열로 직렬화
                serde_json::to_string(value).unwrap_or_default()
            }
        }
    }
}

impl FormatWriter for IniWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        match value {
            Value::Object(map) => {
                let mut output = String::new();

                // 1단계: 최상위 프리미티브 키 출력 (섹션 없는 키)
                for (key, val) in map {
                    if !matches!(val, Value::Object(_)) {
                        output.push_str(&format!("{} = {}\n", key, Self::format_value(val)));
                    }
                }

                // 최상위 키와 섹션 사이에 빈 줄 추가
                let has_top_level = map.values().any(|v| !matches!(v, Value::Object(_)));
                let has_sections = map.values().any(|v| matches!(v, Value::Object(_)));
                if has_top_level && has_sections {
                    output.push('\n');
                }

                // 2단계: 섹션별 출력
                let mut first_section = true;
                for (section, val) in map {
                    if let Value::Object(section_map) = val {
                        if !first_section {
                            output.push('\n');
                        }
                        first_section = false;
                        output.push_str(&format!("[{}]\n", section));
                        for (key, v) in section_map {
                            output.push_str(&format!("{} = {}\n", key, Self::format_value(v)));
                        }
                    }
                }

                Ok(output)
            }
            _ => {
                anyhow::bail!(
                    "INI format requires an Object value (sections with key-value pairs). \
                     Got: {}",
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
                format: "INI".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- IniReader 테스트 ---

    #[test]
    fn test_reader_simple_section() {
        let reader = IniReader;
        let input = "[database]\nhost = localhost\nport = 5432\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        let db = obj.get("database").unwrap().as_object().unwrap();
        assert_eq!(
            db.get("host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(db.get("port"), Some(&Value::Integer(5432)));
    }

    #[test]
    fn test_reader_multiple_sections() {
        let reader = IniReader;
        let input = "[section1]\nkey1 = value1\n\n[section2]\nkey2 = value2\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 2);

        let s1 = obj.get("section1").unwrap().as_object().unwrap();
        assert_eq!(s1.get("key1"), Some(&Value::String("value1".to_string())));

        let s2 = obj.get("section2").unwrap().as_object().unwrap();
        assert_eq!(s2.get("key2"), Some(&Value::String("value2".to_string())));
    }

    #[test]
    fn test_reader_comments_hash() {
        let reader = IniReader;
        let input = "# This is a comment\n[section]\nkey = value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_reader_comments_semicolon() {
        let reader = IniReader;
        let input = "; This is a comment\n[section]\nkey = value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_reader_empty_lines() {
        let reader = IniReader;
        let input = "[section]\n\nkey1 = val1\n\nkey2 = val2\n\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(section.len(), 2);
    }

    #[test]
    fn test_reader_colon_separator() {
        let reader = IniReader;
        let input = "[section]\nkey: value\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_no_section_top_level() {
        let reader = IniReader;
        let input = "key1 = value1\nkey2 = value2\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("key1"), Some(&Value::String("value1".to_string())));
        assert_eq!(obj.get("key2"), Some(&Value::String("value2".to_string())));
    }

    #[test]
    fn test_reader_mixed_top_level_and_sections() {
        let reader = IniReader;
        let input = "global_key = global_value\n\n[section]\nkey = value\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.get("global_key"),
            Some(&Value::String("global_value".to_string()))
        );
        let section = obj.get("section").unwrap().as_object().unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_boolean_values() {
        let reader = IniReader;
        let input = "[flags]\nenabled = true\ndisabled = false\nyes_val = yes\nno_val = no\non_val = on\noff_val = off\n";
        let v = reader.read(input).unwrap();
        let flags = v
            .as_object()
            .unwrap()
            .get("flags")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(flags.get("enabled"), Some(&Value::Bool(true)));
        assert_eq!(flags.get("disabled"), Some(&Value::Bool(false)));
        assert_eq!(flags.get("yes_val"), Some(&Value::Bool(true)));
        assert_eq!(flags.get("no_val"), Some(&Value::Bool(false)));
        assert_eq!(flags.get("on_val"), Some(&Value::Bool(true)));
        assert_eq!(flags.get("off_val"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_reader_integer_values() {
        let reader = IniReader;
        let input = "[numbers]\nport = 8080\nnegative = -42\nzero = 0\n";
        let v = reader.read(input).unwrap();
        let nums = v
            .as_object()
            .unwrap()
            .get("numbers")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(nums.get("port"), Some(&Value::Integer(8080)));
        assert_eq!(nums.get("negative"), Some(&Value::Integer(-42)));
        assert_eq!(nums.get("zero"), Some(&Value::Integer(0)));
    }

    #[test]
    fn test_reader_float_values() {
        let reader = IniReader;
        let input = "[numbers]\npi = 3.14\nrate = 0.5\n";
        let v = reader.read(input).unwrap();
        let nums = v
            .as_object()
            .unwrap()
            .get("numbers")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(nums.get("pi"), Some(&Value::Float(3.14)));
        assert_eq!(nums.get("rate"), Some(&Value::Float(0.5)));
    }

    #[test]
    fn test_reader_quoted_string() {
        let reader = IniReader;
        let input = "[section]\nname = \"hello world\"\nsingle = 'quoted'\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("name"),
            Some(&Value::String("hello world".to_string()))
        );
        assert_eq!(
            section.get("single"),
            Some(&Value::String("quoted".to_string()))
        );
    }

    #[test]
    fn test_reader_empty_value() {
        let reader = IniReader;
        let input = "[section]\nempty =\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(section.get("empty"), Some(&Value::String(String::new())));
    }

    #[test]
    fn test_reader_inline_comment() {
        let reader = IniReader;
        let input = "[section]\nkey = value ; this is a comment\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_inline_comment_hash() {
        let reader = IniReader;
        let input = "[section]\nkey = value # this is a comment\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_value_with_equals() {
        let reader = IniReader;
        let input = "[section]\nurl = https://example.com?key=value\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("url"),
            Some(&Value::String("https://example.com?key=value".to_string()))
        );
    }

    #[test]
    fn test_reader_whitespace_around_key_value() {
        let reader = IniReader;
        let input = "[section]\n  key  =  value  \n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_empty_input() {
        let reader = IniReader;
        let v = reader.read("").unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_only_comments() {
        let reader = IniReader;
        let input = "# comment 1\n; comment 2\n";
        let v = reader.read(input).unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_duplicate_keys_last_wins() {
        let reader = IniReader;
        let input = "[section]\nkey = first\nkey = second\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("second".to_string()))
        );
    }

    #[test]
    fn test_reader_unclosed_section_error() {
        let reader = IniReader;
        let input = "[section\nkey = value\n";
        let result = reader.read(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_empty_section_name_error() {
        let reader = IniReader;
        let input = "[]\nkey = value\n";
        let result = reader.read(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_no_separator_error() {
        let reader = IniReader;
        let input = "[section]\ninvalid line\n";
        let result = reader.read(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = IniReader;
        let input = b"[section]\nkey = value" as &[u8];
        let v = reader.read_from_reader(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(
            section.get("key"),
            Some(&Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_reader_section_with_spaces() {
        let reader = IniReader;
        let input = "[ my section ]\nkey = value\n";
        let v = reader.read(input).unwrap();
        assert!(v.as_object().unwrap().contains_key("my section"));
    }

    #[test]
    fn test_reader_case_sensitive_boolean() {
        let reader = IniReader;
        let input = "[section]\na = True\nb = FALSE\nc = Yes\nd = NO\n";
        let v = reader.read(input).unwrap();
        let section = v
            .as_object()
            .unwrap()
            .get("section")
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(section.get("a"), Some(&Value::Bool(true)));
        assert_eq!(section.get("b"), Some(&Value::Bool(false)));
        assert_eq!(section.get("c"), Some(&Value::Bool(true)));
        assert_eq!(section.get("d"), Some(&Value::Bool(false)));
    }

    // --- IniWriter 테스트 ---

    #[test]
    fn test_writer_simple_section() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut section = IndexMap::new();
            section.insert("host".to_string(), Value::String("localhost".to_string()));
            section.insert("port".to_string(), Value::Integer(5432));
            m.insert("database".to_string(), Value::Object(section));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("[database]"));
        assert!(output.contains("host = localhost"));
        assert!(output.contains("port = 5432"));
    }

    #[test]
    fn test_writer_multiple_sections() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut s1 = IndexMap::new();
            s1.insert("key1".to_string(), Value::String("val1".to_string()));
            m.insert("section1".to_string(), Value::Object(s1));
            let mut s2 = IndexMap::new();
            s2.insert("key2".to_string(), Value::String("val2".to_string()));
            m.insert("section2".to_string(), Value::Object(s2));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("[section1]"));
        assert!(output.contains("[section2]"));
        assert!(output.contains("key1 = val1"));
        assert!(output.contains("key2 = val2"));
    }

    #[test]
    fn test_writer_top_level_keys() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("global".to_string(), Value::String("value".to_string()));
            m
        });
        let output = writer.write(&v).unwrap();
        assert_eq!(output, "global = value\n");
    }

    #[test]
    fn test_writer_mixed_top_level_and_sections() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("global".to_string(), Value::String("value".to_string()));
            let mut section = IndexMap::new();
            section.insert("key".to_string(), Value::String("val".to_string()));
            m.insert("section".to_string(), Value::Object(section));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.starts_with("global = value\n"));
        assert!(output.contains("[section]"));
        assert!(output.contains("key = val"));
    }

    #[test]
    fn test_writer_boolean() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut section = IndexMap::new();
            section.insert("enabled".to_string(), Value::Bool(true));
            section.insert("disabled".to_string(), Value::Bool(false));
            m.insert("flags".to_string(), Value::Object(section));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("enabled = true"));
        assert!(output.contains("disabled = false"));
    }

    #[test]
    fn test_writer_null_value() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut section = IndexMap::new();
            section.insert("empty".to_string(), Value::Null);
            m.insert("section".to_string(), Value::Object(section));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("empty = \n"));
    }

    #[test]
    fn test_writer_quotes_special_chars() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut section = IndexMap::new();
            section.insert(
                "val".to_string(),
                Value::String("has;semicolon".to_string()),
            );
            m.insert("section".to_string(), Value::Object(section));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("val = \"has;semicolon\""));
    }

    #[test]
    fn test_writer_empty_string() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut section = IndexMap::new();
            section.insert("val".to_string(), Value::String(String::new()));
            m.insert("section".to_string(), Value::Object(section));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("val = \"\""));
    }

    #[test]
    fn test_writer_non_object_error() {
        let writer = IniWriter;
        let result = writer.write(&Value::String("hello".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = IniWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            let mut section = IndexMap::new();
            section.insert("key".to_string(), Value::String("value".to_string()));
            m.insert("section".to_string(), Value::Object(section));
            m
        });
        let mut buf = Vec::new();
        writer.write_to_writer(&v, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("[section]"));
        assert!(output.contains("key = value"));
    }

    // --- 왕복 변환 테스트 ---

    #[test]
    fn test_roundtrip_simple() {
        let input = "[database]\nhost = localhost\nport = 5432\n";
        let reader = IniReader;
        let writer = IniWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_multiple_sections() {
        let input =
            "[server]\nhost = 0.0.0.0\nport = 8080\n\n[database]\nhost = localhost\nport = 5432\n";
        let reader = IniReader;
        let writer = IniWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_mixed() {
        let input = "global = value\n\n[section]\nkey = val\n";
        let reader = IniReader;
        let writer = IniWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_booleans_and_numbers() {
        let input = "[config]\nenabled = true\ncount = 42\nrate = 3.14\n";
        let reader = IniReader;
        let writer = IniWriter;

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_reader_empty_section() {
        let reader = IniReader;
        let input = "[empty]\n[notempty]\nkey = val\n";
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert!(obj.get("empty").unwrap().as_object().unwrap().is_empty());
        assert_eq!(obj.get("notempty").unwrap().as_object().unwrap().len(), 1);
    }

    #[test]
    fn test_reader_realistic_config() {
        let reader = IniReader;
        let input = r#"
; MySQL configuration file
[mysqld]
port = 3306
bind-address = 127.0.0.1
max_connections = 100
innodb_buffer_pool_size = 256M

[client]
port = 3306
socket = /var/run/mysqld/mysqld.sock
"#;
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        let mysqld = obj.get("mysqld").unwrap().as_object().unwrap();
        assert_eq!(mysqld.get("port"), Some(&Value::Integer(3306)));
        assert_eq!(
            mysqld.get("bind-address"),
            Some(&Value::String("127.0.0.1".to_string()))
        );
        assert_eq!(mysqld.get("max_connections"), Some(&Value::Integer(100)));
    }
}
