use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// 쿼리 엔진 확장 기능 통합 테스트
// ============================================================

// --- where: 다양한 비교 연산자 ---

#[test]
fn query_where_less_than() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age < 30",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob")); // age=25
    assert!(stdout.contains("Diana")); // age=28
    assert!(!stdout.contains("Alice")); // age=30
    assert!(!stdout.contains("Charlie")); // age=35
}

#[test]
fn query_where_greater_equal() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where score >= 90",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob")); // score=92
    assert!(stdout.contains("Diana")); // score=95
    assert!(!stdout.contains("Alice")); // score=85
}

#[test]
fn query_where_less_equal() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age <= 28",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob")); // age=25
    assert!(stdout.contains("Diana")); // age=28
    assert!(!stdout.contains("Alice")); // age=30
}

#[test]
fn query_where_not_equal() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city != \"Seoul\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob")); // Busan
    assert!(stdout.contains("Diana")); // Incheon
    assert!(!stdout.contains("Alice")); // Seoul
    assert!(!stdout.contains("Charlie")); // Seoul
    assert!(!stdout.contains("Eve")); // Seoul
}

// --- where: 문자열 연산 ---

#[test]
fn query_where_contains() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name contains \"li\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Charlie"));
    assert!(!stdout.contains("Bob"));
}

#[test]
fn query_where_ends_with() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name ends_with \"e\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Eve"));
    assert!(!stdout.contains("Bob"));
    assert!(!stdout.contains("Diana"));
}

// --- where: boolean/null 비교 ---

#[test]
fn query_where_boolean_true() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/mixed_types.json",
            ".[] | where active == true",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Charlie"));
    assert!(!stdout.contains("Bob"));
    assert!(!stdout.contains("Diana"));
}

#[test]
fn query_where_boolean_false() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/mixed_types.json",
            ".[] | where active == false",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Diana"));
    assert!(!stdout.contains("Alice"));
}

#[test]
fn query_where_null_eq() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/mixed_types.json",
            ".[] | where note == null",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Charlie"));
    assert!(!stdout.contains("Bob"));
    assert!(!stdout.contains("Diana"));
}

#[test]
fn query_where_null_ne() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/mixed_types.json",
            ".[] | where note != null",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Diana"));
    assert!(!stdout.contains("Alice"));
    assert!(!stdout.contains("Charlie"));
}

// --- where: int/float 크로스 타입 비교 ---

#[test]
fn query_where_int_vs_float_literal() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/mixed_types.json",
            ".[] | where value > 99.5",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice")); // value=100
    assert!(stdout.contains("Bob")); // value=200.5
    assert!(!stdout.contains("Charlie")); // value=0
    assert!(!stdout.contains("Diana")); // value=-50
}

// --- where + and/or 복합 조건 ---

#[test]
fn query_where_complex_and_or() {
    // age > 30 or (city == "Busan" and role == "designer")
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 30 or city == \"Busan\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Charlie")); // age=35
    assert!(stdout.contains("Eve")); // age=32
    assert!(stdout.contains("Bob")); // Busan
    assert!(!stdout.contains("Diana")); // age=28, Incheon
}

// --- sort 기본 (오름차순) ---

#[test]
fn query_sort_ascending() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | sort name"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Alphabetical: Alice, Bob, Charlie, Diana, Eve
    let alice = stdout.find("Alice").unwrap();
    let bob = stdout.find("Bob").unwrap();
    let charlie = stdout.find("Charlie").unwrap();
    let diana = stdout.find("Diana").unwrap();
    let eve = stdout.find("Eve").unwrap();
    assert!(alice < bob);
    assert!(bob < charlie);
    assert!(charlie < diana);
    assert!(diana < eve);
}

#[test]
fn query_sort_numeric_ascending() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | sort score"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // scores: Charlie(78), Alice(85), Eve(88), Bob(92), Diana(95)
    let charlie = stdout.find("Charlie").unwrap();
    let alice = stdout.find("Alice").unwrap();
    let eve = stdout.find("Eve").unwrap();
    let bob = stdout.find("Bob").unwrap();
    let diana = stdout.find("Diana").unwrap();
    assert!(charlie < alice);
    assert!(alice < eve);
    assert!(eve < bob);
    assert!(bob < diana);
}

// --- select on single object ---

