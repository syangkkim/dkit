use comfy_table::{Cell, ContentArrangement, Table};

use crate::value::Value;

/// Value 데이터를 테이블 문자열로 렌더링한다.
///
/// - Array of Objects → 각 object가 하나의 행, 키가 컬럼 헤더
/// - Array of primitives → 단일 "value" 컬럼
/// - Single Object → key/value 2-컬럼 테이블
/// - Primitive → 단일 값 출력
pub fn render_table(value: &Value, limit: Option<usize>, columns: Option<&[String]>) -> String {
    match value {
        Value::Array(arr) if !arr.is_empty() && arr[0].as_object().is_some() => {
            render_array_of_objects(arr, limit, columns)
        }
        Value::Array(arr) => render_array_of_primitives(arr, limit),
        Value::Object(_) => render_single_object(value, columns),
        _ => format!("{value}"),
    }
}

/// Array<Object> → 테이블 (각 object = 1행)
fn render_array_of_objects(
    arr: &[Value],
    limit: Option<usize>,
    columns: Option<&[String]>,
) -> String {
    // 모든 object에서 키를 수집하여 컬럼 헤더 결정 (순서 보존)
    let all_keys = collect_keys(arr);
    let headers: Vec<&String> = match columns {
        Some(cols) => all_keys.iter().filter(|k| cols.contains(k)).collect(),
        None => all_keys.iter().collect(),
    };

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(headers.iter().copied().map(Cell::new));

    let row_count = match limit {
        Some(n) => n.min(arr.len()),
        None => arr.len(),
    };

    for item in arr.iter().take(row_count) {
        if let Value::Object(obj) = item {
            let row: Vec<Cell> = headers
                .iter()
                .map(|key| {
                    let cell_text = match obj.get(*key) {
                        Some(Value::Null) => "null".to_string(),
                        Some(v) => format_cell_value(v),
                        None => "".to_string(),
                    };
                    Cell::new(cell_text)
                })
                .collect();
            table.add_row(row);
        }
    }

    if let Some(n) = limit {
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
fn render_array_of_primitives(arr: &[Value], limit: Option<usize>) -> String {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![Cell::new("value")]);

    let row_count = match limit {
        Some(n) => n.min(arr.len()),
        None => arr.len(),
    };

    for item in arr.iter().take(row_count) {
        table.add_row(vec![Cell::new(format_cell_value(item))]);
    }

    if let Some(n) = limit {
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
fn render_single_object(value: &Value, columns: Option<&[String]>) -> String {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![Cell::new("key"), Cell::new("value")]);

    if let Value::Object(obj) = value {
        for (k, v) in obj {
            if let Some(cols) = columns {
                if !cols.contains(k) {
                    continue;
                }
            }
            table.add_row(vec![Cell::new(k), Cell::new(format_cell_value(v))]);
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn make_user(name: &str, age: i64) -> Value {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String(name.to_string()));
        m.insert("age".to_string(), Value::Integer(age));
        Value::Object(m)
    }

    #[test]
    fn test_render_array_of_objects() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let output = render_table(&data, None, None);
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
        let output = render_table(&data, Some(2), None);
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(!output.contains("Charlie"));
        assert!(output.contains("1 more rows"));
    }

    #[test]
    fn test_render_with_columns() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let cols = vec!["name".to_string()];
        let output = render_table(&data, None, Some(&cols));
        assert!(output.contains("name"));
        assert!(output.contains("Alice"));
        // age column should not appear in header
        // (the values 30/25 won't appear since the age column is filtered)
    }

    #[test]
    fn test_render_array_of_primitives() {
        let data = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let output = render_table(&data, None, None);
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
        let output = render_table(&data, None, None);
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
        let output = render_table(&data, None, Some(&cols));
        assert!(output.contains("host"));
        assert!(!output.contains("8080"));
    }

    #[test]
    fn test_render_primitive() {
        let data = Value::String("hello".to_string());
        let output = render_table(&data, None, None);
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_render_null_values() {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String("Alice".to_string()));
        m.insert("email".to_string(), Value::Null);
        let data = Value::Array(vec![Value::Object(m)]);
        let output = render_table(&data, None, None);
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
        let output = render_table(&data, None, None);
        assert!(output.contains("[a, b]"));
    }

    #[test]
    fn test_render_empty_array() {
        let data = Value::Array(vec![]);
        let output = render_table(&data, None, None);
        // Empty array of primitives path
        assert!(output.contains("value"));
    }

    #[test]
    fn test_limit_larger_than_data() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let output = render_table(&data, Some(100), None);
        assert!(output.contains("Alice"));
        assert!(!output.contains("more rows"));
    }
}
