//! v1.5.0 integration tests
//!
//! Tests for features added in v1.5.0:
//! - `--template` flag for custom text output formatting
//! - `join` subcommand for cross-file key-based data joining
//! - Window functions (rank, row_number, lag, lead) in query
//! - `profile` subcommand for data profiling and quality analysis
//! - Combination tests across new features

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ── Template Tests ──────────────────────────────────────────────────────────

#[cfg(feature = "template")]
#[test]
fn template_inline_basic() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template",
            "{{ name }} <{{ email }}>",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice <alice@example.com>"))
        .stdout(predicate::str::contains("Bob <bob@example.com>"));
}

#[cfg(feature = "template")]
#[test]
fn template_with_integer_field() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template",
            "{{ name }} is {{ age }} years old",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice is 30 years old"))
        .stdout(predicate::str::contains("Bob is 25 years old"));
}

#[cfg(feature = "template")]
#[test]
fn template_upper_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template",
            "{{ name | upper }}",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("BOB"));
}

#[cfg(feature = "template")]
#[test]
fn template_lower_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template",
            "{{ email | lower }}",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("alice@example.com"));
}

#[cfg(feature = "template")]
#[test]
fn template_default_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template",
            r#"{{ missing | default(value="N/A") }}"#,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("N/A"));
}

#[cfg(feature = "template")]
#[test]
fn template_from_file() {
    let dir = TempDir::new().unwrap();
    let tpl_path = dir.path().join("report.tpl");
    std::fs::write(&tpl_path, "Name: {{ name }}, Email: {{ email }}").unwrap();

    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template-file",
            tpl_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Name: Alice, Email: alice@example.com",
        ))
        .stdout(predicate::str::contains(
            "Name: Bob, Email: bob@example.com",
        ));
}

#[cfg(feature = "template")]
#[test]
fn template_without_template_flag_fails() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "-f", "template"])
        .assert()
        .failure();
}

#[cfg(feature = "template")]
#[test]
fn template_csv_input() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.csv",
            "-f",
            "template",
            "--template",
            "{{ name }}: {{ email }}",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice: alice@example.com"))
        .stdout(predicate::str::contains("Bob: bob@example.com"));
}

// ── Join Tests ──────────────────────────────────────────────────────────────

#[test]
fn join_inner_same_key() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "-f",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Laptop"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Phone"));
}

#[test]
fn join_left_type() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "--type",
            "left",
            "-f",
            "json",
        ])
        .assert()
        .success()
        // Diana (id=4) has no orders, should still appear in left join
        .stdout(predicate::str::contains("Diana"));
}

#[test]
fn join_right_type() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "--type",
            "right",
            "-f",
            "json",
        ])
        .assert()
        .success()
        // All orders should appear
        .stdout(predicate::str::contains("Laptop"))
        .stdout(predicate::str::contains("Monitor"));
}

#[test]
fn join_full_type() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "--type",
            "full",
            "-f",
            "json",
        ])
        .assert()
        .success()
        // Both Diana (no orders) and all orders should appear
        .stdout(predicate::str::contains("Diana"))
        .stdout(predicate::str::contains("Laptop"))
        .stdout(predicate::str::contains("Monitor"));
}

#[test]
fn join_cross_format_json_csv() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.csv",
            "--on",
            "id=user_id",
            "-f",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Laptop"));
}

#[test]
fn join_output_csv() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "-f",
            "csv",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Laptop"));
}

#[test]
fn join_output_to_file() {
    let dir = TempDir::new().unwrap();
    let out_path = dir.path().join("joined.json");

    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "-f",
            "json",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(out_path.exists());
    let content = std::fs::read_to_string(&out_path).unwrap();
    assert!(content.contains("Alice"));
    assert!(content.contains("Laptop"));
}

#[test]
fn join_invalid_type_fails() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "--type",
            "invalid",
            "-f",
            "json",
        ])
        .assert()
        .failure();
}

#[test]
fn join_pretty_output() {
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "-f",
            "json",
            "--pretty",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// ── Window Function Tests ───────────────────────────────────────────────────

#[test]
fn window_row_number() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, row_number() over (order by revenue desc) as rn",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("row_number").or(predicate::str::contains("rn")));
}

#[test]
fn window_rank() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, rank() over (order by revenue desc) as rank",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("rank"));
}

#[test]
fn window_dense_rank() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, dense_rank() over (order by revenue desc) as drank",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("dense_rank").or(predicate::str::contains("drank")));
}

#[test]
fn window_partition_by() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, department, revenue, row_number() over (partition by department order by revenue desc) as dept_rank",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("dept_rank").or(predicate::str::contains("row_number")));
}

