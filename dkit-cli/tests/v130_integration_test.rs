/// v1.3.0 Integration Tests
///
/// Comprehensive integration tests for v1.3.0 features:
/// - `--explode` flag (unnest/flatten arrays into rows)
/// - `--pivot` / `--unpivot` flags (data reshaping)
/// - HCL (HashiCorp Configuration Language) format Reader/Writer
/// - `.plist` (macOS Property List) format Reader/Writer
/// - Recursive descent (`..`) operator for deep key search
/// - Conditional expressions (`if/then/else`, `case/when`) in query
/// - Statistical aggregate functions (median, percentile, stddev, variance, mode, group_concat)
/// - Combined feature tests
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// --explode tests
// ============================================================

#[test]
fn explode_array_field_into_rows() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "tags": ["rust", "python"]},
          {"name": "Bob", "tags": ["java", "go", "rust"]}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--explode",
            "tags",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Alice should appear twice (rust, python)
    assert_eq!(stdout.matches("Alice").count(), 2);
    // Bob should appear three times (java, go, rust)
    assert_eq!(stdout.matches("Bob").count(), 3);
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("python"));
    assert!(stdout.contains("java"));
    assert!(stdout.contains("go"));
}

#[test]
fn explode_empty_array_excludes_record() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "tags": ["x"]},
          {"name": "Bob", "tags": []},
          {"name": "Charlie", "tags": ["y", "z"]}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--explode",
            "tags",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    // Bob has empty array, should be excluded
    assert!(!stdout.contains("Bob"));
    assert!(stdout.contains("Charlie"));
}

#[test]
fn explode_with_csv_output() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "tags": ["a", "b"]},
          {"name": "Bob", "tags": ["c"]}
        ]"#,
    )
    .unwrap();

    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "csv",
            "--explode",
            "tags",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn explode_multiple_fields() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "tags": ["x"], "scores": [10, 20]}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--explode",
            "tags",
            "--explode",
            "scores",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should produce 1 (tags) * 2 (scores) = 2 rows
    assert_eq!(stdout.matches("Alice").count(), 2);
}

// ============================================================
// --pivot / --unpivot tests
// ============================================================

#[test]
fn unpivot_wide_to_long() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("wide.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "jan": 100, "feb": 200, "mar": 300},
          {"name": "Bob", "jan": 150, "feb": 250, "mar": 350}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--unpivot",
            "jan,feb,mar",
            "--key",
            "month",
            "--value",
            "sales",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Each person should appear 3 times (jan, feb, mar)
    assert_eq!(stdout.matches("Alice").count(), 3);
    assert_eq!(stdout.matches("Bob").count(), 3);
    assert!(stdout.contains("month"));
    assert!(stdout.contains("sales"));
    assert!(stdout.contains("jan"));
    assert!(stdout.contains("feb"));
    assert!(stdout.contains("mar"));
}

#[test]
fn unpivot_default_key_value_names() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("wide.json");
    fs::write(&input, r#"[{"name": "A", "x": 1, "y": 2}]"#).unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--unpivot",
            "x,y",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Default key name is "variable", default value name is "value"
    assert!(stdout.contains("variable"));
    assert!(stdout.contains("value"));
}

#[test]
fn pivot_long_to_wide() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("long.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "month": "jan", "sales": 100},
          {"name": "Alice", "month": "feb", "sales": 200},
          {"name": "Bob", "month": "jan", "sales": 150},
          {"name": "Bob", "month": "feb", "sales": 250}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--pivot",
            "--index",
            "name",
            "--columns",
            "month",
            "--values",
            "sales",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Each person should appear once
    assert_eq!(stdout.matches("Alice").count(), 1);
    assert_eq!(stdout.matches("Bob").count(), 1);
    // Month columns should be present
    assert!(stdout.contains("jan"));
    assert!(stdout.contains("feb"));
}

#[test]
fn unpivot_to_csv_output() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("wide.json");
    fs::write(&input, r#"[{"name": "Alice", "q1": 10, "q2": 20}]"#).unwrap();

    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "csv",
            "--unpivot",
            "q1,q2",
            "--key",
            "quarter",
            "--value",
            "amount",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("quarter"))
        .stdout(predicate::str::contains("amount"));
}

