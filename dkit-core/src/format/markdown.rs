use std::io::Write;

use crate::format::FormatWriter;
use crate::value::Value;

/// Markdown 테이블 포맷 Writer (GFM 호환)
///
/// - Array<Object> → 컬럼 헤더 + 데이터 행
/// - Array<Primitive> → 단일 "value" 컬럼
/// - Single Object → key | value 2-컬럼 테이블
/// - Primitive → 단일 값 출력
pub struct MarkdownWriter;

impl FormatWriter for MarkdownWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        Ok(render_markdown(value))
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let output = render_markdown(value);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }
}

fn render_markdown(value: &Value) -> String {
    match value {
        Value::Array(arr) if !arr.is_empty() && arr[0].as_object().is_some() => {
            render_array_of_objects(arr)
        }
        Value::Array(arr) => render_array_of_primitives(arr),
        Value::Object(_) => render_single_object(value),
        _ => format_cell_value(value),
    }
}

/// Array<Object> → Markdown 테이블
fn render_array_of_objects(arr: &[Value]) -> String {
    let headers = collect_keys(arr);
    if headers.is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();

    // 헤더 행
    let header_line = format!(
        "| {} |",
        headers
            .iter()
            .map(|h| escape_pipe(h))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    lines.push(header_line);

    // 구분자 행 (숫자 컬럼은 우측 정렬)
    let alignments: Vec<Alignment> = headers
        .iter()
        .map(|key| detect_column_alignment(arr, key))
        .collect();
    let separator_line = format!(
        "| {} |",
        alignments
            .iter()
            .map(|a| match a {
                Alignment::Right => "---:".to_string(),
                Alignment::Left => "---".to_string(),
            })
            .collect::<Vec<_>>()
            .join(" | ")
    );
    lines.push(separator_line);

    // 데이터 행
    for item in arr {
        if let Value::Object(obj) = item {
            let row = headers
                .iter()
                .map(|key| match obj.get(key) {
                    Some(Value::Null) => "null".to_string(),
                    Some(v) => escape_pipe(&format_cell_value(v)),
                    None => String::new(),
                })
                .collect::<Vec<_>>();
            lines.push(format!("| {} |", row.join(" | ")));
        }
    }

    lines.join("\n") + "\n"
}

/// Array<Primitive> → 단일 컬럼 Markdown 테이블
fn render_array_of_primitives(arr: &[Value]) -> String {
    let mut lines = Vec::new();
    lines.push("| value |".to_string());
    lines.push("| --- |".to_string());

    for item in arr {
        lines.push(format!("| {} |", escape_pipe(&format_cell_value(item))));
    }

    lines.join("\n") + "\n"
}

/// 단일 Object → key | value 테이블
fn render_single_object(value: &Value) -> String {
    let mut lines = Vec::new();
    lines.push("| key | value |".to_string());
    lines.push("| --- | --- |".to_string());

    if let Value::Object(obj) = value {
        for (k, v) in obj {
            lines.push(format!(
                "| {} | {} |",
                escape_pipe(k),
                escape_pipe(&format_cell_value(v))
            ));
        }
    }

    lines.join("\n") + "\n"
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

#[derive(Debug)]
enum Alignment {
    Left,
    Right,
}

/// 컬럼의 값이 모두 숫자이면 우측 정렬, 아니면 좌측 정렬
fn detect_column_alignment(arr: &[Value], key: &str) -> Alignment {
    let mut has_value = false;
    for item in arr {
        if let Value::Object(obj) = item {
            match obj.get(key) {
                Some(Value::Integer(_)) | Some(Value::Float(_)) => {
                    has_value = true;
                }
                Some(Value::Null) | None => {
                    // null/missing은 무시
                }
                _ => return Alignment::Left,
            }
        }
    }
    if has_value {
        Alignment::Right
    } else {
        Alignment::Left
    }
}

/// Value를 셀 표시용 문자열로 변환
fn format_cell_value(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => {
            // 중첩 객체/배열은 JSON 문자열로 inline 표시
            serde_json::to_string(&value_to_json(v)).unwrap_or_else(|_| "{...}".to_string())
        }
    }
}

/// Value를 serde_json::Value로 변환 (중첩 표시용)
fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Integer(n) => serde_json::json!(n),
        Value::Float(f) => serde_json::json!(f),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(value_to_json).collect()),
        Value::Object(obj) => {
            let map: serde_json::Map<String, serde_json::Value> = obj
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect();
            serde_json::Value::Object(map)
        }
    }
}

