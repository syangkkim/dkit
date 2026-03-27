/// v1.2.0 Integration Tests
///
/// Comprehensive integration tests for v1.2.0 features:
/// - `--unique` / `--unique-by` flags (deduplication)
/// - `--add-field` with arithmetic and string concatenation
/// - `--map` with built-in functions and value transformation
/// - Array slicing and wildcard query
/// - `.ini` / `.cfg` format Reader/Writer
/// - `.properties` format Reader/Writer
/// - `IN` / `NOT IN` operators
/// - `matches` regex operator
/// - `--indent`, `--sort-keys`, `--compact` JSON output options
/// - New string functions (index_of, starts_with, pad, reverse, etc.)
/// - Combined feature tests
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// --unique / --unique-by tests
// ============================================================

#[test]
fn unique_removes_duplicate_records() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("dupes.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "age": 30},
          {"name": "Bob", "age": 25},
          {"name": "Alice", "age": 30},
          {"name": "Charlie", "age": 35},
          {"name": "Bob", "age": 25}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "json", "--unique"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("Alice").count(), 1);
    assert_eq!(stdout.matches("Bob").count(), 1);
    assert_eq!(stdout.matches("Charlie").count(), 1);
}

#[test]
fn unique_by_field_keeps_first_occurrence() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("dupes.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "city": "Seoul"},
          {"name": "Bob", "city": "Busan"},
          {"name": "Charlie", "city": "Seoul"},
          {"name": "Diana", "city": "Busan"}
        ]"#,
    )
    .unwrap();

    let output = dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--unique-by",
            "city",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should keep Alice (first Seoul) and Bob (first Busan)
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));
    // Charlie and Diana are duplicates by city
    assert!(!stdout.contains("Charlie"));
    assert!(!stdout.contains("Diana"));
}

#[test]
fn unique_with_csv_output() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("dupes.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "age": 30},
          {"name": "Alice", "age": 30},
          {"name": "Bob", "age": 25}
        ]"#,
    )
    .unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "csv", "--unique"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn unique_by_with_view_command() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--unique-by",
            "city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Busan"));
}

// ============================================================
// --add-field integration tests (arithmetic, string concat)
// ============================================================

#[test]
fn add_field_subtraction() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "remaining = 100 - score",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("remaining"))
        .stdout(predicate::str::contains("15")); // Alice: 100 - 85
}

#[test]
fn add_field_division() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "half_age = age / 2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("half_age"));
}

#[test]
fn add_field_multiple_fields() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "double_age = age * 2",
            "--add-field",
            "label = name + \" (\" + role + \")\"",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("double_age"))
        .stdout(predicate::str::contains("label"));
}

#[test]
fn add_field_with_filter_and_sort() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "double_score = score * 2",
            "--filter",
            "age > 28",
            "--sort-by",
            "score",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("double_score"));
}

// ============================================================
// --map integration tests (built-in functions, value transformation)
// ============================================================

#[test]
fn map_field_trim() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("spaces.json");
    fs::write(&input, r#"[{"name": "  Alice  "}, {"name": "  Bob  "}]"#).unwrap();

    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--map",
            "name = trim(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"Alice\""))
        .stdout(predicate::str::contains("\"Bob\""));
}

#[test]
fn map_field_replace() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--map",
            "city = replace(city, \"Seoul\", \"서울\")",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("서울"));
}

#[test]
fn map_field_multiple_transforms() {
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
fn map_field_with_view() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/employees.json",
            "--map",
            "name = upper(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"));
}

// ============================================================
// Array slicing and wildcard query tests
// ============================================================

