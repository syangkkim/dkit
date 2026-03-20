use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- nested.json ---

#[test]
fn nested_json_query_deep_field() {
    dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.location.city",
        ])
        .assert()
        .success()
        .stdout("\"Seoul\"\n");
}

#[test]
fn nested_json_query_deep_nested() {
    dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.location.address.zip",
        ])
        .assert()
        .success()
        .stdout("\"06000\"\n");
}

#[test]
fn nested_json_query_array_in_nested() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.departments.[0].name",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Engineering"));
}

#[test]
fn nested_json_query_nested_array() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.departments.[0].teams.[1].name",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Frontend"));
}

#[test]
fn nested_json_convert_to_yaml() {
    dkit()
        .args(["convert", "tests/fixtures/nested.json", "--to", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: TechCorp"))
        .stdout(predicate::str::contains("city: Seoul"));
}

// --- empty.json ---

#[test]
fn empty_json_view() {
    dkit()
        .args(["view", "tests/fixtures/empty.json"])
        .assert()
        .success();
}

#[test]
fn empty_json_convert_to_yaml() {
    dkit()
        .args(["convert", "tests/fixtures/empty.json", "--to", "yaml"])
        .assert()
        .success();
}

// --- single.json ---

#[test]
fn single_json_query_field() {
    dkit()
        .args(["query", "tests/fixtures/single.json", ".name"])
        .assert()
        .success()
        .stdout("\"Alice\"\n");
}

#[test]
fn single_json_query_bool() {
    dkit()
        .args(["query", "tests/fixtures/single.json", ".active"])
        .assert()
        .success()
        .stdout("true\n");
}

#[test]
fn single_json_convert_to_yaml() {
    dkit()
        .args(["convert", "tests/fixtures/single.json", "--to", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("active: true"));
}

#[test]
fn single_json_convert_to_toml() {
    dkit()
        .args(["convert", "tests/fixtures/single.json", "--to", "toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name = \"Alice\""))
        .stdout(predicate::str::contains("active = true"));
}

// --- unicode.csv ---

#[test]
fn unicode_csv_convert_to_json() {
    dkit()
        .args(["convert", "tests/fixtures/unicode.csv", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("김철수"))
        .stdout(predicate::str::contains("서울"));
}

#[test]
fn unicode_csv_view() {
    dkit()
        .args(["view", "tests/fixtures/unicode.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("김철수"))
        .stdout(predicate::str::contains("이영희"));
}

// --- quoted.csv ---

#[test]
fn quoted_csv_convert_to_json() {
    dkit()
        .args(["convert", "tests/fixtures/quoted.csv", "--to", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Smith, John"))
        .stdout(predicate::str::contains("Normal Name"));
}

#[test]
fn quoted_csv_view() {
    dkit()
        .args(["view", "tests/fixtures/quoted.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Smith, John"));
}

// --- Cross-format roundtrip ---

#[test]
fn roundtrip_single_json_to_yaml_to_json() {
    let yaml_output = dkit()
        .args(["convert", "tests/fixtures/single.json", "--to", "yaml"])
        .output()
        .unwrap();
    assert!(yaml_output.status.success());
    let yaml_str = String::from_utf8(yaml_output.stdout).unwrap();

    let json_output = dkit()
        .args(["convert", "--from", "yaml", "--to", "json"])
        .write_stdin(yaml_str)
        .output()
        .unwrap();
    assert!(json_output.status.success());
    let json_str = String::from_utf8(json_output.stdout).unwrap();
    assert!(json_str.contains("\"name\": \"Alice\""));
    assert!(json_str.contains("\"age\": 30"));
}
