#![cfg(feature = "sqlite")]

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

fn create_test_db() -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".db").unwrap();
    let conn = rusqlite::Connection::open(file.path()).unwrap();
    conn.execute_batch(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER, score REAL);
         INSERT INTO users VALUES (1, 'Alice', 30, 95.5);
         INSERT INTO users VALUES (2, 'Bob', 25, 88.0);
         INSERT INTO users VALUES (3, 'Charlie', 35, NULL);",
    )
    .unwrap();
    file
}

fn create_multi_table_db() -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".db").unwrap();
    let conn = rusqlite::Connection::open(file.path()).unwrap();
    conn.execute_batch(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
         INSERT INTO users VALUES (1, 'Alice');
         INSERT INTO users VALUES (2, 'Bob');
         CREATE TABLE products (id INTEGER PRIMARY KEY, title TEXT, price REAL);
         INSERT INTO products VALUES (1, 'Widget', 9.99);
         INSERT INTO products VALUES (2, 'Gadget', 24.95);",
    )
    .unwrap();
    file
}

#[test]
fn convert_sqlite_to_json() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["convert", db.path().to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("95.5"));
}

#[test]
fn convert_sqlite_to_csv() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["convert", db.path().to_str().unwrap(), "-f", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id,name,age,score"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn convert_sqlite_with_table_option() {
    let db = create_multi_table_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--table",
            "products",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Widget"))
        .stdout(predicate::str::contains("9.99"));
}

#[test]
fn convert_sqlite_with_sql_query() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT name, age FROM users WHERE age > 25 ORDER BY age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("30"));
}

#[test]
fn view_sqlite() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["view", db.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn view_sqlite_list_tables() {
    let db = create_multi_table_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["view", db.path().to_str().unwrap(), "--list-tables"])
        .assert()
        .success()
        .stdout(predicate::str::contains("products"))
        .stdout(predicate::str::contains("users"));
}

#[test]
fn view_sqlite_with_limit() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["view", db.path().to_str().unwrap(), "-n", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn query_sqlite() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["query", db.path().to_str().unwrap(), ".[0].name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn schema_sqlite() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["schema", db.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("array"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

#[test]
fn stats_sqlite() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["stats", db.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 3"));
}

#[test]
fn sqlite_format_auto_detected() {
    // .db extension should be auto-detected as SQLite
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["convert", db.path().to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn convert_json_to_sqlite_fails() {
    // SQLite is input-only
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["convert", "tests/fixtures/users.json", "-f", "sqlite"])
        .assert()
        .failure();
}

#[test]
fn sqlite_null_handling() {
    let db = create_test_db();
    Command::cargo_bin("dkit")
        .unwrap()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT score FROM users WHERE name = 'Charlie'",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("null"));
}
