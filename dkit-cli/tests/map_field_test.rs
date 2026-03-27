/// Integration tests for --map flag
use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// --map with convert
// ============================================================

#[test]
fn map_field_upper() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "name = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("BOB"))
        .stdout(predicate::str::contains("CHARLIE"));
}

#[test]
fn map_field_lower() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "city = lower(city)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("seoul"))
        .stdout(predicate::str::contains("busan"))
        .stdout(predicate::str::contains("incheon"));
}

#[test]
fn map_field_arithmetic_increment() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "age = age + 1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("31")) // Alice: 30 + 1
        .stdout(predicate::str::contains("26")) // Bob: 25 + 1
        .stdout(predicate::str::contains("36")); // Charlie: 35 + 1
}

#[test]
fn map_field_arithmetic_multiply() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "score = score * 2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("170")) // Alice: 85 * 2
        .stdout(predicate::str::contains("184")) // Bob: 92 * 2
        .stdout(predicate::str::contains("156")); // Charlie: 78 * 2
}

#[test]
fn map_field_multiple_flags() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "name = upper(name)",
            "--map",
            "city = lower(city)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("seoul"));
}

#[test]
fn map_field_with_function_round() {
    // score / 3 produces a float; round it
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "score = round(score / 3, 1)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("28.3")); // Alice: 85/3 ≈ 28.333
}

#[test]
fn map_field_string_concat() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "name = name + \" (\" + role + \")\"",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice (engineer)"))
        .stdout(predicate::str::contains("Bob (designer)"));
}

#[test]
fn map_field_trim() {
    // Create input with whitespace in names
    let input = r#"[{"name": "  Alice  "}, {"name": " Bob "}]"#;
    dkit()
        .args(&[
            "convert",
            "-",
            "-f",
            "json",
            "--from",
            "json",
            "--map",
            "name = trim(name)",
        ])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Alice\""))
        .stdout(predicate::str::contains("\"Bob\""));
}

#[test]
fn map_field_abs() {
    let input = r#"[{"val": -10}, {"val": 5}, {"val": -3}]"#;
    dkit()
        .args(&[
            "convert",
            "-",
            "-f",
            "json",
            "--from",
            "json",
            "--map",
            "val = abs(val)",
        ])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("10"))
        .stdout(predicate::str::contains("5"))
        .stdout(predicate::str::contains("3"));
}

#[test]
fn map_field_length() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "name = length(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("5")) // Alice = 5
        .stdout(predicate::str::contains("3")) // Bob = 3
        .stdout(predicate::str::contains("7")); // Charlie = 7
}

// ============================================================
// --map with view
// ============================================================

#[test]
fn map_field_view_upper() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--map",
            "name = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("BOB"));
}

#[test]
fn map_field_view_arithmetic() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--map",
            "age = age + 10",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("40")) // Alice: 30 + 10
        .stdout(predicate::str::contains("35")); // Bob: 25 + 10
}

// ============================================================
// --map combined with other flags
// ============================================================

#[test]
fn map_field_combined_with_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--filter",
            "age > 28",
            "--map",
            "name = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("CHARLIE"))
        .stdout(predicate::str::contains("EVE"))
        // Bob (25) and Diana (28) should be filtered out
        .stdout(predicate::str::contains("BOB").not());
}

#[test]
fn map_field_combined_with_add_field() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "name_len = length(name)",
            "--map",
            "name = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("name_len"));
}

#[test]
fn map_field_combined_with_select() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "name = upper(name)",
            "--select",
            "name, age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("30"))
        // city should not be in output due to --select
        .stdout(predicate::str::contains("Seoul").not());
}

// ============================================================
// --map error handling
// ============================================================

#[test]
fn map_field_invalid_expression() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "invalid expression without equals",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid --map expression"));
}

// ============================================================
// --map on CSV format
// ============================================================

#[test]
fn map_field_csv_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--map",
            "name = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("BOB"));
}
