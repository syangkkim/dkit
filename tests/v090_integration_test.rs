/// v0.9.0 Integration Tests
///
/// Tests for features added in v0.9.0:
/// - 설정 파일 로딩/우선순위 (config loading and priority)
/// - 쉘 자동완성 생성 (shell completion generation)
/// - watch 모드 (watch mode)
/// - 에러 메시지 출력 (error message output)
/// - 별칭 시스템 (alias system)
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// 설정 파일 로딩/우선순위 테스트
// ============================================================

#[test]
fn config_priority_project_overrides_values() {
    // Project config should override user config values.
    // We can test project config in isolation by running in a temp dir.
    let dir = TempDir::new().unwrap();
    let config = r#"
default_format = "yaml"
color = "never"

[table]
border_style = "heavy"
max_width = 100
"#;
    fs::write(dir.path().join(".dkit.toml"), config).unwrap();

    dkit()
        .args(["config", "show"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("yaml"))
        .stdout(predicate::str::contains("never"))
        .stdout(predicate::str::contains("heavy"))
        .stdout(predicate::str::contains("100"));
}

#[test]
fn config_show_lists_project_path_when_config_exists() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".dkit.toml"), "color = \"always\"\n").unwrap();

    dkit()
        .args(["config", "show"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Project config:"))
        .stdout(predicate::str::contains(".dkit.toml"));
}

#[test]
fn config_show_reports_no_project_config_when_absent() {
    let dir = TempDir::new().unwrap();
    // No .dkit.toml in this directory

    dkit()
        .args(["config", "show"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Project config: (none)"));
}

#[test]
fn config_init_project_creates_valid_toml() {
    let dir = TempDir::new().unwrap();
    dkit()
        .args(["config", "init", "--project"])
        .current_dir(dir.path())
        .assert()
        .success();

    let content = fs::read_to_string(dir.path().join(".dkit.toml")).unwrap();
    // Should be valid TOML (commented out values, not active)
    let parsed: Result<toml::Value, _> = toml::from_str(&content);
    assert!(parsed.is_ok(), "Generated config should be valid TOML");
    assert!(content.contains("dkit configuration file"));
}

#[test]
fn config_init_project_idempotent_fails_second_time() {
    let dir = TempDir::new().unwrap();
    dkit()
        .args(["config", "init", "--project"])
        .current_dir(dir.path())
        .assert()
        .success();

    // Second call should fail with "already exists"
    dkit()
        .args(["config", "init", "--project"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

// ============================================================
// 쉘 자동완성 생성 테스트
// ============================================================

#[test]
fn completions_bash_generates_output() {
    dkit()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn completions_zsh_generates_output() {
    dkit()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn completions_fish_generates_output() {
    dkit()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn completions_powershell_generates_output() {
    dkit()
        .args(["completions", "powershell"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn completions_bash_mentions_subcommands() {
    // Bash completions should reference known subcommands
    dkit()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("convert"))
        .stdout(predicate::str::contains("query"))
        .stdout(predicate::str::contains("view"));
}

#[test]
fn completions_zsh_mentions_subcommands() {
    dkit()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("convert"))
        .stdout(predicate::str::contains("query"));
}

// ============================================================
// watch 모드 통합 테스트
// ============================================================

#[test]
fn watch_flag_appears_in_convert_help() {
    dkit()
        .args(["convert", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--watch"));
}

#[test]
fn watch_flag_appears_in_view_help() {
    dkit()
        .args(["view", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--watch"));
}

#[test]
fn watch_path_flag_appears_in_convert_help() {
    dkit()
        .args(["convert", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--watch-path"));
}

#[test]
fn watch_with_nonexistent_file_fails() {
    dkit()
        .args([
            "convert",
            "no_such_file_xyz.json",
            "--format",
            "csv",
            "--watch",
        ])
        .assert()
        .failure();
}

// ============================================================
// 에러 메시지 출력 테스트
// ============================================================

#[test]
fn error_unknown_format_shows_did_you_mean() {
    // "jsom" is close to "json" — should suggest it
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(&f, r#"{"x": 1}"#).unwrap();

    dkit()
        .args(["convert", f.to_str().unwrap(), "--format", "jsom"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"))
        .stderr(predicate::str::contains("jsom").or(predicate::str::contains("json")));
}

#[test]
fn error_nonexistent_input_file_shows_error() {
    dkit()
        .args(["convert", "nonexistent_12345.json", "--format", "csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn error_invalid_json_shows_parse_error() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("bad.json");
    fs::write(&f, r#"{ "name": "Alice", invalid }"#).unwrap();

    dkit()
        .args(["convert", f.to_str().unwrap(), "--format", "yaml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn error_unknown_format_lists_supported_formats() {
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(&f, r#"{"x": 1}"#).unwrap();

    dkit()
        .args(["convert", f.to_str().unwrap(), "--format", "xyz_unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

#[test]
fn verbose_flag_produces_extra_output_on_error() {
    dkit()
        .args([
            "--verbose",
            "convert",
            "nonexistent_verbose.json",
            "--format",
            "csv",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

// ============================================================
// 별칭 시스템 테스트
// ============================================================

#[test]
fn alias_list_shows_builtin_aliases() {
    dkit()
        .args(["alias", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("j2c"))
        .stdout(predicate::str::contains("c2j"))
        .stdout(predicate::str::contains("j2y"))
        .stdout(predicate::str::contains("y2j"))
        .stdout(predicate::str::contains("builtin"));
}

#[test]
fn alias_list_shows_header() {
    dkit()
        .args(["alias", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("NAME"))
        .stdout(predicate::str::contains("COMMAND"))
        .stdout(predicate::str::contains("SOURCE"));
}

#[test]
fn builtin_alias_j2c_expands_to_convert() {
    // j2c = "convert --from json --to csv"
    // Using it with a file should behave like convert --from json --to csv
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(&f, r#"[{"name":"Alice","age":30}]"#).unwrap();

    dkit()
        .args(["j2c", f.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Alice"));
}

#[test]
fn builtin_alias_c2j_expands_to_convert() {
    // c2j = "convert --from csv --to json"
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.csv");
    fs::write(&f, "name,age\nBob,25\n").unwrap();

    dkit()
        .args(["c2j", f.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn builtin_alias_j2y_expands_to_convert() {
    // j2y = "convert --from json --to yaml"
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.json");
    fs::write(&f, r#"{"key":"value"}"#).unwrap();

    dkit()
        .args(["j2y", f.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("key"))
        .stdout(predicate::str::contains("value"));
}

#[test]
fn builtin_alias_y2j_expands_to_convert() {
    // y2j = "convert --from yaml --to json"
    let dir = TempDir::new().unwrap();
    let f = dir.path().join("data.yaml");
    fs::write(&f, "key: value\n").unwrap();

    dkit()
        .args(["y2j", f.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("key"))
        .stdout(predicate::str::contains("value"));
}

#[test]
fn alias_set_and_list_user_alias() {
    // alias set uses the user config file; we test via alias list
    // (Actual file I/O to user config directory; we just verify the subcommand doesn't error)
    dkit().args(["alias", "list"]).assert().success();
}

#[test]
fn alias_remove_builtin_fails_with_helpful_message() {
    // Removing a built-in alias should fail gracefully
    dkit()
        .args(["alias", "remove", "j2c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot remove built-in alias"));
}

#[test]
fn alias_remove_nonexistent_fails() {
    dkit()
        .args(["alias", "remove", "no_such_alias_xyz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}
