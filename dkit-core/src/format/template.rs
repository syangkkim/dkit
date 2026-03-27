use std::io::Write;

use crate::format::{FormatOptions, FormatWriter};
use crate::value::Value;

/// Writer that renders each record through a Tera template.
pub struct TemplateWriter {
    options: FormatOptions,
}

impl TemplateWriter {
    pub fn new(options: FormatOptions) -> Self {
        Self { options }
    }

    fn resolve_template_string(&self) -> anyhow::Result<String> {
        if let Some(ref tpl) = self.options.template {
            Ok(tpl.clone())
        } else if let Some(ref path) = self.options.template_file {
            std::fs::read_to_string(path)
                .map_err(|e| anyhow::anyhow!("Failed to read template file '{}': {}", path, e))
        } else {
            anyhow::bail!("Template format requires --template <STRING> or --template-file <PATH>")
        }
    }

    fn render_value(&self, tera: &tera::Tera, value: &Value) -> anyhow::Result<String> {
        let mut context = tera::Context::new();
        match value {
            Value::Object(map) => {
                for (k, v) in map {
                    context.insert(k, &value_to_tera(v));
                }
            }
            _ => {
                context.insert("value", &value_to_tera(value));
            }
        }
        tera.render("template", &context)
            .map_err(|e| anyhow::anyhow!("Template render error: {}", e))
    }
}

impl FormatWriter for TemplateWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String> {
        let tpl_str = self.resolve_template_string()?;
        let mut tera = tera::Tera::default();
        register_helpers(&mut tera);
        tera.add_raw_template("template", &tpl_str)
            .map_err(|e| anyhow::anyhow!("Template parse error: {}", e))?;

        match value {
            Value::Array(items) => {
                let mut output = String::new();
                for item in items {
                    let rendered = self.render_value(&tera, item)?;
                    output.push_str(&rendered);
                    output.push('\n');
                }
                Ok(output)
            }
            _ => {
                let mut output = self.render_value(&tera, value)?;
                output.push('\n');
                Ok(output)
            }
        }
    }

    fn write_to_writer(&self, value: &Value, mut writer: impl Write) -> anyhow::Result<()> {
        let output = self.write(value)?;
        writer.write_all(output.as_bytes())?;
        Ok(())
    }
}

/// Convert a dkit Value to a serde_json::Value for Tera context insertion.
fn value_to_tera(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Integer(i) => serde_json::json!(*i),
        Value::Float(f) => serde_json::json!(*f),
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(value_to_tera).collect()),
        Value::Object(map) => {
            let obj: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), value_to_tera(v)))
                .collect();
            serde_json::Value::Object(obj)
        }
    }
}

/// Register built-in helper functions (upper, lower, default).
fn register_helpers(tera: &mut tera::Tera) {
    tera.register_filter(
        "upper",
        |value: &tera::Value, _args: &std::collections::HashMap<String, tera::Value>| match value
            .as_str()
        {
            Some(s) => Ok(tera::Value::String(s.to_uppercase())),
            None => Ok(value.clone()),
        },
    );
    tera.register_filter(
        "lower",
        |value: &tera::Value, _args: &std::collections::HashMap<String, tera::Value>| match value
            .as_str()
        {
            Some(s) => Ok(tera::Value::String(s.to_lowercase())),
            None => Ok(value.clone()),
        },
    );
    // "default" is already a built-in Tera filter, no need to register it.
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn make_record(pairs: Vec<(&str, Value)>) -> Value {
        let map: IndexMap<String, Value> =
            pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        Value::Object(map)
    }

    #[test]
    fn test_single_record() {
        let value = make_record(vec![
            ("name", Value::String("Alice".into())),
            ("email", Value::String("alice@example.com".into())),
        ]);

        let opts = FormatOptions {
            template: Some("{{ name }} <{{ email }}>".into()),
            ..Default::default()
        };
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value).unwrap();
        assert_eq!(result.trim(), "Alice <alice@example.com>");
    }

    #[test]
    fn test_array_of_records() {
        let value = Value::Array(vec![
            make_record(vec![("name", Value::String("Alice".into()))]),
            make_record(vec![("name", Value::String("Bob".into()))]),
        ]);

        let opts = FormatOptions {
            template: Some("Hello, {{ name }}!".into()),
            ..Default::default()
        };
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value).unwrap();
        assert_eq!(result.trim(), "Hello, Alice!\nHello, Bob!");
    }

    #[test]
    fn test_upper_filter() {
        let value = make_record(vec![("name", Value::String("alice".into()))]);

        let opts = FormatOptions {
            template: Some("{{ name | upper }}".into()),
            ..Default::default()
        };
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value).unwrap();
        assert_eq!(result.trim(), "ALICE");
    }

    #[test]
    fn test_lower_filter() {
        let value = make_record(vec![("name", Value::String("ALICE".into()))]);

        let opts = FormatOptions {
            template: Some("{{ name | lower }}".into()),
            ..Default::default()
        };
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value).unwrap();
        assert_eq!(result.trim(), "alice");
    }

    #[test]
    fn test_default_filter() {
        let value = make_record(vec![("name", Value::String("Alice".into()))]);

        let opts = FormatOptions {
            template: Some(r#"{{ missing | default(value="N/A") }}"#.into()),
            ..Default::default()
        };
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value).unwrap();
        assert_eq!(result.trim(), "N/A");
    }

    #[test]
    fn test_integer_and_float() {
        let value = make_record(vec![
            ("count", Value::Integer(42)),
            ("price", Value::Float(9.99)),
        ]);

        let opts = FormatOptions {
            template: Some("Count: {{ count }}, Price: {{ price }}".into()),
            ..Default::default()
        };
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value).unwrap();
        assert_eq!(result.trim(), "Count: 42, Price: 9.99");
    }

    #[test]
    fn test_no_template_error() {
        let value = make_record(vec![("name", Value::String("Alice".into()))]);
        let opts = FormatOptions::default();
        let writer = TemplateWriter::new(opts);
        let result = writer.write(&value);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("--template"));
    }
}
