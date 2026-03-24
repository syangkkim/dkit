use std::fmt;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            Value::String(s) => write!(f, "{s}"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match v {
                        Value::String(s) => write!(f, "\"{s}\"")?,
                        _ => write!(f, "{v}")?,
                    }
                }
                write!(f, "]")
            }
            Value::Object(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match v {
                        Value::String(s) => write!(f, "\"{k}\": \"{s}\"")?,
                        _ => write!(f, "\"{k}\": {v}")?,
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

#[allow(dead_code)]
impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_accessors() {
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Integer(42).as_i64(), Some(42));
        assert_eq!(Value::Float(3.14).as_f64(), Some(3.14));
        assert_eq!(Value::Integer(42).as_f64(), Some(42.0));
        assert_eq!(Value::String("hello".into()).as_str(), Some("hello"));
        assert!(Value::Null.is_null());
        assert!(!Value::Bool(false).is_null());
    }

    #[test]
    fn test_value_array() {
        let arr = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        assert_eq!(arr.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_value_object() {
        let mut map = IndexMap::new();
        map.insert("key".to_string(), Value::String("value".into()));
        let obj = Value::Object(map);
        assert_eq!(
            obj.as_object().unwrap().get("key"),
            Some(&Value::String("value".into()))
        );
    }

    #[test]
    fn test_display_primitives() {
        assert_eq!(Value::Null.to_string(), "null");
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Bool(false).to_string(), "false");
        assert_eq!(Value::Integer(42).to_string(), "42");
        assert_eq!(Value::Float(3.14).to_string(), "3.14");
        assert_eq!(Value::Float(1.0).to_string(), "1.0");
        assert_eq!(Value::String("hello".into()).to_string(), "hello");
    }

    #[test]
    fn test_display_array() {
        let arr = Value::Array(vec![
            Value::Integer(1),
            Value::String("two".into()),
            Value::Bool(true),
        ]);
        assert_eq!(arr.to_string(), r#"[1, "two", true]"#);
    }

    #[test]
    fn test_display_empty_array() {
        assert_eq!(Value::Array(vec![]).to_string(), "[]");
    }

    #[test]
    fn test_display_object() {
        let mut map = IndexMap::new();
        map.insert("name".to_string(), Value::String("dkit".into()));
        map.insert("version".to_string(), Value::Integer(1));
        let obj = Value::Object(map);
        assert_eq!(obj.to_string(), r#"{"name": "dkit", "version": 1}"#);
    }

    #[test]
    fn test_display_empty_object() {
        assert_eq!(Value::Object(IndexMap::new()).to_string(), "{}");
    }

    #[test]
    fn test_accessor_returns_none_for_wrong_type() {
        let v = Value::Integer(42);
        assert_eq!(v.as_bool(), None);
        assert_eq!(v.as_str(), None);
        assert_eq!(v.as_array(), None);
        assert_eq!(v.as_object(), None);
    }
}
