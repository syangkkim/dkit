use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// --log-format: Apache Combined
// ============================================================

#[test]
fn log_format_apache_to_json() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "json",
            "--log-format",
            "apache",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("remote_host"))
        .stdout(predicate::str::contains("127.0.0.1"))
        .stdout(predicate::str::contains("frank"))
        .stdout(predicate::str::contains("user_agent"))
        .stdout(predicate::str::contains("Mozilla/4.08"));
}

#[test]
fn log_format_apache_to_csv() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "csv",
            "--log-format",
            "apache-combined",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("remote_host"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("127.0.0.1"));
}

#[test]
fn log_format_apache_null_fields() {
    // "-" in Apache logs should be parsed as null
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "json",
            "--log-format",
            "apache",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("null"));
}

#[test]
fn log_format_apache_status_as_integer() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "json",
            "--log-format",
            "apache",
        ])
        .assert()
        .success()
        // Status codes should be integers (no quotes around them)
        .stdout(predicate::str::contains("\"status\":200"))
        .stdout(predicate::str::contains("\"status\":404"));
}

// ============================================================
// --log-format: nginx
// ============================================================

#[test]
fn log_format_nginx_to_json() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/nginx.log",
            "-f",
            "json",
            "--log-format",
            "nginx",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("remote_addr"))
        .stdout(predicate::str::contains("10.0.0.5"))
        .stdout(predicate::str::contains("alice"))
        .stdout(predicate::str::contains("http_user_agent"));
}

#[test]
fn log_format_nginx_multiple_records() {
    let output = dkit()
        .args([
            "convert",
            "tests/fixtures/nginx.log",
            "-f",
            "json",
            "--log-format",
            "nginx",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should have 3 records
    assert_eq!(stdout.matches("remote_addr").count(), 3);
}

// ============================================================
// --log-format: syslog
// ============================================================

#[test]
fn log_format_syslog_to_json() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/syslog.log",
            "-f",
            "json",
            "--log-format",
            "syslog",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("hostname"))
        .stdout(predicate::str::contains("myhost"))
        .stdout(predicate::str::contains("app_name"))
        .stdout(predicate::str::contains("sshd"))
        .stdout(predicate::str::contains("message"));
}

#[test]
fn log_format_syslog_pid_as_integer() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/syslog.log",
            "-f",
            "json",
            "--log-format",
            "syslog",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"pid\":1234"));
}

// ============================================================
// --log-format: custom pattern
// ============================================================

#[test]
fn log_format_custom_pattern() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/app.log",
            "-f",
            "json",
            "--log-format",
            "{timestamp} [{level}] {message}",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("timestamp"))
        .stdout(predicate::str::contains("2024-01-15T10:30:00"))
        .stdout(predicate::str::contains("level"))
        .stdout(predicate::str::contains("INFO"))
        .stdout(predicate::str::contains("Server started on port 8080"));
}

#[test]
fn log_format_custom_pattern_to_csv() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/app.log",
            "-f",
            "csv",
            "--log-format",
            "{timestamp} [{level}] {message}",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("timestamp,level,message"));
}

// ============================================================
// --log-error: parse failure handling
// ============================================================

