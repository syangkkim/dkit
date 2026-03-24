use std::sync::Arc;

use anyhow::bail;
use arrow::array::{
    Array, ArrayRef, AsArray, BinaryArray, BooleanArray, BooleanBuilder, Float32Array,
    Float64Array, Float64Builder, Int16Array, Int32Array, Int64Array, Int64Builder, Int8Array,
    LargeBinaryArray, LargeStringArray, StringArray, StringBuilder, StructArray, UInt16Array,
    UInt32Array, UInt64Array, UInt8Array,
};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use bytes::Bytes;
use indexmap::IndexMap;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ArrowWriter;
use parquet::basic::{Compression, GzipLevel, ZstdLevel};
use parquet::file::properties::WriterProperties;

use crate::error::DkitError;
use crate::value::Value;

/// Parquet 파일 읽기 옵션
#[derive(Debug, Clone, Default)]
pub struct ParquetOptions {
    /// 특정 Row Group만 읽기 (None이면 전체)
    pub row_group: Option<usize>,
}

/// Parquet Reader
pub struct ParquetReader {
    options: ParquetOptions,
}

impl ParquetReader {
    pub fn new(options: ParquetOptions) -> Self {
        Self { options }
    }

    /// 바이트 슬라이스에서 Parquet 파일을 읽어 Value로 변환한다.
    pub fn read_from_bytes(&self, bytes: &[u8]) -> anyhow::Result<Value> {
        let bytes = Bytes::copy_from_slice(bytes);
        let builder =
            ParquetRecordBatchReaderBuilder::try_new(bytes).map_err(|e| DkitError::ParseError {
                format: "Parquet".to_string(),
                source: Box::new(e),
            })?;

        // Row Group 필터링
        let builder = if let Some(rg) = self.options.row_group {
            let metadata = builder.metadata().clone();
            let num_row_groups = metadata.num_row_groups();
            if rg >= num_row_groups {
                bail!(
                    "Row group index {} out of range (file has {} row groups)",
                    rg,
                    num_row_groups
                );
            }
            builder.with_row_groups(vec![rg])
        } else {
            builder
        };

        let reader = builder.build().map_err(|e| DkitError::ParseError {
            format: "Parquet".to_string(),
            source: Box::new(e),
        })?;

        let mut rows: Vec<Value> = Vec::new();

        for batch_result in reader {
            let batch = batch_result.map_err(|e| DkitError::ParseError {
                format: "Parquet".to_string(),
                source: Box::new(e),
            })?;

            let schema = batch.schema();
            let num_rows = batch.num_rows();

            for row_idx in 0..num_rows {
                let mut obj = IndexMap::new();

                for (col_idx, field) in schema.fields().iter().enumerate() {
                    let col = batch.column(col_idx);
                    let value = arrow_value_to_value(col.as_ref(), row_idx);
                    obj.insert(field.name().clone(), value);
                }

                rows.push(Value::Object(obj));
            }
        }

        Ok(Value::Array(rows))
    }

    /// Parquet 파일의 메타데이터를 반환한다.
    #[allow(dead_code)]
    pub fn read_metadata(bytes: &[u8]) -> anyhow::Result<ParquetMetadata> {
        let bytes = Bytes::copy_from_slice(bytes);
        let builder =
            ParquetRecordBatchReaderBuilder::try_new(bytes).map_err(|e| DkitError::ParseError {
                format: "Parquet".to_string(),
                source: Box::new(e),
            })?;

        let metadata = builder.metadata().clone();
        let schema = builder.schema().clone();

        let columns: Vec<String> = schema.fields().iter().map(|f| f.name().clone()).collect();
        let column_types: Vec<String> = schema
            .fields()
            .iter()
            .map(|f| format!("{}", f.data_type()))
            .collect();

        Ok(ParquetMetadata {
            num_rows: metadata.file_metadata().num_rows() as usize,
            num_row_groups: metadata.num_row_groups(),
            columns,
            column_types,
        })
    }
}

/// Parquet 파일 메타데이터
#[allow(dead_code)]
pub struct ParquetMetadata {
    pub num_rows: usize,
    pub num_row_groups: usize,
    pub columns: Vec<String>,
    pub column_types: Vec<String>,
}

/// Parquet 압축 방식
#[derive(Debug, Clone, Default)]
pub enum ParquetCompression {
    #[default]
    None,
    Snappy,
    Gzip,
    Zstd,
}