// ============================================================
// HCL format tests
// ============================================================

#[cfg(feature = "hcl")]
mod hcl_tests {
    use super::*;

    #[test]
    fn hcl_to_json_conversion() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("main.tf");
        fs::write(
            &input,
            r#"
variable "region" {
  default = "us-east-1"
}

resource "aws_instance" "web" {
  ami           = "ami-12345678"
  instance_type = "t2.micro"

  tags = {
    Name = "HelloWorld"
  }
}
"#,
        )
        .unwrap();

        let output = dkit()
            .args(&["convert", input.to_str().unwrap(), "-f", "json"])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("variable"));
        assert!(stdout.contains("resource"));
        assert!(stdout.contains("us-east-1"));
        assert!(stdout.contains("t2.micro"));
    }

    #[test]
    fn json_to_hcl_conversion() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("config.json");
        fs::write(&input, r#"{"server": {"host": "localhost", "port": 8080}}"#).unwrap();

        dkit()
            .args(&["convert", input.to_str().unwrap(), "-f", "hcl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("server"))
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn hcl_query() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("main.tf");
        fs::write(
            &input,
            r#"
variable "region" {
  default = "us-west-2"
}
"#,
        )
        .unwrap();

        dkit()
            .args(&["query", input.to_str().unwrap(), ".variable.region.default"])
            .assert()
            .success()
            .stdout(predicate::str::contains("us-west-2"));
    }

    #[test]
    fn hcl_to_yaml_conversion() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("config.tf");
        fs::write(
            &input,
            r#"
database {
  host = "db.example.com"
  port = 5432
}
"#,
        )
        .unwrap();

        dkit()
            .args(&["convert", input.to_str().unwrap(), "-f", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("db.example.com"))
            .stdout(predicate::str::contains("5432"));
    }
}

// ============================================================
// .plist format tests
// ============================================================

#[cfg(feature = "plist")]
mod plist_tests {
    use super::*;

    #[test]
    fn plist_to_json_conversion() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("Info.plist");
        fs::write(
            &input,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>MyApp</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleExecutable</key>
    <string>myapp</string>
</dict>
</plist>"#,
        )
        .unwrap();

        let output = dkit()
            .args(&["convert", input.to_str().unwrap(), "-f", "json"])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("CFBundleName"));
        assert!(stdout.contains("MyApp"));
        assert!(stdout.contains("1.0.0"));
    }

    #[test]
    fn json_to_plist_conversion() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("config.json");
        fs::write(
            &input,
            r#"{"name": "TestApp", "version": "2.0", "debug": true}"#,
        )
        .unwrap();

        dkit()
            .args(&["convert", input.to_str().unwrap(), "-f", "plist"])
            .assert()
            .success()
            .stdout(predicate::str::contains("TestApp"))
            .stdout(predicate::str::contains("2.0"));
    }

    #[test]
    fn plist_query_field() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("Info.plist");
        fs::write(
            &input,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleVersion</key>
    <string>3.2.1</string>
    <key>CFBundleName</key>
    <string>TestApp</string>
</dict>
</plist>"#,
        )
        .unwrap();

        dkit()
            .args(&["query", input.to_str().unwrap(), ".CFBundleVersion"])
            .assert()
            .success()
            .stdout(predicate::str::contains("3.2.1"));
    }

    #[test]
    fn plist_with_array_and_nested_dict() {
        let tmp = TempDir::new().unwrap();
        let input = tmp.path().join("complex.plist");
        fs::write(
            &input,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>items</key>
    <array>
        <dict>
            <key>name</key>
            <string>First</string>
        </dict>
        <dict>
            <key>name</key>
            <string>Second</string>
        </dict>
    </array>
    <key>count</key>
    <integer>42</integer>
</dict>
</plist>"#,
        )
        .unwrap();

        let output = dkit()
            .args(&["convert", input.to_str().unwrap(), "-f", "json"])
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("First"));
        assert!(stdout.contains("Second"));
        assert!(stdout.contains("42"));
    }
}

