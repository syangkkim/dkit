use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- xlsx read via convert ---

#[test]
fn convert_xlsx_to_json() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Tokyo"));
}

#[test]
fn convert_xlsx_to_csv() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age,city"))
        .stdout(predicate::str::contains("Alice"));
}

// --- xlsx read via view ---

#[test]
fn view_xlsx() {
    dkit()
        .args(["view", "tests/fixtures/users.xlsx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn view_xlsx_with_limit() {
    dkit()
        .args(["view", "tests/fixtures/users.xlsx", "--limit", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob").not());
}

// --- --list-sheets ---

#[test]
fn view_xlsx_list_sheets() {
    dkit()
        .args(["view", "tests/fixtures/users.xlsx", "--list-sheets"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0: Users"))
        .stdout(predicate::str::contains("1: Products"));
}

// --- --sheet option ---

#[test]
fn convert_xlsx_sheet_by_name() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.xlsx",
            "-f",
            "json",
            "--sheet",
            "Products",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("product"));
}

#[test]
fn convert_xlsx_sheet_by_index() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.xlsx",
            "-f",
            "json",
            "--sheet",
            "1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("product"));
}

#[test]
fn convert_xlsx_sheet_not_found() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.xlsx",
            "-f",
            "json",
            "--sheet",
            "NonExistent",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

// --- xlsx format detection ---

#[test]
fn xlsx_format_auto_detected() {
    // Should detect xlsx format from extension
    dkit()
        .args(["view", "tests/fixtures/users.xlsx"])
        .assert()
        .success();
}

// --- xlsx as output format should fail ---

#[test]
fn convert_json_to_xlsx_fails() {
    dkit()
        .args(["convert", "tests/fixtures/users.json", "-f", "xlsx"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("input-only"));
}

// --- query xlsx ---

#[test]
fn query_xlsx() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.xlsx",
            ".[0].name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- schema xlsx ---

#[test]
fn schema_xlsx() {
    dkit()
        .args(["schema", "tests/fixtures/users.xlsx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

// --- stats xlsx ---

#[test]
fn stats_xlsx() {
    dkit()
        .args(["stats", "tests/fixtures/users.xlsx"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"));
}
