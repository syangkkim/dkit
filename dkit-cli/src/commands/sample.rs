use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write as _};
use std::path::Path;

use anyhow::{bail, Context, Result};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use super::{
    read_file_bytes, read_file_with_encoding, read_parquet_from_bytes, read_sqlite_from_path,
    read_xlsx_from_bytes, EncodingOptions, ExcelOptions, ParquetWriteOptions, SqliteOptions,
};
use dkit_core::format::csv::{CsvReader, CsvWriter};
use dkit_core::format::env::{EnvReader, EnvWriter};
use dkit_core::format::hcl::{HclReader, HclWriter};
use dkit_core::format::html::HtmlWriter;
use dkit_core::format::ini::{IniReader, IniWriter};
use dkit_core::format::json::{JsonReader, JsonWriter};
use dkit_core::format::jsonl::{JsonlReader, JsonlWriter};
use dkit_core::format::markdown::MarkdownWriter;
use dkit_core::format::msgpack::{MsgpackReader, MsgpackWriter};
use dkit_core::format::properties::{PropertiesReader, PropertiesWriter};
use dkit_core::format::toml::{TomlReader, TomlWriter};
use dkit_core::format::xml::{XmlReader, XmlWriter};
use dkit_core::format::yaml::{YamlReader, YamlWriter};
use dkit_core::format::{
    default_delimiter, default_delimiter_for_format, detect_format, detect_format_from_content,
    Format, FormatOptions, FormatReader, FormatWriter,
};
use dkit_core::value::Value;

pub struct SampleArgs<'a> {
    pub input: &'a str,
    pub count: Option<usize>,
    pub ratio: Option<f64>,
    pub seed: Option<u64>,
    pub method: &'a str,
    pub stratify_by: Option<&'a str>,
    pub from: Option<&'a str>,
    pub format: Option<&'a str>,
    pub output: Option<&'a Path>,
    pub delimiter: Option<char>,
    pub no_header: bool,
    pub pretty: bool,
    pub encoding_opts: EncodingOptions,
    pub excel_opts: ExcelOptions,
    pub sqlite_opts: SqliteOptions,
}

pub fn run(args: &SampleArgs) -> Result<()> {
    // Validate arguments
    if args.count.is_none() && args.ratio.is_none() {
        bail!("Either -n/--count or --ratio is required\n  Hint: use -n 100 for 100 records or --ratio 0.1 for 10%");
    }
    if args.count.is_some() && args.ratio.is_some() {
        bail!("Cannot specify both -n/--count and --ratio");
    }
    if let Some(ratio) = args.ratio {
        if !(0.0..=1.0).contains(&ratio) {
            bail!("--ratio must be between 0.0 and 1.0, got {ratio}");
        }
    }

    let method = SamplingMethod::from_str(args.method)?;
    if matches!(method, SamplingMethod::Stratified) && args.stratify_by.is_none() {
        bail!("--stratify-by is required for stratified sampling\n  Hint: specify the field to stratify by, e.g. --stratify-by category");
    }

    let (value, source_format) = read_input_as_value(args)?;

    let records = match &value {
        Value::Array(arr) => arr.clone(),
        _ => bail!("Input data must be an array of records for sampling"),
    };

    if records.is_empty() {
        bail!("Input data is empty, nothing to sample");
    }

    let sample_count = match args.count {
        Some(n) => n,
        None => {
            let ratio = args.ratio.unwrap();
            ((records.len() as f64 * ratio).ceil() as usize).max(1)
        }
    };

    let sampled = match method {
        SamplingMethod::Random => sample_random(&records, sample_count, args.seed)?,
        SamplingMethod::Systematic => sample_systematic(&records, sample_count)?,
        SamplingMethod::Stratified => {
            let field = args.stratify_by.unwrap();
            sample_stratified(&records, sample_count, field, args.seed)?
        }
    };

    let result = Value::Array(sampled);

    // Determine output format
    let output_format = match args.format {
        Some(f) => Format::from_str(f)?,
        None => source_format,
    };

    let write_options = FormatOptions {
        delimiter: args.delimiter,
        no_header: args.no_header,
        pretty: args.pretty,
        ..Default::default()
    };

    write_output(&result, output_format, &write_options, args.output)?;

    Ok(())
}

enum SamplingMethod {
    Random,
    Systematic,
    Stratified,
}

impl SamplingMethod {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "random" => Ok(Self::Random),
            "systematic" => Ok(Self::Systematic),
            "stratified" => Ok(Self::Stratified),
            other => bail!(
                "Unknown sampling method: '{}'\n  Hint: supported methods are random, systematic, stratified",
                other
            ),
        }
    }
}

