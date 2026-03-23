use std::io::{Read, Write};

use indexmap::IndexMap;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader as XmlEventReader;
use quick_xml::writer::Writer as XmlEventWriter;

use crate::error::DkitError;
use crate::format::{FormatReader, FormatWriter};
use crate::value::Value;

/// 네임스페이스 접두사를 제거 (예: "ns:tag" → "tag")
fn strip_ns_prefix(name: &str) -> &str {
    match name.find(':') {
        Some(pos) => &name[pos + 1..],
        None => name,
    }
}

/// 네임스페이스 선언 속성인지 확인 (xmlns 또는 xmlns:*)
fn is_xmlns_attr(key: &str) -> bool {
    key == "xmlns" || key.starts_with("xmlns:")
}

/// XML → Value 변환
///
/// XML 구조를 Value로 매핑하는 규칙:
/// - 루트 엘리먼트 → Object { tag_name: content }
/// - 속성 → "@attr_name" 키로 저장
/// - 텍스트 내용 → "#text" 키로 저장
/// - 자식 엘리먼트 → 같은 태그가 여러 개면 Array, 하나면 단일 값
/// - 텍스트만 있는 엘리먼트 → 문자열로 단순화
/// - strip_ns: true이면 네임스페이스 접두사 제거 및 xmlns 속성 무시
fn parse_element(
    reader: &mut XmlEventReader<&[u8]>,
    strip_ns: bool,
) -> anyhow::Result<(String, Value)> {
    let mut tag_name = String::new();
    let mut attrs: IndexMap<String, Value> = IndexMap::new();
    let mut children: IndexMap<String, Vec<Value>> = IndexMap::new();
    let mut text_content = String::new();

    // 첫 이벤트는 Start 또는 Empty
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                let raw_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                tag_name = if strip_ns {
                    strip_ns_prefix(&raw_tag).to_string()
                } else {
                    raw_tag
                };
                // 속성 처리
                for attr in e.attributes() {
                    let attr = attr?;
                    let raw_key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    if strip_ns && is_xmlns_attr(&raw_key) {
                        continue;
                    }
                    let attr_name = if strip_ns {
                        strip_ns_prefix(&raw_key).to_string()
                    } else {
                        raw_key
                    };
                    let key = format!("@{}", attr_name);
                    let val = attr.unescape_value()?.to_string();
                    attrs.insert(key, infer_value(&val));
                }
                // 자식 파싱
                parse_children(reader, &mut children, &mut text_content, strip_ns)?;
                break;
            }
            Event::Empty(e) => {
                let raw_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                tag_name = if strip_ns {
                    strip_ns_prefix(&raw_tag).to_string()
                } else {
                    raw_tag
                };
                for attr in e.attributes() {
                    let attr = attr?;
                    let raw_key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    if strip_ns && is_xmlns_attr(&raw_key) {
                        continue;
                    }
                    let attr_name = if strip_ns {
                        strip_ns_prefix(&raw_key).to_string()
                    } else {
                        raw_key
                    };
                    let key = format!("@{}", attr_name);
                    let val = attr.unescape_value()?.to_string();
                    attrs.insert(key, infer_value(&val));
                }
                break;
            }
            Event::Eof => {
                anyhow::bail!("Unexpected EOF while parsing XML element");
            }
            Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_) => continue,
            Event::Text(t) => {
                let s = t.unescape()?.to_string();
                if !s.trim().is_empty() {
                    text_content.push_str(&s);
                }
            }
            Event::End(_) => break,
            Event::CData(cd) => {
                text_content.push_str(&String::from_utf8_lossy(&cd));
            }
        }
    }

    // 결과 구성
    let value = build_element_value(attrs, children, text_content);
    Ok((tag_name, value))
}

