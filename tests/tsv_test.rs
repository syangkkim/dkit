use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- TSV 읽기: .tsv 파일은 자동으로 탭 구분자 사용 ---

#[test]
fn convert_tsv_to_json() {
    dkit()
        .args(&["convert", "tests/fixtures/users.tsv", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"Alice\""))
        .stdout(predicate::str::contains("\"age\": 30"));
}

#[test]
fn convert_tsv_to_yaml() {
    dkit()
        .args(&["convert", "tests/fixtures/users.tsv", "--to", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("age: '30'").or(predicate::str::contains("age: 30")));
}

// --- TSV 쓰기: --to tsv는 자동으로 탭 구분자 사용 ---

#[test]
fn convert_json_to_tsv() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--to", "tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name\tage\temail").or(predicate::str::contains("\t")));
}

#[test]
fn convert_csv_to_tsv() {
    dkit()
        .args(&["convert", "tests/fixtures/users.csv", "--to", "tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\t"));
}

// --- TSV 출력 파일 ---

#[test]
fn convert_json_to_tsv_file() {
    let tmp = TempDir::new().unwrap();
    let out_path = tmp.path().join("output.tsv");

    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "tsv",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    assert!(content.contains('\t'));
    assert!(content.contains("Alice"));
}

// --- TSV view ---

#[test]
fn view_tsv_file() {
    dkit()
        .args(&["view", "tests/fixtures/users.tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

// --- TSV stats ---

#[test]
fn stats_tsv_file() {
    dkit()
        .args(&["stats", "tests/fixtures/users.tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"));
}

// --- TSV schema ---

#[test]
fn schema_tsv_file() {
    dkit()
        .args(&["schema", "tests/fixtures/users.tsv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

// --- stdin with --from tsv ---

#[test]
fn convert_stdin_tsv_to_json() {
    dkit()
        .args(&["convert", "--from", "tsv", "--to", "json"])
        .write_stdin("name\tage\nAlice\t30\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- TSV 라운드트립: JSON → TSV → JSON ---

#[test]
fn tsv_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let tsv_path = tmp.path().join("roundtrip.tsv");

    // JSON → TSV
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--to",
            "tsv",
            "-o",
            tsv_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // TSV → JSON (자동 감지)
    dkit()
        .args(&["convert", tsv_path.to_str().unwrap(), "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}
