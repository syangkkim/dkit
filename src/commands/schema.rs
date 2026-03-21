use std::io::{self, Read};
use std::path::Path;

use crate::format::csv::CsvReader;
use crate::format::json::JsonReader;
use crate::format::toml::TomlReader;
use crate::format::xml::XmlReader;
use crate::format::yaml::YamlReader;
use crate::format::{
    default_delimiter, default_delimiter_for_format, detect_format, Format, FormatOptions,
    FormatReader,
};
use crate::value::Value;
use anyhow::{bail, Result};

pub struct SchemaArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
}

pub fn run(args: &SchemaArgs) -> Result<()> {
    let (content, source_format) = read_input(args)?;
    let auto_delimiter = if args.input == "-" {
        args.from.and_then(default_delimiter_for_format)
    } else {
        default_delimiter(Path::new(args.input))
    };
    let options = FormatOptions {
        delimiter: auto_delimiter,
        ..Default::default()
    };
    let value = read_value(&content, source_format, &options)?;

    let mut output = String::new();
    format_schema(&value, "", true, &mut output);
    print!("{output}");

    Ok(())
}

/// Schema 타입 이름 반환
fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// 배열 요소들의 통합 타입 이름 반환
fn array_element_type(arr: &[Value]) -> String {
    if arr.is_empty() {
        return "empty".to_string();
    }

    // 모든 요소의 타입을 수집 (중복 제거, 순서 유지)
    let mut types = Vec::new();
    for item in arr {
        let t = type_name(item);
        if !types.contains(&t) {
            types.push(t);
        }
    }

    if types.len() == 1 {
        types[0].to_string()
    } else {
        types.join("|")
    }
}

/// 배열 내 object들의 통합 스키마 생성
fn merge_object_schemas(arr: &[Value]) -> indexmap::IndexMap<String, Value> {
    let mut merged = indexmap::IndexMap::new();
    for item in arr {
        if let Value::Object(obj) = item {
            for (key, val) in obj {
                merged.entry(key.clone()).or_insert_with(|| val.clone());
            }
        }
    }
    merged
}

/// 트리 형태 스키마 출력
fn format_schema(value: &Value, prefix: &str, is_root: bool, output: &mut String) {
    if is_root {
        match value {
            Value::Object(obj) => {
                output.push_str("root: object\n");
                format_object_fields(obj, prefix, output);
            }
            Value::Array(arr) => {
                let elem_type = array_element_type(arr);
                output.push_str(&format!("root: array[{elem_type}]\n"));
                format_array_children(arr, prefix, output);
            }
            _ => {
                output.push_str(&format!("root: {}\n", type_name(value)));
            }
        }
    }
}

/// object의 필드들을 트리로 출력
fn format_object_fields(
    obj: &indexmap::IndexMap<String, Value>,
    prefix: &str,
    output: &mut String,
) {
    let entries: Vec<_> = obj.iter().collect();
    let count = entries.len();

    for (i, (key, val)) in entries.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└─" } else { "├─" };
        let child_prefix = if is_last {
            format!("{prefix}   ")
        } else {
            format!("{prefix}│  ")
        };

        match val {
            Value::Object(inner_obj) => {
                output.push_str(&format!("{prefix}{connector} {key}: object\n"));
                format_object_fields(inner_obj, &child_prefix, output);
            }
            Value::Array(arr) => {
                let elem_type = array_element_type(arr);
                output.push_str(&format!("{prefix}{connector} {key}: array[{elem_type}]\n"));
                format_array_children(arr, &child_prefix, output);
            }
            _ => {
                output.push_str(&format!("{prefix}{connector} {key}: {}\n", type_name(val)));
            }
        }
    }
}

/// 배열 내 object 요소의 통합 스키마를 자식으로 출력
fn format_array_children(arr: &[Value], prefix: &str, output: &mut String) {
    // object 배열인 경우 통합 스키마 출력
    let has_objects = arr.iter().any(|v| matches!(v, Value::Object(_)));
    if has_objects {
        let merged = merge_object_schemas(arr);
        format_object_fields(&merged, prefix, output);
    }
    // 중첩 배열인 경우 첫 번째 비어있지 않은 배열의 구조를 출력
    let has_arrays = arr.iter().any(|v| matches!(v, Value::Array(_)));
    if has_arrays && !has_objects {
        if let Some(Value::Array(inner)) = arr.iter().find(|v| matches!(v, Value::Array(_))) {
            let elem_type = array_element_type(inner);
            output.push_str(&format!("{prefix}└─ []: array[{elem_type}]\n"));
            format_array_children(inner, &format!("{prefix}   "), output);
        }
    }
}

