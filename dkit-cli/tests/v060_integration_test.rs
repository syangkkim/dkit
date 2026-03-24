//! v0.6.0 통합 테스트: Excel 읽기, SQLite 읽기, 파이프라인 체이닝, 일괄 변환

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::{NamedTempFile, TempDir};

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// Excel 읽기 테스트 (다양한 셀 타입, 다중 시트)
// ============================================================

#[test]
fn xlsx_convert_to_yaml() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("name: Bob"));
}

#[test]
fn xlsx_convert_to_toml() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_convert_to_jsonl() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "jsonl"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn xlsx_convert_to_xml() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "xml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_convert_to_md() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("|"));
}

#[test]
fn xlsx_convert_to_html() {
    dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "html"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<table>"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_products_sheet_by_name() {
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
fn xlsx_products_sheet_to_csv() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.xlsx",
            "-f",
            "csv",
            "--sheet",
            "Products",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("product"));
}

#[test]
fn xlsx_view_with_columns() {
    dkit()
        .args(["view", "tests/fixtures/users.xlsx", "--columns", "name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_view_with_row_numbers() {
    dkit()
        .args(["view", "tests/fixtures/users.xlsx", "--row-numbers"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_query_with_filter() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.xlsx",
            ".[] | where name == \"Alice\"",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob").not());
}

#[test]
fn xlsx_query_select_fields() {
    dkit()
        .args(["query", "tests/fixtures/users.xlsx", ".[] | select name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_query_with_output_format() {
    dkit()
        .args(["query", "tests/fixtures/users.xlsx", ".", "--to", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_diff_with_json() {
    // Convert xlsx to json first, then diff should show no differences
    let output = dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "-f", "json"])
        .output()
        .unwrap();
    let json_data = String::from_utf8(output.stdout).unwrap();

    let tmp = NamedTempFile::with_suffix(".json").unwrap();
    fs::write(tmp.path(), &json_data).unwrap();

    dkit()
        .args([
            "diff",
            "tests/fixtures/users.xlsx",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn xlsx_merge_with_json() {
    dkit()
        .args([
            "merge",
            "tests/fixtures/users.xlsx",
            "tests/fixtures/users.json",
            "--to",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn xlsx_list_sheets_via_view() {
    dkit()
        .args(["view", "tests/fixtures/users.xlsx", "--list-sheets"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Users"))
        .stdout(predicate::str::contains("Products"));
}

#[test]
fn xlsx_invalid_sheet_index() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.xlsx",
            "-f",
            "json",
            "--sheet",
            "99",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("out of range"));
}

// ============================================================
// SQLite 읽기 테스트 (다양한 타입, 커스텀 쿼리)
// ============================================================

fn create_rich_type_db() -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".db").unwrap();
    let conn = rusqlite::Connection::open(file.path()).unwrap();
    conn.execute_batch(
        "CREATE TABLE data (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            score REAL,
            active INTEGER,
            bio TEXT,
            raw BLOB
        );
        INSERT INTO data VALUES (1, 'Alice', 95.5, 1, 'Engineer', x'CAFE');
        INSERT INTO data VALUES (2, 'Bob', NULL, 0, NULL, NULL);
        INSERT INTO data VALUES (3, 'Charlie', 72.3, 1, 'Designer', x'BEEF');
        INSERT INTO data VALUES (4, '한글이름', 88.0, 1, '한국어 설명', NULL);",
    )
    .unwrap();
    file
}

fn create_multi_table_db() -> NamedTempFile {
    let file = NamedTempFile::with_suffix(".db").unwrap();
    let conn = rusqlite::Connection::open(file.path()).unwrap();
    conn.execute_batch(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);
         INSERT INTO users VALUES (1, 'Alice', 30);
         INSERT INTO users VALUES (2, 'Bob', 25);
         INSERT INTO users VALUES (3, 'Charlie', 35);
         CREATE TABLE orders (id INTEGER PRIMARY KEY, user_id INTEGER, product TEXT, amount REAL);
         INSERT INTO orders VALUES (1, 1, 'Widget', 29.99);
         INSERT INTO orders VALUES (2, 1, 'Gadget', 49.99);
         INSERT INTO orders VALUES (3, 2, 'Widget', 29.99);
         CREATE TABLE categories (id INTEGER PRIMARY KEY, name TEXT);
         INSERT INTO categories VALUES (1, 'Electronics');
         INSERT INTO categories VALUES (2, 'Clothing');",
    )
    .unwrap();
    file
}

#[test]
fn sqlite_rich_types_to_json() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("95.5"))
        .stdout(predicate::str::contains("null"))
        .stdout(predicate::str::contains("cafe"));
}

#[test]
fn sqlite_unicode_data() {
    let db = create_rich_type_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT name, bio FROM data WHERE name = '한글이름'",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("한글이름"))
        .stdout(predicate::str::contains("한국어 설명"));
}

#[test]
fn sqlite_convert_to_yaml() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("score: 95.5"));
}

#[test]
fn sqlite_convert_to_csv() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id,name,score,active,bio,raw"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn sqlite_convert_to_toml() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn sqlite_convert_to_xml() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "xml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn sqlite_convert_to_md() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("|"));
}

#[test]
fn sqlite_convert_to_html() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "html"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<table>"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn sqlite_convert_to_jsonl() {
    let db = create_rich_type_db();
    dkit()
        .args(["convert", db.path().to_str().unwrap(), "-f", "jsonl"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn sqlite_sql_with_where() {
    let db = create_rich_type_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT name, score FROM data WHERE score > 80",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("한글이름"))
        .stdout(predicate::str::contains("Charlie").not());
}

#[test]
fn sqlite_sql_with_order_and_limit() {
    let db = create_rich_type_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT name FROM data ORDER BY name LIMIT 2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie").not());
}

#[test]
fn sqlite_sql_with_aggregate() {
    let db = create_rich_type_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT COUNT(*) as cnt, AVG(score) as avg_score FROM data",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("cnt"))
        .stdout(predicate::str::contains("avg_score"));
}

#[test]
fn sqlite_sql_with_join() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT u.name, o.product, o.amount FROM users u JOIN orders o ON u.id = o.user_id ORDER BY u.name, o.product",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Widget"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn sqlite_sql_with_group_by() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--sql",
            "SELECT u.name, COUNT(o.id) as order_count, SUM(o.amount) as total FROM users u LEFT JOIN orders o ON u.id = o.user_id GROUP BY u.name ORDER BY u.name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("order_count"))
        .stdout(predicate::str::contains("total"));
}

#[test]
fn sqlite_list_tables_multi() {
    let db = create_multi_table_db();
    dkit()
        .args(["view", db.path().to_str().unwrap(), "--list-tables"])
        .assert()
        .success()
        .stdout(predicate::str::contains("users"))
        .stdout(predicate::str::contains("orders"))
        .stdout(predicate::str::contains("categories"));
}

#[test]
fn sqlite_select_specific_table() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "convert",
            db.path().to_str().unwrap(),
            "-f",
            "json",
            "--table",
            "categories",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Electronics"))
        .stdout(predicate::str::contains("Clothing"));
}

#[test]
fn sqlite_view_with_limit() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "view",
            db.path().to_str().unwrap(),
            "--table",
            "users",
            "-n",
            "1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn sqlite_query_with_filter() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "query",
            db.path().to_str().unwrap(),
            ".[] | where age > 28",
            "--table",
            "users",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Bob").not());
}

#[test]
fn sqlite_query_with_select_and_sort() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "query",
            db.path().to_str().unwrap(),
            ".[] | select name, age | sort age desc",
            "--table",
            "users",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn sqlite_schema() {
    let db = create_multi_table_db();
    dkit()
        .args(["schema", db.path().to_str().unwrap(), "--table", "users"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

#[test]
fn sqlite_stats_table() {
    let db = create_multi_table_db();
    dkit()
        .args(["stats", db.path().to_str().unwrap(), "--table", "users"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 3"));
}

#[test]
fn sqlite_diff_two_tables() {
    let db1 = create_multi_table_db();
    let db2 = create_multi_table_db();
    dkit()
        .args([
            "diff",
            db1.path().to_str().unwrap(),
            db2.path().to_str().unwrap(),
            "--table",
            "users",
        ])
        .assert()
        .success();
}

#[test]
fn sqlite_merge_with_json() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "merge",
            db.path().to_str().unwrap(),
            "tests/fixtures/users.json",
            "--to",
            "json",
            "--table",
            "users",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// ============================================================
// 파이프라인 체이닝 테스트
// ============================================================

#[test]
fn pipeline_json_to_csv_to_yaml() {
    // Step 1: JSON → CSV
    let step1 = dkit()
        .args(["convert", "tests/fixtures/users.json", "--to", "csv"])
        .output()
        .unwrap();
    assert!(step1.status.success());
    let csv_data = String::from_utf8(step1.stdout).unwrap();

    // Step 2: CSV → YAML via stdin
    dkit()
        .args(["convert", "-", "--from", "csv", "--to", "yaml"])
        .write_stdin(csv_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("name:"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn pipeline_convert_then_query_then_convert() {
    // Step 1: JSON → CSV
    let step1 = dkit()
        .args(["convert", "tests/fixtures/users.json", "--to", "csv"])
        .output()
        .unwrap();
    assert!(step1.status.success());
    let csv_data = String::from_utf8(step1.stdout).unwrap();

    // Step 2: Query CSV via stdin
    let step2 = dkit()
        .args(["query", "-", ".[0]", "--from", "csv"])
        .write_stdin(csv_data)
        .output()
        .unwrap();
    assert!(step2.status.success());
    let json_data = String::from_utf8(step2.stdout).unwrap();

    // Step 3: Convert query result to YAML
    dkit()
        .args(["convert", "-", "--from", "json", "--to", "yaml"])
        .write_stdin(json_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("name:"));
}

#[test]
fn pipeline_yaml_to_json_query() {
    // Step 1: YAML → JSON
    let step1 = dkit()
        .args(["convert", "tests/fixtures/config.yaml", "--to", "json"])
        .output()
        .unwrap();
    assert!(step1.status.success());
    let json_data = String::from_utf8(step1.stdout).unwrap();

    // Step 2: Query JSON via stdin
    dkit()
        .args(["query", "-", ".database", "--from", "json"])
        .write_stdin(json_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("host"));
}

#[test]
fn pipeline_stdin_auto_detect_json() {
    let json_data = r#"[{"name":"Test","value":42}]"#;
    dkit()
        .args(["convert", "-", "--to", "csv"])
        .write_stdin(json_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("name,value"))
        .stdout(predicate::str::contains("Test,42"));
}

#[test]
fn pipeline_stdin_auto_detect_yaml() {
    let yaml_data = "name: Test\nvalue: 42\n";
    dkit()
        .args(["convert", "-", "--to", "json"])
        .write_stdin(yaml_data)
        .assert()
        .success()
        .stdout(predicate::str::contains("Test"))
        .stdout(predicate::str::contains("42"));
}

#[test]
fn pipeline_query_pipe_compact_output() {
    // Piped output should be compact
    let output = dkit()
        .args(["query", "-", ".", "--from", "json"])
        .write_stdin(r#"[{"a":1},{"a":2}]"#)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Compact: no leading indentation
    assert!(
        !stdout.starts_with("  "),
        "Piped output should be compact: {stdout}"
    );
}

#[test]
fn pipeline_convert_pretty_to_file() {
    let dir = TempDir::new().unwrap();
    let out_path = dir.path().join("out.json");
    dkit()
        .args([
            "convert",
            "-",
            "--from",
            "csv",
            "--to",
            "json",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .write_stdin("name,age\nAlice,30\n")
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    assert!(
        content.contains("  "),
        "File output should be pretty: {content}"
    );
}

#[test]
fn pipeline_schema_from_stdin() {
    dkit()
        .args(["schema", "-", "--from", "json"])
        .write_stdin(r#"{"users":[{"name":"Alice","age":30}]}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("users"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

#[test]
fn pipeline_stats_from_stdin() {
    dkit()
        .args(["stats", "-", "--from", "json"])
        .write_stdin(r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"));
}

// ============================================================
// 일괄 변환 테스트
// ============================================================

#[test]
fn batch_convert_mixed_formats() {
    let input_dir = TempDir::new().unwrap();
    let outdir = TempDir::new().unwrap();

    fs::write(input_dir.path().join("data.json"), r#"[{"x": 1, "y": 2}]"#).unwrap();
    fs::write(input_dir.path().join("data.csv"), "a,b\n1,2\n").unwrap();
    fs::write(
        input_dir.path().join("data.yaml"),
        "- name: test\n  val: 42\n",
    )
    .unwrap();

    dkit()
        .args([
            "convert",
            input_dir.path().to_str().unwrap(),
            "--format",
            "json",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "3 succeeded, 0 failed out of 3 files",
        ));

    assert!(outdir.path().join("data.json").exists());
    assert!(outdir.path().join("data.json").exists());
}

#[test]
fn batch_convert_to_multiple_output_formats() {
    let outdir1 = TempDir::new().unwrap();
    let outdir2 = TempDir::new().unwrap();

    // Batch to CSV
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/employees.json",
            "--format",
            "csv",
            "--outdir",
            outdir1.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(outdir1.path().join("users.csv").exists());
    assert!(outdir1.path().join("employees.csv").exists());

    // Batch to YAML
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/employees.json",
            "--format",
            "yaml",
            "--outdir",
            outdir2.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(outdir2.path().join("users.yaml").exists());
    assert!(outdir2.path().join("employees.yaml").exists());
}

#[test]
fn batch_convert_glob_json_to_yaml() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args([
            "convert",
            "tests/fixtures/*.json",
            "--format",
            "yaml",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("succeeded"));
}

#[test]
fn batch_convert_with_rename() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args([
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/users.csv",
            "--format",
            "yaml",
            "--outdir",
            outdir.path().to_str().unwrap(),
            "--rename",
            "{name}_v060.{ext}",
        ])
        .assert()
        .success();

    assert!(outdir.path().join("users_v060.yaml").exists());
}

#[test]
fn batch_convert_continue_on_error_with_summary() {
    let input_dir = TempDir::new().unwrap();
    let outdir = TempDir::new().unwrap();

    fs::write(input_dir.path().join("good1.json"), r#"[{"a": 1}]"#).unwrap();
    fs::write(input_dir.path().join("good2.json"), r#"[{"b": 2}]"#).unwrap();
    fs::write(input_dir.path().join("bad.json"), "not valid json!!!").unwrap();

    dkit()
        .args([
            "convert",
            input_dir.path().to_str().unwrap(),
            "--format",
            "csv",
            "--outdir",
            outdir.path().to_str().unwrap(),
            "--continue-on-error",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("2 succeeded"))
        .stderr(predicate::str::contains("1 failed"));

    assert!(outdir.path().join("good1.csv").exists());
    assert!(outdir.path().join("good2.csv").exists());
    assert!(!outdir.path().join("bad.csv").exists());
}

#[test]
fn batch_convert_outdir_created_if_missing() {
    let parent = TempDir::new().unwrap();
    let outdir = parent.path().join("new_subdir");

    dkit()
        .args([
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/users.csv",
            "--format",
            "yaml",
            "--outdir",
            outdir.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(outdir.join("users.yaml").exists());
}

// ============================================================
// 크로스 포맷 통합 테스트
// ============================================================

#[test]
fn cross_format_xlsx_to_csv_roundtrip() {
    // xlsx → csv
    let step1 = dkit()
        .args(["convert", "tests/fixtures/users.xlsx", "--to", "csv"])
        .output()
        .unwrap();
    assert!(step1.status.success());
    let csv_data = String::from_utf8(step1.stdout).unwrap();
    assert!(csv_data.contains("Alice"));
    assert!(csv_data.contains("Bob"));
}

#[test]
fn cross_format_sqlite_to_csv() {
    let db = create_rich_type_db();
    let output = dkit()
        .args(["convert", db.path().to_str().unwrap(), "--to", "csv"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let csv = String::from_utf8(output.stdout).unwrap();
    assert!(csv.contains("id,name,score,active,bio,raw"));
    assert!(csv.contains("Alice"));
}

#[test]
fn cross_format_sqlite_query_to_yaml() {
    let db = create_multi_table_db();
    dkit()
        .args([
            "query",
            db.path().to_str().unwrap(),
            ".[] | where age > 28 | select name, age",
            "--table",
            "users",
            "--to",
            "yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("name: Charlie"));
}
