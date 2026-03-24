use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

/// 테스트용 Parquet 파일을 생성한다.
fn create_test_parquet(path: &std::path::Path) {
    use arrow::array::{ArrayRef, BooleanArray, Float64Array, Int64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;

    let schema = Arc::new(Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("age", DataType::Int64, false),
        Field::new("score", DataType::Float64, true),
        Field::new("active", DataType::Boolean, false),
    ]));

    let names: ArrayRef = Arc::new(StringArray::from(vec!["Alice", "Bob", "Charlie"]));
    let ages: ArrayRef = Arc::new(Int64Array::from(vec![30, 25, 35]));
    let scores: ArrayRef = Arc::new(Float64Array::from(vec![Some(95.5), None, Some(87.3)]));
    let actives: ArrayRef = Arc::new(BooleanArray::from(vec![true, false, true]));

    let batch = RecordBatch::try_new(schema.clone(), vec![names, ages, scores, actives]).unwrap();

    let file = std::fs::File::create(path).unwrap();
    let mut writer = ArrowWriter::try_new(file, schema, None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
}

// --- convert ---

#[test]
fn convert_parquet_to_json() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args([
            "convert",
            pq_path.to_str().unwrap(),
            "-f",
            "json",
            "--compact",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie"));
}

#[test]
fn convert_parquet_to_csv() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["convert", pq_path.to_str().unwrap(), "-f", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,score,active"))
        .stdout(predicate::str::contains("Alice,30,95.5,true"));
}

#[test]
fn convert_parquet_to_yaml() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["convert", pq_path.to_str().unwrap(), "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("age: 30"));
}

// --- view ---

#[test]
fn view_parquet() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["view", pq_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie"));
}

#[test]
fn view_parquet_with_limit() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["view", pq_path.to_str().unwrap(), "--limit", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie").not());
}

// --- stats ---

#[test]
fn stats_parquet() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["stats", pq_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 3"))
        .stdout(predicate::str::contains("name"));
}

// --- schema ---

#[test]
fn schema_parquet() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["schema", pq_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

// --- query ---

#[test]
fn query_parquet() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    dkit()
        .args(["query", pq_path.to_str().unwrap(), ".[0].name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- format detection ---

#[test]
fn parquet_format_auto_detected() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    // No --from needed, should auto-detect from .parquet extension
    dkit()
        .args([
            "convert",
            pq_path.to_str().unwrap(),
            "-f",
            "json",
            "--compact",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- parquet as output (write) ---

#[test]
fn convert_json_to_parquet_succeeds() {
    let dir = tempfile::tempdir().unwrap();
    let json_path = dir.path().join("data.json");
    let out_path = dir.path().join("output.parquet");
    std::fs::write(&json_path, r#"[{"id":1,"name":"Alice"}]"#).unwrap();

    dkit()
        .args([
            "convert",
            json_path.to_str().unwrap(),
            "-f",
            "parquet",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // The output file should be a valid Parquet file (starts with PAR1)
    let bytes = std::fs::read(&out_path).unwrap();
    assert!(
        bytes.starts_with(b"PAR1"),
        "Output should be a valid Parquet file"
    );
}

// --- null handling ---

#[test]
fn parquet_null_values_to_json() {
    let dir = tempfile::tempdir().unwrap();
    let pq_path = dir.path().join("data.parquet");
    create_test_parquet(&pq_path);

    let output = dkit()
        .args([
            "convert",
            pq_path.to_str().unwrap(),
            "-f",
            "json",
            "--compact",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Bob's score is null
    assert!(stdout.contains("null"));
}
