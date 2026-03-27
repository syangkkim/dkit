use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, SqliteOptions,
};
use dkit_core::format::csv::CsvReader;
use dkit_core::format::env::{EnvReader, EnvWriter};
use dkit_core::format::hcl::{HclReader, HclWriter};
use dkit_core::format::html::HtmlWriter;
use dkit_core::format::ini::{IniReader, IniWriter};
use dkit_core::format::json::{JsonReader, JsonWriter};
use dkit_core::format::jsonl::{JsonlReader, JsonlWriter};
use dkit_core::format::markdown::MarkdownWriter;
use dkit_core::format::msgpack::{MsgpackReader, MsgpackWriter};
use dkit_core::format::plist::{PlistReader, PlistWriter};
use dkit_core::format::properties::{PropertiesReader, PropertiesWriter};
use dkit_core::format::toml::{TomlReader, TomlWriter};
use dkit_core::format::xml::{XmlReader, XmlWriter};
use dkit_core::format::yaml::{YamlReader, YamlWriter};
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader, FormatWriter,
};
use dkit_core::query::evaluator::evaluate_path;
use dkit_core::query::filter::apply_operations;
use dkit_core::query::parser::{
    parse_query, AggregateFunc, ArithmeticOp, CompareOp, Comparison, Condition, Expr,
    GroupAggregate, LiteralValue, Operation, Path as QueryPath, Query, Segment, SelectExpr,
};
use dkit_core::value::Value;

pub struct QueryArgs<'a> {
    pub input: &'a str,
    pub query: &'a str,
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
    pub explain: bool,
}

/// query 서브커맨드 실행
pub fn run(args: &QueryArgs) -> Result<()> {
    // --explain: 쿼리 실행 계획만 출력하고 종료
    if args.explain {
        let query = parse_query(args.query)?;
        print_explain(&query, args.input);
        return Ok(());
    }

    // 입력 읽기 (바이너리 포맷 자동 처리)
    let value = if args.input == "-" {
        if args.from == Some("msgpack") || args.from == Some("messagepack") {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read from stdin")?;
            MsgpackReader.read_from_bytes(&buf)?
        } else {
            let buf = read_stdin_with_encoding(&args.encoding_opts)?;
            let (source_format, sniffed_delimiter) = match args.from {
                Some(f) => (Format::from_str(f)?, None),
                None => detect_format_from_content(&buf)?,
            };
            let auto_delimiter =
                sniffed_delimiter.or_else(|| args.from.and_then(default_delimiter_for_format));
            let read_options = FormatOptions {
                delimiter: auto_delimiter,
                ..Default::default()
            };
            read_value(&buf, source_format, &read_options)?
        }
    } else {
        let source_format = match args.from {
            Some(f) => Format::from_str(f)?,
            None => detect_format(&PathBuf::from(args.input))?,
        };
        if source_format == Format::Msgpack {
            let bytes = read_file_bytes(Path::new(args.input))?;
            MsgpackReader.read_from_bytes(&bytes)?
        } else if source_format == Format::Xlsx {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_xlsx_from_bytes(&bytes, &args.excel_opts)?
        } else if source_format == Format::Sqlite {
            read_sqlite_from_path(Path::new(args.input), &args.sqlite_opts)?
        } else if source_format == Format::Parquet {
            let bytes = read_file_bytes(Path::new(args.input))?;
            read_parquet_from_bytes(&bytes)?
        } else {
            let content = read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?;
            let auto_delimiter = default_delimiter(Path::new(args.input));
            let read_options = FormatOptions {
                delimiter: auto_delimiter,
                ..Default::default()
            };
            read_value(&content, source_format, &read_options)?
        }
    };

    // 쿼리 파싱 및 실행
    let query = parse_query(args.query)?;
    let path_result = evaluate_path(&value, &query.path)?;
    let result = apply_operations(path_result, &query.operations)?;

    // 출력 포맷 결정: -o 파일 확장자 → --to → 기본 JSON
    let output_format = match args.to {
        Some(f) => Format::from_str(f)?,
        None => match args.output {
            Some(p) => detect_format(p).unwrap_or(Format::Json),
            None => Format::Json,
        },
    };

    // 출력
    if output_format == Format::Msgpack {
        let bytes = MsgpackWriter.write_bytes(&result)?;
        match args.output {
            Some(path) => {
                fs::write(path, &bytes)
                    .with_context(|| format!("Failed to write to {}", path.display()))?;
            }
            None => {
                use std::io::Write as _;
                std::io::stdout()
                    .write_all(&bytes)
                    .context("Failed to write to stdout")?;
            }
        }
    } else {
        // Auto-detect: pretty when writing to terminal or file, compact when piped
        let effective_pretty = match args.output {
            Some(_) => true,
            None => io::stdout().is_terminal(),
        };
        let write_options = FormatOptions {
            pretty: effective_pretty,
            compact: !effective_pretty,
            ..Default::default()
        };
        let output = write_value(&result, output_format, &write_options)?;

        match args.output {
            Some(path) => {
                let content = if output.ends_with('\n') {
                    output
                } else {
                    format!("{output}\n")
                };
                fs::write(path, &content)
                    .with_context(|| format!("Failed to write to {}", path.display()))?;
            }
            None => {
                if output.ends_with('\n') {
                    print!("{output}");
                } else {
                    println!("{output}");
                }
            }
        }
    }

    Ok(())
}