impl std::str::FromStr for ParquetCompression {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "none" | "uncompressed" => Ok(ParquetCompression::None),
            "snappy" => Ok(ParquetCompression::Snappy),
            "gzip" => Ok(ParquetCompression::Gzip),
            "zstd" => Ok(ParquetCompression::Zstd),
            _ => anyhow::bail!(
                "Unknown Parquet compression '{}'. Valid options: none, snappy, gzip, zstd",
                s
            ),
        }
    }
}

/// Parquet Writer 옵션
#[derive(Debug, Clone, Default)]
pub struct ParquetWriteOptions {
    /// 압축 방식 (기본: none)
    pub compression: ParquetCompression,
    /// Row Group 최대 크기 (기본: parquet 라이브러리 기본값)
    pub row_group_size: Option<usize>,
}

/// Parquet Writer - Value 배열을 Parquet 바이트로 직렬화한다.
pub struct ParquetWriter {
    options: ParquetWriteOptions,
}

impl ParquetWriter {
    pub fn new(options: ParquetWriteOptions) -> Self {
        Self { options }
    }

    /// Value를 Parquet 바이트로 변환한다.
    ///
    /// Value는 반드시 Object의 Array이어야 한다.
    pub fn write_to_bytes(&self, value: &Value) -> anyhow::Result<Vec<u8>> {
        let rows = match value {
            Value::Array(rows) => rows,
            _ => anyhow::bail!(
                "Parquet output requires an array of records (got {})\n  Hint: input data must be a JSON array of objects",
                value_type_name(value)
            ),
        };

        if rows.is_empty() {
            let schema = Arc::new(Schema::empty());
            let batch = RecordBatch::new_empty(schema.clone());
            return write_batch_to_bytes(&batch, schema, &self.options);
        }

        // Infer schema from all rows
        let schema = infer_schema(rows);

        // Build Arrow columns
        let columns = build_columns(rows, &schema)?;

        let batch = RecordBatch::try_new(schema.clone(), columns)
            .map_err(|e| anyhow::anyhow!("Failed to create Parquet record batch: {e}"))?;

        write_batch_to_bytes(&batch, schema, &self.options)
    }
}

/// RecordBatch를 Parquet 바이트로 기록한다.
fn write_batch_to_bytes(
    batch: &RecordBatch,
    schema: Arc<Schema>,
    options: &ParquetWriteOptions,
) -> anyhow::Result<Vec<u8>> {
    let compression = match options.compression {
        ParquetCompression::None => Compression::UNCOMPRESSED,
        ParquetCompression::Snappy => Compression::SNAPPY,
        ParquetCompression::Gzip => Compression::GZIP(GzipLevel::default()),
        ParquetCompression::Zstd => Compression::ZSTD(ZstdLevel::default()),
    };

    let mut props_builder = WriterProperties::builder().set_compression(compression);

    if let Some(rg_size) = options.row_group_size {
        props_builder = props_builder.set_max_row_group_size(rg_size);
    }

    let props = props_builder.build();

    let mut buf = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))
        .map_err(|e| anyhow::anyhow!("Failed to create Parquet writer: {e}"))?;

    writer
        .write(batch)
        .map_err(|e| anyhow::anyhow!("Failed to write Parquet data: {e}"))?;

    writer
        .close()
        .map_err(|e| anyhow::anyhow!("Failed to finalize Parquet file: {e}"))?;

    Ok(buf)
}

/// Value 타입 이름을 반환한다 (에러 메시지용)
fn value_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// 행 배열에서 Arrow Schema를 추론한다.
fn infer_schema(rows: &[Value]) -> Arc<Schema> {
    // 모든 행에서 필드 이름을 순서대로 수집한다.
    let mut field_names: IndexMap<String, ()> = IndexMap::new();
    for row in rows {
        if let Value::Object(obj) = row {
            for key in obj.keys() {
                field_names.entry(key.clone()).or_insert(());
            }
        }
    }

    let fields: Vec<Field> = field_names
        .keys()
        .map(|name| {
            let (data_type, nullable) = infer_field_type(rows, name);
            Field::new(name, data_type, nullable)
        })
        .collect();

    Arc::new(Schema::new(fields))
}

