use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// diff 고도화 테스트
// ============================================================

// --- diff modes ---

#[test]
fn diff_mode_structural_default() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31, "email": "a@b.com"}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--mode",
            "structural",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("email"))
        .stdout(predicate::str::contains("(added)"));
}

#[test]
fn diff_mode_value() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--mode",
            "value",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("age"));
}

#[test]
fn diff_mode_key() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Bob", "email": "b@c.com"}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--mode",
            "key",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("email"));
}

#[test]
fn diff_invalid_mode() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("a.json");
    fs::write(&f, "{}").unwrap();

    dkit()
        .args(&[
            "diff",
            f.to_str().unwrap(),
            f.to_str().unwrap(),
            "--mode",
            "invalid",
        ])
        .assert()
        .failure();
}

// --- diff output formats ---

#[test]
fn diff_format_unified() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--diff-format",
            "unified",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("age"));
}

#[test]
fn diff_format_side_by_side() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--diff-format",
            "side-by-side",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("age"));
}

#[test]
fn diff_format_json() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31}"#).unwrap();

    let output = dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--diff-format",
            "json",
        ])
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // JSON output should be valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed.is_object() || parsed.is_array());
}

#[test]
fn diff_format_summary() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31, "email": "a@b.com"}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--diff-format",
            "summary",
        ])
        .assert()
        .failure();
}

#[test]
fn diff_invalid_format() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("a.json");
    fs::write(&f, "{}").unwrap();

    dkit()
        .args(&[
            "diff",
            f.to_str().unwrap(),
            f.to_str().unwrap(),
            "--diff-format",
            "invalid",
        ])
        .assert()
        .failure();
}

// --- diff array strategies ---

#[test]
fn diff_array_strategy_index() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(
        &f1,
        r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#,
    )
    .unwrap();
    fs::write(
        &f2,
        r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Charlie"}]"#,
    )
    .unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--array-diff",
            "index",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("name"));
}

#[test]
fn diff_array_strategy_value() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"[1, 2, 3]"#).unwrap();
    fs::write(&f2, r#"[1, 3, 4]"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--array-diff",
            "value",
        ])
        .assert()
        .failure();
}

#[test]
fn diff_array_strategy_key_field() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(
        &f1,
        r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#,
    )
    .unwrap();
    fs::write(
        &f2,
        r#"[{"id": 2, "name": "Charlie"}, {"id": 1, "name": "Alice"}]"#,
    )
    .unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--array-diff",
            "key=id",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("name"));
}

#[test]
fn diff_invalid_array_strategy() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("a.json");
    fs::write(&f, "[]").unwrap();

    dkit()
        .args(&[
            "diff",
            f.to_str().unwrap(),
            f.to_str().unwrap(),
            "--array-diff",
            "invalid",
        ])
        .assert()
        .failure();
}

// --- diff --ignore-order ---

#[test]
fn diff_ignore_order_arrays() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"[1, 2, 3]"#).unwrap();
    fs::write(&f2, r#"[3, 1, 2]"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--ignore-order",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("No differences found."));
}

#[test]
fn diff_without_ignore_order_arrays_differ() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"[1, 2, 3]"#).unwrap();
    fs::write(&f2, r#"[3, 1, 2]"#).unwrap();

    dkit()
        .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
        .assert()
        .failure();
}

// --- diff --ignore-case ---

#[test]
fn diff_ignore_case() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice"}"#).unwrap();
    fs::write(&f2, r#"{"name": "ALICE"}"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--ignore-case",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("No differences found."));
}

#[test]
fn diff_without_ignore_case_differs() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name": "Alice"}"#).unwrap();
    fs::write(&f2, r#"{"name": "ALICE"}"#).unwrap();

    dkit()
        .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
        .assert()
        .failure();
}

// --- diff combined options ---

#[test]
fn diff_mode_value_with_json_output() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"x": 1, "y": 2}"#).unwrap();
    fs::write(&f2, r#"{"x": 1, "y": 3}"#).unwrap();

    let output = dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--mode",
            "value",
            "--diff-format",
            "json",
        ])
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed.is_object() || parsed.is_array());
}

