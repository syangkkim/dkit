use std::io::Write;

use crate::format::FormatWriter;
use crate::value::Value;

/// HTML 테이블 포맷 Writer (출력 전용)
///
/// - Array<Object> → 컬럼 헤더 + 데이터 행 테이블
/// - Array<Primitive> → 단일 "value" 컬럼 테이블
/// - Single Object → key | value 2-컬럼 테이블
/// - Primitive → 단일 값 출력
pub struct HtmlWriter {
    /// 인라인 CSS 스타일 포함 여부
    styled: bool,
    /// 완전한 HTML 문서로 출력할지 여부
    full_html: bool,
}

impl HtmlWriter {
    pub fn new(styled: bool, full_html: bool) -> Self {
        Self { styled, full_html }
    }
}

impl FormatWriter for HtmlWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        Ok(render_html(value, self.styled, self.full_html))
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let output = render_html(value, self.styled, self.full_html);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }
}

fn render_html(value: &Value, styled: bool, full_html: bool) -> String {
    let table = match value {
        Value::Array(arr) if !arr.is_empty() && arr[0].as_object().is_some() => {
            render_array_of_objects(arr, styled)
        }
        Value::Array(arr) => render_array_of_primitives(arr, styled),
        Value::Object(_) => render_single_object(value, styled),
        _ => return escape_html(&format_cell_value(value)),
    };

    if full_html {
        wrap_full_html(&table, styled)
    } else {
        table
    }
}

fn wrap_full_html(table: &str, styled: bool) -> String {
    let style_block = if styled {
        "\n    <style>\n      table { border-collapse: collapse; width: 100%; font-family: sans-serif; }\n      th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n      th { background-color: #4a4a4a; color: white; }\n      tr:nth-child(even) { background-color: #f9f9f9; }\n      tr:hover { background-color: #f1f1f1; }\n    </style>"
    } else {
        ""
    };

    format!(
        "<!DOCTYPE html>\n<html>\n  <head>\n    <meta charset=\"UTF-8\">{style_block}\n  </head>\n  <body>\n{table}\n  </body>\n</html>\n"
    )
}

const TABLE_STYLE: &str =
    " style=\"border-collapse: collapse; width: 100%; font-family: sans-serif;\"";
const TH_STYLE: &str = " style=\"border: 1px solid #ddd; padding: 8px; text-align: left; background-color: #4a4a4a; color: white;\"";
const TD_STYLE: &str = " style=\"border: 1px solid #ddd; padding: 8px; text-align: left;\"";

/// Array<Object> → HTML 테이블
fn render_array_of_objects(arr: &[Value], styled: bool) -> String {
    let headers = collect_keys(arr);
    if headers.is_empty() {
        return String::new();
    }

    let table_attr = if styled { TABLE_STYLE } else { "" };
    let th_attr = if styled { TH_STYLE } else { "" };
    let td_attr = if styled { TD_STYLE } else { "" };

    let mut lines = Vec::new();
    lines.push(format!("    <table{table_attr}>"));
    lines.push("      <thead>".to_string());
    lines.push("        <tr>".to_string());
    for h in &headers {
        lines.push(format!("          <th{th_attr}>{}</th>", escape_html(h)));
    }
    lines.push("        </tr>".to_string());
    lines.push("      </thead>".to_string());
    lines.push("      <tbody>".to_string());

    for item in arr {
        if let Value::Object(obj) = item {
            lines.push("        <tr>".to_string());
            for key in &headers {
                let cell = match obj.get(key) {
                    Some(v) => escape_html(&format_cell_value(v)),
                    None => String::new(),
                };
                lines.push(format!("          <td{td_attr}>{cell}</td>"));
            }
            lines.push("        </tr>".to_string());
        }
    }

    lines.push("      </tbody>".to_string());
    lines.push("    </table>".to_string());
    lines.join("\n") + "\n"
}

/// Array<Primitive> → 단일 컬럼 HTML 테이블
fn render_array_of_primitives(arr: &[Value], styled: bool) -> String {
    let table_attr = if styled { TABLE_STYLE } else { "" };
    let th_attr = if styled { TH_STYLE } else { "" };
    let td_attr = if styled { TD_STYLE } else { "" };

    let mut lines = Vec::new();
    lines.push(format!("    <table{table_attr}>"));
    lines.push("      <thead>".to_string());
    lines.push(format!("        <tr><th{th_attr}>value</th></tr>"));
    lines.push("      </thead>".to_string());
    lines.push("      <tbody>".to_string());

    for item in arr {
        let cell = escape_html(&format_cell_value(item));
        lines.push(format!("        <tr><td{td_attr}>{cell}</td></tr>"));
    }

    lines.push("      </tbody>".to_string());
    lines.push("    </table>".to_string());
    lines.join("\n") + "\n"
}

