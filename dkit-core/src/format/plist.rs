use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// plist::Value -> internal Value conversion
fn from_plist_value(v: plist::Value) -> Value {
    match v {
        plist::Value::Boolean(b) => Value::Bool(b),
        plist::Value::Integer(n) => {
            if let Some(i) = n.as_signed() {
                Value::Integer(i)
            } else if let Some(u) = n.as_unsigned() {
                // u64 that doesn't fit in i64
                Value::Float(u as f64)
            } else {
                Value::Null
            }
        }
        plist::Value::Real(f) => Value::Float(f),
        plist::Value::String(s) => Value::String(s),
        plist::Value::Array(arr) => Value::Array(arr.into_iter().map(from_plist_value).collect()),
        plist::Value::Dictionary(dict) => {
            let obj: IndexMap<String, Value> = dict
                .into_iter()
                .map(|(k, v)| (k, from_plist_value(v)))
                .collect();
            Value::Object(obj)
        }
        plist::Value::Data(bytes) => {
            use base64::Engine;
            Value::String(base64::engine::general_purpose::STANDARD.encode(bytes))
        }
        plist::Value::Date(date) => Value::String(date.to_xml_format()),
        plist::Value::Uid(uid) => Value::Integer(uid.get() as i64),
        _ => Value::Null,
    }
}

/// internal Value -> plist::Value conversion
fn to_plist_value(v: &Value) -> plist::Value {
    match v {
        Value::Null => plist::Value::String("".to_string()),
        Value::Bool(b) => plist::Value::Boolean(*b),
        Value::Integer(n) => plist::Value::Integer((*n).into()),
        Value::Float(f) => plist::Value::Real(*f),
        Value::String(s) => plist::Value::String(s.clone()),
        Value::Array(arr) => plist::Value::Array(arr.iter().map(to_plist_value).collect()),
        Value::Object(map) => {
            let dict: plist::Dictionary = map
                .iter()
                .map(|(k, v)| (k.clone(), to_plist_value(v)))
                .collect();
            plist::Value::Dictionary(dict)
        }
    }
}

/// macOS Property List (plist) format reader.
///
/// Supports XML plist format. Parses plist data into the internal Value type.
/// Data type mapping:
/// - dict -> Object
/// - array -> Array
/// - string -> String
/// - integer -> Integer
/// - real -> Float
/// - true/false -> Bool
/// - date -> String (XML date format)
/// - data -> String (Base64 encoded)
pub struct PlistReader;

impl FormatReader for PlistReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let plist_val: plist::Value = plist::from_bytes(input.as_bytes()).map_err(|e| {
            crate::error::DkitError::ParseError {
                format: "plist".to_string(),
                source: Box::new(e),
            }
        })?;
        Ok(from_plist_value(plist_val))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .map_err(|e| crate::error::DkitError::ParseError {
                format: "plist".to_string(),
                source: Box::new(e),
            })?;
        let plist_val: plist::Value =
            plist::from_bytes(&buf).map_err(|e| crate::error::DkitError::ParseError {
                format: "plist".to_string(),
                source: Box::new(e),
            })?;
        Ok(from_plist_value(plist_val))
    }
}

/// macOS Property List (plist) format writer.
///
/// Writes data as XML plist format.
pub struct PlistWriter;

