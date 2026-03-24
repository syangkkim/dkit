use colored::Colorize;
use comfy_table::presets;
use comfy_table::{Cell, ColumnConstraint, ContentArrangement, Table, Width};

use dkit_core::value::Value;

/// 테이블 렌더링 옵션
pub struct TableOptions<'a> {
    pub limit: Option<usize>,
    pub columns: Option<&'a [String]>,
    pub max_width: Option<u16>,
    pub hide_header: bool,
    pub row_numbers: bool,
    pub border: &'a str,
    pub color: bool,
}

impl<'a> Default for TableOptions<'a> {
    fn default() -> Self {
        Self {
            limit: None,
            columns: None,
            max_width: None,
            hide_header: false,
            row_numbers: false,
            border: "simple",
            color: false,
        }
    }
}

/// Value 데이터를 테이블 문자열로 렌더링한다.
///
/// - Array of Objects → 각 object가 하나의 행, 키가 컬럼 헤더
/// - Array of primitives → 단일 "value" 컬럼
/// - Single Object → key/value 2-컬럼 테이블
/// - Primitive → 단일 값 출력
pub fn render_table(value: &Value, opts: &TableOptions) -> String {
    match value {
        Value::Array(arr) if !arr.is_empty() && arr[0].as_object().is_some() => {
            render_array_of_objects(arr, opts)
        }
        Value::Array(arr) => render_array_of_primitives(arr, opts),
        Value::Object(_) => render_single_object(value, opts),
        _ => format!("{value}"),
    }
}

/// 테이블에 공통 스타일을 적용한다
fn apply_table_style(table: &mut Table, opts: &TableOptions) {
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let preset = match opts.border {
        "none" => presets::NOTHING,
        "rounded" => presets::UTF8_FULL_CONDENSED,
        "heavy" => presets::UTF8_FULL,
        _ => presets::ASCII_FULL_CONDENSED, // "simple" (default)
    };
    table.load_preset(preset);

    if let Some(max_w) = opts.max_width {
        let col_count = table.column_count();
        let constraints: Vec<ColumnConstraint> = (0..col_count)
            .map(|_| ColumnConstraint::UpperBoundary(Width::Fixed(max_w)))
            .collect();
        table.set_constraints(constraints);
    }
}

/// 셀 값을 색상 적용하여 생성
fn make_cell(v: &Value, color: bool) -> Cell {
    if !color {
        return Cell::new(format_cell_value(v));
    }
    let text = format_cell_value(v);
    let colored_text = match v {
        Value::Integer(_) | Value::Float(_) => text.blue().to_string(),
        Value::Null => text.dimmed().to_string(),
        Value::Bool(_) => text.yellow().to_string(),
        _ => text,
    };
    Cell::new(colored_text)
}

/// Array<Object> → 테이블 (각 object = 1행)
fn render_array_of_objects(arr: &[Value], opts: &TableOptions) -> String {
    let all_keys = collect_keys(arr);
    let headers: Vec<&String> = match opts.columns {
        Some(cols) => all_keys.iter().filter(|k| cols.contains(k)).collect(),
        None => all_keys.iter().collect(),
    };

    let mut table = Table::new();

    if !opts.hide_header {
        let mut header_cells: Vec<Cell> = Vec::new();
        if opts.row_numbers {
            header_cells.push(Cell::new("#"));
        }
        header_cells.extend(headers.iter().copied().map(Cell::new));
        table.set_header(header_cells);
    }

    // Apply style after setting header so column_count is correct
    apply_table_style(&mut table, opts);

    let row_count = match opts.limit {
        Some(n) => n.min(arr.len()),
        None => arr.len(),
    };

    for (i, item) in arr.iter().take(row_count).enumerate() {
        if let Value::Object(obj) = item {
            let mut row: Vec<Cell> = Vec::new();
            if opts.row_numbers {
                row.push(Cell::new(i + 1));
            }
            row.extend(headers.iter().map(|key| {
                let val = obj.get(*key).unwrap_or(&Value::Null);
                if val == &Value::Null && obj.get(*key).is_none() {
                    Cell::new("")
                } else {
                    make_cell(val, opts.color)
                }
            }));
            table.add_row(row);
        }
    }

    if let Some(n) = opts.limit {
        if n < arr.len() {
            table.add_row(vec![Cell::new(format!(
                "... ({} more rows)",
                arr.len() - n
            ))]);
        }
    }

    table.to_string()
}