/// 엘리먼트의 자식을 파싱 (End 이벤트를 만나면 종료)
fn parse_children(
    reader: &mut XmlEventReader<&[u8]>,
    children: &mut IndexMap<String, Vec<Value>>,
    text_content: &mut String,
    strip_ns: bool,
) -> anyhow::Result<()> {
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                let raw_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let child_tag = if strip_ns {
                    strip_ns_prefix(&raw_tag).to_string()
                } else {
                    raw_tag
                };
                let mut child_attrs: IndexMap<String, Value> = IndexMap::new();
                let mut child_children: IndexMap<String, Vec<Value>> = IndexMap::new();
                let mut child_text = String::new();

                for attr in e.attributes() {
                    let attr = attr?;
                    let raw_key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    if strip_ns && is_xmlns_attr(&raw_key) {
                        continue;
                    }
                    let attr_name = if strip_ns {
                        strip_ns_prefix(&raw_key).to_string()
                    } else {
                        raw_key
                    };
                    let key = format!("@{}", attr_name);
                    let val = attr.unescape_value()?.to_string();
                    child_attrs.insert(key, infer_value(&val));
                }

                parse_children(reader, &mut child_children, &mut child_text, strip_ns)?;

                let child_value = build_element_value(child_attrs, child_children, child_text);
                children.entry(child_tag).or_default().push(child_value);
            }
            Event::Empty(e) => {
                let raw_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let child_tag = if strip_ns {
                    strip_ns_prefix(&raw_tag).to_string()
                } else {
                    raw_tag
                };
                let mut child_attrs: IndexMap<String, Value> = IndexMap::new();

                for attr in e.attributes() {
                    let attr = attr?;
                    let raw_key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    if strip_ns && is_xmlns_attr(&raw_key) {
                        continue;
                    }
                    let attr_name = if strip_ns {
                        strip_ns_prefix(&raw_key).to_string()
                    } else {
                        raw_key
                    };
                    let key = format!("@{}", attr_name);
                    let val = attr.unescape_value()?.to_string();
                    child_attrs.insert(key, infer_value(&val));
                }

                let child_value = if child_attrs.is_empty() {
                    Value::Null
                } else {
                    Value::Object(child_attrs)
                };
                children.entry(child_tag).or_default().push(child_value);
            }
            Event::Text(t) => {
                let s = t.unescape()?.to_string();
                if !s.trim().is_empty() {
                    text_content.push_str(s.trim());
                }
            }
            Event::CData(cd) => {
                text_content.push_str(&String::from_utf8_lossy(&cd));
            }
            Event::End(_) => break,
            Event::Eof => break,
            Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_) => continue,
        }
    }
    Ok(())
}

/// 속성, 자식, 텍스트로부터 Value 구성
fn build_element_value(
    attrs: IndexMap<String, Value>,
    children: IndexMap<String, Vec<Value>>,
    text_content: String,
) -> Value {
    let has_attrs = !attrs.is_empty();
    let has_children = !children.is_empty();
    let has_text = !text_content.is_empty();

    // 텍스트만 있는 단순 엘리먼트 → 값으로 단순화
    if !has_attrs && !has_children && has_text {
        return infer_value(&text_content);
    }

    // 아무것도 없는 빈 엘리먼트
    if !has_attrs && !has_children && !has_text {
        return Value::Null;
    }

    // Object 구성
    let mut map: IndexMap<String, Value> = IndexMap::new();

    // 속성 추가
    for (k, v) in attrs {
        map.insert(k, v);
    }

    // 자식 엘리먼트 추가
    for (tag, values) in children {
        if values.len() == 1 {
            map.insert(tag, values.into_iter().next().unwrap());
        } else {
            map.insert(tag, Value::Array(values));
        }
    }

    // 텍스트가 있으면 #text로 추가
    if has_text {
        map.insert("#text".to_string(), infer_value(&text_content));
    }

    Value::Object(map)
}