#[test]
fn log_error_skip_unparseable_lines() {
    // app.log has one line that doesn't match the pattern
    let output = dkit()
        .args([
            "convert",
            "tests/fixtures/app.log",
            "-f",
            "json",
            "--log-format",
            "{timestamp} [{level}] {message}",
            "--log-error",
            "skip",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should have 5 records (the non-matching line is skipped)
    assert_eq!(stdout.matches("timestamp").count(), 5);
    assert!(!stdout.contains("_raw"));
}

#[test]
fn log_error_raw_includes_unparseable() {
    let output = dkit()
        .args([
            "convert",
            "tests/fixtures/app.log",
            "-f",
            "json",
            "--log-format",
            "{timestamp} [{level}] {message}",
            "--log-error",
            "raw",
        ])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should have 6 records (5 parsed + 1 raw)
    assert!(stdout.contains("_raw"));
    assert!(stdout.contains("this line does not match the pattern"));
}

// ============================================================
// log → JSON/CSV + stats pipeline
// ============================================================

#[test]
fn log_format_with_stats_command() {
    dkit()
        .args([
            "stats",
            "tests/fixtures/access.log",
            "--log-format",
            "apache",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("count"))
        .stdout(predicate::str::contains("4"));
}

#[test]
fn log_to_json_pipe_query() {
    // Convert log to JSON, then pipe to query
    let dir = TempDir::new().unwrap();
    let json_out = dir.path().join("access.json");

    // First: convert log to JSON
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "json",
            "--log-format",
            "apache",
            "-o",
            json_out.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Then: query the JSON
    dkit()
        .args([
            "query",
            json_out.to_str().unwrap(),
            ".[] | where status == 200 | select remote_host, request",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("remote_host"))
        .stdout(predicate::str::contains("127.0.0.1"));
}

// ============================================================
// --parallel: multi-file batch conversion
// ============================================================

#[test]
fn parallel_batch_conversion() {
    let dir = TempDir::new().unwrap();
    let input_dir = dir.path().join("input");
    let output_dir = dir.path().join("output");
    fs::create_dir_all(&input_dir).unwrap();

    // Create multiple JSON files
    for i in 0..4 {
        let path = input_dir.join(format!("data{i}.json"));
        fs::write(&path, format!(r#"[{{"id": {i}, "name": "item{i}"}}]"#)).unwrap();
    }

    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            output_dir.to_str().unwrap(),
            "--parallel",
            "2",
        ])
        .assert()
        .success();

    // Check that all output files exist
    for i in 0..4 {
        let out_file = output_dir.join(format!("data{i}.csv"));
        assert!(out_file.exists(), "Expected output file: {:?}", out_file);
    }
}

#[test]
fn parallel_auto_detection() {
    let dir = TempDir::new().unwrap();
    let input_dir = dir.path().join("input");
    let output_dir = dir.path().join("output");
    fs::create_dir_all(&input_dir).unwrap();

    for i in 0..2 {
        let path = input_dir.join(format!("data{i}.json"));
        fs::write(&path, format!(r#"[{{"id": {i}}}]"#)).unwrap();
    }

    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            output_dir.to_str().unwrap(),
            "--parallel",
            "auto",
        ])
        .assert()
        .success();

    assert!(output_dir.join("data0.csv").exists());
    assert!(output_dir.join("data1.csv").exists());
}

#[test]
fn parallel_error_handling() {
    let dir = TempDir::new().unwrap();
    let input_dir = dir.path().join("input");
    let output_dir = dir.path().join("output");
    fs::create_dir_all(&input_dir).unwrap();

    // Create one valid and one invalid JSON file
    fs::write(input_dir.join("good.json"), r#"[{"id": 1, "name": "ok"}]"#).unwrap();
    fs::write(input_dir.join("bad.json"), "this is not valid json").unwrap();

    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            output_dir.to_str().unwrap(),
            "--parallel",
            "2",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("FAILED"));
}

#[test]
fn parallel_zero_threads_error() {
    let dir = TempDir::new().unwrap();
    let input_dir = dir.path().join("input");
    let output_dir = dir.path().join("output");
    fs::create_dir_all(&input_dir).unwrap();
    fs::write(input_dir.join("a.json"), r#"[{"x":1}]"#).unwrap();
    fs::write(input_dir.join("b.json"), r#"[{"x":2}]"#).unwrap();

    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            output_dir.to_str().unwrap(),
            "--parallel",
            "0",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--parallel must be at least 1"));
}

#[test]
fn parallel_sequential_results_match() {
    let dir = TempDir::new().unwrap();
    let input_dir = dir.path().join("input");
    let seq_dir = dir.path().join("seq_output");
    let par_dir = dir.path().join("par_output");
    fs::create_dir_all(&input_dir).unwrap();

    for i in 0..3 {
        let path = input_dir.join(format!("data{i}.json"));
        fs::write(&path, format!(r#"[{{"id": {i}, "value": "test{i}"}}]"#)).unwrap();
    }

    // Sequential
    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            seq_dir.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Parallel
    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            par_dir.to_str().unwrap(),
            "--parallel",
            "2",
        ])
        .assert()
        .success();

    // Compare outputs
    for i in 0..3 {
        let seq_content = fs::read_to_string(seq_dir.join(format!("data{i}.csv"))).unwrap();
        let par_content = fs::read_to_string(par_dir.join(format!("data{i}.csv"))).unwrap();
        assert_eq!(
            seq_content, par_content,
            "Sequential and parallel outputs differ for data{i}.csv"
        );
    }
}

// ============================================================
// --time: execution timing
// ============================================================

#[test]
fn time_flag_convert_log() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "json",
            "--log-format",
            "apache",
            "--time",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_diff_no_diff() {
    // When files are identical, diff exits 0 and timing is printed
    dkit()
        .args([
            "diff",
            "tests/fixtures/users.json",
            "tests/fixtures/users.json",
            "--time",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_schema() {
    dkit()
        .args(["schema", "tests/fixtures/users.json", "--time"])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

#[test]
fn time_flag_merge() {
    dkit()
        .args([
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "-f",
            "json",
            "--time",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"))
        .stderr(predicate::str::contains("total:"));
}

// ============================================================
// --explain: query execution plan
// ============================================================

#[test]
fn explain_with_aggregate() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | count",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Execution Plan:"))
        .stderr(predicate::str::contains("Aggregate:"))
        .stdout(predicate::str::is_empty());
}

#[test]
fn explain_with_distinct() {
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | distinct name",
            "--explain",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Execution Plan:"))
        .stdout(predicate::str::is_empty());
}

#[test]
fn explain_stderr_only() {
    // Verify --explain always outputs to stderr, never stdout
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[] | where age > 20 | select name | sort name | limit 3",
            "--explain",
        ])
        .assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("Execution Plan:"))
        .stderr(predicate::str::contains("Scan:"))
        .stderr(predicate::str::contains("Navigate:"))
        .stderr(predicate::str::contains("Filter:"))
        .stderr(predicate::str::contains("Project:"))
        .stderr(predicate::str::contains("Sort:"))
        .stderr(predicate::str::contains("Limit:"));
}

// ============================================================
// Combined features
// ============================================================

#[test]
fn log_format_with_time_flag() {
    dkit()
        .args([
            "convert",
            "tests/fixtures/access.log",
            "-f",
            "csv",
            "--log-format",
            "apache",
            "--time",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("remote_host"))
        .stderr(predicate::str::contains("timing:"));
}

#[test]
fn parallel_with_time_flag() {
    let dir = TempDir::new().unwrap();
    let input_dir = dir.path().join("input");
    let output_dir = dir.path().join("output");
    fs::create_dir_all(&input_dir).unwrap();

    for i in 0..2 {
        fs::write(
            input_dir.join(format!("d{i}.json")),
            format!(r#"[{{"x": {i}}}]"#),
        )
        .unwrap();
    }

    dkit()
        .args([
            "convert",
            input_dir.join("*.json").to_str().unwrap(),
            "-f",
            "csv",
            "--outdir",
            output_dir.to_str().unwrap(),
            "--parallel",
            "2",
            "--time",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("timing:"));
}