fn sample_random(records: &[Value], count: usize, seed: Option<u64>) -> Result<Vec<Value>> {
    let count = count.min(records.len());
    let mut indices: Vec<usize> = (0..records.len()).collect();

    match seed {
        Some(s) => {
            let mut rng = StdRng::seed_from_u64(s);
            indices.shuffle(&mut rng);
        }
        None => {
            let mut rng = rand::thread_rng();
            indices.shuffle(&mut rng);
        }
    }

    indices.truncate(count);
    indices.sort_unstable(); // preserve original order
    Ok(indices.iter().map(|&i| records[i].clone()).collect())
}

fn sample_systematic(records: &[Value], count: usize) -> Result<Vec<Value>> {
    let count = count.min(records.len());
    if count == 0 {
        return Ok(vec![]);
    }

    let step = records.len() as f64 / count as f64;
    let mut result = Vec::with_capacity(count);

    for i in 0..count {
        let idx = (i as f64 * step).floor() as usize;
        let idx = idx.min(records.len() - 1);
        result.push(records[idx].clone());
    }

    Ok(result)
}

fn sample_stratified(
    records: &[Value],
    count: usize,
    field: &str,
    seed: Option<u64>,
) -> Result<Vec<Value>> {
    // Group records by field value
    let mut groups: HashMap<String, Vec<(usize, &Value)>> = HashMap::new();

    for (i, record) in records.iter().enumerate() {
        let key = match record {
            Value::Object(obj) => match obj.get(field) {
                Some(Value::String(s)) => s.clone(),
                Some(v) => format!("{v}"),
                None => "__null__".to_string(),
            },
            _ => "__non_object__".to_string(),
        };
        groups.entry(key).or_default().push((i, record));
    }

    let total = records.len();
    let count = count.min(total);
    let mut result_indices: Vec<usize> = Vec::with_capacity(count);

    // Allocate proportionally per group
    let mut remaining = count;
    let mut group_entries: Vec<(String, Vec<(usize, &Value)>)> = groups.into_iter().collect();
    group_entries.sort_by(|a, b| a.0.cmp(&b.0)); // deterministic order

    let num_groups = group_entries.len();
    for (gi, (_, group)) in group_entries.iter().enumerate() {
        let group_count = if gi == num_groups - 1 {
            remaining
        } else {
            let proportion = group.len() as f64 / total as f64;
            let alloc = (proportion * count as f64).round() as usize;
            alloc.min(remaining).min(group.len())
        };

        let mut group_indices: Vec<usize> = group.iter().map(|(i, _)| *i).collect();

        match seed {
            Some(s) => {
                let mut rng = StdRng::seed_from_u64(s.wrapping_add(gi as u64));
                group_indices.shuffle(&mut rng);
            }
            None => {
                let mut rng = rand::thread_rng();
                group_indices.shuffle(&mut rng);
            }
        }

        group_indices.truncate(group_count);
        result_indices.extend_from_slice(&group_indices);
        remaining = remaining.saturating_sub(group_count);
    }

    result_indices.sort_unstable(); // preserve original order
    Ok(result_indices.iter().map(|&i| records[i].clone()).collect())
}

// ── Input reading ──

