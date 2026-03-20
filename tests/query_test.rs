use assert_cmd::Command;

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
    dkit()
        .args(["query", "-", ".name"])
        .write_stdin("{\"name\": \"test\"}")
        .assert()
        .failure();
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
