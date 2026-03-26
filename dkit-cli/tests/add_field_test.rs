/// Integration tests for --add-field flag
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// --add-field with convert
// ============================================================

#[test]
fn add_field_arithmetic_multiply() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "double_age = age * 2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("double_age"))
        .stdout(predicate::str::contains("60")) // Alice: 30 * 2
        .stdout(predicate::str::contains("50")); // Bob: 25 * 2
}

#[test]
fn add_field_arithmetic_addition() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "age_plus_score = age + score",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("age_plus_score"))
        .stdout(predicate::str::contains("115")); // Alice: 30 + 85
}

#[test]
fn add_field_string_concat() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "greeting = name + \" from \" + city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice from Seoul"))
        .stdout(predicate::str::contains("Bob from Busan"));
}

#[test]
fn add_field_with_literal() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "bonus = score * 10",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("bonus"))
        .stdout(predicate::str::contains("850")); // Alice: 85 * 10
}

#[test]
fn add_field_multiple_flags() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "double_age = age * 2",
            "--add-field",
            "name_upper = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("double_age"))
        .stdout(predicate::str::contains("name_upper"))
        .stdout(predicate::str::contains("ALICE"));
}

#[test]
fn add_field_with_function() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "name_lower = lower(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice"))
        .stdout(predicate::str::contains("bob"));
}

// ============================================================
// --add-field with view
// ============================================================

#[test]
fn add_field_view_table() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--add-field",
            "double_score = score * 2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("double_score"))
        .stdout(predicate::str::contains("170")); // Alice: 85 * 2
}

// ============================================================
// --add-field combined with other flags
// ============================================================

#[test]
fn add_field_with_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--filter",
            "age > 30",
            "--add-field",
            "senior = age - 30",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("senior"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Alice").not()); // age 30 not > 30
}

#[test]
fn add_field_with_select() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "double_age = age * 2",
            "--select",
            "name, double_age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("double_age"))
        .stdout(predicate::str::contains("city").not());
}

#[test]
fn add_field_csv_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--add-field",
            "bonus = score * 100",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("bonus"))
        .stdout(predicate::str::contains("8500")); // Alice: 85 * 100
}

#[test]
fn add_field_operator_precedence() {
    // price + price * 0.1 should be price + (price * 0.1), not (price + price) * 0.1
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("data.json");
    std::fs::write(&input, r#"[{"price": 100}]"#).unwrap();

    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--add-field",
            "total = price + price * 0.1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("110")); // 100 + 100*0.1 = 110
}

#[test]
fn add_field_invalid_expression_error() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "bad expression without equals",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--add-field"));
}