// ============================================================
// Recursive descent (..) operator tests
// ============================================================

#[test]
fn recursive_descent_find_all_keys() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("nested.json");
    fs::write(
        &input,
        r#"{
          "users": [
            {"name": "Alice", "address": {"city": "Seoul"}},
            {"name": "Bob", "address": {"city": "Busan"}}
          ],
          "admin": {"name": "Charlie"}
        }"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), "..name"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Charlie"));
}

#[test]
fn recursive_descent_deeply_nested() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("deep.json");
    fs::write(
        &input,
        r#"{
          "a": {
            "b": {
              "c": {
                "id": 1
              },
              "id": 2
            },
            "id": 3
          },
          "id": 4
        }"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), "..id"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should find all 4 'id' fields
    assert!(stdout.contains('1'));
    assert!(stdout.contains('2'));
    assert!(stdout.contains('3'));
    assert!(stdout.contains('4'));
}

#[test]
fn recursive_descent_with_path_prefix() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"{
          "config": {
            "db": {"host": "localhost"},
            "cache": {"host": "redis.local"}
          },
          "host": "top-level"
        }"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), ".config..host"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("localhost"));
    assert!(stdout.contains("redis.local"));
    // top-level 'host' should NOT be included (scoped under .config)
    assert!(!stdout.contains("top-level"));
}

#[test]
fn recursive_descent_with_pipeline() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"{
          "teams": [
            {"name": "Alpha", "members": [{"email": "a@test.com"}, {"email": "b@test.com"}]},
            {"name": "Beta", "members": [{"email": "c@test.com"}]}
          ]
        }"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), "..email"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("a@test.com"));
    assert!(stdout.contains("b@test.com"));
    assert!(stdout.contains("c@test.com"));
}

// ============================================================
// Conditional expressions (if/then/else, case/when) tests
// ============================================================

#[test]
fn if_function_in_select() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "age": 15},
          {"name": "Bob", "age": 30},
          {"name": "Charlie", "age": 70}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | select name, if(age < 18, \"minor\", \"adult\") as category",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("minor"));
    assert!(stdout.contains("adult"));
}

#[test]
fn nested_if_function() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "age": 15},
          {"name": "Bob", "age": 30},
          {"name": "Charlie", "age": 70}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | select name, if(age < 18, \"minor\", if(age < 65, \"adult\", \"senior\")) as category",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("minor"));
    assert!(stdout.contains("adult"));
    assert!(stdout.contains("senior"));
}

#[test]
fn case_when_expression() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "score": 95},
          {"name": "Bob", "score": 72},
          {"name": "Charlie", "score": 45}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | select name, case when score >= 90 then \"A\" when score >= 70 then \"B\" else \"C\" end as grade",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Alice: A, Bob: B, Charlie: C
    assert!(stdout.contains('A'));
    assert!(stdout.contains('B'));
    assert!(stdout.contains('C'));
}

#[test]
fn if_with_string_comparison() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "role": "engineer"},
          {"name": "Bob", "role": "manager"}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | select name, if(role == \"engineer\", \"tech\", \"non-tech\") as dept",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("tech"));
    assert!(stdout.contains("non-tech"));
}

// ============================================================
// Statistical aggregate functions tests
// ============================================================

#[test]
fn median_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"value": 10},
          {"value": 20},
          {"value": 30},
          {"value": 40},
          {"value": 50}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), ".[] | median value"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("30"));
}

#[test]
fn median_even_count() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"value": 10},
          {"value": 20},
          {"value": 30},
          {"value": 40}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), ".[] | median value"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // median of [10,20,30,40] = 25
    assert!(stdout.contains("25"));
}

#[test]
fn percentile_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    // 100 values from 1 to 100
    let data: Vec<String> = (1..=100).map(|i| format!("{{\"value\": {}}}", i)).collect();
    fs::write(&input, format!("[{}]", data.join(","))).unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | percentile value 0.5",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // p50 of 1..100 should be around 50
    assert!(stdout.contains("50"));
}

