use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::read_file;
use crate::format::csv::{CsvReader, CsvWriter};
use crate::format::json::{JsonReader, JsonWriter};
use crate::format::toml::{TomlReader, TomlWriter};
use crate::format::xml::{XmlReader, XmlWriter};
use crate::format::yaml::{YamlReader, YamlWriter};
use crate::format::{
    default_delimiter, default_delimiter_for_format, detect_format, Format, FormatOptions,
    FormatReader, FormatWriter,
};
use crate::value::Value;

pub struct MergeArgs<'a> {
    pub input: &'a [PathBuf],
    pub to: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub pretty: bool,
    pub compact: bool,
    pub flow: bool,
}

/// merge 서브커맨드 실행
pub fn run(args: &MergeArgs) -> Result<()> {
    if args.input.len() < 2 {
        bail!("merge requires at least 2 input files\n  Hint: dkit merge file1.json file2.json --to json");
    }

    // 각 파일을 Value로 읽기
    let mut values = Vec::with_capacity(args.input.len());
    for path in args.input {
        let format = detect_format(path)?;
        let read_delimiter = args.delimiter.or_else(|| default_delimiter(path));
        let read_options = FormatOptions {
            delimiter: read_delimiter,
            no_header: args.no_header,
            ..Default::default()
        };
        let content = read_file(path)?;
        let value = read_value(&content, format, &read_options)?;
        values.push(value);
    }

    // 값 합치기
    let merged = merge_values(values)?;

    // 출력 포맷 결정: --to > 출력 파일 확장자 > 첫 번째 입력 파일 확장자
    let target_format = match args.to {
        Some(f) => Format::from_str(f)?,
        None => match args.output {
            Some(p) => detect_format(p).unwrap_or_else(|_| detect_format(&args.input[0]).unwrap()),
            None => detect_format(&args.input[0])?,
        },
    };

    let write_delimiter = args
        .delimiter
        .or_else(|| args.to.and_then(default_delimiter_for_format));
    let write_options = FormatOptions {
        delimiter: write_delimiter,
        no_header: args.no_header,
        pretty: if args.compact {
            false
        } else {
            args.pretty || !args.compact
        },
        compact: args.compact,
        flow_style: args.flow,
    };

    let result = write_value(&merged, target_format, &write_options)?;

    if let Some(out_path) = args.output {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory {}", parent.display()))?;
            }
        }
        fs::write(out_path, &result)
            .with_context(|| format!("Failed to write to {}", out_path.display()))?;
    } else {
        print!("{result}");
    }

    Ok(())
}

/// 여러 Value를 하나로 합치기
/// - 모두 배열: 배열을 concat
/// - 모두 오브젝트: 키를 merge (뒤 파일이 우선)
/// - 혼합: 모든 값을 하나의 배열로 합치기
fn merge_values(values: Vec<Value>) -> Result<Value> {
    if values.is_empty() {
        return Ok(Value::Array(vec![]));
    }

    let all_arrays = values.iter().all(|v| matches!(v, Value::Array(_)));
    let all_objects = values.iter().all(|v| matches!(v, Value::Object(_)));

    if all_arrays {
        // 배열 concat
        let mut merged = Vec::new();
        for v in values {
            if let Value::Array(arr) = v {
                merged.extend(arr);
            }
        }
        Ok(Value::Array(merged))
    } else if all_objects {
        // 오브젝트 merge (뒤 파일이 우선)
        let mut merged = indexmap::IndexMap::new();
        for v in values {
            if let Value::Object(map) = v {
                for (k, val) in map {
                    merged.insert(k, val);
                }
            }
        }
        Ok(Value::Object(merged))
    } else {
        // 혼합: 배열은 펼치고 나머지는 그대로 추가
        let mut merged = Vec::new();
        for v in values {
            match v {
                Value::Array(arr) => merged.extend(arr),
                other => merged.push(other),
            }
        }
        Ok(Value::Array(merged))
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

fn write_value(value: &Value, format: Format, options: &FormatOptions) -> Result<String> {
    match format {
        Format::Json => JsonWriter::new(options.clone()).write(value),
        Format::Csv => CsvWriter::new(options.clone()).write(value),
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty).write(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_arrays() {
        let values = vec![
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Array(vec![Value::Integer(3), Value::Integer(4)]),
        ];
        let result = merge_values(values).unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
                Value::Integer(4),
            ])
        );
    }

    #[test]
    fn test_merge_objects() {
        let mut map1 = indexmap::IndexMap::new();
        map1.insert("a".to_string(), Value::Integer(1));
        map1.insert("b".to_string(), Value::Integer(2));

        let mut map2 = indexmap::IndexMap::new();
        map2.insert("b".to_string(), Value::Integer(99));
        map2.insert("c".to_string(), Value::Integer(3));

        let values = vec![Value::Object(map1), Value::Object(map2)];
        let result = merge_values(values).unwrap();

        if let Value::Object(map) = result {
            assert_eq!(map.get("a"), Some(&Value::Integer(1)));
            assert_eq!(map.get("b"), Some(&Value::Integer(99))); // 뒤 파일 우선
            assert_eq!(map.get("c"), Some(&Value::Integer(3)));
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_merge_mixed() {
        let mut map = indexmap::IndexMap::new();
        map.insert("key".to_string(), Value::String("val".to_string()));

        let values = vec![
            Value::Array(vec![Value::Integer(1)]),
            Value::Object(map.clone()),
        ];
        let result = merge_values(values).unwrap();
        assert_eq!(
            result,
            Value::Array(vec![Value::Integer(1), Value::Object(map)])
        );
    }

    #[test]
    fn test_merge_empty() {
        let result = merge_values(vec![]).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }
}
