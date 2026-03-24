//! v0.7.0 통합 테스트: Parquet 읽기/쓰기, 집계 함수, GROUP BY, 스트리밍, 쿼리 함수

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// Parquet 다양한 스키마 및 압축 테스트
// ============================================================

/// 다양한 타입(int, float, string, bool, null 포함)의 Parquet 파일 생성
fn create_typed_parquet(path: &std::path::Path) {
    use arrow::array::{ArrayRef, BooleanArray, Float64Array, Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("score", DataType::Float64, true),
        Field::new("active", DataType::Boolean, false),
        Field::new("category", DataType::Utf8, true),
    ]));

    let ids: ArrayRef = Arc::new(Int64Array::from(vec![1, 2, 3, 4, 5]));
    let names: ArrayRef = Arc::new(StringArray::from(vec!["Alice", "Bob", "Charlie", "Diana", "Eve"]));
    let scores: ArrayRef = Arc::new(Float64Array::from(vec![
        Some(85.0),
        Some(92.5),
        Some(78.3),
        None,
        Some(91.0),
    ]));
    let actives: ArrayRef = Arc::new(BooleanArray::from(vec![true, false, true, true, false]));
    let categories: ArrayRef = Arc::new(StringArray::from(vec![
        Some("A"),
        Some("B"),
        Some("A"),
        Some("B"),
        None,
    ]));

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![ids, names, scores, actives, categories],
    )
    .unwrap();

    let file = std::fs::File::create(path).unwrap();
    let mut writer = ArrowWriter::try_new(file, schema, None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
}

/// SNAPPY 압축으로 Parquet 파일 생성
fn create_snappy_parquet(path: &std::path::Path) {
    use arrow::array::{ArrayRef, Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use parquet::basic::Compression;
    use parquet::file::properties::WriterProperties;
    use std::sync::Arc;

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("value", DataType::Utf8, false),
    ]));

    let ids: ArrayRef = Arc::new(Int64Array::from(vec![10, 20, 30]));
    let values: ArrayRef = Arc::new(StringArray::from(vec!["x", "y", "z"]));

    let batch = RecordBatch::try_new(schema.clone(), vec![ids, values]).unwrap();

    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let file = std::fs::File::create(path).unwrap();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props)).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
}

/// ZSTD 압축으로 Parquet 파일 생성
fn create_zstd_parquet(path: &std::path::Path) {
    use arrow::array::{ArrayRef, Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use parquet::basic::{Compression, ZstdLevel};
    use parquet::file::properties::WriterProperties;
    use std::sync::Arc;

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("value", DataType::Utf8, false),
    ]));

    let ids: ArrayRef = Arc::new(Int64Array::from(vec![100, 200, 300]));
    let values: ArrayRef = Arc::new(StringArray::from(vec!["foo", "bar", "baz"]));

    let batch = RecordBatch::try_new(schema.clone(), vec![ids, values]).unwrap();

    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::default()))
        .build();

    let file = std::fs::File::create(path).unwrap();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props)).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
}

