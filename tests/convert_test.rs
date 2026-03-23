use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- JSON → other formats ---

#[test]
fn convert_json_to_csv() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn convert_json_to_yaml() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("age: 30"));
}

#[test]
fn convert_json_to_toml() {
    dkit()
        .args(&["convert", "tests/fixtures/config.yaml", "--to", "toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[database]"))
        .stdout(predicate::str::contains("host = \"localhost\""));
}

// --- CSV → other formats ---

#[test]
fn convert_csv_to_json() {
    dkit()
        .args(&["convert", "tests/fixtures/users.csv", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"Alice\""))
        .stdout(predicate::str::contains("\"age\": 30"));
}

#[test]
fn convert_csv_to_yaml() {
    dkit()
        .args(&["convert", "tests/fixtures/users.csv", "--to", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("age: 30"));
}

// --- YAML → other formats ---

#[test]
fn convert_yaml_to_json() {
    dkit()
        .args(&["convert", "tests/fixtures/config.yaml", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"host\": \"localhost\""))
        .stdout(predicate::str::contains("\"port\": 5432"));
}

#[test]
fn convert_yaml_to_toml() {
    dkit()
        .args(&["convert", "tests/fixtures/config.yaml", "--to", "toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[database]"));
}

// --- TOML → other formats ---

#[test]
fn convert_toml_to_json() {
    dkit()
        .args(&["convert", "tests/fixtures/config.toml", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"host\": \"localhost\""));
}

#[test]
fn convert_toml_to_yaml() {
    dkit()
        .args(&["convert", "tests/fixtures/config.toml", "--to", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("host: localhost"));
}

// --- Output file (-o) ---

#[test]
fn convert_with_output_file() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("output.yaml");

    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "yaml",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out).unwrap();
    assert!(content.contains("name: Alice"));
}

// --- Multiple files with --outdir ---

#[test]
fn convert_multiple_files_with_outdir() {
    let dir = TempDir::new().unwrap();
    let outdir = dir.path().join("converted");

    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/config.yaml",
            "--to",
            "toml",
            "--outdir",
            outdir.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(outdir.join("users.toml").exists());
    assert!(outdir.join("config.toml").exists());
}

// --- stdin/stdout pipe ---

#[test]
fn convert_stdin_json_to_csv() {
    dkit()
        .args(&["convert", "--from", "json", "--to", "csv"])
        .write_stdin(r#"[{"name":"Alice","age":30}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn convert_stdin_csv_to_json() {
    dkit()
        .args(&["convert", "--from", "csv", "--to", "json"])
        .write_stdin("name,age\nAlice,30\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"Alice\""));
}

// --- Options ---

#[test]
fn convert_json_compact() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "json",
            "--compact",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\":\"Alice\""));
}

#[test]
fn convert_json_pretty() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "json",
            "--pretty",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("  "));
}

#[test]
fn convert_csv_no_header() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "csv",
            "--no-header",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("name").not());
}

#[test]
fn convert_csv_with_delimiter() {
    // Convert JSON to CSV with tab delimiter
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "csv",
            "--delimiter",
            "\t",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\t"));
}

// --- Error cases ---

#[test]
fn convert_missing_to_flag() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json"])
        .assert()
        .failure();
}

#[test]
fn convert_unknown_format() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "bin"])
        .assert()
        .failure();
}

#[test]
fn convert_json_to_xml() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "xml"])
        .assert()
        .success()
        .stdout(predicates::str::contains("<name>Alice</name>"));
}

#[test]
fn convert_xml_to_json() {
    dkit()
        .args(&["convert", "tests/fixtures/users.xml", "--to", "json"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Alice"));
}

#[test]
fn convert_nonexistent_file() {
    dkit()
        .args(&["convert", "nonexistent.json", "--to", "csv"])
        .assert()
        .failure();
}

#[test]
fn convert_stdin_without_from() {
    // 콘텐츠 스니핑으로 JSON 포맷 자동 감지
    dkit()
        .args(&["convert", "--to", "yaml"])
        .write_stdin("{\"name\": \"test\"}")
        .assert()
        .success()
        .stdout(predicate::str::contains("name: test"));
}

#[test]
fn convert_multiple_files_without_outdir() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/config.yaml",
            "--to",
            "csv",
        ])
        .assert()
        .failure();
}
