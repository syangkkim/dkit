use std::io::{BufRead, Read};

use indexmap::IndexMap;
use regex::Regex;

use crate::error::DkitError;
use crate::format::FormatReader;
use crate::value::Value;

/// Predefined log format names.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogFormat {
    ApacheCombined,
    ApacheCommon,
    Nginx,
    Syslog,
}

/// Options for the log reader.
#[derive(Debug, Clone)]
pub struct LogReaderOptions {
    /// How to handle lines that fail to parse.
    pub on_error: LogParseErrorMode,
}

impl Default for LogReaderOptions {
    fn default() -> Self {
        Self {
            on_error: LogParseErrorMode::Skip,
        }
    }
}

/// How to handle lines that fail to parse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogParseErrorMode {
    /// Skip lines that fail to parse.
    Skip,
    /// Include failed lines as objects with a `_raw` field.
    Raw,
}

/// Log format reader that parses log lines into structured data.
///
/// Supports predefined formats (Apache Combined/Common, nginx, syslog) and
/// custom patterns using `{field_name}` placeholders.
pub struct LogReader {
    pattern: CompiledPattern,
    options: LogReaderOptions,
}

struct CompiledPattern {
    regex: Regex,
    field_names: Vec<String>,
}

impl LogReader {
    /// Create a new LogReader from a `--log-format` string.
    ///
    /// Accepts predefined names (`apache`, `apache-combined`, `apache-common`,
    /// `nginx`, `syslog`) or a custom pattern with `{field}` placeholders.
    pub fn new(format_str: &str, options: LogReaderOptions) -> anyhow::Result<Self> {
        let pattern = match format_str.to_lowercase().as_str() {
            "apache" | "apache-combined" | "combined" => {
                compile_predefined(LogFormat::ApacheCombined)?
            }
            "apache-common" | "common" => compile_predefined(LogFormat::ApacheCommon)?,
            "nginx" => compile_predefined(LogFormat::Nginx)?,
            "syslog" => compile_predefined(LogFormat::Syslog)?,
            _ => compile_custom_pattern(format_str)?,
        };
        Ok(Self { pattern, options })
    }

    fn parse_lines(&self, input: &str) -> anyhow::Result<Value> {
        let mut items = Vec::new();
        for line in input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            match self.parse_line(trimmed) {
                Some(obj) => items.push(obj),
                None => match self.options.on_error {
                    LogParseErrorMode::Skip => {}
                    LogParseErrorMode::Raw => {
                        let mut map = IndexMap::new();
                        map.insert("_raw".to_string(), Value::String(trimmed.to_string()));
                        items.push(Value::Object(map));
                    }
                },
            }
        }
        Ok(Value::Array(items))
    }

    fn parse_line(&self, line: &str) -> Option<Value> {
        let caps = self.pattern.regex.captures(line)?;
        let mut map = IndexMap::new();
        for (i, name) in self.pattern.field_names.iter().enumerate() {
            let val = caps
                .get(i + 1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            // Try to parse as integer, then float, otherwise keep as string.
            // Treat "-" as null (common in Apache logs for missing fields).
            if val == "-" {
                map.insert(name.clone(), Value::Null);
            } else if let Ok(n) = val.parse::<i64>() {
                map.insert(name.clone(), Value::Integer(n));
            } else if let Ok(f) = val.parse::<f64>() {
                if val.contains('.') {
                    map.insert(name.clone(), Value::Float(f));
                } else {
                    map.insert(name.clone(), Value::String(val));
                }
            } else {
                map.insert(name.clone(), Value::String(val));
            }
        }
        Some(Value::Object(map))
    }
}

impl FormatReader for LogReader {
    fn read(&self, input: &str) -> anyhow::Result<Value> {
        self.parse_lines(input)
    }

    fn read_from_reader(&self, reader: impl Read) -> anyhow::Result<Value> {
        let buf_reader = std::io::BufReader::new(reader);
        let mut items = Vec::new();
        for line_result in buf_reader.lines() {
            let line = line_result.map_err(|e| DkitError::ParseError {
                format: "Log".to_string(),
                source: Box::new(e),
            })?;
            let trimmed = line.trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            match self.parse_line(&trimmed) {
                Some(obj) => items.push(obj),
                None => match self.options.on_error {
                    LogParseErrorMode::Skip => {}
                    LogParseErrorMode::Raw => {
                        let mut map = IndexMap::new();
                        map.insert("_raw".to_string(), Value::String(trimmed));
                        items.push(Value::Object(map));
                    }
                },
            }
        }
        Ok(Value::Array(items))
    }
}