/// 문자열 → 적절한 Value 타입 추론
fn infer_value(s: &str) -> Value {
    if s.eq_ignore_ascii_case("null") || s.eq_ignore_ascii_case("~") {
        return Value::Null;
    }
    if s.eq_ignore_ascii_case("true") {
        return Value::Bool(true);
    }
    if s.eq_ignore_ascii_case("false") {
        return Value::Bool(false);
    }
    if let Ok(i) = s.parse::<i64>() {
        return Value::Integer(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        return Value::Float(f);
    }
    Value::String(s.to_string())
}

// ============== Value → XML 변환 ==============

/// Value를 XML 이벤트로 변환하여 Writer에 작성
fn write_element(
    writer: &mut XmlEventWriter<Vec<u8>>,
    tag: &str,
    value: &Value,
) -> anyhow::Result<()> {
    match value {
        Value::Null => {
            // 빈 엘리먼트
            let elem = BytesStart::new(tag);
            writer.write_event(Event::Empty(elem))?;
        }
        Value::Bool(b) => {
            let elem = BytesStart::new(tag);
            writer.write_event(Event::Start(elem))?;
            writer.write_event(Event::Text(BytesText::new(&b.to_string())))?;
            writer.write_event(Event::End(BytesEnd::new(tag)))?;
        }
        Value::Integer(n) => {
            let elem = BytesStart::new(tag);
            writer.write_event(Event::Start(elem))?;
            writer.write_event(Event::Text(BytesText::new(&n.to_string())))?;
            writer.write_event(Event::End(BytesEnd::new(tag)))?;
        }
        Value::Float(f) => {
            let text = if f.is_nan() || f.is_infinite() {
                "null".to_string()
            } else {
                f.to_string()
            };
            let elem = BytesStart::new(tag);
            writer.write_event(Event::Start(elem))?;
            writer.write_event(Event::Text(BytesText::new(&text)))?;
            writer.write_event(Event::End(BytesEnd::new(tag)))?;
        }
        Value::String(s) => {
            let elem = BytesStart::new(tag);
            writer.write_event(Event::Start(elem))?;
            writer.write_event(Event::Text(BytesText::new(s)))?;
            writer.write_event(Event::End(BytesEnd::new(tag)))?;
        }
        Value::Array(arr) => {
            // 배열의 각 항목을 같은 태그로 반복
            for item in arr {
                write_element(writer, tag, item)?;
            }
        }
        Value::Object(map) => {
            let mut elem = BytesStart::new(tag);

            // @로 시작하는 키는 속성으로
            for (k, v) in map {
                if let Some(attr_name) = k.strip_prefix('@') {
                    let attr_val = match v {
                        Value::String(s) => s.clone(),
                        Value::Integer(n) => n.to_string(),
                        Value::Float(f) => f.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "".to_string(),
                        _ => serde_json::to_string(v).unwrap_or_default(),
                    };
                    elem.push_attribute((attr_name, attr_val.as_str()));
                }
            }

            // 속성이 아닌 키가 있는지 확인
            let has_content = map.iter().any(|(k, _)| !k.starts_with('@'));

            if !has_content {
                writer.write_event(Event::Empty(elem))?;
            } else {
                writer.write_event(Event::Start(elem))?;

                for (k, v) in map {
                    if k.starts_with('@') {
                        continue;
                    }
                    if k == "#text" {
                        // 텍스트 내용
                        let text = match v {
                            Value::String(s) => s.clone(),
                            Value::Integer(n) => n.to_string(),
                            Value::Float(f) => f.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => String::new(),
                        };
                        writer.write_event(Event::Text(BytesText::new(&text)))?;
                    } else {
                        write_element(writer, k, v)?;
                    }
                }

                writer.write_event(Event::End(BytesEnd::new(tag)))?;
            }
        }
    }
    Ok(())
}

/// XML 포맷 Reader
///
/// `strip_namespaces` 옵션:
/// - `true` (기본값): 네임스페이스 접두사를 제거하고 xmlns 속성을 무시
///   예: `<ns:tag xmlns:ns="...">` → `{ "tag": ... }`
/// - `false`: 네임스페이스 접두사와 xmlns 속성을 그대로 보존
///   예: `<ns:tag xmlns:ns="...">` → `{ "ns:tag": { "@xmlns:ns": "..." } }`
pub struct XmlReader {
    strip_namespaces: bool,
}

impl XmlReader {
    #[allow(dead_code)]
    pub fn new(strip_namespaces: bool) -> Self {
        Self { strip_namespaces }
    }
}

impl Default for XmlReader {
    fn default() -> Self {
        Self {
            strip_namespaces: true,
        }
    }
}

impl FormatReader for XmlReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        let mut reader = XmlEventReader::from_str(input);
        reader.config_mut().trim_text(true);

        let (tag, value) = parse_element(&mut reader, self.strip_namespaces).map_err(|e| {
            DkitError::ParseError {
                format: "XML".to_string(),
                source: e.into(),
            }
        })?;

        // 루트 엘리먼트를 { tag_name: value } 형태로 반환
        let mut root = IndexMap::new();
        root.insert(tag, value);
        Ok(Value::Object(root))
    }

    fn read_from_reader(&self, mut reader: impl Read) -> anyhow::Result<Value> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        self.read(&buf)
    }
}