#[test]
fn query_select_single_object() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[0] | select name, age",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("30"));
    assert!(!stdout.contains("city"));
    assert!(!stdout.contains("role"));
    assert!(!stdout.contains("score"));
}

// --- limit larger than array ---

#[test]
fn query_limit_exceeds_array_length() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | limit 100"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // All 5 employees should be present
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Diana"));
    assert!(stdout.contains("Eve"));
}

// --- pipeline: select + sort (without where) ---

#[test]
fn query_pipeline_select_sort() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name, score | sort score desc",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Diana(95), Bob(92), Eve(88), Alice(85), Charlie(78)
    let diana = stdout.find("Diana").unwrap();
    let bob = stdout.find("Bob").unwrap();
    let eve = stdout.find("Eve").unwrap();
    let alice = stdout.find("Alice").unwrap();
    let charlie = stdout.find("Charlie").unwrap();
    assert!(diana < bob);
    assert!(bob < eve);
    assert!(eve < alice);
    assert!(alice < charlie);
    // select should exclude city, role, age
    assert!(!stdout.contains("city"));
    assert!(!stdout.contains("role"));
}

// --- pipeline: select + limit ---

#[test]
fn query_pipeline_select_limit() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name | limit 2",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));
    assert!(!stdout.contains("Charlie"));
}

// ============================================================
// stats 서브커맨드 통합 테스트
// ============================================================

#[test]
fn stats_employees_json() {
    dkit()
        .args(&["stats", "tests/fixtures/employees.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 5"))
        .stdout(predicate::str::contains("columns: 5"))
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("city"))
        .stdout(predicate::str::contains("role"))
        .stdout(predicate::str::contains("score"));
}

#[test]
fn stats_employees_score_column() {
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/employees.json",
            "--column",
            "score",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("type: numeric"))
        .stdout(predicate::str::contains("count: 5"))
        .stdout(predicate::str::contains("min: 78"))
        .stdout(predicate::str::contains("max: 95"));
}

#[test]
fn stats_employees_city_column() {
    dkit()
        .args(&["stats", "tests/fixtures/employees.json", "--column", "city"])
        .assert()
        .success()
        .stdout(predicate::str::contains("type: string"))
        .stdout(predicate::str::contains("count: 5"))
        .stdout(predicate::str::contains("unique: 3"));
}

#[test]
fn stats_toml_file() {
    dkit()
        .args(&["stats", "tests/fixtures/config.toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 1"))
        .stdout(predicate::str::contains("columns:"));
}

#[test]
fn stats_yaml_file() {
    dkit()
        .args(&["stats", "tests/fixtures/config.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 1"))
        .stdout(predicate::str::contains("columns:"));
}

#[test]
fn stats_mixed_types_numeric_column() {
    // value 컬럼에 int와 float이 혼합
    dkit()
        .args(&[
            "stats",
            "tests/fixtures/mixed_types.json",
            "--column",
            "value",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("type: numeric"))
        .stdout(predicate::str::contains("count: 4"));
}

// ============================================================
// schema 서브커맨드 통합 테스트
// ============================================================

#[test]
fn schema_employees_json() {
    dkit()
        .args(&["schema", "tests/fixtures/employees.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: array[object]"))
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("age: integer"))
        .stdout(predicate::str::contains("city: string"))
        .stdout(predicate::str::contains("role: string"))
        .stdout(predicate::str::contains("score: integer"));
}

#[test]
fn schema_mixed_types() {
    dkit()
        .args(&["schema", "tests/fixtures/mixed_types.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: array[object]"))
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("active: boolean"));
}

#[test]
fn schema_empty_json() {
    dkit()
        .args(&["schema", "tests/fixtures/empty.json"])
        .assert()
        .success();
}

#[test]
fn schema_single_object() {
    dkit()
        .args(&["schema", "tests/fixtures/single.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: object"));
}

#[test]
fn schema_yaml_nested() {
    dkit()
        .args(&["schema", "tests/fixtures/config.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("root: object"))
        .stdout(predicate::str::contains("database: object"))
        .stdout(predicate::str::contains("server: object"));
}

// ============================================================
// merge 서브커맨드 통합 테스트
// ============================================================

#[test]
fn merge_three_json_files() {
    let output = dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "tests/fixtures/employees.json",
            "--to",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // users.json + users2.json + employees.json
    assert!(stdout.contains("alice@example.com")); // from users.json
    assert!(stdout.contains("Diana")); // from users2.json or employees.json
    assert!(stdout.contains("engineer")); // from employees.json
}

#[test]
fn merge_to_yaml_output() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "--to",
            "yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("name: Diana"));
}

#[test]
fn merge_to_csv_output() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "--to",
            "csv",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Diana"));
}