#[test]
fn window_lag() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/timeseries.json",
            ".[] | select date, value, lag(value, 1) over (order by date) as prev_value",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("prev_value").or(predicate::str::contains("lag")));
}

#[test]
fn window_lead() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/timeseries.json",
            ".[] | select date, value, lead(value, 1) over (order by date) as next_value",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("next_value").or(predicate::str::contains("lead")));
}

#[test]
fn window_aggregate_sum() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/timeseries.json",
            ".[] | select date, value, sum(value) over (order by date) as running_total",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("running_total").or(predicate::str::contains("sum")));
}

#[test]
fn window_first_value() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, first_value(name) over (order by revenue desc) as top_earner",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("top_earner").or(predicate::str::contains("first_value")));
}

#[test]
fn window_last_value() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, last_value(name) over (order by revenue desc) as lowest_earner",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("lowest_earner")
                .or(predicate::str::contains("last_value")),
        );
}

#[test]
fn window_with_json_output() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, rank() over (order by revenue desc) as rank",
            "--to",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"rank\""));
}

// ── Profile Tests ───────────────────────────────────────────────────────────

#[test]
fn profile_basic_csv() {
    dkit()
        .args(&["profile", "tests/fixtures/profile_data.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("email"));
}

#[test]
fn profile_json_input() {
    dkit()
        .args(&["profile", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

#[test]
fn profile_output_json() {
    dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("total_records"))
        .stdout(predicate::str::contains("total_fields"));
}

#[test]
fn profile_output_yaml() {
    dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("total_records"))
        .stdout(predicate::str::contains("total_fields"));
}

#[test]
fn profile_output_markdown() {
    dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "md",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("|"))
        .stdout(predicate::str::contains("name"));
}

#[test]
fn profile_detailed() {
    dkit()
        .args(&["profile", "tests/fixtures/profile_data.csv", "--detailed"])
        .assert()
        .success()
        // Detailed mode should show additional statistics
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"));
}

#[test]
fn profile_null_detection() {
    // profile_data.csv has missing values in age and email columns
    dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("null_percent"));
}

#[test]
fn profile_field_type_inference() {
    dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\""));
}

// ── Combination Tests ───────────────────────────────────────────────────────

#[cfg(feature = "template")]
#[test]
fn template_with_query_pipeline() {
    // Query first, then pipe to template
    let query_output = dkit()
        .args(&[
            "query",
            "tests/fixtures/users.json",
            ".[] | select name, email",
            "--to",
            "json",
        ])
        .output()
        .unwrap();

    assert!(query_output.status.success());

    dkit()
        .args(&[
            "convert",
            "-",
            "--from",
            "json",
            "-f",
            "template",
            "--template",
            "{{ name }}: {{ email }}",
        ])
        .write_stdin(query_output.stdout)
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice: alice@example.com"));
}

#[test]
fn join_with_filter() {
    // Join and verify the join result contains expected data
    dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "-f",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("1200"));
}

#[test]
fn profile_with_json_pipeline() {
    // Profile output as JSON can be piped to query
    let profile_output = dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "json",
        ])
        .output()
        .unwrap();

    assert!(profile_output.status.success());
    let stdout = String::from_utf8_lossy(&profile_output.stdout);
    assert!(stdout.contains("total_records"));
}

#[test]
fn window_with_csv_output() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/sales.json",
            ".[] | select name, revenue, rank() over (order by revenue desc) as rank",
            "--to",
            "csv",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("rank"));
}

#[test]
fn join_multiple_matches() {
    // Alice has 2 orders (id=1), should produce 2 rows in inner join
    let output = dkit()
        .args(&[
            "join",
            "tests/fixtures/users_with_id.json",
            "tests/fixtures/orders.json",
            "--on",
            "id=user_id",
            "-f",
            "json",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Alice should appear twice (for Laptop and Tablet orders)
    let alice_count = stdout.matches("Alice").count();
    assert!(
        alice_count >= 2,
        "Alice should appear at least twice in join result (found {alice_count})"
    );
}

#[test]
fn profile_unique_count() {
    dkit()
        .args(&[
            "profile",
            "tests/fixtures/profile_data.csv",
            "--output-format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("unique_count"));
}

#[cfg(feature = "template")]
#[test]
fn template_multiline_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "-f",
            "template",
            "--template",
            "## {{ name }}\n- Email: {{ email }}\n- Age: {{ age }}",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("## Alice"))
        .stdout(predicate::str::contains("- Email: alice@example.com"));
}
