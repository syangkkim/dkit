use std::io::{self, Read};
use std::path::Path;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use anyhow::{bail, Result};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::env::EnvReader;
use dkit_core::format::hcl::HclReader;
use dkit_core::format::ini::IniReader;
use dkit_core::format::json::JsonReader;
use dkit_core::format::jsonl::JsonlReader;
use dkit_core::format::msgpack::MsgpackReader;
use dkit_core::format::plist::PlistReader;
use dkit_core::format::properties::PropertiesReader;
use dkit_core::format::toml::TomlReader;
use dkit_core::format::xml::XmlReader;
use dkit_core::format::yaml::YamlReader;
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader,
};
use dkit_core::value::Value;

pub struct SchemaArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
    pub output_format: Option<&'a str>,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

/// 스키마 출력 포맷
enum SchemaOutputFormat {
    Tree,
    Json,
    Yaml,
}

impl SchemaOutputFormat {
    fn from_str_opt(s: Option<&str>) -> anyhow::Result<Self> {
        match s {
            None | Some("tree") | Some("table") | Some("text") => Ok(Self::Tree),
            Some("json") => Ok(Self::Json),
            Some("yaml") | Some("yml") => Ok(Self::Yaml),
            Some(other) => bail!(
                "Unsupported schema output format: '{}'. Use json, yaml, or table",
                other
            ),
        }
    }
}

pub fn run(args: &SchemaArgs) -> Result<()> {
    let value = if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
            MsgpackReader.read_from_bytes(&buf)?
        } else {
            let buf = read_stdin_with_encoding(&args.encoding_opts)?;
            let (source_format, sniffed_delimiter) = match args.from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&buf)?,
            };
            let auto_delimiter =
                sniffed_delimiter.or_else(|| args.from.and_then(default_delimiter_for_format));
            let options = FormatOptions {
                delimiter: auto_delimiter,
                ..Default::default()
            };
            read_value(&buf, source_format, &options)?
        }
    } else {
        let source_format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(Path::new(args.input))?,
        };
        if source_format == Format::Msgpack {
            let bytes = read_file_bytes(Path::new(args.input))?;
            MsgpackReader.read_from_bytes(&bytes)?
        } else if source_format == Format::Xlsx {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_xlsx_from_bytes(&bytes, &args.excel_opts)?
        } else if source_format == Format::Sqlite {
            read_sqlite_from_path(Path::new(args.input), &args.sqlite_opts)?
        } else if source_format == Format::Parquet {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_parquet_from_bytes(&bytes)?
        } else {
            let content = read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?;
            let auto_delimiter = default_delimiter(Path::new(args.input));
            let options = FormatOptions {
                delimiter: auto_delimiter,
                ..Default::default()
            };
            read_value(&content, source_format, &options)?
        }
    };

    let output_format = SchemaOutputFormat::from_str_opt(args.output_format)?;

    match output_format {
        SchemaOutputFormat::Tree => {
            let mut output = String::new();
            format_schema(&value, "", true, &mut output);
            print!("{output}");
        }
        SchemaOutputFormat::Json => {
            let json = build_schema_json(&value);
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        SchemaOutputFormat::Yaml => {
            let json = build_schema_json(&value);
            print!("{}", serde_yaml::to_string(&json)?);
        }
    }

    Ok(())
}

/// Value를 JSON Schema 형식의 serde_json::Value로 변환
fn build_schema_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::json!({"type": "null"}),
        Value::Bool(_) => serde_json::json!({"type": "boolean"}),
        Value::Integer(_) => serde_json::json!({"type": "integer"}),
        Value::Float(_) => serde_json::json!({"type": "float"}),
        Value::String(_) => serde_json::json!({"type": "string"}),
        Value::Array(arr) => {
            if arr.is_empty() {
                serde_json::json!({"type": "array", "items": "empty"})
            } else {
                let elem_type = array_element_type(arr);
                let has_objects = arr.iter().any(|v| matches!(v, Value::Object(_)));
                if has_objects {
                    let merged = merge_object_schemas(arr);
                    let properties = build_object_properties_json(&merged);
                    serde_json::json!({
                        "type": "array",
                        "items": {
                            "type": elem_type,
                            "properties": properties,
                        }
                    })
                } else {
                    serde_json::json!({"type": "array", "items": elem_type})
                }
            }
        }
        Value::Object(obj) => {
            let properties = build_object_properties_json(obj);
            serde_json::json!({
                "type": "object",
                "properties": properties,
            })
        }
        _ => serde_json::json!({"type": "unknown"}),
    }
}