impl FormatWriter for PlistWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let plist_val = to_plist_value(value);
        let mut buf = Vec::new();
        plist::to_writer_xml(&mut buf, &plist_val).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "plist".to_string(),
                source: Box::new(e),
            }
        })?;
        Ok(
            String::from_utf8(buf).map_err(|e| crate::error::DkitError::WriteError {
                format: "plist".to_string(),
                source: Box::new(e),
            })?,
        )
    }

    fn write_to_writer(&self, value: &Value, writer: impl Write) -> anyhow::Result<()> {
        let plist_val = to_plist_value(value);
        plist::to_writer_xml(writer, &plist_val).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "plist".to_string(),
                source: Box::new(e),
            }
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_plist_value_bool() {
        assert_eq!(
            from_plist_value(plist::Value::Boolean(true)),
            Value::Bool(true)
        );
        assert_eq!(
            from_plist_value(plist::Value::Boolean(false)),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_from_plist_value_integer() {
        assert_eq!(
            from_plist_value(plist::Value::Integer(42.into())),
            Value::Integer(42)
        );
        assert_eq!(
            from_plist_value(plist::Value::Integer((-10).into())),
            Value::Integer(-10)
        );
    }

    #[test]
    fn test_from_plist_value_real() {
        assert_eq!(
            from_plist_value(plist::Value::Real(3.14)),
            Value::Float(3.14)
        );
    }

    #[test]
    fn test_from_plist_value_string() {
        assert_eq!(
            from_plist_value(plist::Value::String("hello".to_string())),
            Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_from_plist_value_array() {
        let arr = plist::Value::Array(vec![
            plist::Value::Integer(1.into()),
            plist::Value::String("two".to_string()),
        ]);
        let expected = Value::Array(vec![Value::Integer(1), Value::String("two".to_string())]);
        assert_eq!(from_plist_value(arr), expected);
    }

    #[test]
    fn test_from_plist_value_dict() {
        let mut dict = plist::Dictionary::new();
        dict.insert(
            "name".to_string(),
            plist::Value::String("Alice".to_string()),
        );
        dict.insert("age".to_string(), plist::Value::Integer(30.into()));
        let result = from_plist_value(plist::Value::Dictionary(dict));
        match result {
            Value::Object(map) => {
                assert_eq!(map.get("name"), Some(&Value::String("Alice".to_string())));
                assert_eq!(map.get("age"), Some(&Value::Integer(30)));
            }
            _ => panic!("Expected Object"),
        }
    }

    #[test]
    fn test_from_plist_value_data() {
        let data = plist::Value::Data(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"
        let result = from_plist_value(data);
        // Base64 of "Hello" = "SGVsbG8="
        assert_eq!(result, Value::String("SGVsbG8=".to_string()));
    }

    #[test]
    fn test_to_plist_value_null() {
        let result = to_plist_value(&Value::Null);
        assert_eq!(result, plist::Value::String("".to_string()));
    }

    #[test]
    fn test_to_plist_value_bool() {
        assert_eq!(
            to_plist_value(&Value::Bool(true)),
            plist::Value::Boolean(true)
        );
    }

    #[test]
    fn test_to_plist_value_integer() {
        assert_eq!(
            to_plist_value(&Value::Integer(42)),
            plist::Value::Integer(42.into())
        );
    }

    #[test]
    fn test_to_plist_value_float() {
        assert_eq!(to_plist_value(&Value::Float(2.5)), plist::Value::Real(2.5));
    }

    #[test]
    fn test_to_plist_value_string() {
        assert_eq!(
            to_plist_value(&Value::String("test".to_string())),
            plist::Value::String("test".to_string())
        );
    }

    #[test]
    fn test_to_plist_value_array() {
        let val = Value::Array(vec![Value::Integer(1), Value::Bool(true)]);
        let result = to_plist_value(&val);
        let expected = plist::Value::Array(vec![
            plist::Value::Integer(1.into()),
            plist::Value::Boolean(true),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_plist_value_object() {
        let mut map = IndexMap::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let val = Value::Object(map);
        let result = to_plist_value(&val);
        match result {
            plist::Value::Dictionary(dict) => {
                assert_eq!(
                    dict.get("key"),
                    Some(&plist::Value::String("value".to_string()))
                );
            }
            _ => panic!("Expected Dictionary"),
        }
    }

    #[test]
    fn test_reader_xml_plist() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>MyApp</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
</dict>
</plist>"#;
        let reader = PlistReader;
        let result = reader.read(xml).unwrap();
        match &result {
            Value::Object(map) => {
                assert_eq!(
                    map.get("CFBundleName"),
                    Some(&Value::String("MyApp".to_string()))
                );
                assert_eq!(
                    map.get("CFBundleVersion"),
                    Some(&Value::String("1.0".to_string()))
                );
                assert_eq!(map.get("LSRequiresIPhoneOS"), Some(&Value::Bool(true)));
            }
            _ => panic!("Expected Object"),
        }
    }

    #[test]
    fn test_writer_xml_plist() {
        let mut map = IndexMap::new();
        map.insert("name".to_string(), Value::String("Test".to_string()));
        map.insert("version".to_string(), Value::Integer(1));
        let val = Value::Object(map);

        let writer = PlistWriter;
        let output = writer.write(&val).unwrap();
        assert!(output.contains("<?xml version=\"1.0\""));
        assert!(output.contains("<key>name</key>"));
        assert!(output.contains("<string>Test</string>"));
        assert!(output.contains("<key>version</key>"));
        assert!(output.contains("<integer>1</integer>"));
    }

    #[test]
    fn test_roundtrip_plist() {
        let mut map = IndexMap::new();
        map.insert("string_val".to_string(), Value::String("hello".to_string()));
        map.insert("int_val".to_string(), Value::Integer(42));
        map.insert("float_val".to_string(), Value::Float(3.14));
        map.insert("bool_val".to_string(), Value::Bool(true));
        map.insert(
            "array_val".to_string(),
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
        );
        let original = Value::Object(map);

        let writer = PlistWriter;
        let xml = writer.write(&original).unwrap();

        let reader = PlistReader;
        let restored = reader.read(&xml).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_reader_nested_plist() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>outer</key>
    <dict>
        <key>inner</key>
        <array>
            <integer>1</integer>
            <integer>2</integer>
            <integer>3</integer>
        </array>
    </dict>
</dict>
</plist>"#;
        let reader = PlistReader;
        let result = reader.read(xml).unwrap();
        match &result {
            Value::Object(outer) => match outer.get("outer") {
                Some(Value::Object(inner_map)) => match inner_map.get("inner") {
                    Some(Value::Array(arr)) => {
                        assert_eq!(arr.len(), 3);
                        assert_eq!(arr[0], Value::Integer(1));
                    }
                    _ => panic!("Expected Array for 'inner'"),
                },
                _ => panic!("Expected Object for 'outer'"),
            },
            _ => panic!("Expected Object"),
        }
    }

    #[test]
    fn test_reader_invalid_plist() {
        let reader = PlistReader;
        let result = reader.read("not a plist");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_reader() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>test</key>
    <string>value</string>
</dict>
</plist>"#;
        let reader = PlistReader;
        let cursor = std::io::Cursor::new(xml.as_bytes());
        let result = reader.read_from_reader(cursor).unwrap();
        match &result {
            Value::Object(map) => {
                assert_eq!(map.get("test"), Some(&Value::String("value".to_string())));
            }
            _ => panic!("Expected Object"),
        }
    }

    #[test]
    fn test_write_to_writer() {
        let val = Value::Object(IndexMap::from([(
            "key".to_string(),
            Value::String("val".to_string()),
        )]));
        let writer = PlistWriter;
        let mut buf = Vec::new();
        writer.write_to_writer(&val, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("<key>key</key>"));
        assert!(output.contains("<string>val</string>"));
    }
}