/// 단일 Object → key | value 테이블
fn render_single_object(value: &Value, styled: bool) -> String {
    let table_attr = if styled { TABLE_STYLE } else { "" };
    let th_attr = if styled { TH_STYLE } else { "" };
    let td_attr = if styled { TD_STYLE } else { "" };

    let mut lines = Vec::new();
    lines.push(format!("    <table{table_attr}>"));
    lines.push("      <thead>".to_string());
    lines.push(format!(
        "        <tr><th{th_attr}>key</th><th{th_attr}>value</th></tr>"
    ));
    lines.push("      </thead>".to_string());
    lines.push("      <tbody>".to_string());

    if let Value::Object(obj) = value {
        for (k, v) in obj {
            lines.push(format!(
                "        <tr><td{td_attr}>{}</td><td{td_attr}>{}</td></tr>",
                escape_html(k),
                escape_html(&format_cell_value(v))
            ));
        }
    }

    lines.push("      </tbody>".to_string());
    lines.push("    </table>".to_string());
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

/// Value를 셀 표시용 문자열로 변환
fn format_cell_value(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string(&value_to_json(v)).unwrap_or_else(|_| "{...}".to_string())
        }
    }
}

/// Value를 serde_json::Value로 변환
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

/// HTML 특수문자 이스케이프
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
    fn test_array_of_objects_basic() {
        let data = Value::Array(vec![make_user("Alice", 30), make_user("Bob", 25)]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("<table>"));
        assert!(output.contains("<thead>"));
        assert!(output.contains("<tbody>"));
        assert!(output.contains("<th>name</th>"));
        assert!(output.contains("<th>age</th>"));
        assert!(output.contains("<td>Alice</td>"));
        assert!(output.contains("<td>30</td>"));
        assert!(output.contains("<td>Bob</td>"));
        assert!(output.contains("<td>25</td>"));
    }

    #[test]
    fn test_styled_output() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let output = HtmlWriter::new(true, false).write(&data).unwrap();
        assert!(output.contains("style="));
        assert!(output.contains("border-collapse"));
    }

    #[test]
    fn test_full_html_document() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let output = HtmlWriter::new(false, true).write(&data).unwrap();
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("<html>"));
        assert!(output.contains("<head>"));
        assert!(output.contains("<meta charset=\"UTF-8\">"));
        assert!(output.contains("<body>"));
        assert!(output.contains("</html>"));
        // No style block when not styled
        assert!(!output.contains("<style>"));
    }

    #[test]
    fn test_full_html_styled() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let output = HtmlWriter::new(true, true).write(&data).unwrap();
        assert!(output.contains("<!DOCTYPE html>"));
        assert!(output.contains("<style>"));
        assert!(output.contains("border-collapse"));
    }

    #[test]
    fn test_html_entity_escape() {
        let mut m = IndexMap::new();
        m.insert(
            "formula".to_string(),
            Value::String("<script>alert('xss')</script>".to_string()),
        );
        let data = Value::Array(vec![Value::Object(m)]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"));
        assert!(!output.contains("<script>"));
    }

    #[test]
    fn test_ampersand_escape() {
        let mut m = IndexMap::new();
        m.insert("text".to_string(), Value::String("A & B".to_string()));
        let data = Value::Array(vec![Value::Object(m)]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("A &amp; B"));
    }

    #[test]
    fn test_quote_escape() {
        let mut m = IndexMap::new();
        m.insert(
            "attr".to_string(),
            Value::String("say \"hello\"".to_string()),
        );
        let data = Value::Array(vec![Value::Object(m)]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("say &quot;hello&quot;"));
    }

    #[test]
    fn test_single_object() {
        let mut m = IndexMap::new();
        m.insert("host".to_string(), Value::String("localhost".to_string()));
        m.insert("port".to_string(), Value::Integer(8080));
        let data = Value::Object(m);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("<th>key</th>"));
        assert!(output.contains("<th>value</th>"));
        assert!(output.contains("<td>host</td>"));
        assert!(output.contains("<td>localhost</td>"));
        assert!(output.contains("<td>port</td>"));
        assert!(output.contains("<td>8080</td>"));
    }

    #[test]
    fn test_array_of_primitives() {
        let data = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("<th>value</th>"));
        assert!(output.contains("<td>1</td>"));
        assert!(output.contains("<td>2</td>"));
        assert!(output.contains("<td>3</td>"));
    }

    #[test]
    fn test_primitive_value() {
        let data = Value::String("hello".to_string());
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_null_value_in_cell() {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String("Alice".to_string()));
        m.insert("email".to_string(), Value::Null);
        let data = Value::Array(vec![Value::Object(m)]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("<td>null</td>"));
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
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("[&quot;a&quot;,&quot;b&quot;]"));
    }

    #[test]
    fn test_missing_fields() {
        let mut m1 = IndexMap::new();
        m1.insert("a".to_string(), Value::Integer(1));
        m1.insert("b".to_string(), Value::Integer(2));
        let mut m2 = IndexMap::new();
        m2.insert("a".to_string(), Value::Integer(3));
        let data = Value::Array(vec![Value::Object(m1), Value::Object(m2)]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("<th>a</th>"));
        assert!(output.contains("<th>b</th>"));
        // m2 has no "b" field → empty cell
        assert!(output.contains("<td></td>"));
    }

    #[test]
    fn test_empty_array() {
        let data = Value::Array(vec![]);
        let output = HtmlWriter::new(false, false).write(&data).unwrap();
        assert!(output.contains("<th>value</th>"));
    }

    #[test]
    fn test_write_to_writer() {
        let data = Value::Array(vec![make_user("Alice", 30)]);
        let mut buf = Vec::new();
        HtmlWriter::new(false, false)
            .write_to_writer(&data, &mut buf)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("<table>"));
        assert!(output.contains("Alice"));
    }
}
