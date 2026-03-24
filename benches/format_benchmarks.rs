use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dkit::format::{FormatOptions, FormatReader, FormatWriter};
use dkit::format::csv::{CsvReader, CsvWriter};
use dkit::format::json::{JsonReader, JsonWriter};
use dkit::format::yaml::{YamlReader, YamlWriter};
use dkit::value::Value;
use indexmap::IndexMap;

/// Generate a JSON array string with `n` records
fn generate_json_data(n: usize) -> String {
    let records: Vec<String> = (0..n)
        .map(|i| {
            format!(
                r#"{{"id":{i},"name":"User {i}","age":{age},"score":{score:.2},"active":{active}}}"#,
                i = i,
                age = 20 + (i % 60),
                score = (i as f64 * 1.5) % 100.0,
                active = i % 2 == 0,
            )
        })
        .collect();
    format!("[{}]", records.join(","))
}

/// Generate a CSV string with `n` records
fn generate_csv_data(n: usize) -> String {
    let mut lines = vec!["id,name,age,score,active".to_string()];
    for i in 0..n {
        lines.push(format!(
            "{i},User {i},{},{:.2},{}",
            20 + (i % 60),
            (i as f64 * 1.5) % 100.0,
            i % 2 == 0,
        ));
    }
    lines.join("\n")
}

/// Generate a YAML array string with `n` records
fn generate_yaml_data(n: usize) -> String {
    let mut lines = Vec::new();
    for i in 0..n {
        lines.push(format!(
            "- id: {i}\n  name: \"User {i}\"\n  age: {age}\n  score: {score:.2}\n  active: {active}",
            i = i,
            age = 20 + (i % 60),
            score = (i as f64 * 1.5) % 100.0,
            active = i % 2 == 0,
        ));
    }
    lines.join("\n")
}

/// Generate a Value::Array with `n` records for write benchmarks
fn generate_value(n: usize) -> Value {
    let records: Vec<Value> = (0..n)
        .map(|i| {
            let mut map = IndexMap::new();
            map.insert("id".to_string(), Value::Integer(i as i64));
            map.insert("name".to_string(), Value::String(format!("User {i}")));
            map.insert("age".to_string(), Value::Integer((20 + (i % 60)) as i64));
            map.insert(
                "score".to_string(),
                Value::Float((i as f64 * 1.5) % 100.0),
            );
            map.insert("active".to_string(), Value::Bool(i % 2 == 0));
            Value::Object(map)
        })
        .collect();
    Value::Array(records)
}

// Benchmark sizes: small (~1K records), medium (~10K records)
const SMALL: usize = 1_000;
const MEDIUM: usize = 10_000;

fn bench_json_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_read");
    for size in [SMALL, MEDIUM] {
        let data = generate_json_data(size);
        let reader = JsonReader;
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| reader.read(data).unwrap())
        });
    }
    group.finish();
}

fn bench_json_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_write");
    for size in [SMALL, MEDIUM] {
        let value = generate_value(size);
        let writer = JsonWriter::new(FormatOptions {
            compact: true,
            ..Default::default()
        });
        group.bench_with_input(BenchmarkId::from_parameter(size), &value, |b, value| {
            b.iter(|| writer.write(value).unwrap())
        });
    }
    group.finish();
}

fn bench_csv_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_read");
    for size in [SMALL, MEDIUM] {
        let data = generate_csv_data(size);
        let reader = CsvReader::default();
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| reader.read(data).unwrap())
        });
    }
    group.finish();
}

fn bench_csv_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_write");
    for size in [SMALL, MEDIUM] {
        let value = generate_value(size);
        let writer = CsvWriter::default();
        group.bench_with_input(BenchmarkId::from_parameter(size), &value, |b, value| {
            b.iter(|| writer.write(value).unwrap())
        });
    }
    group.finish();
}

fn bench_yaml_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("yaml_read");
    // YAML is significantly slower; use smaller sizes
    for size in [100, 500] {
        let data = generate_yaml_data(size);
        let reader = YamlReader;
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| reader.read(data).unwrap())
        });
    }
    group.finish();
}

fn bench_yaml_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("yaml_write");
    for size in [100, 500] {
        let value = generate_value(size);
        let writer = YamlWriter::default();
        group.bench_with_input(BenchmarkId::from_parameter(size), &value, |b, value| {
            b.iter(|| writer.write(value).unwrap())
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_json_read,
    bench_json_write,
    bench_csv_read,
    bench_csv_write,
    bench_yaml_read,
    bench_yaml_write,
);
criterion_main!(benches);