/// Object 필드들을 JSON 프로퍼티로 변환
fn build_object_properties_json(obj: &indexmap::IndexMap<String, Value>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, val) in obj {
        map.insert(key.clone(), build_schema_json(val));
    }
    serde_json::Value::Object(map)
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
        _ => "unknown",
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

/// stdin에서 인코딩을 고려하여 문자열을 읽는다.
fn read_stdin_with_encoding(opts: &EncodingOptions) -> Result<String> {
    if opts.encoding.is_some() || opts.detect_encoding {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
        super::decode_bytes(&buf, opts)
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {e}"))?;
        Ok(buf)
    }
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Jsonl => JsonlReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
        Format::Xml => XmlReader::default().read(content),
        Format::Env => EnvReader.read(content),
        Format::Ini => IniReader.read(content),
        Format::Properties => PropertiesReader.read(content),
        Format::Hcl => HclReader.read(content),
        Format::Plist => PlistReader.read(content),
        Format::Msgpack => MsgpackReader.read(content),
        Format::Xlsx => {
            bail!("Excel files must be read as binary; use file path input instead of stdin")
        }
        Format::Sqlite => {
            bail!("SQLite files must be read from a file path, not from text input")
        }
        Format::Parquet => {
            bail!("Parquet files must be read from a file path, not from text input")
        }
        Format::Markdown => bail!("Markdown is an output-only format and cannot be used as input"),
        Format::Html => bail!("HTML is an output-only format and cannot be used as input"),
        Format::Table => bail!("Table is an output-only format and cannot be used as input"),
        _ => bail!("Unsupported input format: {format}"),
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

    #[test]
    fn test_schema_output_format_parse() {
        assert!(matches!(
            SchemaOutputFormat::from_str_opt(None).unwrap(),
            SchemaOutputFormat::Tree
        ));
        assert!(matches!(
            SchemaOutputFormat::from_str_opt(Some("json")).unwrap(),
            SchemaOutputFormat::Json
        ));
        assert!(matches!(
            SchemaOutputFormat::from_str_opt(Some("yaml")).unwrap(),
            SchemaOutputFormat::Yaml
        ));
        assert!(matches!(
            SchemaOutputFormat::from_str_opt(Some("yml")).unwrap(),
            SchemaOutputFormat::Yaml
        ));
        assert!(matches!(
            SchemaOutputFormat::from_str_opt(Some("table")).unwrap(),
            SchemaOutputFormat::Tree
        ));
        assert!(SchemaOutputFormat::from_str_opt(Some("invalid")).is_err());
    }

    #[test]
    fn test_build_schema_json_simple_object() {
        let value = make_object(vec![
            ("host", Value::String("localhost".into())),
            ("port", Value::Integer(5432)),
        ]);
        let json = build_schema_json(&value);
        assert_eq!(json["type"], "object");
        assert_eq!(json["properties"]["host"]["type"], "string");
        assert_eq!(json["properties"]["port"]["type"], "integer");
    }

    #[test]
    fn test_build_schema_json_nested_object() {
        let inner = make_object(vec![("host", Value::String("localhost".into()))]);
        let value = make_object(vec![("database", inner)]);
        let json = build_schema_json(&value);
        assert_eq!(json["type"], "object");
        assert_eq!(json["properties"]["database"]["type"], "object");
        assert_eq!(
            json["properties"]["database"]["properties"]["host"]["type"],
            "string"
        );
    }

    #[test]
    fn test_build_schema_json_array_of_objects() {
        let value = Value::Array(vec![
            make_object(vec![("name", Value::String("Alice".into()))]),
            make_object(vec![("name", Value::String("Bob".into()))]),
        ]);
        let json = build_schema_json(&value);
        assert_eq!(json["type"], "array");
        assert_eq!(json["items"]["type"], "object");
        assert_eq!(json["items"]["properties"]["name"]["type"], "string");
    }

    #[test]
    fn test_build_schema_json_primitive() {
        assert_eq!(build_schema_json(&Value::Null)["type"], "null");
        assert_eq!(build_schema_json(&Value::Bool(true))["type"], "boolean");
        assert_eq!(build_schema_json(&Value::Integer(42))["type"], "integer");
        assert_eq!(build_schema_json(&Value::Float(3.14))["type"], "float");
        assert_eq!(
            build_schema_json(&Value::String("hi".into()))["type"],
            "string"
        );
    }
}
