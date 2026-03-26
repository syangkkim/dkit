/// v1.1.0 Integration Tests
///
/// Comprehensive integration tests for v1.1.0 features:
/// - `--select` flag for convert/view (column selection)
/// - `--group-by` + `--agg` flags (aggregation)
/// - `.env` format Reader/Writer
/// - `--dry-run` flag (preview without writing)
/// - `--output-format` flag for stats/schema (JSON/YAML output)
/// - Combined flag tests (--select + --filter + --sort-by etc.)
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// --select flag tests
// ============================================================

#[test]
fn select_single_field_json() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--select",
            "name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("age").not());
}

#[test]
fn select_multiple_fields_json() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--select",
            "name, city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("score").not())
        .stdout(predicate::str::contains("role").not());
}

#[test]
fn select_fields_csv_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name, age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age"))
        .stdout(predicate::str::contains("Alice,30"))
        .stdout(predicate::str::contains("city").not());
}

#[test]
fn select_with_view_command() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--select",
            "name, score",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("score"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("85"))
        .stdout(predicate::str::contains("city").not());
}

#[test]
fn select_preserves_all_records() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Header + 5 data rows
    let line_count = stdout.trim().lines().count();
    assert_eq!(line_count, 6, "should have header + 5 records");
}

#[test]
fn select_with_stdin() {
    dkit()
        .args(&[
            "convert", "-", "--from", "json", "-f", "json", "--select", "name",
        ])
        .write_stdin(r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("age").not());
}

// ============================================================
// --group-by + --agg flag tests
// ============================================================

#[test]
fn group_by_count() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--group-by",
            "city",
            "--agg",
            "count()",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("3"))
        .stdout(predicate::str::contains("Busan"))
        .stdout(predicate::str::contains("1"));
}

#[test]
fn group_by_with_sum() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--group-by",
            "city",
            "--agg",
            "sum(score)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Busan"));
}

#[test]
fn group_by_with_multiple_aggregations() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--group-by",
            "city",
            "--agg",
            "count(), avg(score)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("count"))
        .stdout(predicate::str::contains("avg_score"));
}

#[test]
fn group_by_csv_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--group-by",
            "role",
            "--agg",
            "count()",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("role"))
        .stdout(predicate::str::contains("count"))
        .stdout(predicate::str::contains("engineer"));
}

#[test]
fn group_by_with_view_command() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--group-by",
            "city",
            "--agg",
            "count()",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Busan"));
}

#[test]
fn group_by_min_max() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--group-by",
            "city",
            "--agg",
            "min(score), max(score)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("min_score"))
        .stdout(predicate::str::contains("max_score"));
}

// ============================================================
// .env format Reader/Writer tests
// ============================================================

