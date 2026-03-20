use std::io::{Read, Write};

use indexmap::IndexMap;

use crate::format::{FormatOptions, FormatReader, FormatWriter};
use crate::value::Value;

/// CSV 문자열 값을 적절한 Value 타입으로 변환
/// 숫자 패턴이면 Integer/Float로, 그 외는 String으로 변환
fn infer_value(s: &str) -> Value {
    if s.is_empty() {
        return Value::Null;
    }
    if let Ok(i) = s.parse::<i64>() {
        return Value::Integer(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        if f.is_finite() {
            return Value::Float(f);
        }
    }
    Value::String(s.to_string())
}

/// CSV 포맷 Reader
#[derive(Default)]
pub struct CsvReader {
    options: FormatOptions,
}

impl CsvReader {
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }
}

impl CsvReader {
    fn build_reader<R: Read>(&self, rdr: R) -> csv::Reader<R> {
        let delimiter = self.options.delimiter.unwrap_or(',') as u8;
        csv::ReaderBuilder::new()
            .has_headers(!self.options.no_header)
            .delimiter(delimiter)
            .from_reader(rdr)
    }

    fn records_to_value<R: Read>(&self, mut rdr: csv::Reader<R>) -> anyhow::Result<Value> {
        let headers: Vec<String> = if self.options.no_header {
            // 헤더 없는 모드: 첫 레코드를 읽어서 컬럼 수 파악
            // csv crate가 has_headers(false)이면 헤더를 자동으로 건너뛰지 않음
            Vec::new()
        } else {
            rdr.headers()
                .map_err(|e| crate::error::DkitError::ParseError {
                    format: "CSV".to_string(),
                    source: Box::new(e),
                })?
                .iter()
                .map(|h| h.to_string())
                .collect()
        };

        let mut rows = Vec::new();

        for result in rdr.records() {
            let record = result.map_err(|e| crate::error::DkitError::ParseError {
                format: "CSV".to_string(),
                source: Box::new(e),
            })?;

            let col_names: Vec<String> = if self.options.no_header {
                (0..record.len()).map(|i| format!("col{i}")).collect()
            } else {
                headers.clone()
            };

            let mut obj = IndexMap::new();
            for (i, field) in record.iter().enumerate() {
                let key = col_names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("col{i}"));
                obj.insert(key, infer_value(field));
            }
            rows.push(Value::Object(obj));
        }

        Ok(Value::Array(rows))
    }
}

impl FormatReader for CsvReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let rdr = self.build_reader(input.as_bytes());
        self.records_to_value(rdr)
    }

    fn read_from_reader(&self, reader: impl Read) -> anyhow::Result<Value> {
        let rdr = self.build_reader(reader);
        self.records_to_value(rdr)
    }
}

/// CSV 포맷 Writer
#[derive(Default)]
pub struct CsvWriter {
    options: FormatOptions,
}

impl CsvWriter {
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }
}

impl CsvWriter {
    /// 모든 오브젝트에서 고유 키를 순서 보존하며 수집
    fn collect_headers(rows: &[Value]) -> Vec<String> {
        let mut headers = IndexMap::new();
        for row in rows {
            if let Value::Object(obj) = row {
                for key in obj.keys() {
                    headers.entry(key.clone()).or_insert(());
                }
            }
        }
        headers.into_keys().collect()
    }

    /// Value를 CSV 셀 문자열로 변환
    fn value_to_field(v: &Value) -> String {
        match v {
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(a) => {
                // 배열은 JSON 형태로 직렬화
                let parts: Vec<String> = a.iter().map(|v| format!("{v}")).collect();
                format!("[{}]", parts.join(", "))
            }
            Value::Object(o) => {
                // 오브젝트는 JSON 형태로 직렬화
                let parts: Vec<String> = o.iter().map(|(k, v)| format!("\"{k}\": {v}")).collect();
                format!("{{{}}}", parts.join(", "))
            }
        }
    }

    fn write_csv_to<W: Write>(&self, value: &Value, writer: W) -> anyhow::Result<()> {
        let rows = match value {
            Value::Array(arr) => arr,
            _ => {
                return Err(crate::error::DkitError::WriteError {
                    format: "CSV".to_string(),
                    source: "CSV output requires an Array of Objects".into(),
                }
                .into());
            }
        };

        let delimiter = self.options.delimiter.unwrap_or(',') as u8;
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(writer);

        let headers = Self::collect_headers(rows);

        // 헤더 쓰기
        if !self.options.no_header {
            wtr.write_record(&headers)
                .map_err(|e| crate::error::DkitError::WriteError {
                    format: "CSV".to_string(),
                    source: Box::new(e),
                })?;
        }

        // 데이터 행 쓰기
        for row in rows {
            let fields: Vec<String> = if let Value::Object(obj) = row {
                headers
                    .iter()
                    .map(|h| obj.get(h).map(Self::value_to_field).unwrap_or_default())
                    .collect()
            } else {
                // 오브젝트가 아닌 경우 단일 필드로 처리
                vec![Self::value_to_field(row)]
            };

            wtr.write_record(&fields)
                .map_err(|e| crate::error::DkitError::WriteError {
                    format: "CSV".to_string(),
                    source: Box::new(e),
                })?;
        }

        wtr.flush()
            .map_err(|e| crate::error::DkitError::WriteError {
                format: "CSV".to_string(),
                source: Box::new(e),
            })?;

        Ok(())
    }
}