#[test]
fn merge_csv_to_yaml() {
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.csv",
            "tests/fixtures/users2.csv",
            "--to",
            "yaml",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Diana"));
}

#[test]
fn merge_output_file_format_detection() {
    let tmp = TempDir::new().unwrap();
    let out_path = tmp.path().join("result.yaml");

    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    // Output should be YAML (detected from .yaml extension)
    assert!(content.contains("name: Alice"));
}

#[test]
fn merge_yaml_deep_merge_overwrite() {
    let output = dkit()
        .args(&[
            "merge",
            "tests/fixtures/config.yaml",
            "tests/fixtures/config2.yaml",
            "--to",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // config2 overrides server.port from 8080 to 9090
    assert!(stdout.contains("9090"));
    // config2 adds logging section
    assert!(stdout.contains("logging"));
    // config1's database section preserved
    assert!(stdout.contains("database"));
    assert!(stdout.contains("localhost"));
}

// ============================================================
// 쿼리 결과 --to 옵션 통합 테스트
// ============================================================

#[test]
fn query_pipeline_to_csv() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city == \"Seoul\" | select name, role",
            "--to",
            "csv",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("name,role") || stdout.contains("name") && stdout.contains("role"));
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Eve"));
}

#[test]
fn query_pipeline_to_yaml() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | sort score desc | limit 3",
            "--to",
            "yaml",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Top 3: Diana(95), Bob(92), Eve(88)
    assert!(stdout.contains("Diana"));
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Eve"));
}

#[test]
fn query_pipeline_to_toml_single_object() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[0]",
            "--to",
            "toml",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("name"));
}

#[test]
fn query_pipeline_output_file_csv() {
    let tmp = TempDir::new().unwrap();
    let out_path = tmp.path().join("result.csv");

    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role == \"engineer\" | select name, score",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    // Output file is .csv so should be CSV format
    assert!(content.contains("Alice"));
    assert!(content.contains("Diana"));
    assert!(content.contains("Eve"));
}

#[test]
fn query_to_overrides_output_extension() {
    let tmp = TempDir::new().unwrap();
    let out_path = tmp.path().join("result.csv");

    // --to yaml should override .csv extension
    dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[0]",
            "--to",
            "yaml",
            "-o",
            out_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(&out_path).unwrap();
    // Should be YAML format despite .csv extension
    assert!(content.contains("name: Alice"));
}

// ============================================================
// 에지 케이스 및 에러 케이스 테스트
// ============================================================

// --- 빈 결과 처리 ---

#[test]
fn query_where_no_match_returns_empty_array() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name == \"Nobody\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "[]");
}

#[test]
fn query_empty_array_sort() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 100 | sort name",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "[]");
}

#[test]
fn query_empty_array_limit() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 100 | limit 5",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "[]");
}

// --- select 존재하지 않는 필드 ---

#[test]
fn query_select_nonexistent_field() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name, nonexistent",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // name should be present, nonexistent silently ignored
    assert!(stdout.contains("Alice"));
    assert!(!stdout.contains("nonexistent"));
}

// --- 파이프라인 에러: 비배열에 where/sort/limit ---

