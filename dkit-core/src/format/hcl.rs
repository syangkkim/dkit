use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// hcl::Value → 내부 Value 변환
fn from_hcl_value(v: hcl::Value) -> Value {
    match v {
        hcl::Value::Null => Value::Null,
        hcl::Value::Bool(b) => Value::Bool(b),
        hcl::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::String(n.to_string())
            }
        }
        hcl::Value::String(s) => Value::String(s),
        hcl::Value::Array(arr) => Value::Array(arr.into_iter().map(from_hcl_value).collect()),
        hcl::Value::Object(map) => {
            let obj: IndexMap<String, Value> = map
                .into_iter()
                .map(|(k, v)| (k, from_hcl_value(v)))
                .collect();
            Value::Object(obj)
        }
    }
}

/// 내부 Value → hcl::Value 변환
fn to_hcl_value(v: &Value) -> hcl::Value {
    match v {
        Value::Null => hcl::Value::Null,
        Value::Bool(b) => hcl::Value::Bool(*b),
        Value::Integer(n) => hcl::Value::Number((*n).into()),
        Value::Float(f) => {
            if let Some(n) = hcl::Number::from_f64(*f) {
                hcl::Value::Number(n)
            } else {
                // NaN/Infinity → string
                hcl::Value::String(f.to_string())
            }
        }
        Value::String(s) => hcl::Value::String(s.clone()),
        Value::Array(arr) => hcl::Value::Array(arr.iter().map(to_hcl_value).collect()),
        Value::Object(map) => {
            let hcl_map: hcl::Map<String, hcl::Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), to_hcl_value(v)))
                .collect();
            hcl::Value::Object(hcl_map)
        }
    }
}

/// HCL 포맷 Reader
///
/// HCL v2 문법을 파싱하여 내부 Value로 변환한다.
/// 블록 구조는 중첩된 Object로 매핑된다.
pub struct HclReader;

impl FormatReader for HclReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let hcl_val: hcl::Value =
            hcl::from_str(input).map_err(|e| crate::error::DkitError::ParseError {
                format: "HCL".to_string(),
                source: Box::new(e),
            })?;
        Ok(from_hcl_value(hcl_val))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| crate::error::DkitError::ParseError {
                format: "HCL".to_string(),
                source: Box::new(e),
            })?;
        self.read(&input)
    }
}

/// HCL 포맷 Writer
///
/// 내부 Value를 HCL 형식으로 직렬화한다.
/// 최상위는 반드시 Object여야 한다. 다른 타입은 "data" 키로 감싼다.
pub struct HclWriter;

