use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dkit::format::{FormatOptions, FormatReader, FormatWriter};
use dkit::format::csv::{CsvReader, CsvWriter};
use dkit::format::json::{JsonReader, JsonWriter};
use dkit::format::jsonl::{JsonlReader, JsonlWriter};
use dkit::value::Value;
use indexmap::IndexMap;

fn generate_json_array(n: usize) -> String {
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

fn generate_csv(n: usize) -> String {
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

fn generate_jsonl(n: usize) -> String {
    (0..n)
        .map(|i| {
            format!(
                r#"{{"id":{i},"name":"User {i}","age":{age},"score":{score:.2},"active":{active}}}"#,
                i = i,
                age = 20 + (i % 60),
                score = (i as f64 * 1.5) % 100.0,
                active = i % 2 == 0,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

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

const SMALL: usize = 1_000;
const MEDIUM: usize = 10_000;

fn bench_json_to_csv(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_json_to_csv");
    let json_reader = JsonReader;
    let csv_writer = CsvWriter::default();
    for size in [SMALL, MEDIUM] {
        let input = generate_json_array(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let value = json_reader.read(input).unwrap();
                csv_writer.write(&value).unwrap()
            })
        });
    }
    group.finish();
}

fn bench_csv_to_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_csv_to_json");
    let csv_reader = CsvReader::default();
    let json_writer = JsonWriter::new(FormatOptions {
        compact: true,
        ..Default::default()
    });
    for size in [SMALL, MEDIUM] {
        let input = generate_csv(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let value = csv_reader.read(input).unwrap();
                json_writer.write(&value).unwrap()
            })
        });
    }
    group.finish();
}

fn bench_json_to_jsonl(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_json_to_jsonl");
    let json_reader = JsonReader;
    let jsonl_writer = JsonlWriter;
    for size in [SMALL, MEDIUM] {
        let input = generate_json_array(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let value = json_reader.read(input).unwrap();
                jsonl_writer.write(&value).unwrap()
            })
        });
    }
    group.finish();
}

fn bench_jsonl_to_csv(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert_jsonl_to_csv");
    let jsonl_reader = JsonlReader;
    let csv_writer = CsvWriter::default();
    for size in [SMALL, MEDIUM] {
        let input = generate_jsonl(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let value = jsonl_reader.read(input).unwrap();
                csv_writer.write(&value).unwrap()
            })
        });
    }
    group.finish();
}

fn bench_value_to_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("value_serialize");
    let json_writer = JsonWriter::new(FormatOptions {
        compact: true,
        ..Default::default()
    });
    let csv_writer = CsvWriter::default();
    let jsonl_writer = JsonlWriter;

    let value = generate_value(SMALL);

    group.bench_function("json", |b| b.iter(|| json_writer.write(&value).unwrap()));
    group.bench_function("csv", |b| b.iter(|| csv_writer.write(&value).unwrap()));
    group.bench_function("jsonl", |b| b.iter(|| jsonl_writer.write(&value).unwrap()));
    group.finish();
}

criterion_group!(
    benches,
    bench_json_to_csv,
    bench_csv_to_json,
    bench_json_to_jsonl,
    bench_jsonl_to_csv,
    bench_value_to_formats,
);
criterion_main!(benches);
