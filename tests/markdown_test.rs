use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

#[test]
fn convert_json_to_md() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("| age |"))
        .stdout(predicate::str::contains("| Alice |"));
}

#[test]
fn convert_csv_to_md() {
    dkit()
        .args(&["convert", "tests/fixtures/users.csv", "--to", "md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("| name |"))
        .stdout(predicate::str::contains("| --- |"));
}

#[test]
fn convert_yaml_to_md() {
    dkit()
        .args(&["convert", "tests/fixtures/config.yaml", "--to", "md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("| key | value |"));
}

#[test]
fn convert_json_to_md_output_file() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("output.md");

    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "md",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out).unwrap();
    assert!(content.contains("| age |"));
    assert!(content.contains("| Alice |"));
}

#[test]
fn convert_md_numeric_right_alignment() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "md"])
        .assert()
        .success()
        // age column should be right-aligned (---:)
        .stdout(predicate::str::contains("---:"));
}

#[test]
fn convert_stdin_to_md() {
    dkit()
        .args(&["convert", "--from", "json", "--to", "md"])
        .write_stdin(r#"[{"x": 1}, {"x": 2}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("| x |"))
        .stdout(predicate::str::contains("| 1 |"))
        .stdout(predicate::str::contains("| 2 |"));
}

#[test]
fn convert_md_pipe_escape() {
    dkit()
        .args(&["convert", "--from", "json", "--to", "md"])
        .write_stdin(r#"[{"col": "a | b"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains(r"a \| b"));
}

#[test]
fn convert_md_nested_json_inline() {
    dkit()
        .args(&["convert", "--from", "json", "--to", "md"])
        .write_stdin(r#"[{"tags": ["a", "b"]}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["a","b"]"#));
}

#[test]
fn query_output_as_md() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/users.json",
            ".",
            "--to",
            "md",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("| name |"));
}
