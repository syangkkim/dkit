use std::collections::HashMap;
use std::io::{self, Read};
use std::path::Path;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use anyhow::{bail, Context, Result};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::env::EnvReader;
use dkit_core::format::hcl::HclReader;
use dkit_core::format::ini::IniReader;
use dkit_core::format::json::JsonReader;
use dkit_core::format::jsonl::JsonlReader;
use dkit_core::format::log::{LogParseErrorMode, LogReader, LogReaderOptions};
use dkit_core::format::msgpack::MsgpackReader;
use dkit_core::format::plist::PlistReader;
use dkit_core::format::properties::PropertiesReader;
use dkit_core::format::toml::TomlReader;
use dkit_core::format::xml::XmlReader;
use dkit_core::format::yaml::YamlReader;
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader,
};
use dkit_core::value::Value;

pub struct ProfileArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
    pub output_format: Option<&'a str>,
    pub detailed: bool,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
    pub log_format: Option<&'a str>,
    pub log_error: LogParseErrorMode,
}

enum OutputFormat {
    Text,
    Json,
    Yaml,
    Markdown,
}

impl OutputFormat {
    fn from_str_opt(s: Option<&str>) -> Result<Self> {
        match s {
            None | Some("text") | Some("table") => Ok(Self::Text),
            Some("json") => Ok(Self::Json),
            Some("yaml") | Some("yml") => Ok(Self::Yaml),
            Some("md") | Some("markdown") => Ok(Self::Markdown),
            Some(other) => bail!(
                "Unsupported profile output format: '{}'. Use json, yaml, table, or md",
                other
            ),
        }
    }
}

// ── Profile result types ──

struct DatasetProfile {
    total_records: usize,
    total_fields: usize,
    duplicate_rows: usize,
    file_format: String,
    fields: Vec<FieldProfile>,
}

struct FieldProfile {
    name: String,
    inferred_type: &'static str,
    null_percent: f64,
    unique_count: usize,
    top_value: String,
    pattern: String,
    // numeric details (when detailed)
    numeric: Option<NumericProfile>,
    // string details (when detailed)
    string: Option<StringProfile>,
}

struct NumericProfile {
    min: f64,
    max: f64,
    mean: f64,
    median: f64,
    stddev: f64,
    outlier_percent: f64,
}

struct StringProfile {
    avg_length: f64,
    max_length: usize,
    top_values: Vec<(String, usize)>,
}

pub fn run(args: &ProfileArgs) -> Result<()> {
    let (value, source_format) = read_input_as_value(args)?;
    let output_format = OutputFormat::from_str_opt(args.output_format)?;

    let arr = match &value {
        Value::Array(arr) => arr,
        Value::Object(_) => {
            // Wrap single object in array for uniform handling
            &vec![value.clone()]
        }
        _ => bail!("Profile requires tabular data (array of objects or single object)"),
    };

    if arr.is_empty() {
        bail!("No records to profile");
    }

    let profile = build_profile(arr, &format!("{}", source_format), args.detailed);

    match output_format {
        OutputFormat::Text => print_profile_text(&profile, args.detailed),
        OutputFormat::Json => print_profile_json(&profile, args.detailed)?,
        OutputFormat::Yaml => print_profile_yaml(&profile, args.detailed)?,
        OutputFormat::Markdown => print_profile_markdown(&profile, args.detailed),
    }

    Ok(())
}

// ── Profile building ──

fn build_profile(arr: &[Value], format_name: &str, detailed: bool) -> DatasetProfile {
    let total_records = arr.len();
    let field_names = collect_columns(arr);
    let total_fields = field_names.len();
    let duplicate_rows = count_duplicate_rows(arr);

    let fields: Vec<FieldProfile> = field_names
        .iter()
        .map(|name| {
            let values = extract_column_values(arr, name);
            build_field_profile(name, &values, total_records, detailed)
        })
        .collect();

    DatasetProfile {
        total_records,
        total_fields,
        duplicate_rows,
        file_format: format_name.to_string(),
        fields,
    }
}