fn read_input(args: &SchemaArgs) -> Result<(String, Format)> {
    if args.input == "-" {
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => bail!(
                "--from is required when reading from stdin\n  Hint: specify the input format, e.g. --from json"
            ),
        };
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
        Ok((buf, format))
    } else {
        let path = Path::new(args.input);
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(path)?,
        };
        let content = super::read_file(path)?;
        Ok((content, format))
    }
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
        Format::Xml => XmlReader.read(content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn make_object(fields: Vec<(&str, Value)>) -> Value {
        let mut map = IndexMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v);
        }
        Value::Object(map)
    }

    #[test]
    fn test_type_name() {
        assert_eq!(type_name(&Value::Null), "null");
        assert_eq!(type_name(&Value::Bool(true)), "boolean");
        assert_eq!(type_name(&Value::Integer(42)), "integer");
        assert_eq!(type_name(&Value::Float(3.14)), "float");
        assert_eq!(type_name(&Value::String("hi".into())), "string");
        assert_eq!(type_name(&Value::Array(vec![])), "array");
        assert_eq!(type_name(&Value::Object(IndexMap::new())), "object");
    }

    #[test]
    fn test_array_element_type_uniform() {
        let arr = vec![Value::Integer(1), Value::Integer(2)];
        assert_eq!(array_element_type(&arr), "integer");
    }

    #[test]
    fn test_array_element_type_mixed() {
        let arr = vec![Value::Integer(1), Value::String("a".into())];
        assert_eq!(array_element_type(&arr), "integer|string");
    }

    #[test]
    fn test_array_element_type_empty() {
        assert_eq!(array_element_type(&[]), "empty");
    }

    #[test]
    fn test_simple_object_schema() {
        let value = make_object(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
        ]);

        let mut output = String::new();
        format_schema(&value, "", true, &mut output);
        assert_eq!(output, "root: object\n├─ host: string\n└─ port: integer\n");
    }

    #[test]
    fn test_nested_object_schema() {
        let inner = make_object(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
        ]);
        let value = make_object(vec![("database", inner), ("debug", Value::Bool(false))]);

        let mut output = String::new();
        format_schema(&value, "", true, &mut output);
        assert_eq!(
            output,
            "root: object\n\
             ├─ database: object\n\
             │  ├─ host: string\n\
             │  └─ port: integer\n\
             └─ debug: boolean\n"
        );
    }

    #[test]
    fn test_array_of_objects_schema() {
        let users = Value::Array(vec![
            make_object(vec![
                ("name", Value::String("Alice".into())),
                ("email", Value::String("alice@example.com".into())),
            ]),
            make_object(vec![
                ("name", Value::String("Bob".into())),
                ("email", Value::String("bob@example.com".into())),
            ]),
        ]);
        let value = make_object(vec![("users", users)]);

        let mut output = String::new();
        format_schema(&value, "", true, &mut output);
        assert_eq!(
            output,
            "root: object\n\
             └─ users: array[object]\n   \
                ├─ name: string\n   \
                └─ email: string\n"
        );
    }

    #[test]
    fn test_root_array_schema() {
        let value = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let mut output = String::new();
        format_schema(&value, "", true, &mut output);
        assert_eq!(output, "root: array[integer]\n");
    }

    #[test]
    fn test_primitive_root_schema() {
        let mut output = String::new();
        format_schema(&Value::String("hello".into()), "", true, &mut output);
        assert_eq!(output, "root: string\n");
    }

    #[test]
    fn test_full_example_from_spec() {
        // Matches the example in cli-spec.md
        let database = make_object(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
            ("name", Value::String("mydb".into())),
        ]);
        let server = make_object(vec![
            ("port", Value::Integer(8080)),
            ("debug", Value::Bool(true)),
        ]);
        let users = Value::Array(vec![make_object(vec![
            ("name", Value::String("Alice".into())),
            ("email", Value::String("alice@example.com".into())),
        ])]);
        let value = make_object(vec![
            ("database", database),
            ("server", server),
            ("users", users),
        ]);

        let mut output = String::new();
        format_schema(&value, "", true, &mut output);
        assert_eq!(
            output,
            "root: object\n\
             ├─ database: object\n\
             │  ├─ host: string\n\
             │  ├─ port: integer\n\
             │  └─ name: string\n\
             ├─ server: object\n\
             │  ├─ port: integer\n\
             │  └─ debug: boolean\n\
             └─ users: array[object]\n   \
                ├─ name: string\n   \
                └─ email: string\n"
        );
    }
}
