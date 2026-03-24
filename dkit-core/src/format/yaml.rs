use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatOptions, FormatReader, FormatWriter};
use crate::value::Value;

/// serde_yaml::Value → 내부 Value 변환
fn from_yaml_value(v: serde_yaml::Value) -> Value {
    match v {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Float(n.as_f64().unwrap_or(f64::NAN))
            }
        }
        serde_yaml::Value::String(s) => Value::String(s),
        serde_yaml::Value::Sequence(seq) => {
            Value::Array(seq.into_iter().map(from_yaml_value).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let obj: IndexMap<String, Value> = map
                .into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        serde_yaml::Value::String(s) => s,
                        serde_yaml::Value::Number(n) => n.to_string(),
                        serde_yaml::Value::Bool(b) => b.to_string(),
                        serde_yaml::Value::Null => "null".to_string(),
                        other => serde_yaml::to_string(&other)
                            .unwrap_or_default()
                            .trim()
                            .to_string(),
                    };
                    (key, from_yaml_value(v))
                })
                .collect();
            Value::Object(obj)
        }
        serde_yaml::Value::Tagged(tagged) => from_yaml_value(tagged.value),
    }
}

/// 내부 Value → serde_yaml::Value 변환
fn to_yaml_value(v: &Value) -> serde_yaml::Value {
    match v {
        Value::Null => serde_yaml::Value::Null,
        Value::Bool(b) => serde_yaml::Value::Bool(*b),
        Value::Integer(n) => serde_yaml::Value::Number(serde_yaml::Number::from(*n)),
        Value::Float(f) => {
            if f.is_nan() || f.is_infinite() {
                serde_yaml::Value::Null
            } else {
                serde_yaml::Value::Number(serde_yaml::Number::from(*f))
            }
        }
        Value::String(s) => serde_yaml::Value::String(s.clone()),
        Value::Array(arr) => serde_yaml::Value::Sequence(arr.iter().map(to_yaml_value).collect()),
        Value::Object(map) => {
            let mapping: serde_yaml::Mapping = map
                .iter()
                .map(|(k, v)| (serde_yaml::Value::String(k.clone()), to_yaml_value(v)))
                .collect();
            serde_yaml::Value::Mapping(mapping)
        }
    }
}

/// YAML 포맷 Reader
pub struct YamlReader;

impl FormatReader for YamlReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let yaml_val: serde_yaml::Value =
            serde_yaml::from_str(input).map_err(|e: serde_yaml::Error| {
                if let Some(loc) = e.location() {
                    let line = loc.line();
                    let column = loc.column();
                    let line_text = input
                        .lines()
                        .nth(line.saturating_sub(1))
                        .unwrap_or("")
                        .to_string();
                    crate::error::DkitError::ParseErrorAt {
                        format: "YAML".to_string(),
                        source: Box::new(e),
                        line,
                        column,
                        line_text,
                    }
                } else {
                    crate::error::DkitError::ParseError {
                        format: "YAML".to_string(),
                        source: Box::new(e),
                    }
                }
            })?;
        Ok(from_yaml_value(yaml_val))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| crate::error::DkitError::ParseError {
                format: "YAML".to_string(),
                source: Box::new(e),
            })?;
        self.read(&input)
    }
}

/// YAML 포맷 Writer
#[derive(Default)]
pub struct YamlWriter {
    options: FormatOptions,
}

impl YamlWriter {
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }
}

impl FormatWriter for YamlWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let yaml_val = to_yaml_value(value);

        if self.options.flow_style {
            // Flow style: JSON-like inline format
            let json_val = yaml_to_json_style(&yaml_val);
            let output = serde_json::to_string(&json_val).map_err(|e| {
                crate::error::DkitError::WriteError {
                    format: "YAML".to_string(),
                    source: Box::new(e),
                }
            })?;
            Ok(output)
        } else {
            let output = serde_yaml::to_string(&yaml_val).map_err(|e| {
                crate::error::DkitError::WriteError {
                    format: "YAML".to_string(),
                    source: Box::new(e),
                }
            })?;
            Ok(output)
        }
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let output = self.write(value)?;
        writer
            .write_all(output.as_bytes())
            .map_err(|e| crate::error::DkitError::WriteError {
                format: "YAML".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }
}

