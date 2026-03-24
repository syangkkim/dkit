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

// --- 확장 통계: 숫자 필드 상세 ---

#[test]
fn stats_numeric_extended() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "v"])
        .write_stdin(r#"[{"v":10},{"v":20},{"v":30},{"v":40},{"v":50}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("type: numeric"))
        .stdout(predicate::str::contains("std:"))
        .stdout(predicate::str::contains("p25:"))
        .stdout(predicate::str::contains("p75:"))
        .stdout(predicate::str::contains("median: 30"));
}

// --- 확장 통계: 문자열 필드 상세 ---

#[test]
fn stats_string_extended() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "name"])
        .write_stdin(r#"[{"name":"Alice"},{"name":"Bob"},{"name":"Alice"},{"name":"Charlie"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("type: string"))
        .stdout(predicate::str::contains("unique: 3"))
        .stdout(predicate::str::contains("min_length:"))
        .stdout(predicate::str::contains("max_length:"))
        .stdout(predicate::str::contains("avg_length:"))
        .stdout(predicate::str::contains("top_values:"))
        .stdout(predicate::str::contains("Alice (2)"));
}

// --- null 비율 ---

#[test]
fn stats_missing_ratio() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "x"])
        .write_stdin(r#"[{"x":1},{"x":null},{"x":3}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("missing: 1 (33.3%)"));
}

// --- 타입 일관성 검사 ---

#[test]
fn stats_mixed_types() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "v"])
        .write_stdin(r#"[{"v":1},{"v":"hello"},{"v":3}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("mixed types"));
}

// --- --field 옵션 (--column 별칭) ---

#[test]
fn stats_field_alias() {
    dkit()
        .args(&["stats", "tests/fixtures/users.json", "--field", "age"])
        .assert()
        .success()
        .stdout(predicate::str::contains("type: numeric"))
        .stdout(predicate::str::contains("count: 2"));
}

// --- --format json 출력 ---

#[test]
fn stats_format_json() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--format", "json"])
        .write_stdin(r#"[{"x":10},{"x":20}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"rows\": 2"))
        .stdout(predicate::str::contains("\"columns\""));
}

#[test]
fn stats_column_format_json() {
    dkit()
        .args(&[
            "stats", "-", "--from", "json", "--column", "x", "--format", "json",
        ])
        .write_stdin(r#"[{"x":10},{"x":20},{"x":30}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"numeric\""))
        .stdout(predicate::str::contains("\"mean\""))
        .stdout(predicate::str::contains("\"std\""))
        .stdout(predicate::str::contains("\"p25\""))
        .stdout(predicate::str::contains("\"p75\""));
}

// --- --format md 출력 ---

#[test]
fn stats_format_md() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--format", "md"])
        .write_stdin(r#"[{"x":10},{"x":20}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("# Statistics"))
        .stdout(predicate::str::contains("| Stat | Value |"));
}

// --- --histogram ---

#[test]
fn stats_histogram() {
    dkit()
        .args(&[
            "stats", "-", "--from", "json", "--column", "v", "--histogram",
        ])
        .write_stdin(r#"[{"v":10},{"v":20},{"v":30},{"v":40},{"v":50}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("histogram:"))
        .stdout(predicate::str::contains("█"));
}

// --- 잘못된 --format ---

#[test]
fn stats_invalid_format() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--format", "invalid"])
        .write_stdin(r#"[{"x":1}]"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported stats output format"));
}