fn read_input_as_value(args: &SampleArgs) -> Result<(Value, Format)> {
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
            let content = read_file_with_encoding(Path::new(args.input), &args.encoding_opts)?;
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

// ── Output writing ──

fn write_output(
    value: &Value,
    format: Format,
    options: &FormatOptions,
    output: Option<&Path>,
) -> Result<()> {
    if format == Format::Msgpack {
        let bytes = MsgpackWriter.write_bytes(value)?;
        if let Some(out_path) = output {
            fs::write(out_path, &bytes)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            io::stdout()
                .write_all(&bytes)
                .context("Failed to write to stdout")?;
        }
    } else if format == Format::Parquet {
        let bytes = super::write_parquet_to_bytes(value, &ParquetWriteOptions::default())?;
        if let Some(out_path) = output {
            fs::write(out_path, &bytes)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else {
            io::stdout()
                .write_all(&bytes)
                .context("Failed to write Parquet to stdout")?;
        }
    } else {
        let result = write_value(value, format, options)?;
        if let Some(out_path) = output {
            fs::write(out_path, &result)
                .with_context(|| format!("Failed to write to {}", out_path.display()))?;
        } else if result.ends_with('\n') {
            print!("{result}");
        } else {
            println!("{result}");
        }
    }
    Ok(())
}

fn write_value(value: &Value, format: Format, options: &FormatOptions) -> Result<String> {
    match format {
        Format::Json => JsonWriter::new(options.clone()).write(value),
        Format::Jsonl => JsonlWriter.write(value),
        Format::Csv => CsvWriter::new(options.clone()).write(value),
        Format::Yaml => YamlWriter::new(options.clone()).write(value),
        Format::Toml => TomlWriter::new(options.clone()).write(value),
        Format::Xml => XmlWriter::new(options.pretty, options.root_element.clone()).write(value),
        Format::Env => EnvWriter.write(value),
        Format::Ini => IniWriter.write(value),
        Format::Properties => PropertiesWriter.write(value),
        Format::Hcl => HclWriter.write(value),
        Format::Msgpack => MsgpackWriter.write(value),
        Format::Xlsx => bail!("Excel is an input-only format and cannot be used as output"),
        Format::Sqlite => bail!("SQLite is an input-only format and cannot be used as output"),
        Format::Parquet => {
            bail!("Internal error: Parquet output should be handled via write_output")
        }
        Format::Markdown => MarkdownWriter.write(value),
        Format::Html => HtmlWriter::new(options.styled, options.full_html).write(value),
        Format::Table => {
            use crate::output::table::{render_table, TableOptions};
            Ok(render_table(value, &TableOptions::default()) + "\n")
        }
        _ => bail!("Unsupported output format: {format}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    fn make_record(name: &str, category: &str, age: i64) -> Value {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String(name.to_string()));
        m.insert("category".to_string(), Value::String(category.to_string()));
        m.insert("age".to_string(), Value::Integer(age));
        Value::Object(m)
    }

    fn sample_data() -> Vec<Value> {
        vec![
            make_record("Alice", "A", 30),
            make_record("Bob", "B", 25),
            make_record("Charlie", "A", 35),
            make_record("Diana", "B", 28),
            make_record("Eve", "A", 22),
            make_record("Frank", "C", 40),
            make_record("Grace", "C", 33),
            make_record("Hank", "B", 27),
            make_record("Ivy", "A", 31),
            make_record("Jack", "C", 29),
        ]
    }

    #[test]
    fn test_sample_random_basic() {
        let data = sample_data();
        let result = sample_random(&data, 3, Some(42)).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_sample_random_reproducible() {
        let data = sample_data();
        let r1 = sample_random(&data, 5, Some(123)).unwrap();
        let r2 = sample_random(&data, 5, Some(123)).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_sample_random_different_seeds() {
        let data = sample_data();
        let r1 = sample_random(&data, 5, Some(1)).unwrap();
        let r2 = sample_random(&data, 5, Some(2)).unwrap();
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_sample_random_count_exceeds_total() {
        let data = sample_data();
        let result = sample_random(&data, 100, Some(42)).unwrap();
        assert_eq!(result.len(), 10); // capped at total
    }

    #[test]
    fn test_sample_systematic_basic() {
        let data = sample_data();
        let result = sample_systematic(&data, 3).unwrap();
        assert_eq!(result.len(), 3);
        // Should pick evenly spaced records
        assert_eq!(result[0], data[0]); // index 0
        assert_eq!(result[1], data[3]); // index 3
        assert_eq!(result[2], data[6]); // index 6
    }

    #[test]
    fn test_sample_systematic_all() {
        let data = sample_data();
        let result = sample_systematic(&data, 10).unwrap();
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_sample_stratified_basic() {
        let data = sample_data();
        // 4 in A, 3 in B, 3 in C → proportional allocation of 6
        let result = sample_stratified(&data, 6, "category", Some(42)).unwrap();
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_sample_stratified_reproducible() {
        let data = sample_data();
        let r1 = sample_stratified(&data, 6, "category", Some(42)).unwrap();
        let r2 = sample_stratified(&data, 6, "category", Some(42)).unwrap();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_sample_method_from_str() {
        assert!(matches!(
            SamplingMethod::from_str("random").unwrap(),
            SamplingMethod::Random
        ));
        assert!(matches!(
            SamplingMethod::from_str("systematic").unwrap(),
            SamplingMethod::Systematic
        ));
        assert!(matches!(
            SamplingMethod::from_str("stratified").unwrap(),
            SamplingMethod::Stratified
        ));
        assert!(SamplingMethod::from_str("invalid").is_err());
    }

    #[test]
    fn test_sample_random_preserves_order() {
        let data = sample_data();
        let result = sample_random(&data, 5, Some(42)).unwrap();
        // Check that sampled records maintain their relative order
        let mut prev_idx = None;
        for r in &result {
            let idx = data.iter().position(|d| d == r).unwrap();
            if let Some(pi) = prev_idx {
                assert!(idx > pi, "Records should be in original order");
            }
            prev_idx = Some(idx);
        }
    }
}
