use assert_cmd::Command;
use predicates::prelude::*;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- 기본 view ---

#[test]
fn view_json_array() {
    dkit()
        .args(&["view", "tests/fixtures/users.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn view_csv() {
    dkit()
        .args(&["view", "tests/fixtures/users.csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn view_yaml() {
    dkit()
        .args(&["view", "tests/fixtures/config.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key"))
        .stdout(predicate::str::contains("value"))
        .stdout(predicate::str::contains("database"));
}

#[test]
fn view_toml() {
    dkit()
        .args(&["view", "tests/fixtures/config.toml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("database"));
}

// --- --path 옵션 ---

#[test]
fn view_with_path() {
    dkit()
        .args(&["view", "tests/fixtures/config.yaml", "--path", ".database"])
        .assert()
        .success()
        .stdout(predicate::str::contains("host"))
        .stdout(predicate::str::contains("localhost"))
        .stdout(predicate::str::contains("port"))
        .stdout(predicate::str::contains("5432"));
}

#[test]
fn view_with_path_array_index() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--path", ".[0]"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

// --- --limit 옵션 ---

#[test]
fn view_with_limit() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "-n", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("1 more rows"));
}

// --- --columns 옵션 ---

#[test]
fn view_with_columns() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/users.json",
            "--columns",
            "name,email",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("email"))
        .stdout(predicate::str::contains("Alice"));
}

// --- 에러 케이스 ---

#[test]
fn view_nonexistent_file() {
    dkit()
        .args(&["view", "nonexistent.json"])
        .assert()
        .failure();
}

#[test]
fn view_invalid_path() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/users.json",
            "--path",
            ".nonexistent",
        ])
        .assert()
        .failure();
}

#[test]
fn view_stdin_without_from() {
    // 콘텐츠 스니핑으로 JSON 포맷 자동 감지
    dkit()
        .args(&["view", "-"])
        .write_stdin("[{\"a\": 1}]")
        .assert()
        .success()
        .stdout(predicate::str::contains("a"));
}

#[test]
fn view_stdin_with_from() {
    dkit()
        .args(&["view", "-", "--from", "json"])
        .write_stdin("[{\"name\": \"Test\", \"value\": 42}]")
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Test"));
}

// --- --hide-header 옵션 ---

#[test]
fn view_hide_header() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--hide-header"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("name").not());
}

// --- --row-numbers 옵션 ---

#[test]
fn view_row_numbers() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--row-numbers"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#"))
        .stdout(predicate::str::contains(" 1 "));
}

// --- --border 옵션 ---

#[test]
fn view_border_none() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--border", "none"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("+").not())
        .stdout(predicate::str::contains("|").not());
}

#[test]
fn view_border_rounded() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--border", "rounded"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn view_border_heavy() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--border", "heavy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- --max-width 옵션 ---

#[test]
fn view_max_width() {
    dkit()
        .args(&["view", "-", "--from", "json", "--max-width", "5"])
        .write_stdin("[{\"name\": \"VeryLongNameThatShouldBeTruncated\"}]")
        .assert()
        .success()
        .stdout(predicate::str::contains("VeryLongNameThatShouldBeTruncated").not());
}

// --- --color 옵션 ---

#[test]
fn view_color() {
    dkit()
        .args(&["view", "tests/fixtures/users.json", "--color"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- 옵션 조합 ---

#[test]
fn view_combined_options() {
    dkit()
        .args(&[
            "view",
            "tests/fixtures/users.json",
            "--row-numbers",
            "--border",
            "rounded",
            "-n",
            "1",
            "--columns",
            "name",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("#"))
        .stdout(predicate::str::contains("Alice"));
}