/// Array<Primitive> → 단일 컬럼 테이블
fn render_array_of_primitives(arr: &[Value], opts: &TableOptions) -> String {
    let mut table = Table::new();

    if !opts.hide_header {
        let mut header_cells: Vec<Cell> = Vec::new();
        if opts.row_numbers {
            header_cells.push(Cell::new("#"));
        }
        header_cells.push(Cell::new("value"));
        table.set_header(header_cells);
    }

    apply_table_style(&mut table, opts);

    let row_count = match opts.limit {
        Some(n) => n.min(arr.len()),
        None => arr.len(),
    };

    for (i, item) in arr.iter().take(row_count).enumerate() {
        let mut row: Vec<Cell> = Vec::new();
        if opts.row_numbers {
            row.push(Cell::new(i + 1));
        }
        row.push(make_cell(item, opts.color));
        table.add_row(row);
    }

    if let Some(n) = opts.limit {
        if n < arr.len() {
            table.add_row(vec![Cell::new(format!(
                "... ({} more rows)",
                arr.len() - n
            ))]);
        }
    }

    table.to_string()
}

/// 단일 Object → key | value 테이블
fn render_single_object(value: &Value, opts: &TableOptions) -> String {
    let mut table = Table::new();

    if !opts.hide_header {
        table.set_header(vec![Cell::new("key"), Cell::new("value")]);
    }

    apply_table_style(&mut table, opts);

    if let Value::Object(obj) = value {
        for (k, v) in obj {
            if let Some(cols) = opts.columns {
                if !cols.contains(k) {
                    continue;
                }
            }
            let val_cell = make_cell(v, opts.color);
            table.add_row(vec![Cell::new(k), val_cell]);
        }
    }

    table.to_string()
}

/// 모든 object에서 키를 순서 보존하며 수집
fn collect_keys(arr: &[Value]) -> Vec<String> {
    let mut keys: Vec<String> = Vec::new();
    for item in arr {
        if let Value::Object(obj) = item {
            for k in obj.keys() {
                if !keys.contains(k) {
                    keys.push(k.clone());
                }
            }
        }
    }
    keys
}