#[test]
fn query_array_wildcard() {
    dkit()
        .args(&["query", "tests/fixtures/employees.json", ".[*].name"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie"));
}

#[test]
fn query_array_slice_basic() {
    dkit()
        .args(&["query", "tests/fixtures/employees.json", ".[0:2]"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn query_array_slice_negative() {
    dkit()
        .args(&["query", "tests/fixtures/employees.json", ".[-2:]"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Diana"))
        .stdout(predicate::str::contains("Eve"));
}

#[test]
fn query_array_slice_with_step() {
    dkit()
        .args(&["query", "tests/fixtures/employees.json", ".[::2]"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Eve"));
}

#[test]
fn query_wildcard_with_field_access() {
    dkit()
        .args(&["query", "tests/fixtures/employees.json", ".[*].city"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Busan"))
        .stdout(predicate::str::contains("Incheon"));
}

// ============================================================
// .ini / .cfg format tests
// ============================================================

#[test]
fn ini_to_json_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.ini");
    fs::write(
        &input,
        "[database]\nhost = localhost\nport = 5432\n\n[server]\ndebug = true\n",
    )
    .unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("database"))
        .stdout(predicate::str::contains("localhost"))
        .stdout(predicate::str::contains("5432"))
        .stdout(predicate::str::contains("server"))
        .stdout(predicate::str::contains("debug"));
}

#[test]
fn json_to_ini_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.json");
    fs::write(
        &input,
        r#"{"database": {"host": "localhost", "port": "5432"}, "server": {"debug": "true"}}"#,
    )
    .unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "ini"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[database]"))
        .stdout(
            predicate::str::contains("host=localhost")
                .or(predicate::str::contains("host = localhost")),
        );
}

#[test]
fn ini_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.ini");
    let intermediate = tmp.path().join("config.json");
    let output = tmp.path().join("config_out.ini");

    fs::write(&input, "[section]\nkey1 = value1\nkey2 = value2\n").unwrap();

    // INI → JSON
    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "-o",
            intermediate.to_str().unwrap(),
        ])
        .assert()
        .success();

    // JSON → INI
    dkit()
        .args(&[
            "convert",
            intermediate.to_str().unwrap(),
            "-f",
            "ini",
            "-o",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    let result = fs::read_to_string(&output).unwrap();
    assert!(result.contains("[section]"));
    assert!(result.contains("key1"));
    assert!(result.contains("value1"));
}

#[test]
fn ini_query() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.ini");
    fs::write(&input, "[database]\nhost = localhost\nport = 5432\n").unwrap();

    dkit()
        .args(&["query", input.to_str().unwrap(), ".database.host"])
        .assert()
        .success()
        .stdout(predicate::str::contains("localhost"));
}

#[test]
fn ini_view() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.ini");
    fs::write(
        &input,
        "[app]\nname = myapp\nversion = 1.0\n\n[db]\nhost = localhost\n",
    )
    .unwrap();

    dkit()
        .args(&["view", input.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("app"))
        .stdout(predicate::str::contains("db"));
}

#[test]
fn cfg_extension_detected_as_ini() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.cfg");
    fs::write(&input, "[settings]\ntheme = dark\nlanguage = ko\n").unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("settings"))
        .stdout(predicate::str::contains("dark"));
}

// ============================================================
// .properties format tests
// ============================================================

#[test]
fn properties_to_json_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("app.properties");
    fs::write(
        &input,
        "app.name=MyApp\napp.version=1.0\ndb.host=localhost\ndb.port=5432\n",
    )
    .unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app.name"))
        .stdout(predicate::str::contains("MyApp"))
        .stdout(predicate::str::contains("db.host"))
        .stdout(predicate::str::contains("localhost"));
}

#[test]
fn json_to_properties_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.json");
    fs::write(&input, r#"{"app.name": "MyApp", "db.host": "localhost"}"#).unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "properties"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app.name"))
        .stdout(predicate::str::contains("MyApp"));
}

#[test]
fn properties_with_comments_and_special_chars() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("app.properties");
    fs::write(
        &input,
        "# This is a comment\n! Another comment\nkey1=value1\nkey2 = value with spaces\nkey3=\n",
    )
    .unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key1"))
        .stdout(predicate::str::contains("value1"))
        .stdout(predicate::str::contains("key2"))
        .stdout(predicate::str::contains("value with spaces"));
}

#[test]
fn properties_query() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("app.properties");
    fs::write(&input, "host=localhost\nport=5432\n").unwrap();

    dkit()
        .args(&["query", input.to_str().unwrap(), ".host"])
        .assert()
        .success()
        .stdout(predicate::str::contains("localhost"));
}

#[test]
fn properties_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("app.properties");
    let intermediate = tmp.path().join("config.json");
    let output = tmp.path().join("app_out.properties");

    fs::write(&input, "name=Alice\nage=30\ncity=Seoul\n").unwrap();

    // properties → JSON
    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "-o",
            intermediate.to_str().unwrap(),
        ])
        .assert()
        .success();

    // JSON → properties
    dkit()
        .args(&[
            "convert",
            intermediate.to_str().unwrap(),
            "-f",
            "properties",
            "-o",
            output.to_str().unwrap(),
        ])
        .assert()
        .success();

    let result = fs::read_to_string(&output).unwrap();
    assert!(result.contains("name"));
    assert!(result.contains("Alice"));
}