/// 특정 필드의 Arrow DataType과 nullable 여부를 추론한다.
fn infer_field_type(rows: &[Value], field: &str) -> (DataType, bool) {
    let mut has_null = false;
    let mut seen_bool = false;
    let mut seen_int = false;
    let mut seen_float = false;
    let mut seen_string = false;

    for row in rows {
        match row {
            Value::Object(obj) => match obj.get(field) {
                None | Some(Value::Null) => has_null = true,
                Some(Value::Bool(_)) => seen_bool = true,
                Some(Value::Integer(_)) => seen_int = true,
                Some(Value::Float(_)) => seen_float = true,
                Some(Value::String(_)) => seen_string = true,
                // 중첩 배열/객체는 문자열로 직렬화
                Some(_) => seen_string = true,
            },
            _ => has_null = true,
        }
    }

    let data_type = if seen_string {
        DataType::Utf8
    } else if seen_float {
        // Integer 값도 Float64로 업캐스트한다.
        DataType::Float64
    } else if seen_int && seen_bool {
        // 혼합 타입은 문자열로 직렬화
        DataType::Utf8
    } else if seen_int {
        DataType::Int64
    } else if seen_bool {
        DataType::Boolean
    } else {
        // 모두 null이거나 미지정 → Utf8
        DataType::Utf8
    };

    (data_type, has_null)
}

/// 행 배열에서 Arrow 컬럼 배열을 생성한다.
fn build_columns(rows: &[Value], schema: &Schema) -> anyhow::Result<Vec<ArrayRef>> {
    schema
        .fields()
        .iter()
        .map(|field| Ok(build_column(rows, field.name(), field.data_type())))
        .collect()
}