#[test]
fn query_where_on_object_fails() {
    dkit()
        .args([
            "query",
            "tests/fixtures/config.yaml",
            ".database | where host == \"localhost\"",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("where"));
}

#[test]
fn query_sort_on_object_fails() {
    dkit()
        .args([
            "query",
            "tests/fixtures/config.yaml",
            ".database | sort host",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("sort"));
}

#[test]
fn query_limit_on_object_fails() {
    dkit()
        .args(["query", "tests/fixtures/config.yaml", ".database | limit 1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("limit"));
}

// --- stdin으로 파이프라인 쿼리 ---

#[test]
fn query_pipeline_from_stdin() {
    let input = r#"[{"x": 3}, {"x": 1}, {"x": 2}]"#;
    let output = dkit()
        .args(["query", "-", ".[] | sort x", "--from", "json"])
        .write_stdin(input)
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let pos1 = stdout.find("1").unwrap();
    let pos2 = stdout.find("2").unwrap();
    let pos3 = stdout.find("3").unwrap();
    assert!(pos1 < pos2);
    assert!(pos2 < pos3);
}

#[test]
fn query_pipeline_where_from_stdin() {
    let input = r#"[{"name": "a", "val": 10}, {"name": "b", "val": 20}, {"name": "c", "val": 5}]"#;
    dkit()
        .args(["query", "-", ".[] | where val > 8", "--from", "json"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"a\""))
        .stdout(predicate::str::contains("\"b\""))
        .stdout(predicate::str::contains("\"c\"").not());
}

// --- merge 에러 케이스 ---

#[test]
fn merge_incompatible_format_no_error() {
    // JSON array + YAML object merge should succeed (mixed merge)
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/config.yaml",
            "--to",
            "json",
        ])
        .assert()
        .success();
}

// --- stats 에러 케이스 ---

#[test]
fn stats_column_on_non_array_stdin() {
    dkit()
        .args(&["stats", "-", "--from", "json", "--column", "name"])
        .write_stdin(r#"{"name": "Alice"}"#)
        .assert()
        .failure()
        .stderr(predicate::str::contains("--column"));
}

// --- schema with stdin CSV ---

#[test]
fn schema_stdin_csv() {
    dkit()
        .args(&["schema", "-", "--from", "csv"])
        .write_stdin("name,age\nAlice,30\nBob,25")
        .assert()
        .success()
        .stdout(predicate::str::contains("root: array[object]"))
        .stdout(predicate::str::contains("name: string"));
}

// --- view + query 조합: 깊은 경로 ---

#[test]
fn query_deep_nested_path() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.location.address.street",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Gangnam-daero"));
}

#[test]
fn query_nested_array_iterate_pipeline() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/nested.json",
            ".company.departments[] | select name, lead",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Engineering"));
    assert!(stdout.contains("Marketing"));
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));
    // teams should be excluded
    assert!(!stdout.contains("teams"));
}

// --- 쿼리 결과를 다양한 포맷으로 변환 후 올바른 형식 확인 ---

#[test]
fn query_result_json_format_valid() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 30 | select name",
            "--to",
            "json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should be valid JSON array
    assert!(stdout.trim().starts_with('['));
    assert!(stdout.trim().ends_with(']'));
}

// --- negative value 처리 ---

#[test]
fn query_where_negative_value() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/mixed_types.json",
            ".[] | where value < 0",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Diana")); // value=-50
    assert!(!stdout.contains("Alice"));
    assert!(!stdout.contains("Charlie"));
}

// --- sort + limit + to: 실전적인 "Top N" 쿼리 ---

#[test]
fn query_top_n_scores_to_csv() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | sort score desc | limit 3 | select name, score",
            "--to",
            "csv",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Top 3: Diana(95), Bob(92), Eve(88)
    assert!(stdout.contains("Diana"));
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Eve"));
    assert!(!stdout.contains("Alice"));
    assert!(!stdout.contains("Charlie"));
}

// --- merge then query (cross-command workflow) ---

#[test]
fn merge_then_query_via_file() {
    let tmp = TempDir::new().unwrap();
    let merged_path = tmp.path().join("merged.json");

    // Step 1: merge
    dkit()
        .args(&[
            "merge",
            "tests/fixtures/users.json",
            "tests/fixtures/users2.json",
            "-o",
            merged_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Step 2: query the merged file
    let output = dkit()
        .args([
            "query",
            merged_path.to_str().unwrap(),
            ".[] | sort age | select name, age",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // All 4 users sorted by age: Bob(25), Diana(28), Alice(30), Charlie(35)
    let bob = stdout.find("Bob").unwrap();
    let diana = stdout.find("Diana").unwrap();
    let alice = stdout.find("Alice").unwrap();
    let charlie = stdout.find("Charlie").unwrap();
    assert!(bob < diana);
    assert!(diana < alice);
    assert!(alice < charlie);
}

// --- convert then stats (cross-command workflow) ---

#[test]
fn convert_then_stats_via_file() {
    let tmp = TempDir::new().unwrap();
    let csv_path = tmp.path().join("users.csv");

    // Step 1: convert JSON to CSV
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/employees.json",
            "--to",
            "csv",
            "-o",
            csv_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Step 2: stats on converted CSV
    dkit()
        .args(&["stats", csv_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 5"))
        .stdout(predicate::str::contains("columns: 5"));
}