/// YAML Value를 JSON-compatible value로 변환 (flow style 출력용)
fn yaml_to_json_style(v: &serde_yaml::Value) -> serde_json::Value {
    match v {
        serde_yaml::Value::Null => serde_json::Value::Null,
        serde_yaml::Value::Bool(b) => serde_json::Value::Bool(*b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        serde_yaml::Value::String(s) => serde_json::Value::String(s.clone()),
        serde_yaml::Value::Sequence(seq) => {
            serde_json::Value::Array(seq.iter().map(yaml_to_json_style).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| {
                    let key = match k {
                        serde_yaml::Value::String(s) => s.clone(),
                        other => serde_yaml::to_string(other)
                            .unwrap_or_default()
                            .trim()
                            .to_string(),
                    };
                    (key, yaml_to_json_style(v))
                })
                .collect();
            serde_json::Value::Object(obj)
        }
        serde_yaml::Value::Tagged(tagged) => yaml_to_json_style(&tagged.value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- from_yaml_value 변환 테스트 ---

    #[test]
    fn test_convert_null() {
        let v = from_yaml_value(serde_yaml::Value::Null);
        assert_eq!(v, Value::Null);
    }

    #[test]
    fn test_convert_bool() {
        assert_eq!(
            from_yaml_value(serde_yaml::Value::Bool(true)),
            Value::Bool(true)
        );
        assert_eq!(
            from_yaml_value(serde_yaml::Value::Bool(false)),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_convert_integer() {
        let yaml_val: serde_yaml::Value = serde_yaml::from_str("42").unwrap();
        let v = from_yaml_value(yaml_val);
        assert_eq!(v, Value::Integer(42));
    }

    #[test]
    fn test_convert_float() {
        let yaml_val: serde_yaml::Value = serde_yaml::from_str("3.14").unwrap();
        let v = from_yaml_value(yaml_val);
        assert_eq!(v, Value::Float(3.14));
    }

    #[test]
    fn test_convert_string() {
        let v = from_yaml_value(serde_yaml::Value::String("hello".to_string()));
        assert_eq!(v, Value::String("hello".to_string()));
    }

    #[test]
    fn test_convert_sequence() {
        let yaml_val: serde_yaml::Value = serde_yaml::from_str("[1, two, null]").unwrap();
        let v = from_yaml_value(yaml_val);
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::Integer(1));
        assert_eq!(arr[1], Value::String("two".to_string()));
        assert_eq!(arr[2], Value::Null);
    }

    #[test]
    fn test_convert_mapping() {
        let yaml = "name: dkit\nversion: 1";
        let yaml_val: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let v = from_yaml_value(yaml_val);
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("version"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_convert_nested() {
        let yaml = r#"
users:
  - name: Alice
    age: 30
  - name: Bob
    age: 25
"#;
        let yaml_val: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let v = from_yaml_value(yaml_val);
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
            users[0].as_object().unwrap().get("age"),
            Some(&Value::Integer(30))
        );
    }

    #[test]
    fn test_convert_numeric_key() {
        let yaml = "123: value";
        let yaml_val: serde_yaml::Value = serde_yaml::from_str(yaml).unwrap();
        let v = from_yaml_value(yaml_val);
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("123"), Some(&Value::String("value".to_string())));
    }

    // --- to_yaml_value 왕복 변환 테스트 ---

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
            let yaml = to_yaml_value(&v);
            let back = from_yaml_value(yaml);
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
        let yaml = to_yaml_value(&original);
        let back = from_yaml_value(yaml);
        assert_eq!(back, original);
    }

    // --- YamlReader 테스트 ---

    #[test]
    fn test_reader_simple_mapping() {
        let reader = YamlReader;
        let v = reader.read("name: dkit\ncount: 42").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("count"), Some(&Value::Integer(42)));
    }

    #[test]
    fn test_reader_sequence() {
        let reader = YamlReader;
        let v = reader.read("- 1\n- 2\n- 3").unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_reader_invalid_yaml() {
        let reader = YamlReader;
        let result = reader.read(":\n  :\n    - ][");
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_empty_mapping() {
        let reader = YamlReader;
        let v = reader.read("{}").unwrap();
        assert!(v.as_object().unwrap().is_empty());
    }

    #[test]
    fn test_reader_empty_sequence() {
        let reader = YamlReader;
        let v = reader.read("[]").unwrap();
        assert!(v.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = YamlReader;
        let input = "x: 1".as_bytes();
        let v = reader.read_from_reader(input).unwrap();
        assert_eq!(v.as_object().unwrap().get("x"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_reader_multiline_string() {
        let reader = YamlReader;
        let yaml = "description: |\n  line1\n  line2\n";
        let v = reader.read(yaml).unwrap();
        let desc = v
            .as_object()
            .unwrap()
            .get("description")
            .unwrap()
            .as_str()
            .unwrap();
        assert!(desc.contains("line1"));
        assert!(desc.contains("line2"));
    }

    #[test]
    fn test_reader_anchor_alias() {
        let reader = YamlReader;
        let yaml =
            "defaults: &defaults\n  timeout: 30\nserver:\n  host: localhost\n  timeout: *defaults";
        let v = reader.read(yaml).unwrap();
        let server = v.as_object().unwrap().get("server").unwrap();
        assert_eq!(
            server.as_object().unwrap().get("host"),
            Some(&Value::String("localhost".to_string()))
        );
        // Alias reference resolves to the anchored value
        let defaults = v.as_object().unwrap().get("defaults").unwrap();
        assert_eq!(
            defaults.as_object().unwrap().get("timeout"),
            Some(&Value::Integer(30))
        );
    }

    // --- YamlWriter 테스트 ---

    #[test]
    fn test_writer_default() {
        let writer = YamlWriter::default();
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("a:"));
        assert!(output.contains('1'));
    }

    #[test]
    fn test_writer_flow_style() {
        let writer = YamlWriter::new(FormatOptions {
            flow_style: true,
            ..Default::default()
        });
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("a".to_string(), Value::Integer(1));
            m
        });
        let output = writer.write(&v).unwrap();
        // Flow style produces JSON-like inline output
        assert!(output.contains('{'));
        assert!(output.contains('}'));
    }

    #[test]
    fn test_writer_null() {
        let writer = YamlWriter::default();
        let output = writer.write(&Value::Null).unwrap();
        assert!(output.contains("null"));
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = YamlWriter::default();
        let mut buf = Vec::new();
        writer
            .write_to_writer(&Value::Integer(42), &mut buf)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("42"));
    }

    #[test]
    fn test_writer_nan_becomes_null() {
        let writer = YamlWriter::default();
        let output = writer.write(&Value::Float(f64::NAN)).unwrap();
        assert!(output.contains("null"));
    }

    // --- 왕복 변환 테스트 (Reader → Writer → Reader) ---

    #[test]
    fn test_full_roundtrip() {
        let yaml_input = "name: dkit\nversion: 1\ntags:\n- rust\n- cli\n";
        let reader = YamlReader;
        let writer = YamlWriter::default();

        let value = reader.read(yaml_input).unwrap();
        let yaml_output = writer.write(&value).unwrap();
        let value2 = reader.read(&yaml_output).unwrap();

        assert_eq!(value, value2);
    }

    // --- 특수 케이스 ---

    #[test]
    fn test_unicode_string() {
        let reader = YamlReader;
        let v = reader.read("emoji: 🎉\nkorean: 한글").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("emoji"), Some(&Value::String("🎉".to_string())));
        assert_eq!(obj.get("korean"), Some(&Value::String("한글".to_string())));
    }

    #[test]
    fn test_negative_numbers() {
        let reader = YamlReader;
        let v = reader.read("neg_int: -42\nneg_float: -3.14").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("neg_int"), Some(&Value::Integer(-42)));
        assert_eq!(obj.get("neg_float"), Some(&Value::Float(-3.14)));
    }

    #[test]
    fn test_deeply_nested() {
        let yaml = "a:\n  b:\n    c:\n      d: 1";
        let reader = YamlReader;
        let v = reader.read(yaml).unwrap();
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
        let reader = YamlReader;
        let v = reader.read("yes_val: true\nno_val: false").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("yes_val"), Some(&Value::Bool(true)));
        assert_eq!(obj.get("no_val"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_null_variants() {
        let reader = YamlReader;
        let v = reader.read("a: null\nb: ~\nc:").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("a"), Some(&Value::Null));
        assert_eq!(obj.get("b"), Some(&Value::Null));
        assert_eq!(obj.get("c"), Some(&Value::Null));
    }
}