#[test]
fn stddev_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"value": 2},
          {"value": 4},
          {"value": 4},
          {"value": 4},
          {"value": 5},
          {"value": 5},
          {"value": 7},
          {"value": 9}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), ".[] | stddev value"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // stddev of [2,4,4,4,5,5,7,9] = 2.0 (population stddev)
    assert!(stdout.contains('2'));
}

#[test]
fn variance_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"value": 2},
          {"value": 4},
          {"value": 4},
          {"value": 4},
          {"value": 5},
          {"value": 5},
          {"value": 7},
          {"value": 9}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), ".[] | variance value"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // variance of [2,4,4,4,5,5,7,9] = 4.0 (population variance)
    assert!(stdout.contains('4'));
}

#[test]
fn mode_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"color": "red"},
          {"color": "blue"},
          {"color": "red"},
          {"color": "green"},
          {"color": "red"},
          {"color": "blue"}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), ".[] | mode color"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("red"));
}

#[test]
fn group_concat_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"category": "fruit", "name": "apple"},
          {"category": "fruit", "name": "banana"},
          {"category": "veg", "name": "carrot"},
          {"category": "fruit", "name": "cherry"}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | group_by category group_concat(name, \", \")",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("fruit"));
    assert!(stdout.contains("carrot"));
}

#[test]
fn statistical_functions_in_group_by() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"dept": "eng", "salary": 80000},
          {"dept": "eng", "salary": 90000},
          {"dept": "eng", "salary": 100000},
          {"dept": "sales", "salary": 60000},
          {"dept": "sales", "salary": 70000}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | group_by dept median(salary), stddev(salary)",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("eng"));
    assert!(stdout.contains("sales"));
    // eng median should be 90000
    assert!(stdout.contains("90000"));
}

// ============================================================
// Combined feature tests
// ============================================================

#[test]
fn explode_with_filter_and_sort() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "tags": ["rust", "python", "java"]},
          {"name": "Bob", "tags": ["go", "rust"]}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--explode",
            "tags",
            "--sort-by",
            "tags",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("python"));
}

#[test]
fn recursive_descent_with_different_formats() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.yaml");
    fs::write(
        &input,
        r#"
config:
  database:
    host: db.example.com
  cache:
    host: cache.example.com
host: top-level
"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["query", input.to_str().unwrap(), "..host"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("db.example.com"));
    assert!(stdout.contains("cache.example.com"));
    assert!(stdout.contains("top-level"));
}

#[test]
fn conditional_with_aggregate() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "age": 15, "score": 85},
          {"name": "Bob", "age": 30, "score": 92},
          {"name": "Charlie", "age": 70, "score": 78}
        ]"#,
    )
    .unwrap();

    // Use if() in select, then get results
    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | select name, if(age < 18, \"minor\", \"adult\") as category, score",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("minor"));
    assert!(stdout.contains("adult"));
}

#[test]
fn unpivot_with_csv_and_query() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("wide.csv");
    fs::write(
        &input,
        "name,q1,q2,q3\nAlice,100,200,300\nBob,150,250,350\n",
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--unpivot",
            "q1,q2,q3",
            "--key",
            "quarter",
            "--value",
            "amount",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("quarter"));
    assert!(stdout.contains("amount"));
    // Each person has 3 rows
    assert_eq!(stdout.matches("Alice").count(), 3);
    assert_eq!(stdout.matches("Bob").count(), 3);
}

#[test]
fn explode_with_view_command() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "skills": ["rust", "python"]},
          {"name": "Bob", "skills": ["java"]}
        ]"#,
    )
    .unwrap();

    dkit()
        .args(&["view", input.to_str().unwrap(), "--explode", "skills"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("rust"))
        .stdout(predicate::str::contains("python"))
        .stdout(predicate::str::contains("java"));
}

#[test]
fn statistical_functions_with_filter() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"dept": "eng", "salary": 80000},
          {"dept": "eng", "salary": 90000},
          {"dept": "eng", "salary": 100000},
          {"dept": "sales", "salary": 60000},
          {"dept": "sales", "salary": 70000}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | where dept == \"eng\" | median salary",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // eng salaries: 80000, 90000, 100000 → median = 90000
    assert!(stdout.contains("90000"));
}