fn build_field_profile(name: &str, values: &[Value], total: usize, detailed: bool) -> FieldProfile {
    let null_count = values.iter().filter(|v| v.is_null()).count();
    let null_percent = if total > 0 {
        null_count as f64 / total as f64 * 100.0
    } else {
        0.0
    };

    let non_null: Vec<&Value> = values.iter().filter(|v| !v.is_null()).collect();

    // Determine type
    let numeric_values = extract_numeric_values(values);
    let is_numeric = numeric_values.len() == non_null.len() && !numeric_values.is_empty();

    let inferred_type = infer_type(&non_null);

    // Unique count
    let str_values: Vec<String> = non_null.iter().map(|v| format!("{}", v)).collect();
    let unique: std::collections::HashSet<&str> = str_values.iter().map(|s| s.as_str()).collect();
    let unique_count = unique.len();

    // Top value
    let top_value = compute_top_value(&str_values);

    // Pattern detection
    let pattern = detect_pattern(&non_null, is_numeric, &numeric_values);

    // Detailed stats
    let numeric = if detailed && is_numeric {
        Some(compute_numeric_profile(&numeric_values))
    } else {
        None
    };

    let string = if detailed && !is_numeric {
        Some(compute_string_profile(&str_values))
    } else {
        None
    };

    FieldProfile {
        name: name.to_string(),
        inferred_type,
        null_percent,
        unique_count,
        top_value,
        pattern,
        numeric,
        string,
    }
}

fn infer_type(non_null: &[&Value]) -> &'static str {
    if non_null.is_empty() {
        return "null";
    }

    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    for v in non_null {
        let t = match v {
            Value::Bool(_) => "bool",
            Value::Integer(_) => "int",
            Value::Float(_) => "float",
            Value::String(s) => {
                // Try to infer more specific type from string content
                if s.parse::<i64>().is_ok() {
                    "int"
                } else if s.parse::<f64>().is_ok() {
                    "float"
                } else {
                    "str"
                }
            }
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            _ => "unknown",
        };
        *type_counts.entry(t).or_insert(0) += 1;
    }

    // Return the most common type
    type_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(t, _)| t)
        .unwrap_or("unknown")
}

fn compute_top_value(str_values: &[String]) -> String {
    if str_values.is_empty() {
        return "-".to_string();
    }
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for s in str_values {
        *freq.entry(s.as_str()).or_insert(0) += 1;
    }
    freq.into_iter()
        .max_by_key(|(_, c)| *c)
        .map(|(v, _)| {
            if v.len() > 20 {
                format!("{}...", &v[..17])
            } else {
                v.to_string()
            }
        })
        .unwrap_or_else(|| "-".to_string())
}

fn detect_pattern(non_null: &[&Value], is_numeric: bool, numeric_values: &[f64]) -> String {
    if non_null.is_empty() {
        return "-".to_string();
    }

    if is_numeric && !numeric_values.is_empty() {
        let min = numeric_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = numeric_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        return format!("{}-{}", format_compact(min), format_compact(max));
    }

    // Check string patterns
    let strings: Vec<&str> = non_null.iter().filter_map(|v| v.as_str()).collect();

    if strings.is_empty() {
        return "-".to_string();
    }

    // Check for email pattern
    let email_count = strings
        .iter()
        .filter(|s| s.contains('@') && s.contains('.'))
        .count();
    if email_count > strings.len() / 2 {
        return "*@*.com".to_string();
    }

    // Check for URL pattern
    let url_count = strings
        .iter()
        .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
        .count();
    if url_count > strings.len() / 2 {
        return "URL".to_string();
    }

    // Check for UUID pattern
    let uuid_count = strings
        .iter()
        .filter(|s| {
            s.len() == 36
                && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
                && s.chars().filter(|c| *c == '-').count() == 4
        })
        .count();
    if uuid_count > strings.len() / 2 {
        return "UUID".to_string();
    }

    // Check for date-like patterns (YYYY-MM-DD or similar)
    let date_count = strings
        .iter()
        .filter(|s| {
            (s.len() >= 8 && s.len() <= 25)
                && (s.contains('-') || s.contains('/'))
                && s.chars().filter(|c| c.is_ascii_digit()).count() >= 4
        })
        .count();
    if date_count > strings.len() / 2 {
        return "date".to_string();
    }

    // Check for enum-like (low cardinality)
    let unique: std::collections::HashSet<&str> = strings.iter().copied().collect();
    if unique.len() <= 5 && strings.len() >= 3 {
        return "enum".to_string();
    }

    // Default: show general text indicator
    let lengths: Vec<usize> = strings.iter().map(|s| s.len()).collect();
    let avg_len = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
    format!("text(~{})", avg_len as usize)
}

