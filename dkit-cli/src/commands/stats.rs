use std::collections::HashMap;
use std::io::{self, Read};
use std::path::Path;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use anyhow::{bail, Context, Result};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::json::JsonReader;
use dkit_core::format::jsonl::JsonlReader;
use dkit_core::format::msgpack::MsgpackReader;
use dkit_core::format::toml::TomlReader;
use dkit_core::format::xml::XmlReader;
use dkit_core::format::yaml::YamlReader;
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader,
};
use dkit_core::value::Value;
pub struct StatsArgs<'a> {
    pub input: &'a str,
    pub from: Option<&'a str>,
    pub format: Option<&'a str>,
    pub path: Option<&'a str>,
    pub column: Option<&'a str>,
    pub histogram: bool,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

/// 출력 포맷
enum OutputFormat {
    Text,
    Json,
    Markdown,
}

impl OutputFormat {
    fn from_str_opt(s: Option<&str>) -> Result<Self> {
        match s {
            None | Some("text") | Some("table") => Ok(Self::Text),
            Some("json") => Ok(Self::Json),
            Some("md") | Some("markdown") => Ok(Self::Markdown),
            Some(other) => bail!(
                "Unsupported stats output format: '{}'. Use json, table, or md",
                other
            ),
        }
    }
}

/// 컬럼별 통계 데이터
struct ColumnStats {
    name: String,
    total_count: usize,
    missing_count: usize,
    type_counts: HashMap<&'static str, usize>,
    numeric: Option<NumericStats>,
    string: Option<StringStats>,
}

struct NumericStats {
    count: usize,
    sum: f64,
    mean: f64,
    min: f64,
    max: f64,
    median: f64,
    std: f64,
    p25: f64,
    p75: f64,
}

struct StringStats {
    count: usize,
    min_length: usize,
    max_length: usize,
    avg_length: f64,
    unique_count: usize,
    top_values: Vec<(String, usize)>,
}

pub fn run(args: &StatsArgs) -> Result<()> {
    let (value, _source_format) = read_input_as_value(args)?;
    let output_format = OutputFormat::from_str_opt(args.format)?;

    let target = match args.path {
        Some(path_expr) => resolve_path(&value, path_expr)?,
        None => value,
    };

    if let Some(col_name) = args.column {
        run_column_stats(&target, col_name, &output_format, args.histogram)?;
    } else {
        run_overall_stats(&target, &output_format, args.histogram)?;
    }

    Ok(())
}