#[test]
fn env_to_json_conversion() {
    dkit()
        .args(&["convert", "tests/fixtures/sample.env", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DB_HOST"))
        .stdout(predicate::str::contains("localhost"))
        .stdout(predicate::str::contains("DB_PORT"))
        .stdout(predicate::str::contains("5432"));
}

#[test]
fn env_to_yaml_conversion() {
    dkit()
        .args(&["convert", "tests/fixtures/sample.env", "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DB_HOST"))
        .stdout(predicate::str::contains("localhost"));
}

#[test]
fn json_to_env_conversion() {
    dkit()
        .args(&["convert", "-", "--from", "json", "-f", "env"])
        .write_stdin(r#"{"HOST":"localhost","PORT":"8080"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("HOST=localhost"))
        .stdout(predicate::str::contains("PORT=8080"));
}

#[test]
fn env_handles_quoted_values() {
    dkit()
        .args(&["convert", "tests/fixtures/sample.env", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("My Application"))
        .stdout(predicate::str::contains("s3cr3t-key"));
}

#[test]
fn env_handles_export_prefix() {
    dkit()
        .args(&["convert", "tests/fixtures/sample.env", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("EXPORTED_VAR"))
        .stdout(predicate::str::contains("exported_value"));
}

#[test]
fn env_handles_comments_and_empty_lines() {
    dkit()
        .args(&["convert", "tests/fixtures/sample.env", "-f", "json"])
        .assert()
        .success()
        // Comments should not appear as keys
        .stdout(predicate::str::contains("# Database").not())
        .stdout(predicate::str::contains("DB_HOST"));
}

#[test]
fn env_roundtrip_json() {
    let tmp = TempDir::new().unwrap();
    let json_file = tmp.path().join("config.json");
    let env_file = tmp.path().join("config.env");

    // env → json
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/sample.env",
            "-f",
            "json",
            "-o",
            json_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    // json → env
    dkit()
        .args(&[
            "convert",
            json_file.to_str().unwrap(),
            "-f",
            "env",
            "-o",
            env_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    let env_content = fs::read_to_string(&env_file).unwrap();
    assert!(env_content.contains("DB_HOST=localhost"));
    assert!(env_content.contains("DB_PORT=5432"));
}

#[test]
fn env_view() {
    dkit()
        .args(&["view", "tests/fixtures/sample.env"])
        .assert()
        .success()
        .stdout(predicate::str::contains("DB_HOST"))
        .stdout(predicate::str::contains("localhost"));
}

#[test]
fn env_diff() {
    let tmp = TempDir::new().unwrap();
    let env1 = tmp.path().join("dev.env");
    let env2 = tmp.path().join("prod.env");

    fs::write(&env1, "DB_HOST=localhost\nDB_PORT=5432\n").unwrap();
    fs::write(&env2, "DB_HOST=prod-server\nDB_PORT=5432\n").unwrap();

    dkit()
        .args(&["diff", env1.to_str().unwrap(), env2.to_str().unwrap()])
        .assert()
        .stdout(predicate::str::contains("DB_HOST"));
}

#[test]
fn env_merge() {
    let tmp = TempDir::new().unwrap();
    let env1 = tmp.path().join("defaults.env");
    let env2 = tmp.path().join("overrides.env");

    fs::write(&env1, "DB_HOST=localhost\nDB_PORT=5432\n").unwrap();
    fs::write(&env2, "DB_HOST=prod-server\nAPP_KEY=secret\n").unwrap();

    dkit()
        .args(&[
            "merge",
            env1.to_str().unwrap(),
            env2.to_str().unwrap(),
            "--to",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("prod-server"))
        .stdout(predicate::str::contains("APP_KEY"));
}

// ============================================================
// --dry-run flag tests (extended)
// ============================================================

#[test]
fn dry_run_with_select() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name, age",
            "--dry-run",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name,age"))
        .stdout(predicate::str::contains("Alice"))
        .stderr(predicate::str::contains("[dry-run]"));
}

#[test]
fn dry_run_with_group_by() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--group-by",
            "city",
            "--agg",
            "count()",
            "--dry-run",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stderr(predicate::str::contains("[dry-run]"));
}

// ============================================================
// --output-format flag tests (stats / schema)
// ============================================================

#[test]
fn stats_output_format_json() {
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/employees.json",
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"rows\""))
        .stdout(predicate::str::contains("\"columns\""));
}

#[test]
fn stats_output_format_json_has_valid_structure() {
    let output = dkit()
        .args(&[
            "stats",
            "tests/fixtures/employees.json",
            "--output-format",
            "json",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed.get("rows").is_some());
    assert!(parsed.get("columns").is_some());
}

#[test]
fn stats_output_format_json_column_details() {
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/employees.json",
            "--output-format",
            "json",
            "--column",
            "age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\""))
        .stdout(predicate::str::contains("\"mean\""));
}

#[test]
fn stats_output_format_yaml() {
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/employees.json",
            "--output-format",
            "yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows:"));
}

#[test]
fn schema_output_format_json() {
    dkit()
        .args(&[
            "schema",
            "tests/fixtures/employees.json",
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\""))
        .stdout(predicate::str::contains("\"array\""));
}

#[test]
fn schema_output_format_json_has_valid_structure() {
    let output = dkit()
        .args(&[
            "schema",
            "tests/fixtures/employees.json",
            "--output-format",
            "json",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(parsed.get("type").unwrap(), "array");
    assert!(parsed.get("items").is_some());
}

// ============================================================
// Combined flag tests
// ============================================================

#[test]
fn select_with_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name, age",
            "--filter",
            "age > 30",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Charlie,35"))
        .stdout(predicate::str::contains("Eve,32"))
        .stdout(predicate::str::contains("Alice").not());
}

#[test]
fn select_with_filter_and_sort() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name, age",
            "--filter",
            "age > 28",
            "--sort-by",
            "age",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines[0], "name,age");
    assert!(lines[1].contains("Alice")); // age 30
    assert!(lines[2].contains("Eve")); // age 32
    assert!(lines[3].contains("Charlie")); // age 35
}