fn compute_numeric_profile(values: &[f64]) -> NumericProfile {
    let count = values.len();
    let sum: f64 = values.iter().sum();
    let mean = sum / count as f64;

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let min = sorted[0];
    let max = sorted[count - 1];
    let median = percentile_sorted(&sorted, 50.0);
    let p25 = percentile_sorted(&sorted, 25.0);
    let p75 = percentile_sorted(&sorted, 75.0);

    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
    let stddev = variance.sqrt();

    // IQR-based outlier detection
    let iqr = p75 - p25;
    let lower_fence = p25 - 1.5 * iqr;
    let upper_fence = p75 + 1.5 * iqr;
    let outlier_count = values
        .iter()
        .filter(|&&v| v < lower_fence || v > upper_fence)
        .count();
    let outlier_percent = outlier_count as f64 / count as f64 * 100.0;

    NumericProfile {
        min,
        max,
        mean,
        median,
        stddev,
        outlier_percent,
    }
}

fn compute_string_profile(str_values: &[String]) -> StringProfile {
    let lengths: Vec<usize> = str_values.iter().map(|s| s.len()).collect();
    let avg_length = if lengths.is_empty() {
        0.0
    } else {
        lengths.iter().sum::<usize>() as f64 / lengths.len() as f64
    };
    let max_length = lengths.iter().copied().max().unwrap_or(0);

    let mut freq: HashMap<&str, usize> = HashMap::new();
    for s in str_values {
        *freq.entry(s.as_str()).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<(String, usize)> =
        freq.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    freq_vec.truncate(5);

    StringProfile {
        avg_length,
        max_length,
        top_values: freq_vec,
    }
}

fn percentile_sorted(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let frac = rank - lower as f64;
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

fn count_duplicate_rows(arr: &[Value]) -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut dup_count = 0;
    for item in arr {
        let key = format!("{}", item);
        if !seen.insert(key) {
            dup_count += 1;
        }
    }
    dup_count
}

// ── Output: Text (table) ──

fn print_profile_text(profile: &DatasetProfile, detailed: bool) {
    println!("=== Dataset Profile ===");
    println!(
        "Records: {}  Fields: {}  Duplicates: {}  Format: {}",
        format_number(profile.total_records as f64),
        profile.total_fields,
        format_number(profile.duplicate_rows as f64),
        profile.file_format
    );
    println!();

    // Summary table
    let name_w = profile
        .fields
        .iter()
        .map(|f| f.name.len())
        .max()
        .unwrap_or(5)
        .max(5);
    let type_w = 6;
    let null_w = 6;
    let uniq_w = 6;
    let top_w = profile
        .fields
        .iter()
        .map(|f| f.top_value.len())
        .max()
        .unwrap_or(9)
        .clamp(9, 20);
    let pat_w = profile
        .fields
        .iter()
        .map(|f| f.pattern.len())
        .max()
        .unwrap_or(7)
        .clamp(7, 15);

    println!(
        "{:<name_w$}  {:<type_w$}  {:>null_w$}  {:>uniq_w$}  {:<top_w$}  {:<pat_w$}",
        "Field", "Type", "Null%", "Unique", "Top Value", "Pattern",
    );
    println!(
        "{:-<name_w$}  {:-<type_w$}  {:-<null_w$}  {:-<uniq_w$}  {:-<top_w$}  {:-<pat_w$}",
        "", "", "", "", "", "",
    );

    for fp in &profile.fields {
        let top_display = if fp.top_value.len() > top_w {
            format!("{}...", &fp.top_value[..top_w.saturating_sub(3)])
        } else {
            fp.top_value.clone()
        };
        let pat_display = if fp.pattern.len() > pat_w {
            format!("{}...", &fp.pattern[..pat_w.saturating_sub(3)])
        } else {
            fp.pattern.clone()
        };
        println!(
            "{:<name_w$}  {:<type_w$}  {:>null_w$.1}  {:>uniq_w$}  {:<top_w$}  {:<pat_w$}",
            fp.name, fp.inferred_type, fp.null_percent, fp.unique_count, top_display, pat_display,
        );
    }

    if detailed {
        println!();
        for fp in &profile.fields {
            if let Some(ref np) = fp.numeric {
                println!("--- {} (numeric) ---", fp.name);
                println!(
                    "  min: {}  max: {}  mean: {}  median: {}",
                    format_decimal(np.min),
                    format_decimal(np.max),
                    format_decimal(np.mean),
                    format_decimal(np.median),
                );
                println!(
                    "  stddev: {}  outliers: {:.1}%",
                    format_decimal(np.stddev),
                    np.outlier_percent,
                );
            }
            if let Some(ref sp) = fp.string {
                println!("--- {} (string) ---", fp.name);
                println!(
                    "  avg_length: {:.1}  max_length: {}",
                    sp.avg_length, sp.max_length,
                );
                if !sp.top_values.is_empty() {
                    let top: Vec<String> = sp
                        .top_values
                        .iter()
                        .map(|(v, c)| format!("{}({})", v, c))
                        .collect();
                    println!("  top: {}", top.join(", "));
                }
            }
        }
    }
}

// ── Output: JSON ──

fn print_profile_json(profile: &DatasetProfile, detailed: bool) -> Result<()> {
    let json = build_profile_json(profile, detailed);
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

fn build_profile_json(profile: &DatasetProfile, detailed: bool) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert(
        "total_records".to_string(),
        serde_json::json!(profile.total_records),
    );
    map.insert(
        "total_fields".to_string(),
        serde_json::json!(profile.total_fields),
    );
    map.insert(
        "duplicate_rows".to_string(),
        serde_json::json!(profile.duplicate_rows),
    );
    map.insert("format".to_string(), serde_json::json!(profile.file_format));

    let fields: Vec<serde_json::Value> = profile
        .fields
        .iter()
        .map(|fp| build_field_json(fp, detailed))
        .collect();
    map.insert("fields".to_string(), serde_json::Value::Array(fields));

    serde_json::Value::Object(map)
}

fn build_field_json(fp: &FieldProfile, detailed: bool) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("name".to_string(), serde_json::json!(fp.name));
    map.insert("type".to_string(), serde_json::json!(fp.inferred_type));
    map.insert(
        "null_percent".to_string(),
        serde_json::json!(round2(fp.null_percent)),
    );
    map.insert(
        "unique_count".to_string(),
        serde_json::json!(fp.unique_count),
    );
    map.insert("top_value".to_string(), serde_json::json!(fp.top_value));
    map.insert("pattern".to_string(), serde_json::json!(fp.pattern));

    if detailed {
        if let Some(ref np) = fp.numeric {
            let mut nm = serde_json::Map::new();
            nm.insert("min".to_string(), serde_json::json!(np.min));
            nm.insert("max".to_string(), serde_json::json!(np.max));
            nm.insert("mean".to_string(), serde_json::json!(round2(np.mean)));
            nm.insert("median".to_string(), serde_json::json!(round2(np.median)));
            nm.insert("stddev".to_string(), serde_json::json!(round2(np.stddev)));
            nm.insert(
                "outlier_percent".to_string(),
                serde_json::json!(round2(np.outlier_percent)),
            );
            map.insert("numeric_details".to_string(), serde_json::Value::Object(nm));
        }
        if let Some(ref sp) = fp.string {
            let mut sm = serde_json::Map::new();
            sm.insert(
                "avg_length".to_string(),
                serde_json::json!(round2(sp.avg_length)),
            );
            sm.insert("max_length".to_string(), serde_json::json!(sp.max_length));
            if !sp.top_values.is_empty() {
                let top: Vec<serde_json::Value> = sp
                    .top_values
                    .iter()
                    .map(|(v, c)| serde_json::json!({"value": v, "count": c}))
                    .collect();
                sm.insert("top_values".to_string(), serde_json::Value::Array(top));
            }
            map.insert("string_details".to_string(), serde_json::Value::Object(sm));
        }
    }

    serde_json::Value::Object(map)
}

