use std::io::{Cursor, Read};

use calamine::{open_workbook_auto_from_rs, Data, Reader};
use indexmap::IndexMap;

use crate::error::DkitError;
use crate::value::Value;

/// Excel (.xlsx) 파일의 읽기 옵션
#[derive(Debug, Clone)]
pub struct XlsxOptions {
    /// 읽을 시트 이름 또는 인덱스 (기본: 첫 번째 시트)
    pub sheet: Option<String>,
    /// 헤더 행 번호 (1-based, 기본: 1)
    pub header_row: usize,
}

impl Default for XlsxOptions {
    fn default() -> Self {
        Self {
            sheet: None,
            header_row: 1,
        }
    }
}

pub struct XlsxReader {
    options: XlsxOptions,
}

impl XlsxReader {
    pub fn new(options: XlsxOptions) -> Self {
        Self { options }
    }

    /// 바이트 슬라이스에서 Excel 파일을 읽어 Value로 변환
    pub fn read_from_bytes(&self, bytes: &[u8]) -> anyhow::Result<Value> {
        let cursor = Cursor::new(bytes);
        let mut workbook =
            open_workbook_auto_from_rs(cursor).map_err(|e| DkitError::ParseError {
                format: "Excel".to_string(),
                source: Box::new(e),
            })?;

        let sheet_name = self.resolve_sheet_name(&workbook)?;
        let range = workbook
            .worksheet_range(&sheet_name)
            .map_err(|e| DkitError::ParseError {
                format: "Excel".to_string(),
                source: Box::new(e),
            })?;

        let rows: Vec<Vec<Data>> = range.rows().map(|r| r.to_vec()).collect();

        if rows.is_empty() {
            return Ok(Value::Array(vec![]));
        }

        let header_idx = self.options.header_row.saturating_sub(1);
        if header_idx >= rows.len() {
            anyhow::bail!(
                "Header row {} exceeds sheet row count ({})\n  Hint: use --header-row to specify the correct header row",
                self.options.header_row,
                rows.len()
            );
        }

        // Extract headers from the header row
        let headers: Vec<String> = rows[header_idx]
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let s = cell_to_string(cell);
                if s.is_empty() {
                    format!("col{}", i + 1)
                } else {
                    s
                }
            })
            .collect();

        // Convert data rows (everything after header row) to array of objects
        let mut result = Vec::new();
        for row in rows.iter().skip(header_idx + 1) {
            let mut obj = IndexMap::new();
            for (i, cell) in row.iter().enumerate() {
                let key = headers
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("col{}", i + 1));
                obj.insert(key, cell_to_value(cell));
            }
            // Fill missing columns with null
            for header in headers.iter().skip(row.len()) {
                obj.insert(header.clone(), Value::Null);
            }
            result.push(Value::Object(obj));
        }

        Ok(Value::Array(result))
    }

    /// 시트 목록을 반환
    pub fn list_sheets(bytes: &[u8]) -> anyhow::Result<Vec<String>> {
        let cursor = Cursor::new(bytes);
        let workbook = open_workbook_auto_from_rs(cursor).map_err(|e| DkitError::ParseError {
            format: "Excel".to_string(),
            source: Box::new(e),
        })?;
        Ok(workbook.sheet_names().to_vec())
    }

    /// 옵션에 따라 시트 이름을 결정
    fn resolve_sheet_name<RS: Read + std::io::Seek>(
        &self,
        workbook: &calamine::Sheets<RS>,
    ) -> anyhow::Result<String> {
        let sheet_names = workbook.sheet_names();
        if sheet_names.is_empty() {
            anyhow::bail!("Excel file contains no sheets");
        }

        match &self.options.sheet {
            None => Ok(sheet_names[0].clone()),
            Some(spec) => {
                // Try as index first (0-based)
                if let Ok(idx) = spec.parse::<usize>() {
                    if idx < sheet_names.len() {
                        return Ok(sheet_names[idx].clone());
                    }
                    anyhow::bail!(
                        "Sheet index {} out of range (0..{})\n  Available sheets: {}",
                        idx,
                        sheet_names.len(),
                        sheet_names.join(", ")
                    );
                }
                // Try as name
                if sheet_names.iter().any(|n| n == spec) {
                    return Ok(spec.clone());
                }
                anyhow::bail!(
                    "Sheet '{}' not found\n  Available sheets: {}",
                    spec,
                    sheet_names.join(", ")
                );
            }
        }
    }
}