/// stdin에서 인코딩을 고려하여 문자열을 읽는다.
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

/// Print a human-readable execution plan for a parsed query.
fn print_explain(query: &Query, input: &str) {
    eprintln!("Execution Plan:");

    let mut step = 1;

    // Step 1: Scan
    eprintln!("  {}. Scan: {}", step, input);
    step += 1;

    // Step 2: Path navigation
    if !query.path.segments.is_empty() {
        eprintln!("  {}. Navigate: {}", step, format_path(&query.path));
        step += 1;
    }

    // Remaining steps: pipeline operations
    for op in &query.operations {
        eprintln!("  {}. {}", step, format_operation(op));
        step += 1;
    }
}

fn format_path(path: &QueryPath) -> String {
    let mut s = String::from(".");
    for seg in &path.segments {
        match seg {
            Segment::Field(f) => {
                s.push('.');
                s.push_str(f);
            }
            Segment::Index(i) => {
                s.push_str(&format!("[{}]", i));
            }
            Segment::Iterate => s.push_str("[]"),
            Segment::Slice { start, end, step } => {
                let start_s = start.map_or(String::new(), |v| v.to_string());
                let end_s = end.map_or(String::new(), |v| v.to_string());
                match step {
                    Some(st) => s.push_str(&format!("[{}:{}:{}]", start_s, end_s, st)),
                    None => s.push_str(&format!("[{}:{}]", start_s, end_s)),
                }
            }
            Segment::Wildcard => s.push_str("[*]"),
            Segment::RecursiveDescent(key) => {
                s.push_str("..");
                s.push_str(key);
            }
            _ => s.push_str("[?]"),
        }
    }
    s
}