// ── Output: YAML ──

fn print_profile_yaml(profile: &DatasetProfile, detailed: bool) -> Result<()> {
    let json = build_profile_json(profile, detailed);
    print!("{}", serde_yaml::to_string(&json)?);
    Ok(())
}

// ── Output: Markdown ──

fn print_profile_markdown(profile: &DatasetProfile, detailed: bool) {
    println!("# Data Profile");
    println!();
    println!(
        "- **Records**: {}",
        format_number(profile.total_records as f64)
    );
    println!("- **Fields**: {}", profile.total_fields);
    println!(
        "- **Duplicates**: {}",
        format_number(profile.duplicate_rows as f64)
    );
    println!("- **Format**: {}", profile.file_format);
    println!();

    println!("| Field | Type | Null% | Unique | Top Value | Pattern |");
    println!("|-------|------|-------|--------|-----------|---------|");
    for fp in &profile.fields {
        println!(
            "| {} | {} | {:.1} | {} | {} | {} |",
            fp.name, fp.inferred_type, fp.null_percent, fp.unique_count, fp.top_value, fp.pattern,
        );
    }

    if detailed {
        for fp in &profile.fields {
            if let Some(ref np) = fp.numeric {
                println!();
                println!("## {} (numeric)", fp.name);
                println!();
                println!("| Stat | Value |");
                println!("|------|-------|");
                println!("| min | {} |", format_decimal(np.min));
                println!("| max | {} |", format_decimal(np.max));
                println!("| mean | {} |", format_decimal(np.mean));
                println!("| median | {} |", format_decimal(np.median));
                println!("| stddev | {} |", format_decimal(np.stddev));
                println!("| outliers | {:.1}% |", np.outlier_percent);
            }
            if let Some(ref sp) = fp.string {
                println!();
                println!("## {} (string)", fp.name);
                println!();
                println!("| Stat | Value |");
                println!("|------|-------|");
                println!("| avg_length | {:.1} |", sp.avg_length);
                println!("| max_length | {} |", sp.max_length);
                if !sp.top_values.is_empty() {
                    let top: Vec<String> = sp
                        .top_values
                        .iter()
                        .map(|(v, c)| format!("{}({})", v, c))
                        .collect();
                    println!("| top_values | {} |", top.join(", "));
                }
            }
        }
    }
}