#[test]
fn diff_ignore_case_and_order() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"["Alice", "Bob"]"#).unwrap();
    fs::write(&f2, r#"["BOB", "ALICE"]"#).unwrap();

    dkit()
        .args(&[
            "diff",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--ignore-case",
            "--ignore-order",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("No differences found."));
}

// ============================================================
// validate 테스트
// ============================================================

#[test]
fn validate_valid_json() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name", "age"]
        }"#,
    )
    .unwrap();
    fs::write(&data, r#"{"name": "Alice", "age": 30}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn validate_invalid_json_type_mismatch() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name", "age"]
        }"#,
    )
    .unwrap();
    fs::write(&data, r#"{"name": "Alice", "age": "thirty"}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("Validation failed"));
}

#[test]
fn validate_missing_required_field() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{"type": "object", "required": ["name", "age"]}"#,
    )
    .unwrap();
    fs::write(&data, r#"{"name": "Alice"}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("error:"))
        .stdout(predicate::str::contains("age"));
}

#[test]
fn validate_quiet_mode_valid() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(&schema, r#"{"type": "object"}"#).unwrap();
    fs::write(&data, r#"{"name": "Alice"}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
            "--quiet",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn validate_quiet_mode_invalid() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(&schema, r#"{"type": "string"}"#).unwrap();
    fs::write(&data, r#"{"name": "Alice"}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
            "--quiet",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("invalid"));
}

#[test]
fn validate_yaml_input() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.yaml");
    fs::write(
        &schema,
        r#"{
            "type": "object",
            "properties": {"host": {"type": "string"}, "port": {"type": "integer"}},
            "required": ["host", "port"]
        }"#,
    )
    .unwrap();
    fs::write(&data, "host: localhost\nport: 8080\n").unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn validate_toml_input() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.toml");
    fs::write(
        &schema,
        r#"{"type": "object", "properties": {"name": {"type": "string"}}}"#,
    )
    .unwrap();
    fs::write(&data, "name = \"test\"\n").unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn validate_array_schema() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{
            "type": "array",
            "items": {
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"]
            }
        }"#,
    )
    .unwrap();
    fs::write(&data, r#"[{"name": "Alice"}, {"name": "Bob"}]"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn validate_array_schema_invalid_item() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{
            "type": "array",
            "items": {
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"]
            }
        }"#,
    )
    .unwrap();
    fs::write(&data, r#"[{"name": "Alice"}, {"age": 30}]"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn validate_enum_schema() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{
            "type": "object",
            "properties": {
                "status": {"type": "string", "enum": ["active", "inactive"]}
            }
        }"#,
    )
    .unwrap();
    fs::write(&data, r#"{"status": "unknown"}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn validate_invalid_schema_file() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("bad_schema.json");
    let data = dir.path().join("data.json");
    fs::write(&schema, "not valid json").unwrap();
    fs::write(&data, "{}").unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse schema"));
}

#[test]
fn validate_nonexistent_schema() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, "{}").unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            "/nonexistent/schema.json",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read schema"));
}

#[test]
fn validate_nonexistent_data() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    fs::write(&schema, r#"{"type": "object"}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            "nonexistent.json",
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn validate_stdin_json() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    fs::write(
        &schema,
        r#"{"type": "object", "properties": {"x": {"type": "integer"}}}"#,
    )
    .unwrap();

    dkit()
        .args(&[
            "validate",
            "-",
            "--schema",
            schema.to_str().unwrap(),
            "--from",
            "json",
        ])
        .write_stdin(r#"{"x": 42}"#)
        .assert()
        .success();
}

#[test]
fn validate_multiple_errors() {
    let dir = TempDir::new().unwrap();
    let schema = dir.path().join("schema.json");
    let data = dir.path().join("data.json");
    fs::write(
        &schema,
        r#"{
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "email": {"type": "string", "format": "email"}
            },
            "required": ["name", "age", "email"]
        }"#,
    )
    .unwrap();
    fs::write(&data, r#"{"name": 123}"#).unwrap();

    dkit()
        .args(&[
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("error(s)"));
}

// ============================================================
// stats 확장 테스트
// ============================================================

#[test]
fn stats_extended_numeric_percentiles() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "v"])
        .write_stdin(r#"[{"v":10},{"v":20},{"v":30},{"v":40},{"v":50},{"v":60},{"v":70},{"v":80},{"v":90},{"v":100}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("count: 10"))
        .stdout(predicate::str::contains("sum: 550"))
        .stdout(predicate::str::contains("avg: 55.00"))
        .stdout(predicate::str::contains("median: 55"))
        .stdout(predicate::str::contains("p25:"))
        .stdout(predicate::str::contains("p75:"))
        .stdout(predicate::str::contains("std:"));
}