fn format_operation(op: &Operation) -> String {
    match op {
        Operation::Where(cond) => format!("Filter: {}", format_condition(cond)),
        Operation::Select(exprs) => {
            let cols: Vec<String> = exprs.iter().map(format_select_expr).collect();
            format!("Project: {}", cols.join(", "))
        }
        Operation::Sort { field, descending } => {
            let dir = if *descending { "DESC" } else { "ASC" };
            format!("Sort: {} {}", field, dir)
        }
        Operation::Limit(n) => format!("Limit: {}", n),
        Operation::Count { field } => match field {
            Some(f) => format!("Aggregate: count({})", f),
            None => "Aggregate: count()".to_string(),
        },
        Operation::Sum { field } => format!("Aggregate: sum({})", field),
        Operation::Avg { field } => format!("Aggregate: avg({})", field),
        Operation::Min { field } => format!("Aggregate: min({})", field),
        Operation::Max { field } => format!("Aggregate: max({})", field),
        Operation::Median { field } => format!("Aggregate: median({})", field),
        Operation::Percentile { field, p } => {
            format!("Aggregate: percentile({}, {})", field, p)
        }
        Operation::Stddev { field } => format!("Aggregate: stddev({})", field),
        Operation::Variance { field } => format!("Aggregate: variance({})", field),
        Operation::Mode { field } => format!("Aggregate: mode({})", field),
        Operation::GroupConcat { field, separator } => {
            format!("Aggregate: group_concat({}, \"{}\")", field, separator)
        }
        Operation::Distinct { field } => format!("Distinct: {}", field),
        Operation::Unique => "Unique: (full record)".to_string(),
        Operation::UniqueBy { field } => format!("Unique by: {}", field),
        Operation::GroupBy {
            fields,
            having,
            aggregates,
        } => {
            let mut s = format!("Group By: {}", fields.join(", "));
            if !aggregates.is_empty() {
                let aggs: Vec<String> = aggregates.iter().map(format_group_aggregate).collect();
                s.push_str(&format!(" → {}", aggs.join(", ")));
            }
            if let Some(h) = having {
                s.push_str(&format!(" having {}", format_condition(h)));
            }
            s
        }
        Operation::AddField { name, expr } => {
            format!("Add Field: {} = {}", name, format_expr(expr))
        }
        Operation::MapField { name, expr } => {
            format!("Map Field: {} = {}", name, format_expr(expr))
        }
        Operation::Explode { field } => format!("Explode: {}", field),
        Operation::Unpivot {
            value_columns,
            key_name,
            value_name,
        } => {
            format!(
                "Unpivot: [{}] → ({}, {})",
                value_columns.join(", "),
                key_name,
                value_name
            )
        }
        Operation::Pivot {
            index_fields,
            columns_field,
            values_field,
        } => {
            format!(
                "Pivot: index=[{}], columns={}, values={}",
                index_fields.join(", "),
                columns_field,
                values_field
            )
        }
        _ => format!("{:?}", op),
    }
}

fn format_condition(cond: &Condition) -> String {
    match cond {
        Condition::Comparison(cmp) => format_comparison(cmp),
        Condition::And(l, r) => {
            format!("({} AND {})", format_condition(l), format_condition(r))
        }
        Condition::Or(l, r) => {
            format!("({} OR {})", format_condition(l), format_condition(r))
        }
        _ => format!("{:?}", cond),
    }
}

fn format_comparison(cmp: &Comparison) -> String {
    let op = match cmp.op {
        CompareOp::Eq => "==",
        CompareOp::Ne => "!=",
        CompareOp::Gt => ">",
        CompareOp::Lt => "<",
        CompareOp::Ge => ">=",
        CompareOp::Le => "<=",
        CompareOp::Contains => "contains",
        CompareOp::StartsWith => "starts_with",
        CompareOp::EndsWith => "ends_with",
        CompareOp::In => "in",
        CompareOp::NotIn => "not in",
        CompareOp::Matches => "matches",
        CompareOp::NotMatches => "not matches",
        _ => "?",
    };
    format!("{} {} {}", cmp.field, op, format_literal(&cmp.value))
}