// ============================================================
// IN / NOT IN operator tests
// ============================================================

#[test]
fn query_where_in_operator() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city in (\"Seoul\", \"Busan\") | select name, city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Incheon").not());
}

#[test]
fn query_where_not_in_operator() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city not in (\"Seoul\") | select name, city",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Diana"))
        .stdout(predicate::str::contains("Alice").not());
}

#[test]
fn filter_flag_in_operator() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--filter",
            "role in (\"engineer\", \"manager\")",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("designer").not());
}

#[test]
fn in_operator_with_numbers() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age in (25, 30) | select name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

// ============================================================
// matches regex operator tests
// ============================================================

#[test]
fn query_where_matches_regex() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name matches \"^[A-C]\" | select name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Diana").not())
        .stdout(predicate::str::contains("Eve").not());
}

#[test]
fn query_where_not_matches() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name not matches \"^[A-C]\" | select name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Diana"))
        .stdout(predicate::str::contains("Eve"))
        .stdout(predicate::str::contains("Alice").not());
}

#[test]
fn filter_flag_matches_operator() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--filter",
            "city matches \"^S\"",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Seoul"))
        .stdout(predicate::str::contains("Busan").not());
}

// ============================================================
// --indent, --sort-keys, --compact JSON output tests
// ============================================================

#[test]
fn json_compact_output() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--compact",
            "--head",
            "1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  ").not());
}

#[test]
fn json_indent_2_spaces() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--indent",
            "2",
            "--head",
            "1",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should have 2-space indentation
    assert!(stdout.contains("  \"name\"") || stdout.contains("  \"age\""));
}

#[test]
fn json_indent_4_spaces() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--indent",
            "4",
            "--head",
            "1",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("    \"name\"") || stdout.contains("    \"age\""));
}

#[test]
fn json_indent_tab() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--indent",
            "tab",
            "--head",
            "1",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\t\"name\"") || stdout.contains("\t\"age\""));
}

#[test]
fn json_sort_keys() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--sort-keys",
            "--head",
            "1",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // With sort-keys, "age" should come before "city" and "name"
    let age_pos = stdout.find("\"age\"").unwrap_or(usize::MAX);
    let city_pos = stdout.find("\"city\"").unwrap_or(usize::MAX);
    let name_pos = stdout.find("\"name\"").unwrap_or(usize::MAX);
    assert!(age_pos < city_pos);
    assert!(city_pos < name_pos);
}

#[test]
fn json_compact_with_sort_keys() {
    let output = dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--compact",
            "--sort-keys",
            "--head",
            "1",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be compact (no pretty-printing) with sorted keys
    assert!(stdout.contains("\"age\""));
    assert!(!stdout.contains("  \"age\""));
}