fn run_overall_stats(value: &Value, output_format: &OutputFormat, histogram: bool) -> Result<()> {
    match value {
        Value::Array(arr) => {
            let rows = arr.len();
            let columns = collect_columns(arr);
            let col_values: Vec<Vec<Value>> = columns
                .iter()
                .map(|col| extract_column_values(arr, col))
                .collect();
            let col_stats: Vec<ColumnStats> = columns
                .iter()
                .zip(col_values.iter())
                .map(|(col, vals)| compute_column_stats(col, vals))
                .collect();

            match output_format {
                OutputFormat::Text => {
                    println!("rows: {}", format_number(rows as f64));
                    if !columns.is_empty() {
                        println!("columns: {} ({})", columns.len(), columns.join(", "));
                    }
                    for (cs, vals) in col_stats.iter().zip(col_values.iter()) {
                        println!();
                        println!("--- {} ---", cs.name);
                        print_column_stats_text(cs, histogram, vals);
                    }
                }
                OutputFormat::Json => {
                    let json = build_overall_json(rows, &col_stats);
                    println!("{}", serde_json::to_string_pretty(&json)?);
                }
                OutputFormat::Markdown => {
                    println!("# Statistics");
                    println!();
                    println!("- **rows**: {}", format_number(rows as f64));
                    if !columns.is_empty() {
                        println!("- **columns**: {} ({})", columns.len(), columns.join(", "));
                    }
                    for cs in &col_stats {
                        println!();
                        println!("## {}", cs.name);
                        println!();
                        print_column_stats_md(cs);
                    }
                }
            }
        }
        Value::Object(obj) => match output_format {
            OutputFormat::Text => {
                println!("rows: 1");
                let columns: Vec<&String> = obj.keys().collect();
                println!(
                    "columns: {} ({})",
                    columns.len(),
                    columns
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            OutputFormat::Json => {
                let mut map = serde_json::Map::new();
                map.insert("rows".to_string(), serde_json::Value::Number(1.into()));
                let columns: Vec<&String> = obj.keys().collect();
                map.insert(
                    "columns".to_string(),
                    serde_json::Value::Array(
                        columns
                            .iter()
                            .map(|c| serde_json::Value::String(c.to_string()))
                            .collect(),
                    ),
                );
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::Value::Object(map))?
                );
            }
            OutputFormat::Markdown => {
                println!("# Statistics");
                println!();
                println!("- **rows**: 1");
                let columns: Vec<&String> = obj.keys().collect();
                println!(
                    "- **columns**: {} ({})",
                    columns.len(),
                    columns
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        },
        _ => match output_format {
            OutputFormat::Text => {
                println!("type: {}", value_type_name(value));
                println!("value: {}", value);
            }
            OutputFormat::Json => {
                let mut map = serde_json::Map::new();
                map.insert(
                    "type".to_string(),
                    serde_json::Value::String(value_type_name(value).to_string()),
                );
                map.insert(
                    "value".to_string(),
                    serde_json::Value::String(format!("{}", value)),
                );
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::Value::Object(map))?
                );
            }
            OutputFormat::Markdown => {
                println!("- **type**: {}", value_type_name(value));
                println!("- **value**: {}", value);
            }
        },
    }
    Ok(())
}

