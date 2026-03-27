use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- --time flag tests ---

#[test]
fn time_flag_query() {
    dkit()
        .args(["query", "tests/fixtures/users.json", ".[0].name", "--time"])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_convert() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    fs::write(&input, r#"[{"a":1},{"a":2}]"#).unwrap();

    dkit()
        .args(["convert", input.to_str().unwrap(), "-f", "csv", "--time"])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_view() {
    dkit()
        .args(["view", "tests/fixtures/users.json", "--time"])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_stats() {
    dkit()
        .args(["stats", "tests/fixtures/users.json", "--time"])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_not_present_by_default() {
    dkit()
        .args(["query", "tests/fixtures/users.json", ".[0].name"])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:").not());
}

// --- --explain flag tests ---

#[test]
fn explain_simple_path() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[0].name",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Execution Plan:"))
        .stderr(predicate::str::contains("Scan:"))
        .stderr(predicate::str::contains("Navigate:"))
        // Should NOT produce data output
        .stdout(predicate::str::is_empty());
}

#[test]
fn explain_with_filter() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | where name == \"Alice\"",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Execution Plan:"))
        .stderr(predicate::str::contains("Filter: name == \"Alice\""));
}

#[test]
fn explain_with_select() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | select name, age",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Project: name, age"));
}

#[test]
fn explain_with_sort_and_limit() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | sort age desc | limit 5",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Sort: age DESC"))
        .stderr(predicate::str::contains("Limit: 5"));
}

#[test]
fn explain_with_group_by() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | group_by name",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Group By: name"));
}

#[test]
fn explain_complex_pipeline() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | where age > 20 | select name, age | sort age desc | limit 10",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("1. Scan:"))
        .stderr(predicate::str::contains("2. Navigate:"))
        .stderr(predicate::str::contains("3. Filter:"))
        .stderr(predicate::str::contains("4. Project:"))
        .stderr(predicate::str::contains("5. Sort:"))
        .stderr(predicate::str::contains("6. Limit:"));
}

#[test]
fn explain_does_not_execute_query() {
    // --explain should work even if the query would fail at execution time
    // (e.g., file doesn't need to exist for parsing)
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | where nonexistent_field == \"value\" | sort another_field",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Execution Plan:"))
        .stdout(predicate::str::is_empty());
}

// --- --time and --explain combined ---

#[test]
fn time_and_explain_together() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | where name == \"Alice\"",
            "--explain",
            "--time",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Execution Plan:"))
        .stderr(predicate::str::contains("timing:"));
}
