use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- JSON object schema ---

#[test]
fn schema_json_nested_object() {
    dkit()
        .args(&["schema", "tests/fixtures/config.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: object"))
        .stdout(predicate::str::contains("database: object"))
        .stdout(predicate::str::contains("host: string"))
        .stdout(predicate::str::contains("port: integer"))
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("server: object"))
        .stdout(predicate::str::contains("debug: boolean"));
}

#[test]
fn schema_json_array_of_objects() {
    dkit()
        .args(&["schema", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: array[object]"))
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("age: integer"))
        .stdout(predicate::str::contains("email: string"));
}

#[test]
fn schema_deeply_nested() {
    dkit()
        .args(&["schema", "tests/fixtures/nested.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: object"))
        .stdout(predicate::str::contains("company: object"))
        .stdout(predicate::str::contains("location: object"))
        .stdout(predicate::str::contains("country: string"))
        .stdout(predicate::str::contains("address: object"))
        .stdout(predicate::str::contains("street: string"))
        .stdout(predicate::str::contains("departments: array[object]"))
        .stdout(predicate::str::contains("teams: array[object]"));
}

// --- TOML schema ---

#[test]
fn schema_toml() {
    dkit()
        .args(&["schema", "tests/fixtures/config.toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: object"))
        .stdout(predicate::str::contains("database: object"))
        .stdout(predicate::str::contains("host: string"))
        .stdout(predicate::str::contains("port: integer"));
}

// --- CSV schema ---

#[test]
fn schema_csv() {
    dkit()
        .args(&["schema", "tests/fixtures/users.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: array[object]"))
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("email: string"));
}

// --- stdin ---

#[test]
fn schema_stdin_with_from() {
    dkit()
        .args(&["schema", "-", "--from", "json"])
        .write_stdin(r#"{"a": 1, "b": "hello"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("root: object"))
        .stdout(predicate::str::contains("a: integer"))
        .stdout(predicate::str::contains("b: string"));
}

#[test]
fn schema_stdin_without_from_fails() {
    dkit()
        .args(&["schema", "-"])
        .write_stdin(r#"{"a": 1}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("--from is required"));
}

// --- Tree structure verification ---

#[test]
fn schema_tree_connectors() {
    // Verify the exact tree structure with ├─ and └─ connectors
    dkit()
        .args(&["schema", "-", "--from", "json"])
        .write_stdin(r#"{"x": 1, "y": 2, "z": 3}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("├─ x: integer"))
        .stdout(predicate::str::contains("├─ y: integer"))
        .stdout(predicate::str::contains("└─ z: integer"));
}

// --- File not found ---

#[test]
fn schema_file_not_found() {
    dkit()
        .args(&["schema", "nonexistent.json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read"));
}