/// Value를 셀 표시용 문자열로 변환
fn format_cell_value(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_cell_value).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(_) => "{...}".to_string(),
        _ => format!("{v}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn default_opts() -> TableOptions<'static> {
        TableOptions::default()
    }

    fn make_user(name: &str, age: i64) -> Value {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String(name.to_string()));
        m.insert("age".to_string(), Value::Integer(age));
        Value::Object(m)
    }

    #[test]
    fn test_render_array_of_objects() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let output = render_table(&data, &default_opts());
        assert!(output.contains("name"));
        assert!(output.contains("age"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(output.contains("30"));
        assert!(output.contains("25"));
    }

    #[test]
    fn test_render_with_limit() {
        let data = Value::Array(vec![
            make_user("Alice", 30),
            make_user("Bob", 25),
            make_user("Charlie", 35),
        ]);
        let opts = TableOptions {
            limit: Some(2),
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(!output.contains("Charlie"));
        assert!(output.contains("1 more rows"));
    }

    #[test]
    fn test_render_with_columns() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let cols = vec!["name".to_string()];
        let opts = TableOptions {
            columns: Some(&cols),
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("name"));
        assert!(output.contains("Alice"));
    }

    #[test]
    fn test_render_array_of_primitives() {
        let data = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let output = render_table(&data, &default_opts());
        assert!(output.contains("value"));
        assert!(output.contains('1'));
        assert!(output.contains('2'));
        assert!(output.contains('3'));
    }

    #[test]
    fn test_render_single_object() {
        let mut m = IndexMap::new();
        m.insert("host".to_string(), Value::String("localhost".to_string()));
        m.insert("port".to_string(), Value::Integer(8080));
        let data = Value::Object(m);
        let output = render_table(&data, &default_opts());
        assert!(output.contains("key"));
        assert!(output.contains("value"));
        assert!(output.contains("host"));
        assert!(output.contains("localhost"));
        assert!(output.contains("port"));
        assert!(output.contains("8080"));
    }

    #[test]
    fn test_render_single_object_with_columns() {
        let mut m = IndexMap::new();
        m.insert("host".to_string(), Value::String("localhost".to_string()));
        m.insert("port".to_string(), Value::Integer(8080));
        let data = Value::Object(m);
        let cols = vec!["host".to_string()];
        let opts = TableOptions {
            columns: Some(&cols),
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("host"));
        assert!(!output.contains("8080"));
    }

    #[test]
    fn test_render_primitive() {
        let data = Value::String("hello".to_string());
        let output = render_table(&data, &default_opts());
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_render_null_values() {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String("Alice".to_string()));
        m.insert("email".to_string(), Value::Null);
        let data = Value::Array(vec![Value::Object(m)]);
        let output = render_table(&data, &default_opts());
        assert!(output.contains("null"));
    }

    #[test]
    fn test_render_nested_value_in_cell() {
        let mut m = IndexMap::new();
        m.insert(
            "tags".to_string(),
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
            ]),
        );
        let data = Value::Array(vec![Value::Object(m)]);
        let output = render_table(&data, &default_opts());
        assert!(output.contains("[a, b]"));
    }

    #[test]
    fn test_render_empty_array() {
        let data = Value::Array(vec![]);
        let output = render_table(&data, &default_opts());
        assert!(output.contains("value"));
    }

    #[test]
    fn test_limit_larger_than_data() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let opts = TableOptions {
            limit: Some(100),
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("Alice"));
        assert!(!output.contains("more rows"));
    }

    #[test]
    fn test_hide_header() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let opts = TableOptions {
            hide_header: true,
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("Alice"));
        assert!(output.contains("30"));
        // Header row should not appear as a labeled row
        assert!(!output.contains(" name "));
    }

    #[test]
    fn test_row_numbers() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let opts = TableOptions {
            row_numbers: true,
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains('#'));
        assert!(output.contains(" 1 "));
        assert!(output.contains(" 2 "));
    }

    #[test]
    fn test_border_none() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let opts = TableOptions {
            border: "none",
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("Alice"));
        // No border characters
        assert!(!output.contains('+'));
        assert!(!output.contains('|'));
    }

    #[test]
    fn test_border_rounded() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let opts = TableOptions {
            border: "rounded",
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("Alice"));
        // UTF8 border characters
        assert!(output.contains('┌') || output.contains('│'));
    }

    #[test]
    fn test_border_heavy() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let opts = TableOptions {
            border: "heavy",
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains("Alice"));
        assert!(output.contains('┌') || output.contains('│'));
    }

    #[test]
    fn test_max_width_truncation() {
        let mut m = IndexMap::new();
        m.insert(
            "text".to_string(),
            Value::String("This is a very long text that should be truncated".to_string()),
        );
        let data = Value::Array(vec![Value::Object(m)]);
        let opts = TableOptions {
            max_width: Some(10),
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        // The full text should not appear
        assert!(!output.contains("This is a very long text that should be truncated"));
    }

    #[test]
    fn test_row_numbers_with_primitives() {
        let data = Value::Array(vec![
            Value::Integer(10),
            Value::Integer(20),
            Value::Integer(30),
        ]);
        let opts = TableOptions {
            row_numbers: true,
            ..default_opts()
        };
        let output = render_table(&data, &opts);
        assert!(output.contains('#'));
        assert!(output.contains(" 1 "));
        assert!(output.contains(" 2 "));
        assert!(output.contains(" 3 "));
    }
}