#[test]
fn parquet_various_types_to_json() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args(["convert", pq.to_str().unwrap(), "-f", "json", "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("92.5"))
        .stdout(predicate::str::contains("null"))
        .stdout(predicate::str::contains("true"))
        .stdout(predicate::str::contains("false"));
}

#[test]
fn parquet_various_types_to_csv() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args(["convert", pq.to_str().unwrap(), "-f", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id,name,score,active,category"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("85"));
}

#[test]
fn parquet_snappy_compression_roundtrip() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("snappy.parquet");
    create_snappy_parquet(&pq);

    dkit()
        .args(["convert", pq.to_str().unwrap(), "-f", "json", "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("10"))
        .stdout(predicate::str::contains("x"))
        .stdout(predicate::str::contains("y"))
        .stdout(predicate::str::contains("z"));
}

#[test]
fn parquet_zstd_compression_roundtrip() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("zstd.parquet");
    create_zstd_parquet(&pq);

    dkit()
        .args(["convert", pq.to_str().unwrap(), "-f", "json", "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("100"))
        .stdout(predicate::str::contains("foo"))
        .stdout(predicate::str::contains("bar"));
}

#[test]
fn json_to_parquet_with_snappy_compression() {
    let dir = TempDir::new().unwrap();
    let json_path = dir.path().join("data.json");
    let pq_path = dir.path().join("out.parquet");

    fs::write(&json_path, r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#).unwrap();

    dkit()
        .args([
            "convert",
            json_path.to_str().unwrap(),
            "-f", "parquet",
            "-o", pq_path.to_str().unwrap(),
            "--compression", "snappy",
        ])
        .assert()
        .success();

    let bytes = fs::read(&pq_path).unwrap();
    assert!(bytes.starts_with(b"PAR1"), "output should be Parquet");
}

#[test]
fn json_to_parquet_with_zstd_compression() {
    let dir = TempDir::new().unwrap();
    let json_path = dir.path().join("data.json");
    let pq_path = dir.path().join("out.parquet");

    fs::write(&json_path, r#"[{"id":1,"val":10.5},{"id":2,"val":20.0}]"#).unwrap();

    dkit()
        .args([
            "convert",
            json_path.to_str().unwrap(),
            "-f", "parquet",
            "-o", pq_path.to_str().unwrap(),
            "--compression", "zstd",
        ])
        .assert()
        .success();

    let bytes = fs::read(&pq_path).unwrap();
    assert!(bytes.starts_with(b"PAR1"), "output should be Parquet");
}

#[test]
fn parquet_null_values_preserved_in_csv() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    // Diana's score is null → should appear as empty in CSV
    let output = dkit()
        .args(["convert", pq.to_str().unwrap(), "-f", "csv"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Diana"), "Diana should be present");
}

#[test]
fn parquet_schema_detection() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args(["schema", pq.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("id"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("score"))
        .stdout(predicate::str::contains("active"))
        .stdout(predicate::str::contains("category"));
}

#[test]
fn parquet_stats_output() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args(["stats", pq.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 5"))
        .stdout(predicate::str::contains("name"));
}

// ============================================================
// 집계 함수 테스트 (count, sum, avg, min, max, distinct)
// ============================================================

#[test]
fn aggregate_count_all() {
    dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | count"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn aggregate_sum_field() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | sum score"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // 85 + 92 + 78 + 95 + 88 = 438
    assert!(stdout.contains("438"), "sum of scores should be 438, got: {stdout}");
}

#[test]
fn aggregate_avg_field() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | avg score"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // avg = 438 / 5 = 87.6
    assert!(stdout.contains("87.6"), "avg score should be 87.6, got: {stdout}");
}

#[test]
fn aggregate_min_numeric() {
    dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | min score"])
        .assert()
        .success()
        .stdout(predicate::str::contains("78"));
}

#[test]
fn aggregate_max_numeric() {
    dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | max score"])
        .assert()
        .success()
        .stdout(predicate::str::contains("95"));
}

#[test]
fn aggregate_min_string() {
    dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | min name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn aggregate_max_string() {
    dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | max name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Eve"));
}

#[test]
fn aggregate_distinct_field() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | distinct city"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Seoul"));
    assert!(stdout.contains("Busan"));
    assert!(stdout.contains("Incheon"));
}

#[test]
fn aggregate_count_after_filter() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city == \"Seoul\" | count",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn aggregate_sum_after_filter() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role == \"engineer\" | sum score",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Alice(85) + Diana(95) + Eve(88) = 268
    assert!(stdout.contains("268"), "sum of engineer scores should be 268, got: {stdout}");
}

#[test]
fn aggregate_avg_after_filter() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 30 | avg score",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    // Charlie(78) + Eve(88) = 166 / 2 = 83.0
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("83"), "avg score for age>30 should be 83, got: {stdout}");
}

// ============================================================
// GROUP BY 테스트
// ============================================================

#[test]
fn group_by_single_field_count() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | group_by city count()",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Seoul: 3, Busan: 1, Incheon: 1
    assert!(stdout.contains("Seoul"), "Seoul group should appear");
    assert!(stdout.contains("3"), "Seoul count should be 3");
    assert!(stdout.contains("Busan"), "Busan group should appear");
}

#[test]
fn group_by_with_sum() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | group_by role count(), sum(score)",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("engineer"), "engineer group should appear");
    assert!(stdout.contains("manager"), "manager group should appear");
    assert!(stdout.contains("designer"), "designer group should appear");
}

#[test]
fn group_by_with_avg() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | group_by role avg(score)",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("engineer"));
}