impl FormatWriter for CsvWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        self.write_csv_to(value, &mut buf)?;
        String::from_utf8(buf).map_err(|e| {
            crate::error::DkitError::WriteError {
                format: "CSV".to_string(),
                source: Box::new(e),
            }
            .into()
        })
    }

    fn write_to_writer(&self, value: &Value, writer: impl Write) -> anyhow::Result<()> {
        self.write_csv_to(value, writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- infer_value 테스트 ---

    #[test]
    fn test_infer_integer() {
        assert_eq!(infer_value("42"), Value::Integer(42));
        assert_eq!(infer_value("-7"), Value::Integer(-7));
        assert_eq!(infer_value("0"), Value::Integer(0));
    }

    #[test]
    fn test_infer_float() {
        assert_eq!(infer_value("3.14"), Value::Float(3.14));
        assert_eq!(infer_value("-0.5"), Value::Float(-0.5));
    }

    #[test]
    fn test_infer_string() {
        assert_eq!(infer_value("hello"), Value::String("hello".to_string()));
        assert_eq!(infer_value("true"), Value::String("true".to_string()));
    }

    #[test]
    fn test_infer_empty() {
        assert_eq!(infer_value(""), Value::Null);
    }

    // --- CsvReader 기본 테스트 ---

    #[test]
    fn test_read_simple_csv() {
        let reader = CsvReader::default();
        let input = "name,age,city\nAlice,30,Seoul\nBob,25,Busan\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(row0.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(row0.get("age"), Some(&Value::Integer(30)));
        assert_eq!(row0.get("city"), Some(&Value::String("Seoul".to_string())));

        let row1 = arr[1].as_object().unwrap();
        assert_eq!(row1.get("name"), Some(&Value::String("Bob".to_string())));
        assert_eq!(row1.get("age"), Some(&Value::Integer(25)));
        assert_eq!(row1.get("city"), Some(&Value::String("Busan".to_string())));
    }

    #[test]
    fn test_read_no_header() {
        let reader = CsvReader::new(FormatOptions {
            no_header: true,
            ..Default::default()
        });
        let input = "Alice,30,Seoul\nBob,25,Busan\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(row0.get("col0"), Some(&Value::String("Alice".to_string())));
        assert_eq!(row0.get("col1"), Some(&Value::Integer(30)));
        assert_eq!(row0.get("col2"), Some(&Value::String("Seoul".to_string())));
    }

    #[test]
    fn test_read_custom_delimiter() {
        let reader = CsvReader::new(FormatOptions {
            delimiter: Some('\t'),
            ..Default::default()
        });
        let input = "name\tage\nAlice\t30\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 1);

        let row = arr[0].as_object().unwrap();
        assert_eq!(row.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(row.get("age"), Some(&Value::Integer(30)));
    }

    #[test]
    fn test_read_quoted_fields() {
        let reader = CsvReader::default();
        let input = "name,description\nAlice,\"Hello, World!\"\nBob,\"He said \"\"hi\"\"\"\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(
            row0.get("description"),
            Some(&Value::String("Hello, World!".to_string()))
        );

        let row1 = arr[1].as_object().unwrap();
        assert_eq!(
            row1.get("description"),
            Some(&Value::String("He said \"hi\"".to_string()))
        );
    }

    #[test]
    fn test_read_unicode() {
        let reader = CsvReader::default();
        let input = "이름,도시\n김철수,서울\n이영희,부산\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(row0.get("이름"), Some(&Value::String("김철수".to_string())));
        assert_eq!(row0.get("도시"), Some(&Value::String("서울".to_string())));
    }

    #[test]
    fn test_read_emoji() {
        let reader = CsvReader::default();
        let input = "name,emoji\nAlice,🎉\nBob,🚀\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(row0.get("emoji"), Some(&Value::String("🎉".to_string())));
    }

    #[test]
    fn test_read_empty_csv() {
        let reader = CsvReader::default();
        let input = "name,age\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_read_empty_fields() {
        let reader = CsvReader::default();
        let input = "name,age,city\nAlice,,Seoul\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();

        let row = arr[0].as_object().unwrap();
        assert_eq!(row.get("age"), Some(&Value::Null));
    }

    #[test]
    fn test_read_float_values() {
        let reader = CsvReader::default();
        let input = "name,score\nAlice,98.5\nBob,87.3\n";
        let v = reader.read(input).unwrap();
        let arr = v.as_array().unwrap();

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(row0.get("score"), Some(&Value::Float(98.5)));
    }

    #[test]
    fn test_read_from_reader() {
        let reader = CsvReader::default();
        let input = b"name,age\nAlice,30\n";
        let v = reader.read_from_reader(input.as_slice()).unwrap();
        let arr = v.as_array().unwrap();
        assert_eq!(arr.len(), 1);
    }

    // --- CsvWriter 테스트 ---

    #[test]
    fn test_write_simple() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert("age".to_string(), Value::Integer(30));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert("age".to_string(), Value::Integer(25));
                m
            }),
        ]);

        let output = writer.write(&value).unwrap();
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines[0], "name,age");
        assert_eq!(lines[1], "Alice,30");
        assert_eq!(lines[2], "Bob,25");
    }

    #[test]
    fn test_write_no_header() {
        let writer = CsvWriter::new(FormatOptions {
            no_header: true,
            ..Default::default()
        });
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m.insert("age".to_string(), Value::Integer(30));
            m
        })]);

        let output = writer.write(&value).unwrap();
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Alice,30");
    }

    #[test]
    fn test_write_custom_delimiter() {
        let writer = CsvWriter::new(FormatOptions {
            delimiter: Some('\t'),
            ..Default::default()
        });
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m.insert("age".to_string(), Value::Integer(30));
            m
        })]);

        let output = writer.write(&value).unwrap();
        assert!(output.contains("name\tage"));
        assert!(output.contains("Alice\t30"));
    }

    #[test]
    fn test_write_quoted_fields() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "desc".to_string(),
                Value::String("Hello, World!".to_string()),
            );
            m
        })]);

        let output = writer.write(&value).unwrap();
        assert!(output.contains("\"Hello, World!\""));
    }

    #[test]
    fn test_write_null_values() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m.insert("age".to_string(), Value::Null);
            m
        })]);

        let output = writer.write(&value).unwrap();
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines[1], "Alice,");
    }

    #[test]
    fn test_write_unicode() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert("이름".to_string(), Value::String("김철수".to_string()));
            m.insert("도시".to_string(), Value::String("서울".to_string()));
            m
        })]);

        let output = writer.write(&value).unwrap();
        assert!(output.contains("이름"));
        assert!(output.contains("김철수"));
    }

    #[test]
    fn test_write_non_array_error() {
        let writer = CsvWriter::default();
        let result = writer.write(&Value::Object(IndexMap::new()));
        assert!(result.is_err());
    }

    #[test]
    fn test_write_to_writer() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert("x".to_string(), Value::Integer(1));
            m
        })]);

        let mut buf = Vec::new();
        writer.write_to_writer(&value, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("x\n1\n") || output.contains("x\r\n1\r\n"));
    }

    #[test]
    fn test_write_missing_keys() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("a".to_string(), Value::Integer(1));
                m.insert("b".to_string(), Value::Integer(2));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("a".to_string(), Value::Integer(3));
                // b is missing
                m
            }),
        ]);

        let output = writer.write(&value).unwrap();
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines[0], "a,b");
        assert_eq!(lines[1], "1,2");
        assert_eq!(lines[2], "3,");
    }

    // --- 왕복 변환 테스트 ---

    #[test]
    fn test_roundtrip() {
        let input = "name,age,score\nAlice,30,98.5\nBob,25,87.3\n";
        let reader = CsvReader::default();
        let writer = CsvWriter::default();

        let value = reader.read(input).unwrap();
        let output = writer.write(&value).unwrap();
        let value2 = reader.read(&output).unwrap();

        assert_eq!(value, value2);
    }

    #[test]
    fn test_empty_array_write() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![]);
        // 빈 배열은 헤더도 데이터도 없으므로 빈 출력
        let output = writer.write(&value).unwrap();
        // csv crate may add BOM or trailing bytes; just check no data rows
        let reader = CsvReader::default();
        let value2 = reader.read(&output).unwrap();
        assert_eq!(value2.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_write_bool_values() {
        let writer = CsvWriter::default();
        let value = Value::Array(vec![Value::Object({
            let mut m = IndexMap::new();
            m.insert("flag".to_string(), Value::Bool(true));
            m
        })]);

        let output = writer.write(&value).unwrap();
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines[1], "true");
    }
}