// ── Utility functions ──

fn collect_columns(arr: &[Value]) -> Vec<String> {
    let mut columns = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for item in arr {
        if let Value::Object(obj) = item {
            for key in obj.keys() {
                if seen.insert(key.clone()) {
                    columns.push(key.clone());
                }
            }
        }
    }
    columns
}

fn extract_column_values(arr: &[Value], col: &str) -> Vec<Value> {
    arr.iter()
        .map(|item| {
            if let Value::Object(obj) = item {
                obj.get(col).cloned().unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        })
        .collect()
}

fn extract_numeric_values(values: &[Value]) -> Vec<f64> {
    values
        .iter()
        .filter(|v| !v.is_null())
        .filter_map(|v| v.as_f64())
        .collect()
}

fn format_number(n: f64) -> String {
    if n.fract() != 0.0 {
        return format_decimal(n);
    }
    let n = n as i64;
    if n < 0 {
        return format!("-{}", format_number((-n) as f64));
    }
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

fn format_decimal(n: f64) -> String {
    let formatted = format!("{:.2}", n);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = format_number(parts[0].parse::<f64>().unwrap_or(0.0));
    format!("{}.{}", integer_part, parts[1])
}

fn format_compact(n: f64) -> String {
    if n == n.floor() {
        format!("{}", n as i64)
    } else {
        format!("{:.1}", n)
    }
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

// ── Input reading (mirrors stats.rs pattern) ──

fn read_input(args: &ProfileArgs) -> Result<(String, Format)> {
    let path = Path::new(args.input);
    let format = match args.from {
        Some(f) => Format::from_str(f)?,
        None => detect_format(path)?,
    };
    let content = read_file_with_encoding(path, &args.encoding_opts)?;
    Ok((content, format))
}

fn read_stdin_with_encoding(opts: &EncodingOptions) -> Result<String> {
    if opts.encoding.is_some() || opts.detect_encoding {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .context("Failed to read from stdin")?;
        super::decode_bytes(&buf, opts)
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read from stdin")?;
        Ok(buf)
    }
}

fn read_input_as_value(args: &ProfileArgs) -> Result<(Value, Format)> {
    if let Some(log_fmt) = args.log_format {
        let log_opts = LogReaderOptions {
            on_error: args.log_error,
        };
        let log_reader = LogReader::new(log_fmt, log_opts)?;
        let content = if args.input == "-" {
            read_stdin_with_encoding(&args.encoding_opts)?
        } else {
            read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?
        };
        let value = log_reader.read(&content)?;
        return Ok((value, Format::Jsonl));
    }
    if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            let value = MsgpackReader.read_from_bytes(&buf)?;
            Ok((value, Format::Msgpack))
        } else {
            let buf = read_stdin_with_encoding(&args.encoding_opts)?;
            let (format, sniffed_delimiter) = match args.from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&buf)?,
            };
            let auto_delimiter =
                sniffed_delimiter.or_else(|| args.from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: args.delimiter.or(auto_delimiter),
                no_header: args.no_header,
                ..Default::default()
            };
            let value = read_value(&buf, format, &read_options)?;
            Ok((value, format))
        }
    } else {
        let format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(Path::new(args.input))?,
        };
        if format == Format::Msgpack {
            let bytes = read_file_bytes(Path::new(args.input))?;
            let value = MsgpackReader.read_from_bytes(&bytes)?;
            Ok((value, format))
        } else if format == Format::Xlsx {
            let bytes = read_file_bytes(Path::new(args.input))?;
            let value = read_xlsx_from_bytes(&bytes, &args.excel_opts)?;
            Ok((value, format))
        } else if format == Format::Sqlite {
            let value = read_sqlite_from_path(Path::new(args.input), &args.sqlite_opts)?;
            Ok((value, format))
        } else if format == Format::Parquet {
            let bytes = read_file_bytes(Path::new(args.input))?;
            let value = read_parquet_from_bytes(&bytes)?;
            Ok((value, format))
        } else {
            let (content, format) = read_input(args)?;
            let auto_delimiter = default_delimiter(Path::new(args.input));
            let read_options = FormatOptions {
                delimiter: args.delimiter.or(auto_delimiter),
                no_header: args.no_header,
                ..Default::default()
            };
            let value = read_value(&content, format, &read_options)?;
            Ok((value, format))
        }
    }
}