/// 특정 필드에 대한 Arrow 배열을 생성한다.
fn build_column(rows: &[Value], field_name: &str, data_type: &DataType) -> ArrayRef {
    match data_type {
        DataType::Boolean => {
            let mut builder = BooleanBuilder::new();
            for row in rows {
                match get_field_value(row, field_name) {
                    Some(Value::Bool(b)) => builder.append_value(*b),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        DataType::Int64 => {
            let mut builder = Int64Builder::new();
            for row in rows {
                match get_field_value(row, field_name) {
                    Some(Value::Integer(i)) => builder.append_value(*i),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        DataType::Float64 => {
            let mut builder = Float64Builder::new();
            for row in rows {
                match get_field_value(row, field_name) {
                    Some(Value::Float(f)) => builder.append_value(*f),
                    Some(Value::Integer(i)) => builder.append_value(*i as f64),
                    _ => builder.append_null(),
                }
            }
            Arc::new(builder.finish())
        }
        _ => {
            let mut builder = StringBuilder::new();
            for row in rows {
                match get_field_value(row, field_name) {
                    Some(Value::String(s)) => builder.append_value(s),
                    Some(Value::Null) | None => builder.append_null(),
                    Some(other) => builder.append_value(other.to_string()),
                }
            }
            Arc::new(builder.finish())
        }
    }
}

/// 행에서 특정 필드 값을 가져온다.
fn get_field_value<'a>(row: &'a Value, field_name: &str) -> Option<&'a Value> {
    if let Value::Object(obj) = row {
        obj.get(field_name)
    } else {
        None
    }
}

/// Arrow 배열의 특정 행 값을 Value로 변환한다.
fn arrow_value_to_value(array: &dyn Array, idx: usize) -> Value {
    if array.is_null(idx) {
        return Value::Null;
    }

    match array.data_type() {
        DataType::Null => Value::Null,
        DataType::Boolean => {
            let arr = array.as_any().downcast_ref::<BooleanArray>().unwrap();
            Value::Bool(arr.value(idx))
        }
        DataType::Int8 => {
            let arr = array.as_any().downcast_ref::<Int8Array>().unwrap();
            Value::Integer(arr.value(idx) as i64)
        }
        DataType::Int16 => {
            let arr = array.as_any().downcast_ref::<Int16Array>().unwrap();
            Value::Integer(arr.value(idx) as i64)
        }
        DataType::Int32 => {
            let arr = array.as_any().downcast_ref::<Int32Array>().unwrap();
            Value::Integer(arr.value(idx) as i64)
        }
        DataType::Int64 => {
            let arr = array.as_any().downcast_ref::<Int64Array>().unwrap();
            Value::Integer(arr.value(idx))
        }
        DataType::UInt8 => {
            let arr = array.as_any().downcast_ref::<UInt8Array>().unwrap();
            Value::Integer(arr.value(idx) as i64)
        }
        DataType::UInt16 => {
            let arr = array.as_any().downcast_ref::<UInt16Array>().unwrap();
            Value::Integer(arr.value(idx) as i64)
        }
        DataType::UInt32 => {
            let arr = array.as_any().downcast_ref::<UInt32Array>().unwrap();
            Value::Integer(arr.value(idx) as i64)
        }
        DataType::UInt64 => {
            let arr = array.as_any().downcast_ref::<UInt64Array>().unwrap();
            let v = arr.value(idx);
            if v <= i64::MAX as u64 {
                Value::Integer(v as i64)
            } else {
                Value::Float(v as f64)
            }
        }
        DataType::Float32 => {
            let arr = array.as_any().downcast_ref::<Float32Array>().unwrap();
            Value::Float(arr.value(idx) as f64)
        }
        DataType::Float64 => {
            let arr = array.as_any().downcast_ref::<Float64Array>().unwrap();
            Value::Float(arr.value(idx))
        }
        DataType::Utf8 => {
            let arr = array.as_any().downcast_ref::<StringArray>().unwrap();
            Value::String(arr.value(idx).to_string())
        }
        DataType::LargeUtf8 => {
            let arr = array.as_any().downcast_ref::<LargeStringArray>().unwrap();
            Value::String(arr.value(idx).to_string())
        }
        DataType::Binary => {
            let arr = array.as_any().downcast_ref::<BinaryArray>().unwrap();
            let hex = arr
                .value(idx)
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            Value::String(format!("0x{}", hex))
        }
        DataType::LargeBinary => {
            let arr = array.as_any().downcast_ref::<LargeBinaryArray>().unwrap();
            let hex = arr
                .value(idx)
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            Value::String(format!("0x{}", hex))
        }
        DataType::Date32 => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow::array::Date32Array>()
                .unwrap();
            let days = arr.value(idx);
            Value::String(chrono_date_from_days(days))
        }
        DataType::Date64 => {
            let arr = array
                .as_any()
                .downcast_ref::<arrow::array::Date64Array>()
                .unwrap();
            let ms = arr.value(idx);
            Value::String(format_epoch_secs(ms / 1000))
        }
        DataType::Timestamp(unit, tz) => {
            let secs = match unit {
                arrow::datatypes::TimeUnit::Second => {
                    let arr = array
                        .as_any()
                        .downcast_ref::<arrow::array::TimestampSecondArray>()
                        .unwrap();
                    arr.value(idx)
                }
                arrow::datatypes::TimeUnit::Millisecond => {
                    let arr = array
                        .as_any()
                        .downcast_ref::<arrow::array::TimestampMillisecondArray>()
                        .unwrap();
                    arr.value(idx) / 1000
                }
                arrow::datatypes::TimeUnit::Microsecond => {
                    let arr = array
                        .as_any()
                        .downcast_ref::<arrow::array::TimestampMicrosecondArray>()
                        .unwrap();
                    arr.value(idx) / 1_000_000
                }
                arrow::datatypes::TimeUnit::Nanosecond => {
                    let arr = array
                        .as_any()
                        .downcast_ref::<arrow::array::TimestampNanosecondArray>()
                        .unwrap();
                    arr.value(idx) / 1_000_000_000
                }
            };
            let s = format_epoch_secs(secs);
            if tz.is_some() {
                Value::String(format!("{}Z", s))
            } else {
                Value::String(s)
            }
        }
        DataType::List(_) => {
            let list_arr = array.as_list::<i32>();
            let value_arr = list_arr.value(idx);
            let mut items = Vec::new();
            for i in 0..value_arr.len() {
                items.push(arrow_value_to_value(value_arr.as_ref(), i));
            }
            Value::Array(items)
        }
        DataType::LargeList(_) => {
            let list_arr = array.as_list::<i64>();
            let value_arr = list_arr.value(idx);
            let mut items = Vec::new();
            for i in 0..value_arr.len() {
                items.push(arrow_value_to_value(value_arr.as_ref(), i));
            }
            Value::Array(items)
        }
        DataType::Struct(_) => {
            let struct_arr = array.as_any().downcast_ref::<StructArray>().unwrap();
            let mut obj = IndexMap::new();
            for (i, field) in struct_arr.fields().iter().enumerate() {
                let col = struct_arr.column(i);
                obj.insert(
                    field.name().clone(),
                    arrow_value_to_value(col.as_ref(), idx),
                );
            }
            Value::Object(obj)
        }
        DataType::Map(_, _) => {
            let map_arr = array
                .as_any()
                .downcast_ref::<arrow::array::MapArray>()
                .unwrap();
            let entries = map_arr.value(idx);
            let struct_arr = entries.as_any().downcast_ref::<StructArray>().unwrap();
            let keys = struct_arr.column(0);
            let values = struct_arr.column(1);
            let mut obj = IndexMap::new();
            for i in 0..struct_arr.len() {
                let key = arrow_value_to_value(keys.as_ref(), i);
                let val = arrow_value_to_value(values.as_ref(), i);
                let key_str = match key {
                    Value::String(s) => s,
                    other => format!("{}", other),
                };
                obj.insert(key_str, val);
            }
            Value::Object(obj)
        }
        _ => {
            // Fallback for Dictionary, FixedSizeBinary, FixedSizeList, etc.
            // Use Arrow's display formatting
            let formatter = arrow::util::display::ArrayFormatter::try_new(
                array,
                &arrow::util::display::FormatOptions::default(),
            );
            match formatter {
                Ok(f) => Value::String(f.value(idx).to_string()),
                Err(_) => Value::String("<unsupported>".to_string()),
            }
        }
    }
}

/// epoch days → "YYYY-MM-DD" 문자열 변환
fn chrono_date_from_days(days: i32) -> String {
    let (y, m, d) = civil_from_days(days as i64);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

/// epoch seconds → "YYYY-MM-DDTHH:MM:SS" 문자열 변환
fn format_epoch_secs(secs: i64) -> String {
    let days = if secs >= 0 {
        secs / 86400
    } else {
        (secs - 86399) / 86400
    };
    let day_secs = (secs - days * 86400) as u32;
    let h = day_secs / 3600;
    let m = (day_secs % 3600) / 60;
    let s = day_secs % 60;

    let (y, mo, d) = civil_from_days(days);
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", y, mo, d, h, m, s)
}

/// Unix epoch 기준 일 수에서 (year, month, day) 계산
/// Howard Hinnant's civil_from_days algorithm
/// https://howardhinnant.github.io/date_algorithms.html
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::{ArrayRef, BooleanArray, Float64Array, Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;

    /// 테스트용 Parquet 바이트를 생성한다
    fn make_test_parquet() -> Vec<u8> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("age", DataType::Int64, false),
            Field::new("score", DataType::Float64, true),
            Field::new("active", DataType::Boolean, false),
        ]));

        let names: ArrayRef = Arc::new(StringArray::from(vec!["Alice", "Bob", "Charlie"]));
        let ages: ArrayRef = Arc::new(Int64Array::from(vec![30, 25, 35]));
        let scores: ArrayRef = Arc::new(Float64Array::from(vec![Some(95.5), None, Some(87.3)]));
        let actives: ArrayRef = Arc::new(BooleanArray::from(vec![true, false, true]));

        let batch =
            RecordBatch::try_new(schema.clone(), vec![names, ages, scores, actives]).unwrap();

        let mut buf = Vec::new();
        let mut writer = ArrowWriter::try_new(&mut buf, schema, None).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();

        buf
    }

    #[test]
    fn test_read_basic_parquet() {
        let bytes = make_test_parquet();
        let reader = ParquetReader::new(ParquetOptions::default());
        let value = reader.read_from_bytes(&bytes).unwrap();

        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 3);

        let row0 = arr[0].as_object().unwrap();
        assert_eq!(row0["name"], Value::String("Alice".to_string()));
        assert_eq!(row0["age"], Value::Integer(30));
        assert_eq!(row0["score"], Value::Float(95.5));
        assert_eq!(row0["active"], Value::Bool(true));

        let row1 = arr[1].as_object().unwrap();
        assert_eq!(row1["name"], Value::String("Bob".to_string()));
        assert_eq!(row1["score"], Value::Null);

        let row2 = arr[2].as_object().unwrap();
        assert_eq!(row2["name"], Value::String("Charlie".to_string()));
        assert_eq!(row2["age"], Value::Integer(35));
    }

    #[test]
    fn test_read_metadata() {
        let bytes = make_test_parquet();
        let meta = ParquetReader::read_metadata(&bytes).unwrap();

        assert_eq!(meta.num_rows, 3);
        assert_eq!(meta.num_row_groups, 1);
        assert_eq!(meta.columns, vec!["name", "age", "score", "active"]);
    }

    #[test]
    fn test_read_row_group_filter() {
        let bytes = make_test_parquet();
        let reader = ParquetReader::new(ParquetOptions { row_group: Some(0) });
        let value = reader.read_from_bytes(&bytes).unwrap();
        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_read_row_group_out_of_range() {
        let bytes = make_test_parquet();
        let reader = ParquetReader::new(ParquetOptions {
            row_group: Some(99),
        });
        let result = reader.read_from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_parquet() {
        let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)]));

        let ids: ArrayRef = Arc::new(Int64Array::from(Vec::<i64>::new()));
        let batch = RecordBatch::try_new(schema.clone(), vec![ids]).unwrap();

        let mut buf = Vec::new();
        let mut writer = ArrowWriter::try_new(&mut buf, schema, None).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();

        let reader = ParquetReader::new(ParquetOptions::default());
        let value = reader.read_from_bytes(&buf).unwrap();
        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_chrono_date_from_days() {
        assert_eq!(chrono_date_from_days(0), "1970-01-01");
        assert_eq!(chrono_date_from_days(1), "1970-01-02");
        assert_eq!(chrono_date_from_days(365), "1971-01-01");
        assert_eq!(chrono_date_from_days(18628), "2021-01-01");
    }

    #[test]
    fn test_format_epoch_secs() {
        assert_eq!(format_epoch_secs(0), "1970-01-01T00:00:00");
        assert_eq!(format_epoch_secs(86400), "1970-01-02T00:00:00");
    }

    #[test]
    fn test_nested_struct_parquet() {
        let inner_fields = vec![
            Field::new("city", DataType::Utf8, false),
            Field::new("zip", DataType::Int32, false),
        ];
        let schema = Arc::new(Schema::new(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("address", DataType::Struct(inner_fields.into()), true),
        ]));

        let names: ArrayRef = Arc::new(StringArray::from(vec!["Alice"]));
        let cities: ArrayRef = Arc::new(StringArray::from(vec!["Seoul"]));
        let zips: ArrayRef = Arc::new(arrow::array::Int32Array::from(vec![12345]));
        let address: ArrayRef = Arc::new(arrow::array::StructArray::from(vec![
            (
                Arc::new(Field::new("city", DataType::Utf8, false)),
                cities as ArrayRef,
            ),
            (
                Arc::new(Field::new("zip", DataType::Int32, false)),
                zips as ArrayRef,
            ),
        ]));

        let batch = RecordBatch::try_new(schema.clone(), vec![names, address]).unwrap();

        let mut buf = Vec::new();
        let mut writer = ArrowWriter::try_new(&mut buf, schema, None).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();

        let reader = ParquetReader::new(ParquetOptions::default());
        let value = reader.read_from_bytes(&buf).unwrap();
        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 1);

        let row = arr[0].as_object().unwrap();
        let addr = row["address"].as_object().unwrap();
        assert_eq!(addr["city"], Value::String("Seoul".to_string()));
        assert_eq!(addr["zip"], Value::Integer(12345));
    }

    #[test]
    fn test_list_parquet() {
        let list_field = Field::new("item", DataType::Int32, true);
        let schema = Arc::new(Schema::new(vec![Field::new(
            "values",
            DataType::List(Arc::new(list_field)),
            true,
        )]));

        let values_builder = arrow::array::ListBuilder::new(arrow::array::Int32Builder::new());
        let mut builder = values_builder;
        builder.values().append_value(1);
        builder.values().append_value(2);
        builder.values().append_value(3);
        builder.append(true);
        builder.values().append_value(4);
        builder.values().append_value(5);
        builder.append(true);

        let list_array: ArrayRef = Arc::new(builder.finish());
        let batch = RecordBatch::try_new(schema.clone(), vec![list_array]).unwrap();

        let mut buf = Vec::new();
        let mut writer = ArrowWriter::try_new(&mut buf, schema, None).unwrap();
        writer.write(&batch).unwrap();
        writer.close().unwrap();

        let reader = ParquetReader::new(ParquetOptions::default());
        let value = reader.read_from_bytes(&buf).unwrap();
        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        let row0 = arr[0].as_object().unwrap();
        let vals = row0["values"].as_array().unwrap();
        assert_eq!(vals.len(), 3);
        assert_eq!(vals[0], Value::Integer(1));
    }
}
