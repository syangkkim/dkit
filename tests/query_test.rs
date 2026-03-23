use assert_cmd::Command;
use predicates;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- 필드 접근 ---

#[test]
fn query_field_access() {
    dkit()
        .args(["query", "tests/fixtures/config.toml", ".database.host"])
        .assert()
        .success()
        .stdout("\"localhost\"\n");
}

#[test]
fn query_nested_field() {
    dkit()
        .args(["query", "tests/fixtures/config.toml", ".database.port"])
        .assert()
        .success()
        .stdout("5432\n");
}

#[test]
fn query_root() {
    let output = dkit()
        .args(["query", "tests/fixtures/config.toml", "."])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("database"));
    assert!(stdout.contains("localhost"));
}

// --- 배열 인덱싱 ---

#[test]
fn query_array_index() {
    let output = dkit()
        .args(["query", "tests/fixtures/users.json", ".[0].name"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
}

#[test]
fn query_array_negative_index() {
    let output = dkit()
        .args(["query", "tests/fixtures/users.json", ".[-1].name"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Bob"));
}

// --- 배열 이터레이션 ---

#[test]
fn query_array_iterate() {
    let output = dkit()
        .args(["query", "tests/fixtures/users.json", ".[].name"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Bob"));
}

// --- YAML 입력 ---

#[test]
fn query_yaml_field() {
    let output = dkit()
        .args(["query", "tests/fixtures/config.yaml", ".database.host"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("localhost"));
}

// --- --to 옵션 ---

#[test]
fn query_with_to_yaml() {
    let output = dkit()
        .args(["query", "tests/fixtures/users.json", ".[0]", "--to", "yaml"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("name: Alice"));
}

// --- stdin 입력 ---

#[test]
fn query_stdin() {
    dkit()
        .args(["query", "-", ".name", "--from", "json"])
        .write_stdin("{\"name\": \"test\"}")
        .assert()
        .success()
        .stdout("\"test\"\n");
}

#[test]
fn query_stdin_without_from() {
    // 콘텐츠 스니핑으로 JSON 포맷 자동 감지
    dkit()
        .args(["query", "-", ".name"])
        .write_stdin("{\"name\": \"test\"}")
        .assert()
        .success()
        .stdout(predicates::str::contains("test"));
}

// --- 에러 케이스 ---

#[test]
fn query_nonexistent_file() {
    dkit()
        .args(["query", "nonexistent.json", ".name"])
        .assert()
        .failure();
}

#[test]
fn query_invalid_query() {
    dkit()
        .args(["query", "tests/fixtures/users.json", "invalid"])
        .assert()
        .failure();
}

#[test]
fn query_path_not_found() {
    dkit()
        .args(["query", "tests/fixtures/users.json", ".nonexistent"])
        .assert()
        .failure();
}

#[test]
fn query_index_out_of_bounds() {
    dkit()
        .args(["query", "tests/fixtures/users.json", ".[99]"])
        .assert()
        .failure();
}

// --- -o 출력 파일 옵션 ---

#[test]
fn query_output_to_json_file() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("result.json");
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[0].name",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout("");
    let content = std::fs::read_to_string(&out).unwrap();
    assert_eq!(content.trim(), "\"Alice\"");
}

#[test]
fn query_output_to_yaml_file() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("result.yaml");
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[0]",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("name: Alice"));
}

#[test]
fn query_output_to_csv_file() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("result.csv");
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("Alice"));
    assert!(content.contains("Bob"));
}

#[test]
fn query_output_to_toml_file() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("result.toml");
    dkit()
        .args([
            "query",
            "tests/fixtures/config.yaml",
            ".database",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("host"));
    assert!(content.contains("localhost"));
}

#[test]
fn query_to_flag_overrides_file_extension() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("result.yaml");
    // --to json should override .yaml extension
    dkit()
        .args([
            "query",
            "tests/fixtures/users.json",
            ".[0]",
            "--to",
            "json",
            "-o",
            out.to_str().unwrap(),
        ])
        .assert()
        .success();
    let content = std::fs::read_to_string(&out).unwrap();
    // JSON output should have braces, not YAML
    assert!(content.contains("{"));
    assert!(content.contains("\"name\""));
}

// --- --to 포맷 변환 ---

#[test]
fn query_with_to_toml() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/config.yaml",
            ".database",
            "--to",
            "toml",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("host"));
    assert!(stdout.contains("localhost"));
}

// --- 파이프라인 체이닝 ---

#[test]
fn query_pipeline_where() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 30",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Eve"));
    assert!(!stdout.contains("Bob"));
    assert!(!stdout.contains("Diana"));
}

