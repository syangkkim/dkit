use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- `-` as explicit stdin marker for convert ---

#[test]
fn convert_dash_stdin_json_to_csv() {
    dkit()
        .args(&["convert", "-", "--from", "json", "--to", "csv"])
        .write_stdin(r#"[{"name":"Alice","age":30}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn convert_dash_stdin_csv_to_yaml() {
    dkit()
        .args(&["convert", "-", "--from", "csv", "--to", "yaml"])
        .write_stdin("name,age\nAlice,30\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"));
}

#[test]
fn convert_dash_stdin_auto_detect_format() {
    // `-` with no --from should auto-detect JSON via content sniffing
    dkit()
        .args(&["convert", "-", "--to", "yaml"])
        .write_stdin(r#"{"name": "test"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("name: test"));
}

// --- Pipe output is compact (no pretty-printing) ---

#[test]
fn convert_pipe_output_is_compact_json() {
    // When stdout is not a terminal (which is the case in tests), output should be compact
    let output = dkit()
        .args(&["convert", "--from", "json", "--to", "json"])
        .write_stdin(r#"[{"name":"Alice","age":30}]"#)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Compact JSON should not have indentation
    assert!(
        !stdout.contains("  "),
        "Piped output should be compact, but got:\n{stdout}"
    );
    assert!(stdout.contains("\"name\":\"Alice\"") || stdout.contains("\"name\": \"Alice\""));
}

#[test]
fn convert_explicit_pretty_overrides_pipe_detection() {
    // --pretty should force pretty-print even when piped
    let output = dkit()
        .args(&["convert", "--from", "json", "--to", "json", "--pretty"])
        .write_stdin(r#"[{"name":"Alice","age":30}]"#)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("  "),
        "--pretty should force indented output, but got:\n{stdout}"
    );
}

#[test]
fn convert_explicit_compact_flag() {
    let output = dkit()
        .args(&["convert", "--from", "json", "--to", "json", "--compact"])
        .write_stdin(r#"{"name": "Alice", "age": 30}"#)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout.contains("  "),
        "--compact should produce compact output, but got:\n{stdout}"
    );
}

// --- Pipeline chaining: convert | query ---

#[test]
fn pipeline_convert_to_query() {
    // Simulate: dkit convert data.json -f csv | dkit query - '.name' --from csv
    // Step 1: convert JSON to CSV
    let convert_output = dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "csv"])
        .output()
        .unwrap();
    assert!(convert_output.status.success());
    let csv_data = String::from_utf8(convert_output.stdout).unwrap();

    // Step 2: query the CSV output (select first element's name)
    dkit()
        .args(&["query", "-", ".[0].name", "--from", "csv"])
        .write_stdin(csv_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- Convert to file should be pretty (default) ---

#[test]
fn convert_to_file_is_pretty_by_default() {
    let dir = tempfile::tempdir().unwrap();
    let out_path = dir.path().join("output.json");

    dkit()
        .args(&[
            "convert",
            "--from",
            "json",
            "--to",
            "json",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .write_stdin(r#"[{"name":"Alice","age":30}]"#)
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    assert!(
        content.contains("  "),
        "File output should be pretty-printed by default, but got:\n{content}"
    );
}

// --- stdin with various format conversions ---

#[test]
fn convert_dash_stdin_json_to_toml() {
    dkit()
        .args(&["convert", "-", "--from", "json", "--to", "toml"])
        .write_stdin(r#"{"database": {"host": "localhost", "port": 5432}}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("[database]"))
        .stdout(predicate::str::contains("host"));
}

#[test]
fn convert_dash_stdin_yaml_to_json() {
    dkit()
        .args(&["convert", "-", "--from", "yaml", "--to", "json"])
        .write_stdin("name: Alice\nage: 30\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- Query piped output is compact ---

#[test]
fn query_pipe_output_is_compact_json() {
    let output = dkit()
        .args(&["query", "-", ".", "--from", "json"])
        .write_stdin(r#"{"name": "Alice", "age": 30}"#)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // When piped (not a terminal), query output should be compact
    assert!(
        !stdout.contains("  "),
        "Piped query output should be compact, but got:\n{stdout}"
    );
}
