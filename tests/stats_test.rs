use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- 기본 stats ---

#[test]
fn stats_json_array() {
    dkit()
        .args(&["stats", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"))
        .stdout(predicate::str::contains("columns: 3"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("email"));
}

#[test]
fn stats_csv() {
    dkit()
        .args(&["stats", "tests/fixtures/users.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"))
        .stdout(predicate::str::contains("columns: 3"));
}

// --- 숫자형 컬럼 통계 ---

#[test]
fn stats_numeric_column() {
    dkit()
        .args(&["stats", "tests/fixtures/users.json", "--column", "age"])
        .assert()
        .success()
        .stdout(predicate::str::contains("type: numeric"))
        .stdout(predicate::str::contains("count: 2"))
        .stdout(predicate::str::contains("sum: 55"))
        .stdout(predicate::str::contains("avg: 27.50"))
        .stdout(predicate::str::contains("min: 25"))
        .stdout(predicate::str::contains("max: 30"))
        .stdout(predicate::str::contains("median: 27"));
}

// --- 문자열형 컬럼 통계 ---

#[test]
fn stats_string_column() {
    dkit()
        .args(&["stats", "tests/fixtures/users.json", "--column", "name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("type: string"))
        .stdout(predicate::str::contains("count: 2"))
        .stdout(predicate::str::contains("unique: 2"));
}

// --- --path 옵션 ---

#[test]
fn stats_with_path() {
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/nested.json",
            "--path",
            ".company.departments",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"))
        .stdout(predicate::str::contains("columns: 3"));
}

// --- stdin ---

#[test]
fn stats_stdin() {
    dkit()
        .args(&["stats", "-", "--from", "json"])
        .write_stdin(r#"[{"x": 10}, {"x": 20}, {"x": 30}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 3"))
        .stdout(predicate::str::contains("columns: 1"));
}

#[test]
fn stats_stdin_column() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "x"])
        .write_stdin(r#"[{"x": 10}, {"x": 20}, {"x": 30}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("type: numeric"))
        .stdout(predicate::str::contains("sum: 60"))
        .stdout(predicate::str::contains("avg: 20.00"))
        .stdout(predicate::str::contains("median: 20"));
}

// --- 에러 케이스 ---

#[test]
fn stats_nonexistent_file() {
    dkit()
        .args(&["stats", "nonexistent.json"])
        .assert()
        .failure();
}

#[test]
fn stats_stdin_without_from() {
    // 콘텐츠 스니핑으로 JSON 포맷 자동 감지
    dkit()
        .args(&["stats", "-"])
        .write_stdin("[{\"a\": 1}]")
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 1"));
}

#[test]
fn stats_invalid_column() {
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/users.json",
            "--column",
            "nonexistent",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// --- object 입력 ---

#[test]
fn stats_single_object() {
    dkit()
        .args(&["stats", "-", "--from", "json"])
        .write_stdin(r#"{"name": "Alice", "age": 30}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 1"))
        .stdout(predicate::str::contains("columns: 2"));
}
