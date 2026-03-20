use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- 배열 합치기 (concat) ---

#[test]
fn merge_json_arrays() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "--to",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Diana"));
}

#[test]
fn merge_csv_files() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.csv",
            "tests/fixtures/users2.csv",
            "--to",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"));
}

// --- 오브젝트 합치기 (merge) ---

#[test]
fn merge_yaml_objects() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
            "--to",
            "yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("database"))
        .stdout(predicate::str::contains("logging"))
        // server.port는 config2의 9090으로 덮어쓰기
        .stdout(predicate::str::contains("9090"));
}

// --- 다른 포맷 간 합치기 ---

#[test]
fn merge_different_formats() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.csv",
            "--to",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"));
}

// --- 출력 파일 지정 ---

#[test]
fn merge_to_output_file() {
    let tmp = TempDir::new().unwrap();
    let out_path = tmp.path().join("merged.json");

    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "--to",
            "json",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    assert!(content.contains("Alice"));
    assert!(content.contains("Diana"));
}

// --- 에러 케이스 ---

#[test]
fn merge_single_file_error() {
    dkit()
        .args(&["merge", "tests/fixtures/users.json", "--to", "json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("at least 2"));
}

#[test]
fn merge_nonexistent_file_error() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/nonexistent.json",
            "--to",
            "json",
        ])
        .assert()
        .failure();
}

// --- 포맷 자동 감지 ---

#[test]
fn merge_auto_detect_output_format() {
    // --to 없이 첫 번째 입력 파일의 포맷을 사용
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"));
}

// --- 출력 포맷 변환 ---

#[test]
fn merge_json_to_csv() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "--to",
            "csv",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Diana"));
}