#[test]
fn stats_string_top_values() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "city"])
        .write_stdin(r#"[{"city":"Seoul"},{"city":"Seoul"},{"city":"Seoul"},{"city":"Busan"},{"city":"Busan"},{"city":"Incheon"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("unique: 3"))
        .stdout(predicate::str::contains("Seoul (3)"))
        .stdout(predicate::str::contains("Busan (2)"));
}

#[test]
fn stats_histogram_distribution() {
    dkit()
        .args(&[
            "stats",
            "-",
            "--from",
            "json",
            "--column",
            "v",
            "--histogram",
        ])
        .write_stdin(
            r#"[{"v":1},{"v":2},{"v":3},{"v":4},{"v":5},{"v":6},{"v":7},{"v":8},{"v":9},{"v":10}]"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("histogram:"))
        .stdout(predicate::str::contains("█"));
}

#[test]
fn stats_format_json_with_column() {
    dkit()
        .args(&[
            "stats", "-", "--from", "json", "--column", "v", "--format", "json",
        ])
        .write_stdin(r#"[{"v":10},{"v":20},{"v":30}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"numeric\""))
        .stdout(predicate::str::contains("\"mean\""))
        .stdout(predicate::str::contains("\"median\""));
}

#[test]
fn stats_format_markdown() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--format", "md"])
        .write_stdin(r#"[{"x":1,"y":"a"},{"x":2,"y":"b"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("# Statistics"))
        .stdout(predicate::str::contains("|"));
}

#[test]
fn stats_all_null_column() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "x"])
        .write_stdin(r#"[{"x":null},{"x":null},{"x":null}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("missing: 3"));
}

// ============================================================
// sample 테스트
// ============================================================

#[test]
fn sample_random_count() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"[{"id":1},{"id":2},{"id":3},{"id":4},{"id":5},{"id":6},{"id":7},{"id":8},{"id":9},{"id":10}]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["sample", data.to_str().unwrap(), "-n", "3", "--seed", "42"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 3);
}

#[test]
fn sample_random_reproducible() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"[{"id":1},{"id":2},{"id":3},{"id":4},{"id":5},{"id":6},{"id":7},{"id":8},{"id":9},{"id":10}]"#,
    )
    .unwrap();

    let out1 = dkit()
        .args(&["sample", data.to_str().unwrap(), "-n", "5", "--seed", "123"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let out2 = dkit()
        .args(&["sample", data.to_str().unwrap(), "-n", "5", "--seed", "123"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    assert_eq!(out1, out2);
}

#[test]
fn sample_ratio() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"[{"id":1},{"id":2},{"id":3},{"id":4},{"id":5},{"id":6},{"id":7},{"id":8},{"id":9},{"id":10}]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "--ratio",
            "0.3",
            "--seed",
            "42",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let count = parsed.as_array().unwrap().len();
    assert!(count >= 1 && count <= 5); // ~30% of 10
}

#[test]
fn sample_systematic() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"[{"id":1},{"id":2},{"id":3},{"id":4},{"id":5},{"id":6},{"id":7},{"id":8},{"id":9},{"id":10}]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "-n",
            "3",
            "--method",
            "systematic",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 3);
}

