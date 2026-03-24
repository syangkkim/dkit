use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dkit::query::filter::apply_operations;
use dkit::query::parser::parse_query;
use dkit::value::Value;
use indexmap::IndexMap;

/// Generate a Value::Array with `n` records for query benchmarks
fn make_records(n: usize) -> Value {
    let records: Vec<Value> = (0..n)
        .map(|i| {
            let mut map = IndexMap::new();
            map.insert("id".to_string(), Value::Integer(i as i64));
            map.insert("name".to_string(), Value::String(format!("User {i}")));
            map.insert("age".to_string(), Value::Integer((20 + (i % 60)) as i64));
            map.insert("score".to_string(), Value::Float((i as f64 * 1.5) % 100.0));
            map.insert(
                "category".to_string(),
                Value::String(["A", "B", "C", "D"][i % 4].to_string()),
            );
            map.insert("active".to_string(), Value::Bool(i % 2 == 0));
            Value::Object(map)
        })
        .collect();
    Value::Array(records)
}

const SMALL: usize = 1_000;
const MEDIUM: usize = 10_000;

fn bench_filter(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_filter");
    let query = parse_query(". | where age > 30").unwrap();
    for size in [SMALL, MEDIUM] {
        let data = make_records(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| apply_operations(data.clone(), &query.operations).unwrap())
        });
    }
    group.finish();
}

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_sort");
    let query = parse_query(". | sort age").unwrap();
    for size in [SMALL, MEDIUM] {
        let data = make_records(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| apply_operations(data.clone(), &query.operations).unwrap())
        });
    }
    group.finish();
}

fn bench_sort_desc(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_sort_desc");
    let query = parse_query(". | sort score desc").unwrap();
    for size in [SMALL, MEDIUM] {
        let data = make_records(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| apply_operations(data.clone(), &query.operations).unwrap())
        });
    }
    group.finish();
}

fn bench_aggregate_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_sum");
    let query = parse_query(". | sum score").unwrap();
    for size in [SMALL, MEDIUM] {
        let data = make_records(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| apply_operations(data.clone(), &query.operations).unwrap())
        });
    }
    group.finish();
}

fn bench_aggregate_group_by(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_group_by");
    let query = parse_query(". | group_by category").unwrap();
    for size in [SMALL, MEDIUM] {
        let data = make_records(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| apply_operations(data.clone(), &query.operations).unwrap())
        });
    }
    group.finish();
}

fn bench_filter_and_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_filter_sort");
    let query = parse_query(". | where age > 30 | sort score desc").unwrap();
    for size in [SMALL, MEDIUM] {
        let data = make_records(size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| apply_operations(data.clone(), &query.operations).unwrap())
        });
    }
    group.finish();
}

fn bench_query_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_parse");
    let queries = [
        ". | where age > 30",
        ". | sort score desc | limit 100",
        ". | group_by category | select category, count",
        ". | where active == true | sort age | limit 50",
    ];
    for query_str in queries {
        group.bench_with_input(BenchmarkId::from_parameter(query_str), query_str, |b, q| {
            b.iter(|| parse_query(q).unwrap())
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_filter,
    bench_sort,
    bench_sort_desc,
    bench_aggregate_sum,
    bench_aggregate_group_by,
    bench_filter_and_sort,
    bench_query_parse,
);
criterion_main!(benches);