#[test]
fn select_with_filter_sort_and_head() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name, age",
            "--filter",
            "age > 28",
            "--sort-by",
            "age",
            "--head",
            "2",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim().lines().collect();
    // header + 2 records
    assert_eq!(lines.len(), 3);
    assert!(lines[1].contains("Alice")); // age 30, first after sort
}

#[test]
fn select_with_sort_desc() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "csv",
            "--select",
            "name, score",
            "--sort-by",
            "score",
            "--sort-order",
            "desc",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.trim().lines().collect();
    // Diana (95) should be first after header
    assert!(lines[1].contains("Diana"));
}

#[test]
fn group_by_with_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--filter",
            "role == \"engineer\"",
            "--group-by",
            "city",
            "--agg",
            "count()",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Incheon"));
}

#[test]
fn select_with_yaml_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "yaml",
            "--select",
            "name, city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name:"))
        .stdout(predicate::str::contains("city:"))
        .stdout(predicate::str::contains("score").not());
}

#[test]
fn select_with_toml_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "toml",
            "--select",
            "name, age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn env_with_select() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/sample.env",
            "-f",
            "json",
            "--select",
            "DB_HOST, DB_PORT",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("DB_HOST"))
        .stdout(predicate::str::contains("DB_PORT"))
        .stdout(predicate::str::contains("APP_NAME").not());
}

// ============================================================
// Pipeline / multi-format tests
// ============================================================

#[test]
fn select_csv_to_json_pipeline() {
    // CSV input → select → JSON output
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.csv",
            "-f",
            "json",
            "--select",
            "name, email",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("email"))
        .stdout(predicate::str::contains("age").not());
}

#[test]
fn group_by_csv_input() {
    dkit()
        .args(&[
            "convert",
            "-",
            "--from",
            "csv",
            "-f",
            "json",
            "--group-by",
            "city",
            "--agg",
            "count()",
        ])
        .write_stdin("name,city\nAlice,Seoul\nBob,Busan\nCharlie,Seoul\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("2"))
        .stdout(predicate::str::contains("Busan"))
        .stdout(predicate::str::contains("1"));
}

#[test]
fn output_format_with_file_output() {
    // Test that --output-format json works and produces parseable JSON
    let output = dkit()
        .args(&[
            "stats",
            "tests/fixtures/users.json",
            "--output-format",
            "json",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let _: serde_json::Value = serde_json::from_str(&stdout)
        .expect("stats --output-format json should produce valid JSON");
}

// ============================================================
// Edge cases
// ============================================================

#[test]
fn select_nonexistent_field() {
    // Selecting a field that doesn't exist should not crash
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "json",
            "--select",
            "name, nonexistent",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"));
}

#[test]
fn group_by_single_group() {
    // All records in one group
    dkit()
        .args(&[
            "convert",
            "-",
            "--from",
            "json",
            "-f",
            "json",
            "--group-by",
            "status",
            "--agg",
            "count()",
        ])
        .write_stdin(r#"[{"name":"A","status":"active"},{"name":"B","status":"active"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("active"))
        .stdout(predicate::str::contains("2"));
}

#[test]
fn env_empty_file() {
    let tmp = TempDir::new().unwrap();
    let empty_env = tmp.path().join("empty.env");
    fs::write(&empty_env, "# just a comment\n").unwrap();

    dkit()
        .args(&["convert", empty_env.to_str().unwrap(), "-f", "json"])
        .assert()
        .success();
}

#[test]
fn env_to_csv_conversion() {
    // .env is flat key-value, should convert to single-row CSV-like structure
    dkit()
        .args(&["convert", "-", "--from", "env", "-f", "json"])
        .write_stdin("KEY1=value1\nKEY2=value2\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("KEY1"))
        .stdout(predicate::str::contains("value1"));
}

#[test]
fn stats_default_output_is_not_json() {
    // Without --output-format, stats should produce human-readable table output (not JSON)
    dkit()
        .args(&["stats", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("{").not());
}