#[test]
fn query_pipeline_where_select() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city == \"Seoul\" | select name, role",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Eve"));
    assert!(!stdout.contains("Busan"));
    assert!(!stdout.contains("Incheon"));
    // select should exclude age, city, score
    assert!(!stdout.contains("score"));
}

#[test]
fn query_pipeline_where_sort() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role == \"engineer\" | sort score desc",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Diana(95) > Eve(88) > Alice(85)
    let diana_pos = stdout.find("Diana").unwrap();
    let eve_pos = stdout.find("Eve").unwrap();
    let alice_pos = stdout.find("Alice").unwrap();
    assert!(diana_pos < eve_pos);
    assert!(eve_pos < alice_pos);
}

#[test]
fn query_pipeline_where_sort_limit() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role == \"engineer\" | sort score desc | limit 2",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Diana"));
    assert!(stdout.contains("Eve"));
    assert!(!stdout.contains("Alice")); // 3rd engineer, excluded by limit
}

#[test]
fn query_pipeline_where_select_sort() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 25 | select name, age | sort name",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Sorted by name: Alice, Charlie, Diana, Eve (Bob age=25 excluded)
    let alice_pos = stdout.find("Alice").unwrap();
    let charlie_pos = stdout.find("Charlie").unwrap();
    let diana_pos = stdout.find("Diana").unwrap();
    let eve_pos = stdout.find("Eve").unwrap();
    assert!(alice_pos < charlie_pos);
    assert!(charlie_pos < diana_pos);
    assert!(diana_pos < eve_pos);
    assert!(!stdout.contains("Bob"));
}

#[test]
fn query_pipeline_full_chain() {
    // where + select + sort + limit
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city == \"Seoul\" | select name, score | sort score desc | limit 2",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Seoul: Alice(85), Charlie(78), Eve(88) → sort desc: Eve(88), Alice(85), Charlie(78) → limit 2
    assert!(stdout.contains("Eve"));
    assert!(stdout.contains("Alice"));
    assert!(!stdout.contains("Charlie"));
}

#[test]
fn query_pipeline_sort_limit() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | sort age | limit 3",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Youngest 3: Bob(25), Diana(28), Alice(30)
    assert!(stdout.contains("Bob"));
    assert!(stdout.contains("Diana"));
    assert!(stdout.contains("Alice"));
    assert!(!stdout.contains("Charlie")); // 35
    assert!(!stdout.contains("Eve")); // 32
}

#[test]
fn query_pipeline_select_only() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | select name, city",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("name"));
    assert!(stdout.contains("city"));
    assert!(!stdout.contains("age"));
    assert!(!stdout.contains("score"));
    assert!(!stdout.contains("role"));
}

#[test]
fn query_pipeline_limit_only() {
    let output = dkit()
        .args(["query", "tests/fixtures/employees.json", ".[] | limit 1"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(!stdout.contains("Bob"));
}

#[test]
fn query_pipeline_where_and_condition() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where city == \"Seoul\" and role == \"engineer\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(stdout.contains("Eve"));
    assert!(!stdout.contains("Charlie")); // manager
    assert!(!stdout.contains("Bob")); // Busan
}

#[test]
fn query_pipeline_where_or_condition() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where role == \"manager\" or role == \"designer\"",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Bob"));
    assert!(!stdout.contains("Alice"));
}

#[test]
fn query_pipeline_where_string_op() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where name starts_with \"A\" | select name",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Alice"));
    assert!(!stdout.contains("Bob"));
}

#[test]
fn query_pipeline_with_output_format() {
    // Pipeline + --to yaml
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 30 | select name",
            "--to",
            "yaml",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Charlie"));
    assert!(stdout.contains("Eve"));
}

#[test]
fn query_pipeline_empty_result() {
    let output = dkit()
        .args([
            "query",
            "tests/fixtures/employees.json",
            ".[] | where age > 100",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "[]");
}

#[test]
fn query_pipeline_with_nested_path() {
    // Test pipeline with object having nested structure
    let output = dkit()
        .args(["query", "tests/fixtures/nested.json", "."])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn query_with_to_csv() {
    let output = dkit()
        .args(["query", "tests/fixtures/users.json", ".", "--to", "csv"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("name"));
    assert!(stdout.contains("Alice"));
}