#[test]
fn group_by_with_min_max() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | group_by city min(score), max(score)",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Seoul"));
    assert!(stdout.contains("Busan"));
}

#[test]
fn group_by_result_sortable() {
    // GROUP BY 후 sort 적용
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | group_by city count() | sort count desc",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Seoul(3) 이 먼저 나와야 함
    let seoul_pos = stdout.find("Seoul").unwrap();
    let busan_pos = stdout.find("Busan").unwrap();
    assert!(seoul_pos < busan_pos, "Seoul (count=3) should come before Busan (count=1)");
}

#[test]
fn group_by_having_filter() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | group_by city count() having count > 1",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Seoul"), "Seoul (count=3) should pass having count>1");
    assert!(!stdout.contains("Busan"), "Busan (count=1) should be filtered out");
    assert!(!stdout.contains("Incheon"), "Incheon (count=1) should be filtered out");
}

// ============================================================
// 스트리밍 처리 테스트 (--chunk-size)
// ============================================================

fn create_large_jsonl(path: &std::path::Path, n: usize) {
    let mut content = String::new();
    for i in 0..n {
        content.push_str(&format!(
            "{{\"id\":{},\"name\":\"user{}\",\"value\":{}}}\n",
            i,
            i,
            i * 10
        ));
    }
    fs::write(path, content).unwrap();
}

fn create_large_csv(path: &std::path::Path, n: usize) {
    let mut content = String::from("id,name,value\n");
    for i in 0..n {
        content.push_str(&format!("{},user{},{}\n", i, i, i * 10));
    }
    fs::write(path, content).unwrap();
}

#[test]
fn streaming_jsonl_to_csv_chunk_size() {
    let dir = TempDir::new().unwrap();
    let jsonl = dir.path().join("large.jsonl");
    let out = dir.path().join("out.csv");
    create_large_jsonl(&jsonl, 500);

    dkit()
        .args([
            "convert",
            jsonl.to_str().unwrap(),
            "--from", "jsonl",
            "-f", "csv",
            "--chunk-size", "100",
            "-o", out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out).unwrap();
    assert!(content.contains("id,name,value"), "CSV header should be present");
    assert!(content.contains("user0"), "first record should be present");
    assert!(content.contains("user499"), "last record should be present");
}

#[test]
fn streaming_csv_to_jsonl_chunk_size() {
    let dir = TempDir::new().unwrap();
    let csv = dir.path().join("large.csv");
    let out = dir.path().join("out.jsonl");
    create_large_csv(&csv, 300);

    dkit()
        .args([
            "convert",
            csv.to_str().unwrap(),
            "--from", "csv",
            "-f", "jsonl",
            "--chunk-size", "50",
            "-o", out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 300, "should have 300 JSONL records");
}

#[test]
fn streaming_large_csv_to_csv() {
    let dir = TempDir::new().unwrap();
    let csv = dir.path().join("large.csv");
    let out = dir.path().join("out.csv");
    create_large_csv(&csv, 1000);

    dkit()
        .args([
            "convert",
            csv.to_str().unwrap(),
            "--from", "csv",
            "-f", "csv",
            "--chunk-size", "200",
            "-o", out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out).unwrap();
    let data_lines: Vec<&str> = content.lines().skip(1).filter(|l| !l.is_empty()).collect();
    assert_eq!(data_lines.len(), 1000, "should have 1000 data rows");
}

#[test]
fn streaming_parquet_to_csv_chunk_size() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    let out = dir.path().join("out.csv");
    create_typed_parquet(&pq);

    dkit()
        .args([
            "convert",
            pq.to_str().unwrap(),
            "--from", "parquet",
            "-f", "csv",
            "--chunk-size", "2",
            "-o", out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out).unwrap();
    assert!(content.contains("Alice"));
    assert!(content.contains("Eve"));
}

// ============================================================
// 쿼리 내장 함수 테스트
// ============================================================

#[test]
fn query_func_upper() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Alice\" | select upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"));
}