#[test]
fn sample_stratified() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"[
            {"id":1,"cat":"A"},{"id":2,"cat":"A"},{"id":3,"cat":"A"},{"id":4,"cat":"A"},
            {"id":5,"cat":"B"},{"id":6,"cat":"B"},{"id":7,"cat":"B"},
            {"id":8,"cat":"C"},{"id":9,"cat":"C"},{"id":10,"cat":"C"}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "-n",
            "6",
            "--method",
            "stratified",
            "--stratify-by",
            "cat",
            "--seed",
            "42",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 6);
}

#[test]
fn sample_stdin() {
    dkit()
        .args(&["sample", "-", "--from", "json", "-n", "2", "--seed", "42"])
        .write_stdin(r#"[{"x":1},{"x":2},{"x":3},{"x":4},{"x":5}]"#)
        .assert()
        .success();
}

#[test]
fn sample_output_format() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"[{"name":"Alice","age":30},{"name":"Bob","age":25},{"name":"Charlie","age":35}]"#,
    )
    .unwrap();

    dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "-n",
            "2",
            "--seed",
            "42",
            "-f",
            "csv",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"));
}

#[test]
fn sample_to_file() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    let output = dir.path().join("sample.json");
    fs::write(&data, r#"[{"id":1},{"id":2},{"id":3},{"id":4},{"id":5}]"#).unwrap();

    dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "-n",
            "2",
            "--seed",
            "42",
            "-o",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(output.exists());
    let content = fs::read_to_string(&output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.as_array().unwrap().len(), 2);
}

// --- sample error cases ---

#[test]
fn sample_no_count_or_ratio() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"[{"id":1}]"#).unwrap();

    dkit()
        .args(&["sample", data.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Either -n/--count or --ratio is required",
        ));
}

#[test]
fn sample_both_count_and_ratio() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"[{"id":1}]"#).unwrap();

    dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "-n",
            "1",
            "--ratio",
            "0.5",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot specify both"));
}

#[test]
fn sample_invalid_ratio() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"[{"id":1}]"#).unwrap();

    dkit()
        .args(&["sample", data.to_str().unwrap(), "--ratio", "1.5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--ratio must be between"));
}

#[test]
fn sample_stratified_without_field() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"[{"id":1}]"#).unwrap();

    dkit()
        .args(&[
            "sample",
            data.to_str().unwrap(),
            "-n",
            "1",
            "--method",
            "stratified",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--stratify-by is required"));
}

#[test]
fn sample_non_array_input() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"id": 1}"#).unwrap();

    dkit()
        .args(&["sample", data.to_str().unwrap(), "-n", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be an array"));
}

#[test]
fn sample_csv_input() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.csv");
    fs::write(&data, "name,age\nAlice,30\nBob,25\nCharlie,35\n").unwrap();

    let output = dkit()
        .args(&["sample", data.to_str().unwrap(), "-n", "2", "--seed", "42"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    // Default output format matches input (CSV)
    assert!(stdout.contains("name"));
}

// ============================================================
// flatten/unflatten 테스트
// ============================================================

#[test]
fn flatten_basic_json() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a": {"b": {"c": 1}}, "d": 2}"#).unwrap();

    dkit()
        .args(&["flatten", data.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("a.b.c"))
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("\"d\""));
}

#[test]
fn flatten_custom_separator() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a": {"b": 1}}"#).unwrap();

    dkit()
        .args(&["flatten", data.to_str().unwrap(), "--separator", "/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("a/b"));
}

#[test]
fn flatten_bracket_format() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"items": [{"name": "a"}, {"name": "b"}]}"#).unwrap();

    dkit()
        .args(&[
            "flatten",
            data.to_str().unwrap(),
            "--array-format",
            "bracket",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("items[0]"))
        .stdout(predicate::str::contains("items[1]"));
}

#[test]
fn flatten_index_format() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"items": [{"name": "a"}]}"#).unwrap();

    dkit()
        .args(&["flatten", data.to_str().unwrap(), "--array-format", "index"])
        .assert()
        .success()
        .stdout(predicate::str::contains("items.0.name"));
}

#[test]
fn flatten_max_depth() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a": {"b": {"c": 1}}}"#).unwrap();

    // max-depth 1: only flatten 1 level
    dkit()
        .args(&["flatten", data.to_str().unwrap(), "--max-depth", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\""));
}