/// Compile a predefined log format into a regex pattern.
fn compile_predefined(format: LogFormat) -> anyhow::Result<CompiledPattern> {
    match format {
        LogFormat::ApacheCombined => {
            // Apache Combined Log Format:
            // %h %l %u %t "%r" %>s %b "%{Referer}i" "%{User-agent}i"
            // Example: 127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08"
            let regex = Regex::new(
                r#"^(\S+) (\S+) (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\S+) "([^"]*)" "([^"]*)"$"#,
            )?;
            Ok(CompiledPattern {
                regex,
                field_names: vec![
                    "remote_host".into(),
                    "ident".into(),
                    "remote_user".into(),
                    "timestamp".into(),
                    "request".into(),
                    "status".into(),
                    "bytes".into(),
                    "referer".into(),
                    "user_agent".into(),
                ],
            })
        }
        LogFormat::ApacheCommon => {
            // Apache Common Log Format:
            // %h %l %u %t "%r" %>s %b
            let regex = Regex::new(r#"^(\S+) (\S+) (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\S+)$"#)?;
            Ok(CompiledPattern {
                regex,
                field_names: vec![
                    "remote_host".into(),
                    "ident".into(),
                    "remote_user".into(),
                    "timestamp".into(),
                    "request".into(),
                    "status".into(),
                    "bytes".into(),
                ],
            })
        }
        LogFormat::Nginx => {
            // nginx default combined log format (same structure as Apache Combined)
            let regex = Regex::new(
                r#"^(\S+) - (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\S+) "([^"]*)" "([^"]*)"$"#,
            )?;
            Ok(CompiledPattern {
                regex,
                field_names: vec![
                    "remote_addr".into(),
                    "remote_user".into(),
                    "time_local".into(),
                    "request".into(),
                    "status".into(),
                    "body_bytes_sent".into(),
                    "http_referer".into(),
                    "http_user_agent".into(),
                ],
            })
        }
        LogFormat::Syslog => {
            // RFC 3164 syslog format:
            // <priority>timestamp hostname app[pid]: message
            // Or without priority: timestamp hostname app[pid]: message
            // Example: Mar 10 13:55:36 myhost sshd[1234]: Accepted publickey
            let regex = Regex::new(
                r"^(?:<(\d+)>)?(\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(\S+)\s+(\S+?)(?:\[(\d+)\])?:\s+(.+)$",
            )?;
            Ok(CompiledPattern {
                regex,
                field_names: vec![
                    "priority".into(),
                    "timestamp".into(),
                    "hostname".into(),
                    "app_name".into(),
                    "pid".into(),
                    "message".into(),
                ],
            })
        }
    }
}

/// Compile a custom pattern string with `{field}` placeholders into a regex.
///
/// Supported placeholders:
/// - `{field_name}` — matches non-whitespace by default
/// - Literal text between placeholders is matched exactly (regex-escaped)
/// - `[...]` brackets in the pattern are matched literally
fn compile_custom_pattern(pattern: &str) -> anyhow::Result<CompiledPattern> {
    let mut regex_str = String::from("^");
    let mut field_names = Vec::new();
    let mut chars = pattern.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch == '{' {
            chars.next(); // consume '{'
            let mut name = String::new();
            loop {
                match chars.next() {
                    Some('}') => break,
                    Some(c) => name.push(c),
                    None => anyhow::bail!(
                        "Unclosed '{{' in log format pattern. Expected '}}' to close field '{name}'"
                    ),
                }
            }
            if name.is_empty() {
                anyhow::bail!("Empty field name '{{}}' in log format pattern");
            }
            field_names.push(name);

            // Determine capture group regex based on what follows
            match chars.peek() {
                None => {
                    // Last field: match everything remaining
                    regex_str.push_str("(.+)");
                }
                Some(&next_ch) => {
                    if next_ch == '[' || next_ch == '"' {
                        // Match up to the next literal delimiter
                        regex_str.push_str("([^");
                        regex_str.push_str(&regex::escape(&next_ch.to_string()));
                        regex_str.push_str("]*)");
                    } else if next_ch == ' ' {
                        // Match non-whitespace
                        regex_str.push_str(r"(\S+)");
                    } else {
                        // Match everything up to the next literal character
                        regex_str.push_str("([^");
                        regex_str.push_str(&regex::escape(&next_ch.to_string()));
                        regex_str.push_str("]+)");
                    }
                }
            }
        } else {
            chars.next();
            regex_str.push_str(&regex::escape(&ch.to_string()));
        }
    }
    regex_str.push('$');

    let regex = Regex::new(&regex_str).map_err(|e| {
        anyhow::anyhow!(
            "Failed to compile log format pattern into regex: {e}\n  Pattern: {pattern}\n  Generated regex: {regex_str}"
        )
    })?;

    Ok(CompiledPattern { regex, field_names })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_opts() -> LogReaderOptions {
        LogReaderOptions::default()
    }

    // --- Apache Combined ---

    #[test]
    fn test_apache_combined_basic() {
        let reader = LogReader::new("apache-combined", default_opts()).unwrap();
        let input = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08""#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(
            obj.get("remote_host"),
            Some(&Value::String("127.0.0.1".to_string()))
        );
        assert_eq!(
            obj.get("remote_user"),
            Some(&Value::String("frank".to_string()))
        );
        assert_eq!(obj.get("status"), Some(&Value::Integer(200)));
        assert_eq!(obj.get("bytes"), Some(&Value::Integer(2326)));
        assert_eq!(
            obj.get("user_agent"),
            Some(&Value::String("Mozilla/4.08".to_string()))
        );
    }

    #[test]
    fn test_apache_combined_alias() {
        // "apache" should be an alias for "apache-combined"
        let reader = LogReader::new("apache", default_opts()).unwrap();
        let input = r#"10.0.0.1 - - [01/Jan/2024:00:00:00 +0000] "POST /api HTTP/1.1" 201 512 "-" "curl/7.68""#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(obj.get("ident"), Some(&Value::Null)); // "-" → Null
        assert_eq!(obj.get("remote_user"), Some(&Value::Null));
        assert_eq!(obj.get("status"), Some(&Value::Integer(201)));
    }

    // --- Apache Common ---

    #[test]
    fn test_apache_common() {
        let reader = LogReader::new("apache-common", default_opts()).unwrap();
        let input = r#"192.168.1.1 - admin [15/Mar/2024:10:30:00 +0900] "GET /index.html HTTP/1.1" 200 1024"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(
            obj.get("remote_host"),
            Some(&Value::String("192.168.1.1".to_string()))
        );
        assert_eq!(
            obj.get("request"),
            Some(&Value::String("GET /index.html HTTP/1.1".to_string()))
        );
        assert_eq!(obj.get("status"), Some(&Value::Integer(200)));
        assert_eq!(obj.get("bytes"), Some(&Value::Integer(1024)));
    }

    // --- Nginx ---

    #[test]
    fn test_nginx() {
        let reader = LogReader::new("nginx", default_opts()).unwrap();
        let input = r#"10.0.0.5 - alice [20/Feb/2024:08:15:00 +0000] "GET /api/users HTTP/2.0" 200 4096 "https://example.com" "Mozilla/5.0""#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(
            obj.get("remote_addr"),
            Some(&Value::String("10.0.0.5".to_string()))
        );
        assert_eq!(
            obj.get("remote_user"),
            Some(&Value::String("alice".to_string()))
        );
        assert_eq!(obj.get("status"), Some(&Value::Integer(200)));
    }

    // --- Syslog ---

    #[test]
    fn test_syslog_with_pid() {
        let reader = LogReader::new("syslog", default_opts()).unwrap();
        let input = "Mar 10 13:55:36 myhost sshd[1234]: Accepted publickey for user from 10.0.0.1";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(
            obj.get("timestamp"),
            Some(&Value::String("Mar 10 13:55:36".to_string()))
        );
        assert_eq!(
            obj.get("hostname"),
            Some(&Value::String("myhost".to_string()))
        );
        assert_eq!(
            obj.get("app_name"),
            Some(&Value::String("sshd".to_string()))
        );
        assert_eq!(obj.get("pid"), Some(&Value::Integer(1234)));
        assert_eq!(
            obj.get("message"),
            Some(&Value::String(
                "Accepted publickey for user from 10.0.0.1".to_string()
            ))
        );
    }

    #[test]
    fn test_syslog_with_priority() {
        let reader = LogReader::new("syslog", default_opts()).unwrap();
        let input = "<34>Mar  5 09:00:00 server01 cron[456]: job completed";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(obj.get("priority"), Some(&Value::Integer(34)));
        assert_eq!(
            obj.get("hostname"),
            Some(&Value::String("server01".to_string()))
        );
    }

    // --- Custom patterns ---

    #[test]
    fn test_custom_pattern_basic() {
        let reader = LogReader::new("{timestamp} [{level}] {message}", default_opts()).unwrap();
        let input = "2024-01-15T10:30:00 [INFO] Server started successfully";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(
            obj.get("timestamp"),
            Some(&Value::String("2024-01-15T10:30:00".to_string()))
        );
        assert_eq!(obj.get("level"), Some(&Value::String("INFO".to_string())));
        assert_eq!(
            obj.get("message"),
            Some(&Value::String("Server started successfully".to_string()))
        );
    }

    #[test]
    fn test_custom_pattern_with_delimiters() {
        let reader = LogReader::new("{ip} - {user} [{time}] {msg}", default_opts()).unwrap();
        let input = "10.0.0.1 - admin [2024-01-01 00:00:00] request processed";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let obj = arr[0].as_object().unwrap();
        assert_eq!(obj.get("ip"), Some(&Value::String("10.0.0.1".to_string())));
        assert_eq!(obj.get("user"), Some(&Value::String("admin".to_string())));
        assert_eq!(
            obj.get("time"),
            Some(&Value::String("2024-01-01 00:00:00".to_string()))
        );
    }

    // --- Error handling ---

    #[test]
    fn test_skip_unparseable_lines() {
        let reader = LogReader::new("apache-common", default_opts()).unwrap();
        let input = r#"192.168.1.1 - - [15/Mar/2024:10:30:00 +0900] "GET / HTTP/1.1" 200 512
this is not a valid log line
10.0.0.2 - - [15/Mar/2024:10:31:00 +0900] "POST /api HTTP/1.1" 201 256"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // skipped the invalid line
    }

    #[test]
    fn test_raw_mode_for_unparseable_lines() {
        let opts = LogReaderOptions {
            on_error: LogParseErrorMode::Raw,
        };
        let reader = LogReader::new("apache-common", opts).unwrap();
        let input = r#"192.168.1.1 - - [15/Mar/2024:10:30:00 +0900] "GET / HTTP/1.1" 200 512
this is not a valid log line"#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        let raw_obj = arr[1].as_object().unwrap();
        assert_eq!(
            raw_obj.get("_raw"),
            Some(&Value::String("this is not a valid log line".to_string()))
        );
    }

    #[test]
    fn test_empty_input() {
        let reader = LogReader::new("apache", default_opts()).unwrap();
        let result = reader.read("").unwrap();
        let arr = result.as_array().unwrap();
        assert!(arr.is_empty());
    }

    #[test]
    fn test_blank_lines_skipped() {
        let reader = LogReader::new("syslog", default_opts()).unwrap();
        let input = "\n\n  \n";
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert!(arr.is_empty());
    }

    // --- Multiple lines ---

    #[test]
    fn test_multiple_apache_lines() {
        let reader = LogReader::new("apache", default_opts()).unwrap();
        let input = r#"10.0.0.1 - - [01/Jan/2024:00:00:00 +0000] "GET / HTTP/1.1" 200 1024 "-" "curl/7.68"
10.0.0.2 - user [01/Jan/2024:00:01:00 +0000] "POST /login HTTP/1.1" 302 0 "http://example.com" "Mozilla/5.0"
10.0.0.3 - - [01/Jan/2024:00:02:00 +0000] "GET /favicon.ico HTTP/1.1" 404 0 "-" "Mozilla/5.0""#;
        let result = reader.read(input).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(
            arr[2].as_object().unwrap().get("status"),
            Some(&Value::Integer(404))
        );
    }

    // --- read_from_reader ---

    #[test]
    fn test_read_from_reader() {
        let reader = LogReader::new("apache-common", default_opts()).unwrap();
        let input = br#"192.168.1.1 - - [15/Mar/2024:10:30:00 +0900] "GET / HTTP/1.1" 200 512"#;
        let result = reader.read_from_reader(&input[..]).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
    }

    // --- Invalid pattern ---

    #[test]
    fn test_unclosed_brace_error() {
        let result = LogReader::new("{timestamp [{level} {message", default_opts());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_field_name_error() {
        let result = LogReader::new("{} some text", default_opts());
        assert!(result.is_err());
    }
}