// ============================================================
// New string function tests (index_of, starts_with, pad, reverse, etc.)
// ============================================================

#[test]
fn query_function_index_of() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name, index_of(name, \"li\") as pos",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("pos"));
}

#[test]
fn query_function_starts_with() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name, starts_with(name, \"A\") as starts_a",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("starts_a"));
}

#[test]
fn query_function_ends_with() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name, ends_with(name, \"e\") as ends_e",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ends_e"));
}

#[test]
fn query_function_reverse() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select reverse(name) as reversed | limit 1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ecilA"));
}

#[test]
fn query_function_repeat() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select repeat(\"*\", 3) as stars | limit 1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("***"));
}

#[test]
fn query_function_pad_left() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select pad_left(to_string(age), 5, \"0\") as padded | limit 1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("00030"));
}

#[test]
fn query_function_pad_right() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | select pad_right(name, 10, \".\") as padded | limit 1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice....."));
}

#[test]
fn query_function_rindex_of() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(&input, r#"[{"text": "hello world hello"}]"#).unwrap();

    dkit()
        .args(&[
            "query",
            input.to_str().unwrap(),
            ".[] | select rindex_of(text, \"hello\") as pos",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("12"));
}

// ============================================================
// Combined feature tests
// ============================================================

#[test]
fn unique_with_add_field_and_filter() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.json");
    fs::write(
        &input,
        r#"[
          {"name": "Alice", "age": 30, "city": "Seoul"},
          {"name": "Bob", "age": 25, "city": "Busan"},
          {"name": "Charlie", "age": 30, "city": "Seoul"},
          {"name": "Diana", "age": 28, "city": "Incheon"}
        ]"#,
    )
    .unwrap();

    dkit()
        .args(&[
            "convert",
            input.to_str().unwrap(),
            "-f",
            "json",
            "--unique-by",
            "city",
            "--add-field",
            "double_age = age * 2",
            "--filter",
            "age >= 28",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("double_age"));
}

#[test]
fn map_with_select_and_sort() {
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
            "--sort-by",
            "age",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("BOB"));
}

#[test]
fn in_operator_with_sort_and_select() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role in (\"engineer\") | select name, score | sort score desc",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Diana"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn ini_to_yaml_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.ini");
    fs::write(&input, "[app]\nname = myapp\nversion = 1.0\n").unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app"))
        .stdout(predicate::str::contains("myapp"));
}

#[test]
fn properties_to_yaml_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("config.properties");
    fs::write(&input, "key1=value1\nkey2=value2\n").unwrap();

    dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key1"))
        .stdout(predicate::str::contains("value1"));
}

#[test]
fn matches_with_in_operator_combined() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name matches \"^[A-B]\" and role in (\"engineer\", \"designer\") | select name, role",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn json_output_options_with_filter() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--sort-keys",
            "--indent",
            "2",
            "--filter",
            "age > 30",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Charlie"));
}

#[test]
fn string_functions_in_where_with_starts_with_operator() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name starts_with \"A\" | select name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob").not());
}

#[test]
fn add_field_with_string_functions() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "-f",
            "json",
            "--add-field",
            "upper_name = upper(name)",
            "--add-field",
            "name_len = length(name)",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("upper_name"))
        .stdout(predicate::str::contains("ALICE"))
        .stdout(predicate::str::contains("name_len"));
}

#[test]
fn unique_with_csv_to_json_conversion() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("data.csv");
    fs::write(
        &input,
        "name,city\nAlice,Seoul\nBob,Busan\nAlice,Seoul\nCharlie,Seoul\n",
    )
    .unwrap();

    let output = dkit()
        .args(&["convert", input.to_str().unwrap(), "-f", "json", "--unique"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.matches("Alice").count(), 1);
}

#[test]
fn slice_with_filter_pipeline() {
    dkit()
        .args(&[
            "query",
            "tests/fixtures/employees.json",
            ".[0:3] | where age > 25 | select name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"));
}