/// XML 포맷 Writer
pub struct XmlWriter {
    pretty: bool,
    root_element: String,
}

impl XmlWriter {
    pub fn new(pretty: bool, root_element: Option<String>) -> Self {
        Self {
            pretty,
            root_element: root_element.unwrap_or_else(|| "root".to_string()),
        }
    }
}

impl Default for XmlWriter {
    fn default() -> Self {
        Self {
            pretty: true,
            root_element: "root".to_string(),
        }
    }
}

impl FormatWriter for XmlWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let mut buf = Vec::new();
        self.write_to_writer(value, &mut buf)?;
        Ok(String::from_utf8(buf)?)
    }

    fn write_to_writer(&self, value: &Value, writer: impl Write) -> anyhow::Result<()> {
        let xml_buf = Vec::new();
        let mut xml_writer = if self.pretty {
            XmlEventWriter::new_with_indent(xml_buf, b' ', 2)
        } else {
            XmlEventWriter::new(xml_buf)
        };

        // XML 선언
        xml_writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            None,
        )))?;

        let root_tag = &self.root_element;
        match value {
            Value::Object(map) => {
                if map.len() == 1 {
                    // 단일 루트: { "root": ... } → <root>...</root>
                    let (tag, val) = map.iter().next().unwrap();
                    write_element(&mut xml_writer, tag, val)?;
                } else {
                    // 여러 키 → 루트 엘리먼트로 감싸기
                    write_element(&mut xml_writer, root_tag, value)?;
                }
            }
            Value::Array(_) => {
                // 배열 → <root><item>...</item></root> 형태
                let mut wrapper = IndexMap::new();
                wrapper.insert("item".to_string(), value.clone());
                let wrapped = Value::Object(wrapper);
                write_element(&mut xml_writer, root_tag, &wrapped)?;
            }
            _ => {
                // 단순 값 → <root>value</root>
                write_element(&mut xml_writer, root_tag, value)?;
            }
        }

        let xml_buf = xml_writer.into_inner();

        let mut dest = writer;
        dest.write_all(&xml_buf)?;
        if self.pretty {
            dest.write_all(b"\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{FormatReader, FormatWriter};

    // --- XmlReader 기본 테스트 ---

    #[test]
    fn test_read_simple_element() {
        let xml = "<root><name>dkit</name><version>1</version></root>";
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let obj = root.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert_eq!(obj.get("version"), Some(&Value::Integer(1)));
    }

    #[test]
    fn test_read_attributes() {
        let xml = r#"<user id="42" active="true"><name>Alice</name></user>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let user = value.as_object().unwrap().get("user").unwrap();
        let obj = user.as_object().unwrap();
        assert_eq!(obj.get("@id"), Some(&Value::Integer(42)));
        assert_eq!(obj.get("@active"), Some(&Value::Bool(true)));
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
    }

    #[test]
    fn test_read_repeated_elements_as_array() {
        let xml = "<users><user>Alice</user><user>Bob</user><user>Charlie</user></users>";
        let value = XmlReader::default().read(xml).unwrap();
        let users = value.as_object().unwrap().get("users").unwrap();
        let arr = users
            .as_object()
            .unwrap()
            .get("user")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::String("Alice".to_string()));
        assert_eq!(arr[1], Value::String("Bob".to_string()));
    }

    #[test]
    fn test_read_nested() {
        let xml = "<config><db><host>localhost</host><port>5432</port></db></config>";
        let value = XmlReader::default().read(xml).unwrap();
        let config = value.as_object().unwrap().get("config").unwrap();
        let db = config.as_object().unwrap().get("db").unwrap();
        let obj = db.as_object().unwrap();
        assert_eq!(
            obj.get("host"),
            Some(&Value::String("localhost".to_string()))
        );
        assert_eq!(obj.get("port"), Some(&Value::Integer(5432)));
    }

    #[test]
    fn test_read_empty_element() {
        let xml = "<root><empty/></root>";
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let obj = root.as_object().unwrap();
        assert_eq!(obj.get("empty"), Some(&Value::Null));
    }

    #[test]
    fn test_read_text_with_attributes() {
        let xml = r#"<item price="9.99">Widget</item>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let item = value.as_object().unwrap().get("item").unwrap();
        let obj = item.as_object().unwrap();
        assert_eq!(obj.get("@price"), Some(&Value::Float(9.99)));
        assert_eq!(obj.get("#text"), Some(&Value::String("Widget".to_string())));
    }

    #[test]
    fn test_read_xml_declaration() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><root><value>42</value></root>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        assert_eq!(
            root.as_object().unwrap().get("value"),
            Some(&Value::Integer(42))
        );
    }

    #[test]
    fn test_read_unicode() {
        let xml = "<root><greeting>안녕하세요</greeting><emoji>🎉</emoji></root>";
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let obj = root.as_object().unwrap();
        assert_eq!(
            obj.get("greeting"),
            Some(&Value::String("안녕하세요".to_string()))
        );
        assert_eq!(obj.get("emoji"), Some(&Value::String("🎉".to_string())));
    }

    #[test]
    fn test_read_type_inference() {
        let xml =
            "<data><int>42</int><float>3.14</float><bool>true</bool><text>hello</text></data>";
        let value = XmlReader::default().read(xml).unwrap();
        let data = value.as_object().unwrap().get("data").unwrap();
        let obj = data.as_object().unwrap();
        assert_eq!(obj.get("int"), Some(&Value::Integer(42)));
        assert_eq!(obj.get("float"), Some(&Value::Float(3.14)));
        assert_eq!(obj.get("bool"), Some(&Value::Bool(true)));
        assert_eq!(obj.get("text"), Some(&Value::String("hello".to_string())));
    }

    #[test]
    fn test_read_invalid_xml() {
        // 빈 입력은 루트 엘리먼트가 없으므로 에러
        let result = XmlReader::default().read("");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_reader() {
        let xml = b"<root><x>1</x></root>" as &[u8];
        let value = XmlReader::default().read_from_reader(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        assert_eq!(root.as_object().unwrap().get("x"), Some(&Value::Integer(1)));
    }

    // --- XmlWriter 기본 테스트 ---

    #[test]
    fn test_write_simple_object() {
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert("name".to_string(), Value::String("dkit".to_string()));
        inner.insert("version".to_string(), Value::Integer(1));
        map.insert("root".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains("<root>"));
        assert!(output.contains("<name>dkit</name>"));
        assert!(output.contains("<version>1</version>"));
        assert!(output.contains("</root>"));
    }

    #[test]
    fn test_write_with_attributes() {
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert("@id".to_string(), Value::Integer(42));
        inner.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("user".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains(r#"<user id="42">"#));
        assert!(output.contains("<name>Alice</name>"));
    }

    #[test]
    fn test_write_array() {
        let arr = Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert("item".to_string(), arr);
        map.insert("root".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains("<item>a</item>"));
        assert!(output.contains("<item>b</item>"));
    }

    #[test]
    fn test_write_null() {
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert("empty".to_string(), Value::Null);
        map.insert("root".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains("<empty/>"));
    }

    #[test]
    fn test_write_pretty() {
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert("x".to_string(), Value::Integer(1));
        map.insert("root".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(true, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains('\n'));
    }

    #[test]
    fn test_write_primitive_root() {
        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Integer(42)).unwrap();
        assert!(output.contains("<root>42</root>"));
    }

    #[test]
    fn test_write_bool() {
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert("flag".to_string(), Value::Bool(true));
        map.insert("root".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains("<flag>true</flag>"));
    }

    // --- 왕복(roundtrip) 테스트 ---

    #[test]
    fn test_roundtrip_simple() {
        let xml = "<config><host>localhost</host><port>8080</port></config>";
        let value = XmlReader::default().read(xml).unwrap();
        let writer = XmlWriter::new(false, None);
        let output = writer.write(&value).unwrap();
        let value2 = XmlReader::default().read(&output).unwrap();
        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_with_attributes() {
        let xml = r#"<server host="localhost" port="8080"><name>main</name></server>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let writer = XmlWriter::new(false, None);
        let output = writer.write(&value).unwrap();
        let value2 = XmlReader::default().read(&output).unwrap();
        assert_eq!(value, value2);
    }

    #[test]
    fn test_roundtrip_nested() {
        let xml = "<root><a><b><c>deep</c></b></a></root>";
        let value = XmlReader::default().read(xml).unwrap();
        let writer = XmlWriter::new(false, None);
        let output = writer.write(&value).unwrap();
        let value2 = XmlReader::default().read(&output).unwrap();
        assert_eq!(value, value2);
    }

    #[test]
    fn test_write_to_writer() {
        let mut map = IndexMap::new();
        map.insert("root".to_string(), Value::String("hello".to_string()));

        let writer = XmlWriter::new(false, None);
        let mut buf = Vec::new();
        writer
            .write_to_writer(&Value::Object(map), &mut buf)
            .unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("<root>hello</root>"));
    }

    // --- 특수 문자 이스케이프 ---

    #[test]
    fn test_special_chars_escaped() {
        let xml = "<root><text>&lt;hello&gt; &amp; &quot;world&quot;</text></root>";
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let text = root.as_object().unwrap().get("text").unwrap();
        assert_eq!(text, &Value::String("<hello> & \"world\"".to_string()));
    }

    #[test]
    fn test_write_special_chars() {
        let mut map = IndexMap::new();
        let mut inner = IndexMap::new();
        inner.insert(
            "text".to_string(),
            Value::String("<hello> & \"world\"".to_string()),
        );
        map.insert("root".to_string(), Value::Object(inner));

        let writer = XmlWriter::new(false, None);
        let output = writer.write(&Value::Object(map)).unwrap();
        // quick-xml은 자동으로 이스케이프 처리
        assert!(output.contains("&lt;hello&gt;"));
        assert!(output.contains("&amp;"));
    }

    #[test]
    fn test_empty_element_with_attrs() {
        let xml = r#"<root><link href="https://example.com"/></root>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let link = root.as_object().unwrap().get("link").unwrap();
        let obj = link.as_object().unwrap();
        assert_eq!(
            obj.get("@href"),
            Some(&Value::String("https://example.com".to_string()))
        );
    }

    // --- 네임스페이스 처리 테스트 ---

    #[test]
    fn test_namespace_strip_default() {
        // 기본(strip_namespaces=true): 네임스페이스 접두사 제거, xmlns 무시
        let xml = r#"<ns:root xmlns:ns="http://example.com"><ns:name>dkit</ns:name></ns:root>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap();
        assert!(root.contains_key("root"));
        let inner = root.get("root").unwrap().as_object().unwrap();
        assert_eq!(inner.get("name"), Some(&Value::String("dkit".to_string())));
        // xmlns 속성은 제거됨
        assert!(!inner.contains_key("@xmlns:ns"));
    }

    #[test]
    fn test_namespace_preserve() {
        // strip_namespaces=false: 접두사와 xmlns 속성 보존
        let xml = r#"<ns:root xmlns:ns="http://example.com"><ns:name>dkit</ns:name></ns:root>"#;
        let value = XmlReader::new(false).read(xml).unwrap();
        let root = value.as_object().unwrap();
        assert!(root.contains_key("ns:root"));
        let inner = root.get("ns:root").unwrap().as_object().unwrap();
        assert_eq!(
            inner.get("@xmlns:ns"),
            Some(&Value::String("http://example.com".to_string()))
        );
        assert_eq!(
            inner.get("ns:name"),
            Some(&Value::String("dkit".to_string()))
        );
    }

    #[test]
    fn test_namespace_strip_multiple_ns() {
        let xml = r#"<root xmlns:a="http://a.com" xmlns:b="http://b.com">
            <a:item>1</a:item><b:item>2</b:item>
        </root>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let obj = root.as_object().unwrap();
        // 두 네임스페이스의 item이 같은 키로 합쳐짐
        let items = obj.get("item").unwrap().as_array().unwrap();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_namespace_strip_default_xmlns() {
        // 기본 xmlns (접두사 없는 네임스페이스)도 무시
        let xml = r#"<root xmlns="http://example.com"><name>dkit</name></root>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let obj = root.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("dkit".to_string())));
        assert!(!obj.contains_key("@xmlns"));
    }

    #[test]
    fn test_namespace_strip_attributes() {
        // 속성의 네임스페이스 접두사도 제거
        let xml = r#"<root xmlns:x="http://x.com"><item x:id="42">val</item></root>"#;
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let item = root.as_object().unwrap().get("item").unwrap();
        let obj = item.as_object().unwrap();
        assert_eq!(obj.get("@id"), Some(&Value::Integer(42)));
        assert_eq!(obj.get("#text"), Some(&Value::String("val".to_string())));
    }

    #[test]
    fn test_namespace_preserve_attributes() {
        let xml = r#"<root xmlns:x="http://x.com"><item x:id="42">val</item></root>"#;
        let value = XmlReader::new(false).read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let item = root.as_object().unwrap().get("item").unwrap();
        let obj = item.as_object().unwrap();
        assert_eq!(obj.get("@x:id"), Some(&Value::Integer(42)));
        assert_eq!(
            obj.get("@xmlns:x"),
            None // xmlns:x는 root 레벨에 있으므로 item에는 없음
        );
    }

    #[test]
    fn test_namespace_no_prefix_no_effect() {
        // 네임스페이스가 없는 XML은 strip_ns 설정에 관계없이 동일
        let xml = "<root><name>dkit</name></root>";
        let v1 = XmlReader::new(true).read(xml).unwrap();
        let v2 = XmlReader::new(false).read(xml).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_cdata_section() {
        let xml = "<root><data><![CDATA[<script>alert('hi')</script>]]></data></root>";
        let value = XmlReader::default().read(xml).unwrap();
        let root = value.as_object().unwrap().get("root").unwrap();
        let data = root.as_object().unwrap().get("data").unwrap();
        assert_eq!(
            data,
            &Value::String("<script>alert('hi')</script>".to_string())
        );
    }

    // --- root_element 옵션 테스트 ---

    #[test]
    fn test_write_custom_root_element_multi_key() {
        let mut map = IndexMap::new();
        map.insert("name".to_string(), Value::String("dkit".to_string()));
        map.insert("version".to_string(), Value::Integer(1));

        let writer = XmlWriter::new(false, Some("config".to_string()));
        let output = writer.write(&Value::Object(map)).unwrap();
        assert!(output.contains("<config>"));
        assert!(output.contains("</config>"));
        assert!(!output.contains("<root>"));
    }

    #[test]
    fn test_write_custom_root_element_array() {
        let arr = Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);

        let writer = XmlWriter::new(false, Some("items".to_string()));
        let output = writer.write(&arr).unwrap();
        assert!(output.contains("<items>"));
        assert!(output.contains("</items>"));
        assert!(!output.contains("<root>"));
    }

    #[test]
    fn test_write_custom_root_element_primitive() {
        let writer = XmlWriter::new(false, Some("value".to_string()));
        let output = writer.write(&Value::Integer(42)).unwrap();
        assert!(output.contains("<value>42</value>"));
        assert!(!output.contains("<root>"));
    }

    #[test]
    fn test_write_default_root_element() {
        // 단일 키 Object는 root_element 설정과 무관하게 해당 키를 사용
        let mut map = IndexMap::new();
        map.insert("data".to_string(), Value::Integer(1));

        let writer = XmlWriter::new(false, Some("custom".to_string()));
        let output = writer.write(&Value::Object(map)).unwrap();
        // 단일 키이므로 "data"를 루트로 사용
        assert!(output.contains("<data>1</data>"));
    }
}