/// 파이프 문자(`|`) 이스케이프 처리
fn escape_pipe(s: &str) -> String {
    s.replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::FormatWriter;
    use indexmap::IndexMap;

    fn make_user(name: &str, age: i64) -> Value {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String(name.to_string()));
        m.insert("age".to_string(), Value::Integer(age));
        Value::Object(m)
    }

    #[test]
    fn test_array_of_objects() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains("| name | age |"));
        assert!(output.contains("| --- | ---: |")); // age는 숫자이므로 우측 정렬
        assert!(output.contains("| Alice | 30 |"));
        assert!(output.contains("| Bob | 25 |"));
    }

    #[test]
    fn test_numeric_right_alignment() {
        let mut m1 = IndexMap::new();
        m1.insert("label".to_string(), Value::String("x".to_string()));
        m1.insert("count".to_string(), Value::Integer(10));
        let mut m2 = IndexMap::new();
        m2.insert("label".to_string(), Value::String("y".to_string()));
        m2.insert("count".to_string(), Value::Integer(20));
        let data = Value::Array(vec![Value::Object(m1), Value::Object(m2)]);
        let output = MarkdownWriter.write(&data).unwrap();
        // label은 좌측, count는 우측 정렬
        assert!(output.contains("| --- | ---: |"));
    }

    #[test]
    fn test_mixed_column_left_alignment() {
        let mut m1 = IndexMap::new();
        m1.insert("val".to_string(), Value::Integer(10));
        let mut m2 = IndexMap::new();
        m2.insert("val".to_string(), Value::String("text".to_string()));
        let data = Value::Array(vec![Value::Object(m1), Value::Object(m2)]);
        let output = MarkdownWriter.write(&data).unwrap();
        // 혼합 타입이므로 좌측 정렬
        assert!(output.contains("| --- |"));
        assert!(!output.contains("---:"));
    }

    #[test]
    fn test_single_object() {
        let mut m = IndexMap::new();
        m.insert("host".to_string(), Value::String("localhost".to_string()));
        m.insert("port".to_string(), Value::Integer(8080));
        let data = Value::Object(m);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains("| key | value |"));
        assert!(output.contains("| host | localhost |"));
        assert!(output.contains("| port | 8080 |"));
    }

    #[test]
    fn test_array_of_primitives() {
        let data = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains("| value |"));
        assert!(output.contains("| 1 |"));
        assert!(output.contains("| 2 |"));
        assert!(output.contains("| 3 |"));
    }

    #[test]
    fn test_primitive_value() {
        let data = Value::String("hello".to_string());
        let output = MarkdownWriter.write(&data).unwrap();
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_null_value_in_cell() {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String("Alice".to_string()));
        m.insert("email".to_string(), Value::Null);
        let data = Value::Array(vec![Value::Object(m)]);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains("null"));
    }

    #[test]
    fn test_nested_value_json_inline() {
        let mut m = IndexMap::new();
        m.insert(
            "tags".to_string(),
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
            ]),
        );
        let data = Value::Array(vec![Value::Object(m)]);
        let output = MarkdownWriter.write(&data).unwrap();
        // 중첩 배열은 JSON 문자열로 표시
        assert!(output.contains(r#"["a","b"]"#));
    }

    #[test]
    fn test_pipe_escape() {
        let mut m = IndexMap::new();
        m.insert("formula".to_string(), Value::String("a | b".to_string()));
        let data = Value::Array(vec![Value::Object(m)]);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains(r"a \| b"));
    }

    #[test]
    fn test_empty_array() {
        let data = Value::Array(vec![]);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains("| value |"));
    }

    #[test]
    fn test_missing_fields() {
        let mut m1 = IndexMap::new();
        m1.insert("a".to_string(), Value::Integer(1));
        m1.insert("b".to_string(), Value::Integer(2));
        let mut m2 = IndexMap::new();
        m2.insert("a".to_string(), Value::Integer(3));
        // m2 has no "b" field
        let data = Value::Array(vec![Value::Object(m1), Value::Object(m2)]);
        let output = MarkdownWriter.write(&data).unwrap();
        assert!(output.contains("| a | b |"));
        assert!(output.contains("| 3 |  |")); // missing field → empty
    }
}