fn run_column_stats(
    value: &Value,
    col_name: &str,
    output_format: &OutputFormat,
    histogram: bool,
) -> Result<()> {
    let arr = match value {
        Value::Array(arr) => arr,
        _ => bail!("--column/--field requires array data (rows of objects)"),
    };

    let columns = collect_columns(arr);
    if !columns.contains(&col_name.to_string()) {
        bail!(
            "Column '{}' not found\n  Available columns: {}",
            col_name,
            columns.join(", ")
        );
    }

    let values = extract_column_values(arr, col_name);
    let cs = compute_column_stats(col_name, &values);

    match output_format {
        OutputFormat::Text => print_column_stats_text(&cs, histogram, &values),
        OutputFormat::Json => {
            let json = build_column_json(&cs);
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Markdown => {
            println!("## {}", cs.name);
            println!();
            print_column_stats_md(&cs);
        }
    }
    Ok(())
}

// ── 통계 계산 ──

fn compute_column_stats(name: &str, values: &[Value]) -> ColumnStats {
    let total_count = values.len();
    let missing_count = values.iter().filter(|v| v.is_null()).count();

    // 타입 카운트
    let mut type_counts: HashMap<&'static str, usize> = HashMap::new();
    for v in values {
        let tn = value_type_name(v);
        *type_counts.entry(tn).or_insert(0) += 1;
    }

    let numeric_values = extract_numeric_values(values);
    let non_null_count = values.iter().filter(|v| !v.is_null()).count();

    let is_numeric = numeric_values.len() == non_null_count && !numeric_values.is_empty();

    let numeric = if is_numeric {
        Some(compute_numeric_stats(&numeric_values))
    } else {
        None
    };

    let string = if !is_numeric {
        Some(compute_string_stats(values))
    } else {
        None
    };

    ColumnStats {
        name: name.to_string(),
        total_count,
        missing_count,
        type_counts,
        numeric,
        string,
    }
}

fn compute_numeric_stats(values: &[f64]) -> NumericStats {
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

    // 표준편차 (population)
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
    let std = variance.sqrt();

    NumericStats {
        count,
        sum,
        mean,
        min,
        max,
        median,
        std,
        p25,
        p75,
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

fn compute_string_stats(values: &[Value]) -> StringStats {
    let non_null: Vec<String> = values
        .iter()
        .filter(|v| !v.is_null())
        .map(|v| match v {
            Value::String(s) => s.clone(),
            other => format!("{}", other),
        })
        .collect();

    let count = non_null.len();
    let (min_length, max_length, avg_length) = if count > 0 {
        let lengths: Vec<usize> = non_null.iter().map(|s| s.len()).collect();
        let min_l = *lengths.iter().min().unwrap();
        let max_l = *lengths.iter().max().unwrap();
        let avg_l = lengths.iter().sum::<usize>() as f64 / count as f64;
        (min_l, max_l, avg_l)
    } else {
        (0, 0, 0.0)
    };

    // unique count
    let unique: std::collections::HashSet<&str> = non_null.iter().map(|s| s.as_str()).collect();
    let unique_count = unique.len();

    // top values
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for s in &non_null {
        *freq.entry(s.as_str()).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<(String, usize)> =
        freq.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    freq_vec.truncate(5);

    StringStats {
        count,
        min_length,
        max_length,
        avg_length,
        unique_count,
        top_values: freq_vec,
    }
}

// ── Text 출력 ──

fn print_column_stats_text(cs: &ColumnStats, histogram: bool, raw_values: &[Value]) {
    // 타입 일관성 검사
    let non_null_types: Vec<(&str, usize)> = cs
        .type_counts
        .iter()
        .filter(|(k, _)| **k != "null")
        .map(|(k, v)| (*k, *v))
        .collect();
    if non_null_types.len() > 1 {
        let mixed: Vec<String> = non_null_types
            .iter()
            .map(|(t, c)| format!("{}({})", t, c))
            .collect();
        println!("⚠ mixed types: {}", mixed.join(", "));
    }

    if let Some(ref ns) = cs.numeric {
        println!("type: numeric");
        println!("count: {}", format_number(ns.count as f64));
        if cs.missing_count > 0 {
            println!(
                "missing: {} ({:.1}%)",
                format_number(cs.missing_count as f64),
                cs.missing_count as f64 / cs.total_count as f64 * 100.0
            );
        }
        println!("sum: {}", format_number(ns.sum));
        println!("avg: {}", format_decimal(ns.mean));
        println!("std: {}", format_decimal(ns.std));
        println!("min: {}", format_number(ns.min));
        println!("p25: {}", format_number(ns.p25));
        println!("median: {}", format_number(ns.median));
        println!("p75: {}", format_number(ns.p75));
        println!("max: {}", format_number(ns.max));

        if histogram {
            println!();
            let nums = extract_numeric_values(raw_values);
            print_histogram(&nums);
        }
    } else if let Some(ref ss) = cs.string {
        println!("type: string");
        println!("count: {}", format_number(ss.count as f64));
        if cs.missing_count > 0 {
            println!(
                "missing: {} ({:.1}%)",
                format_number(cs.missing_count as f64),
                cs.missing_count as f64 / cs.total_count as f64 * 100.0
            );
        }
        println!("unique: {}", format_number(ss.unique_count as f64));
        println!("min_length: {}", ss.min_length);
        println!("max_length: {}", ss.max_length);
        println!("avg_length: {}", format_decimal(ss.avg_length));
        if !ss.top_values.is_empty() {
            println!("top_values:");
            for (val, count) in &ss.top_values {
                println!("  {} ({})", val, count);
            }
        }
    }
}

// ── Markdown 출력 ──

fn print_column_stats_md(cs: &ColumnStats) {
    let non_null_types: Vec<(&str, usize)> = cs
        .type_counts
        .iter()
        .filter(|(k, _)| **k != "null")
        .map(|(k, v)| (*k, *v))
        .collect();
    if non_null_types.len() > 1 {
        let mixed: Vec<String> = non_null_types
            .iter()
            .map(|(t, c)| format!("{}({})", t, c))
            .collect();
        println!("| ⚠ mixed types | {} |", mixed.join(", "));
    }

    println!("| Stat | Value |");
    println!("|------|-------|");

    if let Some(ref ns) = cs.numeric {
        println!("| type | numeric |");
        println!("| count | {} |", format_number(ns.count as f64));
        if cs.missing_count > 0 {
            println!(
                "| missing | {} ({:.1}%) |",
                format_number(cs.missing_count as f64),
                cs.missing_count as f64 / cs.total_count as f64 * 100.0
            );
        }
        println!("| sum | {} |", format_number(ns.sum));
        println!("| avg | {} |", format_decimal(ns.mean));
        println!("| std | {} |", format_decimal(ns.std));
        println!("| min | {} |", format_number(ns.min));
        println!("| p25 | {} |", format_number(ns.p25));
        println!("| median | {} |", format_number(ns.median));
        println!("| p75 | {} |", format_number(ns.p75));
        println!("| max | {} |", format_number(ns.max));
    } else if let Some(ref ss) = cs.string {
        println!("| type | string |");
        println!("| count | {} |", format_number(ss.count as f64));
        if cs.missing_count > 0 {
            println!(
                "| missing | {} ({:.1}%) |",
                format_number(cs.missing_count as f64),
                cs.missing_count as f64 / cs.total_count as f64 * 100.0
            );
        }
        println!("| unique | {} |", format_number(ss.unique_count as f64));
        println!("| min_length | {} |", ss.min_length);
        println!("| max_length | {} |", ss.max_length);
        println!("| avg_length | {} |", format_decimal(ss.avg_length));
        if !ss.top_values.is_empty() {
            let top: Vec<String> = ss
                .top_values
                .iter()
                .map(|(v, c)| format!("{}({})", v, c))
                .collect();
            println!("| top_values | {} |", top.join(", "));
        }
    }
}

// ── JSON 출력 ──

fn build_overall_json(rows: usize, col_stats: &[ColumnStats]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("rows".to_string(), serde_json::json!(rows));

    let columns: Vec<serde_json::Value> = col_stats.iter().map(build_column_json).collect();
    map.insert("columns".to_string(), serde_json::Value::Array(columns));

    serde_json::Value::Object(map)
}

fn build_column_json(cs: &ColumnStats) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("name".to_string(), serde_json::json!(cs.name));
    map.insert("total_count".to_string(), serde_json::json!(cs.total_count));
    map.insert(
        "missing_count".to_string(),
        serde_json::json!(cs.missing_count),
    );

    if cs.total_count > 0 {
        let ratio = cs.missing_count as f64 / cs.total_count as f64;
        map.insert(
            "missing_ratio".to_string(),
            serde_json::json!(round2(ratio)),
        );
    }

    // 타입 일관성
    let non_null_types: Vec<&str> = cs
        .type_counts
        .iter()
        .filter(|(k, _)| **k != "null")
        .map(|(k, _)| *k)
        .collect();
    if non_null_types.len() > 1 {
        map.insert("mixed_types".to_string(), serde_json::json!(true));
        let tc: serde_json::Map<String, serde_json::Value> = cs
            .type_counts
            .iter()
            .map(|(k, v)| (k.to_string(), serde_json::json!(v)))
            .collect();
        map.insert("type_counts".to_string(), serde_json::Value::Object(tc));
    }

    if let Some(ref ns) = cs.numeric {
        map.insert("type".to_string(), serde_json::json!("numeric"));
        map.insert("count".to_string(), serde_json::json!(ns.count));
        map.insert("sum".to_string(), serde_json::json!(round2(ns.sum)));
        map.insert("mean".to_string(), serde_json::json!(round2(ns.mean)));
        map.insert("std".to_string(), serde_json::json!(round2(ns.std)));
        map.insert("min".to_string(), serde_json::json!(ns.min));
        map.insert("p25".to_string(), serde_json::json!(round2(ns.p25)));
        map.insert("median".to_string(), serde_json::json!(round2(ns.median)));
        map.insert("p75".to_string(), serde_json::json!(round2(ns.p75)));
        map.insert("max".to_string(), serde_json::json!(ns.max));
    } else if let Some(ref ss) = cs.string {
        map.insert("type".to_string(), serde_json::json!("string"));
        map.insert("count".to_string(), serde_json::json!(ss.count));
        map.insert(
            "unique_count".to_string(),
            serde_json::json!(ss.unique_count),
        );
        map.insert("min_length".to_string(), serde_json::json!(ss.min_length));
        map.insert("max_length".to_string(), serde_json::json!(ss.max_length));
        map.insert(
            "avg_length".to_string(),
            serde_json::json!(round2(ss.avg_length)),
        );
        if !ss.top_values.is_empty() {
            let top: Vec<serde_json::Value> = ss
                .top_values
                .iter()
                .map(|(v, c)| serde_json::json!({"value": v, "count": c}))
                .collect();
            map.insert("top_values".to_string(), serde_json::Value::Array(top));
        }
    }

    serde_json::Value::Object(map)
}

fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

// ── 히스토그램 ──

fn print_histogram(values: &[f64]) {
    if values.is_empty() {
        return;
    }
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if (max - min).abs() < f64::EPSILON {
        println!("histogram: all values = {}", format_number(min));
        return;
    }

    let bin_count = 10usize;
    let bin_width = (max - min) / bin_count as f64;
    let mut bins = vec![0usize; bin_count];

    for &v in values {
        let idx = ((v - min) / bin_width).floor() as usize;
        let idx = idx.min(bin_count - 1);
        bins[idx] += 1;
    }

    let max_count = *bins.iter().max().unwrap_or(&1);
    let bar_max_width = 30;

    println!("histogram:");
    for (i, &count) in bins.iter().enumerate() {
        let lo = min + i as f64 * bin_width;
        let hi = lo + bin_width;
        let bar_len = if max_count > 0 {
            (count as f64 / max_count as f64 * bar_max_width as f64).round() as usize
        } else {
            0
        };
        let bar: String = "█".repeat(bar_len);
        println!(
            "  [{:>8} - {:>8}) {} {}",
            format_number(lo),
            format_number(hi),
            bar,
            count
        );
    }
}

// ── 유틸리티 함수 ──

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

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        _ => "unknown",
    }
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

// ── 입력 읽기 ──

fn read_input(args: &StatsArgs) -> Result<(String, Format)> {
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

fn read_input_as_value(args: &StatsArgs) -> Result<(Value, Format)> {
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

fn resolve_path(value: &Value, path_expr: &str) -> Result<Value> {
    let path_expr = path_expr.trim();
    if path_expr.is_empty() || path_expr == "." {
        return Ok(value.clone());
    }

    let path_expr = path_expr.strip_prefix('.').unwrap_or(path_expr);
    let mut current = value.clone();

    for segment in split_path_segments(path_expr) {
        if let Some((field, idx)) = parse_index_segment(&segment) {
            if !field.is_empty() {
                current = access_field(&current, &field)?;
            }
            current = access_index(&current, idx)?;
        } else {
            current = access_field(&current, &segment)?;
        }
    }

    Ok(current)
}

fn split_path_segments(path: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut in_bracket = false;

    for ch in path.chars() {
        match ch {
            '[' => {
                in_bracket = true;
                current.push(ch);
            }
            ']' => {
                in_bracket = false;
                current.push(ch);
            }
            '.' if !in_bracket => {
                if !current.is_empty() {
                    segments.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }

    segments
}

fn parse_index_segment(segment: &str) -> Option<(String, i64)> {
    let bracket_start = segment.find('[')?;
    let bracket_end = segment.find(']')?;
    let field = segment[..bracket_start].to_string();
    let idx_str = &segment[bracket_start + 1..bracket_end];
    let idx: i64 = idx_str.parse().ok()?;
    Some((field, idx))
}

fn access_field(value: &Value, field: &str) -> Result<Value> {
    match value {
        Value::Object(obj) => obj
            .get(field)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", field)),
        _ => bail!("Cannot access field '{}' on non-object value", field),
    }
}

fn access_index(value: &Value, idx: i64) -> Result<Value> {
    match value {
        Value::Array(arr) => {
            let actual_idx = if idx < 0 {
                (arr.len() as i64 + idx) as usize
            } else {
                idx as usize
            };
            arr.get(actual_idx).cloned().ok_or_else(|| {
                anyhow::anyhow!("Index {} out of bounds (length {})", idx, arr.len())
            })
        }
        _ => bail!("Cannot index into non-array value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(42.0), "42");
        assert_eq!(format_number(1234.0), "1,234");
        assert_eq!(format_number(1234567.0), "1,234,567");
    }

    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(3.14), "3.14");
        assert_eq!(format_decimal(37017.34), "37,017.34");
    }

    #[test]
    fn test_percentile_sorted() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile_sorted(&sorted, 0.0), 1.0);
        assert_eq!(percentile_sorted(&sorted, 50.0), 3.0);
        assert_eq!(percentile_sorted(&sorted, 100.0), 5.0);
        assert_eq!(percentile_sorted(&sorted, 25.0), 2.0);
        assert_eq!(percentile_sorted(&sorted, 75.0), 4.0);
    }

    #[test]
    fn test_compute_numeric_stats() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let ns = compute_numeric_stats(&values);
        assert_eq!(ns.count, 5);
        assert_eq!(ns.sum, 150.0);
        assert_eq!(ns.mean, 30.0);
        assert_eq!(ns.min, 10.0);
        assert_eq!(ns.max, 50.0);
        assert_eq!(ns.median, 30.0);
        assert_eq!(ns.p25, 20.0);
        assert_eq!(ns.p75, 40.0);
        // std of [10,20,30,40,50] = sqrt(200) ≈ 14.14
        assert!((ns.std - 14.142135).abs() < 0.01);
    }

    #[test]
    fn test_compute_string_stats() {
        let values = vec![
            Value::String("apple".to_string()),
            Value::String("banana".to_string()),
            Value::String("apple".to_string()),
            Value::Null,
        ];
        let ss = compute_string_stats(&values);
        assert_eq!(ss.count, 3);
        assert_eq!(ss.unique_count, 2);
        assert_eq!(ss.min_length, 5);
        assert_eq!(ss.max_length, 6);
        assert!((ss.avg_length - 5.333).abs() < 0.01);
        assert_eq!(ss.top_values[0].0, "apple");
        assert_eq!(ss.top_values[0].1, 2);
    }

    #[test]
    fn test_compute_column_stats_mixed_types() {
        let values = vec![
            Value::Integer(10),
            Value::String("hello".to_string()),
            Value::Integer(20),
        ];
        let cs = compute_column_stats("mixed", &values);
        assert!(cs.type_counts.len() >= 2);
        // mixed types → treated as string
        assert!(cs.string.is_some());
        assert!(cs.numeric.is_none());
    }

    #[test]
    fn test_collect_columns() {
        let mut obj1 = IndexMap::new();
        obj1.insert("name".to_string(), Value::String("Alice".to_string()));
        obj1.insert("age".to_string(), Value::Integer(30));
        let mut obj2 = IndexMap::new();
        obj2.insert("name".to_string(), Value::String("Bob".to_string()));
        obj2.insert("age".to_string(), Value::Integer(25));

        let arr = vec![Value::Object(obj1), Value::Object(obj2)];
        assert_eq!(collect_columns(&arr), vec!["name", "age"]);
    }

    #[test]
    fn test_extract_numeric_values() {
        let values = vec![
            Value::Integer(10),
            Value::Float(20.5),
            Value::Null,
            Value::Integer(30),
        ];
        let nums = extract_numeric_values(&values);
        assert_eq!(nums, vec![10.0, 20.5, 30.0]);
    }

    #[test]
    fn test_value_type_name() {
        assert_eq!(value_type_name(&Value::Null), "null");
        assert_eq!(value_type_name(&Value::Bool(true)), "boolean");
        assert_eq!(value_type_name(&Value::Integer(1)), "integer");
        assert_eq!(value_type_name(&Value::Float(1.0)), "float");
        assert_eq!(value_type_name(&Value::String("s".into())), "string");
    }

    #[test]
    fn test_round2() {
        assert_eq!(round2(3.14159), 3.14);
        assert_eq!(round2(0.0), 0.0);
        assert_eq!(round2(100.005), 100.01);
    }
}