impl FormatWriter for HclWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let hcl_val = match value {
            Value::Object(_) => to_hcl_value(value),
            _ => {
                let mut map = hcl::Map::new();
                map.insert("data".to_string(), to_hcl_value(value));
                hcl::Value::Object(map)
            }
        };
        hcl::to_string(&hcl_val).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "HCL".to_string(),
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
                format: "HCL".to_string(),
                source: Box::new(e),
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- from_hcl_value 변환 테스트 ---

    #[test]
    fn test_convert_null() {
        assert_eq!(from_hcl_value(hcl::Value::Null), Value::Null);
    }

    #[test]
    fn test_convert_bool() {
        assert_eq!(from_hcl_value(hcl::Value::Bool(true)), Value::Bool(true));
        assert_eq!(from_hcl_value(hcl::Value::Bool(false)), Value::Bool(false));
    }

    #[test]
    fn test_convert_integer() {
        assert_eq!(
            from_hcl_value(hcl::Value::Number(42.into())),
            Value::Integer(42)
        );
    }

    #[test]
    fn test_convert_float() {
        let n = hcl::Number::from_f64(3.14).unwrap();
        assert_eq!(from_hcl_value(hcl::Value::Number(n)), Value::Float(3.14));
    }

    #[test]
    fn test_convert_string() {
        assert_eq!(
            from_hcl_value(hcl::Value::String("hello".to_string())),
            Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_convert_array() {
        let arr = hcl::Value::Array(vec![
            hcl::Value::Number(1.into()),
            hcl::Value::String("two".to_string()),
            hcl::Value::Bool(true),
        ]);
        let v = from_hcl_value(arr);
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::Integer(1));
        assert_eq!(arr[1], Value::String("two".to_string()));
        assert_eq!(arr[2], Value::Bool(true));
    }

    #[test]
    fn test_convert_object() {
        let mut map = hcl::Map::new();
        map.insert("name".to_string(), hcl::Value::String("dkit".to_string()));
        map.insert("version".to_string(), hcl::Value::Number(1.into()));
        let v = from_hcl_value(hcl::Value::Object(map));
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("version"), Some(&Value::Integer(1)));
    }

    // --- to_hcl_value 왕복 변환 테스트 ---

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
            let hcl_v = to_hcl_value(&v);
            let back = from_hcl_value(hcl_v);
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
        let hcl_v = to_hcl_value(&original);
        let back = from_hcl_value(hcl_v);
        assert_eq!(back, original);
    }

    #[test]
    fn test_nan_converts_to_string() {
        let hcl_v = to_hcl_value(&Value::Float(f64::NAN));
        assert_eq!(hcl_v, hcl::Value::String("NaN".to_string()));
    }

    #[test]
    fn test_infinity_converts_to_string() {
        let hcl_v = to_hcl_value(&Value::Float(f64::INFINITY));
        assert_eq!(hcl_v, hcl::Value::String("inf".to_string()));
    }

    // --- HclReader 테스트 ---

    #[test]
    fn test_reader_simple_attributes() {
        let reader = HclReader;
        let input = r#"
name = "dkit"
count = 42
enabled = true
"#;
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("count"), Some(&Value::Integer(42)));
        assert_eq!(obj.get("enabled"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_reader_block_structure() {
        let reader = HclReader;
        let input = r#"
resource "aws_instance" "example" {
  ami           = "ami-12345678"
  instance_type = "t2.micro"
}
"#;
        let v = reader.read(input).unwrap();
        let resource = v.as_object().unwrap().get("resource").unwrap();
        let aws_instance = resource.as_object().unwrap().get("aws_instance").unwrap();
        let example = aws_instance.as_object().unwrap().get("example").unwrap();
        assert_eq!(
            example.as_object().unwrap().get("ami"),
            Some(&Value::String("ami-12345678".to_string()))
        );
        assert_eq!(
            example.as_object().unwrap().get("instance_type"),
            Some(&Value::String("t2.micro".to_string()))
        );
    }

    #[test]
    fn test_reader_nested_blocks() {
        let reader = HclReader;
        let input = r#"
resource "aws_instance" "web" {
  ami = "ami-abc123"

  tags = {
    Name = "web-server"
    Env  = "production"
  }
}
"#;
        let v = reader.read(input).unwrap();
        let resource = v.as_object().unwrap().get("resource").unwrap();
        let instance = resource
            .as_object()
            .unwrap()
            .get("aws_instance")
            .unwrap()
            .as_object()
            .unwrap()
            .get("web")
            .unwrap();
        let tags = instance.as_object().unwrap().get("tags").unwrap();
        assert_eq!(
            tags.as_object().unwrap().get("Name"),
            Some(&Value::String("web-server".to_string()))
        );
    }

    #[test]
    fn test_reader_array_attribute() {
        let reader = HclReader;
        let input = r#"
tags = ["a", "b", "c"]
ports = [80, 443]
"#;
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        let tags = obj.get("tags").unwrap().as_array().unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], Value::String("a".to_string()));
        let ports = obj.get("ports").unwrap().as_array().unwrap();
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0], Value::Integer(80));
    }

    #[test]
    fn test_reader_invalid_hcl() {
        let reader = HclReader;
        let result = reader.read("{ invalid hcl syntax !!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_reader_empty_input() {
        let reader = HclReader;
        let v = reader.read("").unwrap();
        assert_eq!(v, Value::Object(IndexMap::new()));
    }

    #[test]
    fn test_reader_from_reader() {
        let reader = HclReader;
        let input = b"name = \"test\"" as &[u8];
        let v = reader.read_from_reader(input).unwrap();
        assert_eq!(
            v.as_object().unwrap().get("name"),
            Some(&Value::String("test".to_string()))
        );
    }

    // --- HclWriter 테스트 ---

    #[test]
    fn test_writer_simple_object() {
        let writer = HclWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("test".to_string()));
            m.insert("count".to_string(), Value::Integer(42));
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("name"));
        assert!(output.contains("\"test\""));
        assert!(output.contains("42"));
    }

    #[test]
    fn test_writer_wraps_non_object() {
        let writer = HclWriter;
        let output = writer.write(&Value::Integer(42)).unwrap();
        assert!(output.contains("data = 42"));
    }

    #[test]
    fn test_writer_wraps_array() {
        let writer = HclWriter;
        let v = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let output = writer.write(&v).unwrap();
        assert!(output.contains("data"));
    }

    #[test]
    fn test_writer_nested_object() {
        let writer = HclWriter;
        let v = Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "server".to_string(),
                Value::Object({
                    let mut inner = IndexMap::new();
                    inner.insert("host".to_string(), Value::String("localhost".to_string()));
                    inner.insert("port".to_string(), Value::Integer(8080));
                    inner
                }),
            );
            m
        });
        let output = writer.write(&v).unwrap();
        assert!(output.contains("server"));
        assert!(output.contains("localhost"));
        assert!(output.contains("8080"));
    }

    #[test]
    fn test_writer_to_writer() {
        let writer = HclWriter;
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
        let hcl_input = r#"
name = "dkit"
version = 1
enabled = true
tags = ["cli", "data"]
"#;
        let reader = HclReader;
        let writer = HclWriter;

        let value = reader.read(hcl_input).unwrap();
        let hcl_output = writer.write(&value).unwrap();
        let value2 = reader.read(&hcl_output).unwrap();

        assert_eq!(value, value2);
    }

    // --- 특수 케이스 ---

    #[test]
    fn test_unicode_string() {
        let reader = HclReader;
        let v = reader.read("emoji = \"🎉\"\nkorean = \"한글\"").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("emoji"), Some(&Value::String("🎉".to_string())));
        assert_eq!(obj.get("korean"), Some(&Value::String("한글".to_string())));
    }

    #[test]
    fn test_negative_numbers() {
        let reader = HclReader;
        let v = reader.read("neg_int = -42\nneg_float = -3.14").unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.get("neg_int"), Some(&Value::Integer(-42)));
        assert_eq!(obj.get("neg_float"), Some(&Value::Float(-3.14)));
    }

    #[test]
    fn test_terraform_style_config() {
        let reader = HclReader;
        let input = r#"
terraform {
  required_version = ">= 1.0"
}

variable "region" {
  type    = "string"
  default = "us-west-2"
}

provider "aws" {
  region = "us-west-2"
}
"#;
        let v = reader.read(input).unwrap();
        let obj = v.as_object().unwrap();
        assert!(obj.contains_key("terraform"));
        assert!(obj.contains_key("variable"));
        assert!(obj.contains_key("provider"));
    }
}