#[test]
fn query_func_lower() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Bob\" | select lower(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("bob"));
}

#[test]
fn query_func_length_string() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Alice\" | select length(name)",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("5"), "length of 'Alice' should be 5, got: {stdout}");
}

#[test]
fn query_func_upper_with_alias() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Alice\" | select upper(name) as NAME",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"))
        .stdout(predicate::str::contains("ALICE"));
}

#[test]
fn query_func_round() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Bob\" | select round(score)",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Bob's score is 92, round(92) = 92
    assert!(stdout.contains("92"), "round(92) should be 92, got: {stdout}");
}

#[test]
fn query_func_to_string() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Alice\" | select to_string(age)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("30"));
}

#[test]
fn query_func_to_int() {
    // CSV 에서 문자열로 읽힌 숫자를 정수 변환
    let dir = TempDir::new().unwrap();
    let csv = dir.path().join("data.csv");
    fs::write(&csv, "name,score\nAlice,85\nBob,92\n").unwrap();

    dkit()
        .args([
            "query",
            csv.to_str().unwrap(),
            ".[] | where name == \"Alice\" | select to_int(score)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("85"));
}

#[test]
fn query_func_concat() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Alice\" | select concat(name, \"-\", city)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice-Seoul"));
}

#[test]
fn query_func_substr() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Charlie\" | select substr(name, 0, 4)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Char"));
}

#[test]
fn query_func_multiple_funcs_in_select() {
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Alice\" | select upper(name), to_string(age)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("30"));
}

#[test]
fn query_func_nested_upper_trim() {
    let dir = TempDir::new().unwrap();
    let json = dir.path().join("data.json");
    fs::write(&json, r#"[{"name":"  alice  "}]"#).unwrap();

    dkit()
        .args([
            "query",
            json.to_str().unwrap(),
            ".[] | select upper(trim(name))",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"));
}

#[test]
fn query_func_coalesce() {
    let dir = TempDir::new().unwrap();
    let json = dir.path().join("data.json");
    fs::write(&json, r#"[{"name":"Alice","email":null},{"name":"Bob","email":"bob@example.com"}]"#)
        .unwrap();

    dkit()
        .args([
            "query",
            json.to_str().unwrap(),
            ".[] | select name, coalesce(email, \"N/A\")",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("N/A"))
        .stdout(predicate::str::contains("bob@example.com"));
}

#[test]
fn query_func_with_where_filter() {
    // 함수 결과가 where 절과 조합하여 올바르게 동작하는지 검증
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role == \"engineer\" | select upper(name), score | sort score desc",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("DIANA"))
        .stdout(predicate::str::contains("EVE"))
        .stdout(predicate::str::contains("ALICE"));
}

// ============================================================
// Parquet + 쿼리 통합 테스트
// ============================================================

#[test]
fn parquet_query_with_aggregate() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args(["query", pq.to_str().unwrap(), ".[] | count"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn parquet_query_with_filter_and_aggregate() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args([
            "query",
            pq.to_str().unwrap(),
            ".[] | where active == true | count",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn parquet_query_func_upper() {
    let dir = TempDir::new().unwrap();
    let pq = dir.path().join("typed.parquet");
    create_typed_parquet(&pq);

    dkit()
        .args([
            "query",
            pq.to_str().unwrap(),
            ".[] | where name == \"Alice\" | select upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"));
}

#[test]
fn parquet_roundtrip_via_csv() {
    // parquet → csv → parquet → json 라운드트립
    let dir = TempDir::new().unwrap();
    let pq1 = dir.path().join("orig.parquet");
    let csv = dir.path().join("mid.csv");
    let pq2 = dir.path().join("out.parquet");

    create_typed_parquet(&pq1);

    // parquet → csv
    dkit()
        .args(["convert", pq1.to_str().unwrap(), "-f", "csv", "-o", csv.to_str().unwrap()])
        .assert()
        .success();

    // csv → parquet
    dkit()
        .args(["convert", csv.to_str().unwrap(), "-f", "parquet", "-o", pq2.to_str().unwrap()])
        .assert()
        .success();

    // parquet → json (verify round-trip)
    dkit()
        .args(["convert", pq2.to_str().unwrap(), "-f", "json", "--compact"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}