#[test]
fn flatten_array_of_objects() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"[{"a": {"b": 1}}, {"a": {"b": 2}}]"#).unwrap();

    dkit()
        .args(&["flatten", data.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("a.b"));
}

#[test]
fn flatten_stdin() {
    dkit()
        .args(&["flatten", "-", "--from", "json"])
        .write_stdin(r#"{"a": {"b": 1}}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("a.b"));
}

#[test]
fn flatten_output_format() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a": {"b": 1}}"#).unwrap();

    dkit()
        .args(&["flatten", data.to_str().unwrap(), "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("a.b:"));
}

#[test]
fn flatten_to_file() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    let output = dir.path().join("flat.json");
    fs::write(&data, r#"{"a": {"b": 1}}"#).unwrap();

    dkit()
        .args(&[
            "flatten",
            data.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(output.exists());
    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("a.b"));
}

// --- unflatten tests ---

#[test]
fn unflatten_basic_json() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a.b.c": 1, "d": 2}"#).unwrap();

    dkit()
        .args(&["unflatten", data.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\""))
        .stdout(predicate::str::contains("\"b\""))
        .stdout(predicate::str::contains("\"c\""));
}

#[test]
fn unflatten_custom_separator() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a/b": 1}"#).unwrap();

    dkit()
        .args(&["unflatten", data.to_str().unwrap(), "--separator", "/"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\""))
        .stdout(predicate::str::contains("\"b\""));
}

#[test]
fn unflatten_bracket_notation() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(
        &data,
        r#"{"items[0].name": "Alice", "items[1].name": "Bob"}"#,
    )
    .unwrap();

    dkit()
        .args(&["unflatten", data.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("items"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn unflatten_stdin() {
    dkit()
        .args(&["unflatten", "-", "--from", "json"])
        .write_stdin(r#"{"a.b": 1}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\""))
        .stdout(predicate::str::contains("\"b\""));
}

#[test]
fn unflatten_output_format() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    fs::write(&data, r#"{"a.b": 1}"#).unwrap();

    dkit()
        .args(&["unflatten", data.to_str().unwrap(), "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("a:"));
}

// --- flatten/unflatten roundtrip ---

#[test]
fn flatten_unflatten_roundtrip() {
    let dir = TempDir::new().unwrap();
    let original = dir.path().join("original.json");
    let flat_file = dir.path().join("flat.json");
    let restored = dir.path().join("restored.json");

    let data = r#"{"server": {"host": "localhost", "port": 8080}, "debug": true}"#;
    fs::write(&original, data).unwrap();

    // Flatten
    dkit()
        .args(&[
            "flatten",
            original.to_str().unwrap(),
            "-o",
            flat_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Unflatten
    dkit()
        .args(&[
            "unflatten",
            flat_file.to_str().unwrap(),
            "-o",
            restored.to_str().unwrap(),
        ])
        .assert()
        .success();

    let orig: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&original).unwrap()).unwrap();
    let rest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&restored).unwrap()).unwrap();
    assert_eq!(orig, rest);
}

#[test]
fn flatten_unflatten_roundtrip_with_arrays() {
    let dir = TempDir::new().unwrap();
    let original = dir.path().join("original.json");
    let flat_file = dir.path().join("flat.json");
    let restored = dir.path().join("restored.json");

    let data = r#"{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}"#;
    fs::write(&original, data).unwrap();

    dkit()
        .args(&[
            "flatten",
            original.to_str().unwrap(),
            "-o",
            flat_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    dkit()
        .args(&[
            "unflatten",
            flat_file.to_str().unwrap(),
            "-o",
            restored.to_str().unwrap(),
        ])
        .assert()
        .success();

    let orig: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&original).unwrap()).unwrap();
    let rest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&restored).unwrap()).unwrap();
    assert_eq!(orig, rest);
}

#[test]
fn flatten_yaml_input() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.yaml");
    fs::write(&data, "server:\n  host: localhost\n  port: 8080\n").unwrap();

    dkit()
        .args(&["flatten", data.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("server.host"))
        .stdout(predicate::str::contains("server.port"));
}