fn format_literal(lit: &LiteralValue) -> String {
    match lit {
        LiteralValue::String(s) => format!("\"{}\"", s),
        LiteralValue::Integer(n) => n.to_string(),
        LiteralValue::Float(f) => f.to_string(),
        LiteralValue::Bool(b) => b.to_string(),
        LiteralValue::Null => "null".to_string(),
        LiteralValue::List(items) => {
            let parts: Vec<String> = items.iter().map(format_literal).collect();
            format!("[{}]", parts.join(", "))
        }
        _ => format!("{:?}", lit),
    }
}

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Field(f) => f.clone(),
        Expr::Literal(lit) => format_literal(lit),
        Expr::FuncCall { name, args } => {
            let arg_strs: Vec<String> = args.iter().map(format_expr).collect();
            format!("{}({})", name, arg_strs.join(", "))
        }
        Expr::BinaryOp { op, left, right } => {
            let op_str = match op {
                ArithmeticOp::Add => "+",
                ArithmeticOp::Sub => "-",
                ArithmeticOp::Mul => "*",
                ArithmeticOp::Div => "/",
            };
            format!("({} {} {})", format_expr(left), op_str, format_expr(right))
        }
        Expr::If {
            condition,
            then_expr,
            else_expr,
        } => {
            format!(
                "if({}, {}, {})",
                format_condition(condition),
                format_expr(then_expr),
                format_expr(else_expr)
            )
        }
        Expr::Case { branches, default } => {
            let mut s = String::from("case");
            for (cond, expr) in branches {
                s.push_str(&format!(
                    " when {} then {}",
                    format_condition(cond),
                    format_expr(expr)
                ));
            }
            if let Some(d) = default {
                s.push_str(&format!(" else {}", format_expr(d)));
            }
            s.push_str(" end");
            s
        }
        _ => format!("{:?}", expr),
    }
}

fn format_select_expr(se: &SelectExpr) -> String {
    let base = format_expr(&se.expr);
    match &se.alias {
        Some(alias) => format!("{} as {}", base, alias),
        None => base,
    }
}

fn format_group_aggregate(ga: &GroupAggregate) -> String {
    let func = match &ga.func {
        AggregateFunc::Count => "count".to_string(),
        AggregateFunc::Sum => "sum".to_string(),
        AggregateFunc::Avg => "avg".to_string(),
        AggregateFunc::Min => "min".to_string(),
        AggregateFunc::Max => "max".to_string(),
        AggregateFunc::Median => "median".to_string(),
        AggregateFunc::Percentile(p) => format!("percentile({})", p),
        AggregateFunc::Stddev => "stddev".to_string(),
        AggregateFunc::Variance => "variance".to_string(),
        AggregateFunc::Mode => "mode".to_string(),
        AggregateFunc::GroupConcat(sep) => format!("group_concat(\"{}\")", sep),
        _ => format!("{:?}", ga.func),
    };
    match &ga.field {
        Some(f) => format!("{}({}) as {}", func, f, ga.alias),
        None => format!("{}() as {}", func, ga.alias),
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

fn write_value(value: &Value, format: Format, options: &FormatOptions) -> Result<String> {
    match format {
        Format::Json => JsonWriter::new(options.clone()).write(value),
        Format::Jsonl => JsonlWriter.write(value),
        Format::Csv => {
            // CSV 출력 시 배열 형태가 아닌 단일 값은 JSON으로 출력
            match value {
                Value::Array(_) => {
                    use dkit_core::format::csv::CsvWriter;
                    CsvWriter::new(options.clone()).write(value)
                }
                _ => JsonWriter::new(options.clone()).write(value),
            }
        }
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty, options.root_element.clone()).write(value),
        Format::Env => EnvWriter.write(value),
        Format::Ini => IniWriter.write(value),
        Format::Properties => PropertiesWriter.write(value),
        Format::Hcl => HclWriter.write(value),
        Format::Plist => PlistWriter.write(value),
        Format::Msgpack => MsgpackWriter.write(value),
        Format::Xlsx => bail!("Excel is an input-only format and cannot be used as output"),
        Format::Sqlite => bail!("SQLite is an input-only format and cannot be used as output"),
        Format::Parquet => bail!("Parquet is an input-only format and cannot be used as output"),
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
        _ => bail!("Unsupported output format: {format}"),
    }
}