fn read_value(content: &str, format: Format, options: &FormatOptions) -> Result<Value> {
    match format {
        Format::Json => JsonReader.read(content),
        Format::Jsonl => JsonlReader.read(content),
        Format::Csv => CsvReader::new(options.clone()).read(content),
        Format::Yaml => YamlReader.read(content),
        Format::Toml => TomlReader.read(content),
        Format::Xml => XmlReader::default().read(content),
        Format::Env => EnvReader.read(content),
        Format::Ini => IniReader.read(content),
        Format::Properties => PropertiesReader.read(content),
        Format::Hcl => HclReader.read(content),
        Format::Plist => PlistReader.read(content),
        Format::Msgpack => MsgpackReader.read(content),
        Format::Xlsx => {
            bail!("Excel files must be read as binary; use file path input instead of stdin")
        }
        Format::Sqlite => {
            bail!("SQLite files must be read from a file path, not from text input")
        }
        Format::Parquet => {
            bail!("Parquet files must be read from a file path, not from text input")
        }
        Format::Markdown => bail!("Markdown is an output-only format and cannot be used as input"),
        Format::Html => bail!("HTML is an output-only format and cannot be used as input"),
        Format::Table => bail!("Table is an output-only format and cannot be used as input"),
        _ => bail!("Unsupported input format: {format}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn make_obj(pairs: Vec<(&str, Value)>) -> Value {
        let mut map = IndexMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v);
        }
        Value::Object(map)
    }

    #[test]
    fn test_build_profile_basic() {
        let arr = vec![
            make_obj(vec![
                ("name", Value::String("Alice".to_string())),
                ("age", Value::Integer(30)),
            ]),
            make_obj(vec![
                ("name", Value::String("Bob".to_string())),
                ("age", Value::Integer(25)),
            ]),
            make_obj(vec![
                ("name", Value::String("Charlie".to_string())),
                ("age", Value::Integer(35)),
            ]),
        ];

        let profile = build_profile(&arr, "json", false);
        assert_eq!(profile.total_records, 3);
        assert_eq!(profile.total_fields, 2);
        assert_eq!(profile.duplicate_rows, 0);
        assert_eq!(profile.fields.len(), 2);
        assert_eq!(profile.fields[0].name, "name");
        assert_eq!(profile.fields[1].name, "age");
    }

    #[test]
    fn test_null_percent() {
        let arr = vec![
            make_obj(vec![("x", Value::Integer(1))]),
            make_obj(vec![("x", Value::Null)]),
            make_obj(vec![("x", Value::Integer(3))]),
            make_obj(vec![("x", Value::Null)]),
        ];
        let profile = build_profile(&arr, "json", false);
        assert!((profile.fields[0].null_percent - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_duplicate_rows() {
        let arr = vec![
            make_obj(vec![("x", Value::Integer(1))]),
            make_obj(vec![("x", Value::Integer(1))]),
            make_obj(vec![("x", Value::Integer(2))]),
        ];
        assert_eq!(count_duplicate_rows(&arr), 1);
    }

    #[test]
    fn test_detect_pattern_numeric() {
        let vals = vec![Value::Integer(10), Value::Integer(50), Value::Integer(100)];
        let refs: Vec<&Value> = vals.iter().collect();
        let nums = vec![10.0, 50.0, 100.0];
        let pattern = detect_pattern(&refs, true, &nums);
        assert_eq!(pattern, "10-100");
    }

    #[test]
    fn test_detect_pattern_enum() {
        let vals = vec![
            Value::String("active".to_string()),
            Value::String("inactive".to_string()),
            Value::String("active".to_string()),
            Value::String("pending".to_string()),
        ];
        let refs: Vec<&Value> = vals.iter().collect();
        let pattern = detect_pattern(&refs, false, &[]);
        assert_eq!(pattern, "enum");
    }

    #[test]
    fn test_detect_pattern_email() {
        let vals = vec![
            Value::String("alice@example.com".to_string()),
            Value::String("bob@test.org".to_string()),
            Value::String("charlie@demo.net".to_string()),
            Value::String("dave@work.com".to_string()),
            Value::String("eve@mail.org".to_string()),
            Value::String("frank@site.com".to_string()),
        ];
        let refs: Vec<&Value> = vals.iter().collect();
        let pattern = detect_pattern(&refs, false, &[]);
        assert_eq!(pattern, "*@*.com");
    }

    #[test]
    fn test_infer_type() {
        let vals = vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)];
        let refs: Vec<&Value> = vals.iter().collect();
        assert_eq!(infer_type(&refs), "int");

        let vals2 = vec![
            Value::String("hello".to_string()),
            Value::String("world".to_string()),
        ];
        let refs2: Vec<&Value> = vals2.iter().collect();
        assert_eq!(infer_type(&refs2), "str");
    }

    #[test]
    fn test_compute_numeric_profile() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let np = compute_numeric_profile(&values);
        assert_eq!(np.min, 10.0);
        assert_eq!(np.max, 50.0);
        assert_eq!(np.mean, 30.0);
        assert_eq!(np.median, 30.0);
        assert_eq!(np.outlier_percent, 0.0);
    }

    #[test]
    fn test_compute_string_profile() {
        let values = vec![
            "apple".to_string(),
            "banana".to_string(),
            "apple".to_string(),
        ];
        let sp = compute_string_profile(&values);
        assert!((sp.avg_length - 5.333).abs() < 0.01);
        assert_eq!(sp.max_length, 6);
        assert_eq!(sp.top_values[0].0, "apple");
        assert_eq!(sp.top_values[0].1, 2);
    }

    #[test]
    fn test_format_compact() {
        assert_eq!(format_compact(10.0), "10");
        assert_eq!(format_compact(3.5), "3.5");
    }
}