/// Excel 셀 데이터를 dkit Value로 변환
fn cell_to_value(cell: &Data) -> Value {
    match cell {
        Data::Empty => Value::Null,
        Data::Bool(b) => Value::Bool(*b),
        Data::Int(n) => Value::Integer(*n),
        Data::Float(f) => {
            // If the float is actually an integer value, store as Integer
            if f.fract() == 0.0 && f.is_finite() && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                Value::Integer(*f as i64)
            } else {
                Value::Float(*f)
            }
        }
        Data::String(s) => Value::String(s.clone()),
        Data::DateTime(dt) => {
            // Excel serial date number → string
            Value::String(excel_serial_to_string(dt.as_f64()))
        }
        Data::DateTimeIso(s) => Value::String(s.clone()),
        Data::DurationIso(s) => Value::String(s.clone()),
        Data::Error(e) => Value::String(format!("#ERROR:{:?}", e)),
    }
}

/// Excel 셀 데이터를 문자열로 변환 (헤더용)
fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::Bool(b) => b.to_string(),
        Data::Int(n) => n.to_string(),
        Data::Float(f) => {
            if f.fract() == 0.0 && f.is_finite() {
                format!("{}", *f as i64)
            } else {
                f.to_string()
            }
        }
        Data::String(s) => s.clone(),
        Data::DateTime(dt) => excel_serial_to_string(dt.as_f64()),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
        Data::Error(e) => format!("{:?}", e),
    }
}

/// Excel serial date number을 문자열로 변환
/// Excel은 1900-01-01을 1로 취급 (1899-12-30 기준)
fn excel_serial_to_string(serial: f64) -> String {
    let days = serial.trunc() as i64;
    let frac = serial.fract();

    // Base date: 1899-12-30 (Excel epoch)
    // Excel has a bug treating 1900 as leap year, so dates after Feb 28, 1900
    // are off by one day. We handle the common case.
    // Adjust: serial 1 = 1900-01-01, so base_days 0 = 1900-01-01
    let base_days = days - 1;
    let (year, month, day) = serial_to_ymd(base_days);

    if frac.abs() < 1e-10 {
        // Date only
        format!("{:04}-{:02}-{:02}", year, month, day)
    } else {
        // Date + time
        let total_seconds = (frac * 86400.0).round() as u32;
        let hours = total_seconds / 3600;
        let mins = (total_seconds % 3600) / 60;
        let secs = total_seconds % 60;
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hours, mins, secs
        )
    }
}

/// Convert days since 1900-01-01 (0-based) to (year, month, day)
fn serial_to_ymd(days_since_1900: i64) -> (i32, u32, u32) {
    // Use a simple algorithm
    // days_since_1900: 0 = 1900-01-01
    let mut remaining = days_since_1900;
    let mut year: i32 = 1900;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let months_days: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1u32;
    for &md in &months_days {
        if remaining < md {
            break;
        }
        remaining -= md;
        month += 1;
    }

    (year, month, (remaining + 1) as u32)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_to_value_empty() {
        assert_eq!(cell_to_value(&Data::Empty), Value::Null);
    }

    #[test]
    fn test_cell_to_value_bool() {
        assert_eq!(cell_to_value(&Data::Bool(true)), Value::Bool(true));
        assert_eq!(cell_to_value(&Data::Bool(false)), Value::Bool(false));
    }

    #[test]
    fn test_cell_to_value_int() {
        assert_eq!(cell_to_value(&Data::Int(42)), Value::Integer(42));
    }

    #[test]
    fn test_cell_to_value_float_integer() {
        // Float that is actually an integer
        assert_eq!(cell_to_value(&Data::Float(100.0)), Value::Integer(100));
    }

    #[test]
    fn test_cell_to_value_float() {
        assert_eq!(cell_to_value(&Data::Float(3.14)), Value::Float(3.14));
    }

    #[test]
    fn test_cell_to_value_string() {
        assert_eq!(
            cell_to_value(&Data::String("hello".to_string())),
            Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_cell_to_string_empty() {
        assert_eq!(cell_to_string(&Data::Empty), "");
    }

    #[test]
    fn test_cell_to_string_string() {
        assert_eq!(cell_to_string(&Data::String("Name".to_string())), "Name");
    }

    #[test]
    fn test_xlsx_options_default() {
        let opts = XlsxOptions::default();
        assert_eq!(opts.sheet, None);
        assert_eq!(opts.header_row, 1);
    }
}
