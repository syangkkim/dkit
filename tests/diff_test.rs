use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- 동일 파일 비교 ---

#[test]
fn diff_identical_files() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config.yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("No differences found."));
}

// --- 같은 포맷 비교 (YAML) ---

#[test]
fn diff_yaml_files_shows_changes() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
        ])
        .assert()
        .failure() // exit code 1 when files differ
        .stdout(predicate::str::contains("server.port"))
        .stdout(predicate::str::contains("server.debug"));
}

#[test]
fn diff_yaml_shows_added_fields() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("logging.level"))
        .stdout(predicate::str::contains("(added)"));
}

#[test]
fn diff_yaml_shows_removed_fields() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("database"))
        .stdout(predicate::str::contains("(removed)"));
}

// --- 다른 포맷 간 비교 ---

#[test]
fn diff_cross_format_yaml_vs_toml() {
    // config.yaml and config.toml have the same data
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config.toml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("No differences found."));
}

// --- --path 옵션 ---

#[test]
fn diff_with_path_option() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
            "--path",
            ".server",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::contains("port"))
        .stdout(predicate::str::contains("debug"));
}

// --- --quiet 옵션 ---

#[test]
fn diff_quiet_same_files() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config.yaml",
            "--quiet",
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn diff_quiet_different_files() {
    dkit()
        .args(&[
            "diff",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
            "--quiet",
        ])
        .assert()
        .failure()
        .stdout(predicate::str::is_empty());
}

// --- JSON 비교 ---

#[test]
fn diff_json_files() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");

    fs::write(&f1, r#"{"name": "Alice", "age": 30}"#).unwrap();
    fs::write(&f2, r#"{"name": "Alice", "age": 31}"#).unwrap();

    dkit()
        .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("(unchanged)"))
        .stdout(predicate::str::contains("age"));
}

// --- 배열 비교 ---

#[test]
fn diff_json_arrays() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");

    fs::write(&f1, r#"[1, 2, 3]"#).unwrap();
    fs::write(&f2, r#"[1, 2, 4, 5]"#).unwrap();

    dkit()
        .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("[2]")) // changed element
        .stdout(predicate::str::contains("[3]")); // added element
}

// --- 에러 처리 ---

#[test]
fn diff_nonexistent_file() {
    dkit()
        .args(&["diff", "nonexistent.json", "tests/fixtures/config.yaml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn diff_unknown_format() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.bin");
    fs::write(&f, "data").unwrap();

    dkit()
        .args(&["diff", f.to_str().unwrap(), f.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}
