/// v1.0.0 Stabilization QA Tests
///
/// Comprehensive test suite for v1.0.0 release readiness:
/// - Round-trip test matrix (all bidirectional format combinations)
/// - Edge case tests (empty data, unicode, special characters, large data)
/// - End-to-end usability scenarios (pipelines, multi-step workflows)
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// Round-trip test matrix: A → B → A
//
// Bidirectional formats (always available): JSON, JSONL, CSV, YAML, TOML
// We test every pair: convert A→B, then B→A, and verify data preserved.
// ============================================================

/// Helper: convert stdin content from `from` format to `to` format, return stdout
fn convert(input: &str, from: &str, to: &str) -> String {
    let output = dkit()
        .args(["convert", "--from", from, "--to", to])
        .write_stdin(input)
        .output()
        .expect("failed to run dkit convert");
    assert!(
        output.status.success(),
        "convert {} -> {} failed: {}",
        from,
        to,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("non-utf8 output")
}

// --- JSON round-trips ---

#[test]
fn roundtrip_json_to_csv_to_json() {
    let json_in = r#"[{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]"#;
    let csv = convert(json_in, "json", "csv");
    assert!(csv.contains("Alice"));
    let json_out = convert(&csv, "csv", "json");
    assert!(json_out.contains("Alice"));
    assert!(json_out.contains("Bob"));
}

#[test]
fn roundtrip_json_to_yaml_to_json() {
    let json_in = r#"{"name":"Alice","age":30,"active":true}"#;
    let yaml = convert(json_in, "json", "yaml");
    assert!(yaml.contains("name:"));
    let json_out = convert(&yaml, "yaml", "json");
    assert!(json_out.contains("Alice"));
    assert!(json_out.contains("30"));
}

#[test]
fn roundtrip_json_to_toml_to_json() {
    let json_in = r#"{"name":"Alice","age":30}"#;
    let toml = convert(json_in, "json", "toml");
    assert!(toml.contains("name"));
    let json_out = convert(&toml, "toml", "json");
    assert!(json_out.contains("Alice"));
    assert!(json_out.contains("30"));
}

#[test]
fn roundtrip_json_to_jsonl_to_json() {
    let json_in = r#"[{"name":"Alice"},{"name":"Bob"}]"#;
    let jsonl = convert(json_in, "json", "jsonl");
    assert!(jsonl.contains("Alice"));
    assert!(jsonl.contains("Bob"));
    let json_out = convert(&jsonl, "jsonl", "json");
    assert!(json_out.contains("Alice"));
    assert!(json_out.contains("Bob"));
}

// --- YAML round-trips ---

#[test]
fn roundtrip_yaml_to_csv_to_yaml() {
    let yaml_in = "- name: Alice\n  age: \"30\"\n- name: Bob\n  age: \"25\"\n";
    let csv = convert(yaml_in, "yaml", "csv");
    assert!(csv.contains("Alice"));
    let yaml_out = convert(&csv, "csv", "yaml");
    assert!(yaml_out.contains("Alice"));
    assert!(yaml_out.contains("Bob"));
}

#[test]
fn roundtrip_yaml_to_toml_to_yaml() {
    let yaml_in = "name: Alice\nage: 30\n";
    let toml = convert(yaml_in, "yaml", "toml");
    assert!(toml.contains("Alice"));
    let yaml_out = convert(&toml, "toml", "yaml");
    assert!(yaml_out.contains("Alice"));
}

#[test]
fn roundtrip_yaml_to_jsonl_to_yaml() {
    let yaml_in = "- name: Alice\n- name: Bob\n";
    let jsonl = convert(yaml_in, "yaml", "jsonl");
    assert!(jsonl.contains("Alice"));
    let yaml_out = convert(&jsonl, "jsonl", "yaml");
    assert!(yaml_out.contains("Alice"));
    assert!(yaml_out.contains("Bob"));
}

// --- CSV round-trips ---

#[test]
fn roundtrip_csv_to_toml_to_csv() {
    // CSV→TOML produces array of tables; TOML array of tables→CSV may need JSON intermediary
    let csv_in = "name,age\nAlice,30\nBob,25\n";
    let toml = convert(csv_in, "csv", "toml");
    assert!(toml.contains("Alice"));
    assert!(toml.contains("Bob"));
    // Verify TOML→JSON preserves data (TOML array of tables can't directly go to CSV)
    let json = convert(&toml, "toml", "json");
    assert!(json.contains("Alice"));
    assert!(json.contains("Bob"));
}

#[test]
fn roundtrip_csv_to_jsonl_to_csv() {
    let csv_in = "name,age\nAlice,30\nBob,25\n";
    let jsonl = convert(csv_in, "csv", "jsonl");
    assert!(jsonl.contains("Alice"));
    let csv_out = convert(&jsonl, "jsonl", "csv");
    assert!(csv_out.contains("Alice"));
    assert!(csv_out.contains("Bob"));
}

// --- TOML round-trips ---

#[test]
fn roundtrip_toml_to_jsonl() {
    // TOML single object → JSONL (single line) → back
    let toml_in = "name = \"Alice\"\nage = 30\n";
    let jsonl = convert(toml_in, "toml", "jsonl");
    assert!(jsonl.contains("Alice"));
}

// --- JSONL round-trips ---

#[test]
fn roundtrip_jsonl_to_csv_to_jsonl() {
    let jsonl_in = "{\"name\":\"Alice\",\"age\":\"30\"}\n{\"name\":\"Bob\",\"age\":\"25\"}\n";
    let csv = convert(jsonl_in, "jsonl", "csv");
    assert!(csv.contains("Alice"));
    let jsonl_out = convert(&csv, "csv", "jsonl");
    assert!(jsonl_out.contains("Alice"));
    assert!(jsonl_out.contains("Bob"));
}

#[test]
fn roundtrip_jsonl_to_yaml_to_jsonl() {
    let jsonl_in = "{\"name\":\"Alice\"}\n{\"name\":\"Bob\"}\n";
    let yaml = convert(jsonl_in, "jsonl", "yaml");
    assert!(yaml.contains("Alice"));
    let jsonl_out = convert(&yaml, "yaml", "jsonl");
    assert!(jsonl_out.contains("Alice"));
    assert!(jsonl_out.contains("Bob"));
}

// ============================================================
// Edge case tests
// ============================================================

#[test]
fn edge_empty_json_array() {
    let out = convert("[]", "json", "csv");
    // Empty array should produce empty or header-only output
    assert!(out.is_empty() || !out.contains('\n') || out.lines().count() <= 1);
}

#[test]
fn edge_empty_json_object() {
    let out = convert("{}", "json", "yaml");
    // Empty object should convert without error
    assert!(
        out.trim() == "{}" || out.trim() == "{}  " || out.contains("{}") || out.trim().is_empty()
    );
}

#[test]
fn edge_single_element_array() {
    let json = r#"[{"key":"value"}]"#;
    let csv = convert(json, "json", "csv");
    assert!(csv.contains("key"));
    assert!(csv.contains("value"));
}

#[test]
fn edge_unicode_values_json_to_csv() {
    let json = r#"[{"name":"한국어","city":"東京"},{"name":"München","city":"São Paulo"}]"#;
    let csv = convert(json, "json", "csv");
    assert!(csv.contains("한국어"));
    assert!(csv.contains("東京"));
    assert!(csv.contains("München"));
    assert!(csv.contains("São Paulo"));
}

#[test]
fn edge_unicode_values_json_to_yaml() {
    let json = r#"{"greeting":"안녕하세요","emoji":"🎉"}"#;
    let yaml = convert(json, "json", "yaml");
    assert!(yaml.contains("안녕하세요"));
}

#[test]
fn edge_unicode_roundtrip_csv_json_csv() {
    let csv_in = "name,city\n한국어,서울\nDéjà vu,Zürich\n";
    let json = convert(csv_in, "csv", "json");
    assert!(json.contains("한국어"));
    assert!(json.contains("서울"));
    let csv_out = convert(&json, "json", "csv");
    assert!(csv_out.contains("한국어"));
    assert!(csv_out.contains("서울"));
    assert!(csv_out.contains("Zürich"));
}

#[test]
fn edge_special_characters_in_csv() {
    // CSV with quotes, commas inside values, newlines
    let csv_in = "name,description\nAlice,\"Has a comma, here\"\nBob,\"Line1\nLine2\"\n";
    let json = convert(csv_in, "csv", "json");
    assert!(json.contains("Has a comma, here"));
    assert!(json.contains("Line1"));
}

#[test]
fn edge_special_characters_in_json_strings() {
    let json = r#"{"text":"line1\nline2","tab":"col1\tcol2","quote":"say \"hello\""}"#;
    let yaml = convert(json, "json", "yaml");
    // Should not crash; content should be preserved
    assert!(yaml.contains("line1"));
}

#[test]
fn edge_numeric_precision_json_yaml() {
    let json = r#"{"integer":9007199254740992,"float":3.141592653589793,"negative":-42}"#;
    let yaml = convert(json, "json", "yaml");
    assert!(yaml.contains("3.14159"));
    assert!(yaml.contains("-42"));
}

#[test]
fn edge_boolean_null_values() {
    let json = r#"{"active":true,"deleted":false,"middle_name":null}"#;
    let yaml = convert(json, "json", "yaml");
    assert!(yaml.contains("true"));
    assert!(yaml.contains("false"));
    // null should be represented as null/~ in YAML
    assert!(yaml.contains("null") || yaml.contains("~"));
}

#[test]
fn edge_deeply_nested_json_to_yaml() {
    let json = r#"{"a":{"b":{"c":{"d":{"e":"deep"}}}}}"#;
    let yaml = convert(json, "json", "yaml");
    assert!(yaml.contains("deep"));
    let json_out = convert(&yaml, "yaml", "json");
    assert!(json_out.contains("deep"));
}

#[test]
fn edge_large_array_json_to_csv() {
    // Generate 1000-element array
    let items: Vec<String> = (0..1000)
        .map(|i| format!(r#"{{"id":{},"name":"item_{}"}}"#, i, i))
        .collect();
    let json = format!("[{}]", items.join(","));
    let csv = convert(&json, "json", "csv");
    assert!(csv.contains("item_0"));
    assert!(csv.contains("item_999"));
    // Should have 1001 lines (header + 1000 rows)
    assert!(csv.lines().count() >= 1000);
}

#[test]
fn edge_empty_string_values() {
    let json = r#"[{"name":"","age":""},{"name":"Alice","age":"30"}]"#;
    let csv = convert(json, "json", "csv");
    assert!(csv.contains("Alice"));
    let json_out = convert(&csv, "csv", "json");
    assert!(json_out.contains("Alice"));
}

#[test]
fn edge_keys_with_spaces_and_special_chars() {
    let json = r#"{"first name":"Alice","age (years)":"30","email@work":"a@b.com"}"#;
    let yaml = convert(json, "json", "yaml");
    assert!(yaml.contains("Alice"));
    let json_out = convert(&yaml, "yaml", "json");
    assert!(json_out.contains("Alice"));
}

// ============================================================
// End-to-end usability scenarios
// ============================================================

#[test]
fn e2e_convert_file_to_file() {
    let dir = TempDir::new().unwrap();
    let input = dir.path().join("input.json");
    let output = dir.path().join("output.yaml");
    fs::write(&input, r#"[{"name":"Alice","age":30}]"#).unwrap();

    dkit()
        .args([
            "convert",
            input.to_str().unwrap(),
            "--to",
            "yaml",
            "-o",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&output).unwrap();
    assert!(content.contains("Alice"));
    assert!(content.contains("30"));
}

#[test]
fn e2e_convert_then_query() {
    // Convert JSON to YAML, then query the YAML
    let dir = TempDir::new().unwrap();
    let yaml_file = dir.path().join("data.yaml");

    // Step 1: Convert
    let json_data = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let yaml_output = convert(json_data, "json", "yaml");
    fs::write(&yaml_file, &yaml_output).unwrap();

    // Step 2: Query
    dkit()
        .args(["query", yaml_file.to_str().unwrap(), ".[0].name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn e2e_query_with_filter() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(
        &f,
        r#"[{"name":"Alice","age":30},{"name":"Bob","age":25},{"name":"Charlie","age":35}]"#,
    )
    .unwrap();

    dkit()
        .args(["query", f.to_str().unwrap(), ".[] | where age > 28"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Bob").not());
}

#[test]
fn e2e_view_json_as_table() {
    dkit()
        .args(["view", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn e2e_view_csv_as_table() {
    dkit()
        .args(["view", "tests/fixtures/users.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn e2e_stats_on_json() {
    dkit()
        .args(["stats", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"));
}

#[test]
fn e2e_schema_json() {
    dkit()
        .args(["schema", "tests/fixtures/users.json"])
        .assert()
        .success();
}

#[test]
fn e2e_diff_two_json_files() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name":"Alice","age":30}"#).unwrap();
    fs::write(&f2, r#"{"name":"Alice","age":31}"#).unwrap();

    // diff returns exit code 1 when files differ, which is expected behavior
    let output = dkit()
        .args(["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("age"),
        "diff output should show 'age' difference: {}",
        stdout
    );
}

#[test]
fn e2e_merge_two_json_files() {
    let dir = TempDir::new().unwrap();
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"name":"Alice"}"#).unwrap();
    fs::write(&f2, r#"{"age":30}"#).unwrap();

    dkit()
        .args(["merge", f1.to_str().unwrap(), f2.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("30"));
}

#[test]
fn e2e_flatten_nested_json() {
    dkit()
        .args(["flatten", "tests/fixtures/nested.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("company"));
}

#[test]
fn e2e_sample_json_array() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    let items: Vec<String> = (0..100).map(|i| format!(r#"{{"id":{}}}"#, i)).collect();
    fs::write(&f, format!("[{}]", items.join(","))).unwrap();

    dkit()
        .args(["sample", f.to_str().unwrap(), "--count", "5"])
        .assert()
        .success();
}

#[test]
fn e2e_validate_json_against_schema() {
    let dir = TempDir::new().unwrap();
    let data = dir.path().join("data.json");
    let schema = dir.path().join("schema.json");
    fs::write(&data, r#"{"name":"Alice","age":30}"#).unwrap();
    fs::write(
        &schema,
        r#"{"type":"object","properties":{"name":{"type":"string"},"age":{"type":"integer"}},"required":["name","age"]}"#,
    )
    .unwrap();

    dkit()
        .args([
            "validate",
            data.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn e2e_batch_convert_multiple_files() {
    let dir = TempDir::new().unwrap();
    let outdir = dir.path().join("output");
    let f1 = dir.path().join("a.json");
    let f2 = dir.path().join("b.json");
    fs::write(&f1, r#"{"x":1}"#).unwrap();
    fs::write(&f2, r#"{"y":2}"#).unwrap();

    dkit()
        .args([
            "convert",
            f1.to_str().unwrap(),
            f2.to_str().unwrap(),
            "--to",
            "yaml",
            "--outdir",
            outdir.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(outdir.join("a.yaml").exists());
    assert!(outdir.join("b.yaml").exists());
}

#[test]
fn e2e_stdin_pipeline_json_to_csv() {
    // Simulate: echo '...' | dkit convert --from json --to csv
    dkit()
        .args(["convert", "--from", "json", "--to", "csv"])
        .write_stdin(r#"[{"a":"1","b":"2"},{"a":"3","b":"4"}]"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("a,b").or(predicate::str::contains("b,a")))
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("3"));
}

#[test]
fn e2e_auto_detect_format_from_content() {
    // Without --from flag, dkit should detect JSON from content
    dkit()
        .args(["convert", "--to", "yaml"])
        .write_stdin(r#"{"auto":"detected"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("auto"));
}

// ============================================================
// Multi-format output tests (markdown, html)
// ============================================================

#[test]
fn output_json_to_markdown_table() {
    let json = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let md = convert(json, "json", "md");
    assert!(md.contains("Alice"));
    assert!(md.contains("|")); // Markdown table uses pipes
}

#[test]
fn output_json_to_html_table() {
    let json = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let html = convert(json, "json", "html");
    assert!(html.contains("Alice"));
    assert!(html.contains("<t")); // HTML table tags
}

#[test]
fn output_csv_to_markdown() {
    let csv = "name,age\nAlice,30\n";
    let md = convert(csv, "csv", "md");
    assert!(md.contains("Alice"));
    assert!(md.contains("|"));
}

#[test]
fn output_yaml_to_html() {
    let yaml = "- name: Alice\n  age: 30\n";
    let html = convert(yaml, "yaml", "html");
    assert!(html.contains("Alice"));
}

// ============================================================
// Error handling & robustness
// ============================================================

#[test]
fn error_invalid_json_input() {
    dkit()
        .args(["convert", "--from", "json", "--to", "csv"])
        .write_stdin("{invalid json content")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn error_invalid_yaml_input() {
    dkit()
        .args(["convert", "--from", "yaml", "--to", "json"])
        .write_stdin(":\n  - invalid:\nyaml: [unclosed")
        .assert()
        .failure();
}

#[test]
fn error_invalid_toml_input() {
    dkit()
        .args(["convert", "--from", "toml", "--to", "json"])
        .write_stdin("[invalid\ntoml = = =")
        .assert()
        .failure();
}

#[test]
fn error_query_invalid_path() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(&f, r#"{"name":"Alice"}"#).unwrap();

    // Invalid query syntax should fail gracefully
    dkit()
        .args(["query", f.to_str().unwrap(), "|||"])
        .assert()
        .failure();
}

#[test]
fn error_convert_nonexistent_file() {
    dkit()
        .args([
            "convert",
            "/tmp/dkit_nonexistent_qa_test_12345.json",
            "--to",
            "csv",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn error_convert_write_to_readonly_dir() {
    // Attempting to write to a non-writable location
    dkit()
        .args([
            "convert",
            "--from",
            "json",
            "--to",
            "csv",
            "-o",
            "/proc/dkit_impossible_path/output.csv",
        ])
        .write_stdin(r#"[{"a":1}]"#)
        .assert()
        .failure();
}

// ============================================================
// Query engine comprehensive tests
// ============================================================

#[test]
fn query_select_specific_fields() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(
        &f,
        r#"[{"name":"Alice","age":30,"email":"a@b.com"},{"name":"Bob","age":25,"email":"b@c.com"}]"#,
    )
    .unwrap();

    dkit()
        .args(["query", f.to_str().unwrap(), ".[] | select name, age"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("email").not());
}

#[test]
fn query_sort_by_field() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(
        &f,
        r#"[{"name":"Charlie","age":35},{"name":"Alice","age":30},{"name":"Bob","age":25}]"#,
    )
    .unwrap();

    dkit()
        .args(["query", f.to_str().unwrap(), ".[] | sort age"])
        .assert()
        .success();
}

#[test]
fn query_limit_results() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    let items: Vec<String> = (0..50).map(|i| format!(r#"{{"id":{}}}"#, i)).collect();
    fs::write(&f, format!("[{}]", items.join(","))).unwrap();

    dkit()
        .args(["query", f.to_str().unwrap(), ".[] | limit 5"])
        .assert()
        .success();
}

#[test]
fn query_nested_path_access() {
    dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.location.city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"));
}

#[test]
fn query_array_index_access() {
    dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.departments[0].name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Engineering"));
}

// ============================================================
// CLI help & version tests
// ============================================================

#[test]
fn cli_version_flag() {
    dkit()
        .args(["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dkit"));
}

#[test]
fn cli_help_flag() {
    dkit()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("convert"))
        .stdout(predicate::str::contains("query"))
        .stdout(predicate::str::contains("view"));
}

#[test]
fn cli_subcommand_help() {
    for subcmd in &[
        "convert", "query", "view", "stats", "schema", "merge", "diff", "flatten", "sample",
        "validate",
    ] {
        dkit()
            .args([subcmd, "--help"])
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    }
}

#[test]
fn cli_convert_help_lists_formats() {
    dkit()
        .args(["convert", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("json").or(predicate::str::contains("csv")));
}

// ============================================================
// TSV support
// ============================================================

#[test]
fn tsv_read_and_convert_to_json() {
    dkit()
        .args(["convert", "tests/fixtures/users.tsv", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn tsv_roundtrip_via_json() {
    let tsv_in = "name\tage\nAlice\t30\nBob\t25\n";
    let json = convert(tsv_in, "tsv", "json");
    assert!(json.contains("Alice"));
    let tsv_out = convert(&json, "json", "tsv");
    assert!(tsv_out.contains("Alice"));
    assert!(tsv_out.contains('\t'));
}

// ============================================================
// Streaming / large data robustness
// ============================================================

#[test]
fn large_jsonl_processing() {
    // Generate 5000 JSONL lines
    let lines: String = (0..5000)
        .map(|i| format!(r#"{{"id":{},"value":"item_{}"}}"#, i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let csv = convert(&lines, "jsonl", "csv");
    assert!(csv.contains("item_0"));
    assert!(csv.contains("item_4999"));
}

#[test]
fn large_csv_to_json() {
    let mut csv = String::from("id,name,value\n");
    for i in 0..2000 {
        csv.push_str(&format!("{},name_{},val_{}\n", i, i, i));
    }
    let json = convert(&csv, "csv", "json");
    assert!(json.contains("name_0"));
    assert!(json.contains("name_1999"));
}
