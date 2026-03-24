use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn config_show_runs_successfully() {
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Effective configuration"));
}

#[test]
fn config_init_creates_project_file() {
    let dir = tempdir().unwrap();
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["config", "init", "--project"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created config file"));

    let config_path = dir.path().join(".dkit.toml");
    assert!(config_path.exists());

    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("dkit configuration file"));
    assert!(content.contains("[table]"));
}

#[test]
fn config_init_project_fails_if_exists() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join(".dkit.toml");
    std::fs::write(&config_path, "# existing").unwrap();

    Command::cargo_bin("dkit")
        .unwrap()
        .args(["config", "init", "--project"])
        .current_dir(dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn config_show_displays_sources() {
    Command::cargo_bin("dkit")
        .unwrap()
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("User config:"))
        .stdout(predicate::str::contains("Project config:"));
}

#[test]
fn config_show_with_project_config() {
    let dir = tempdir().unwrap();
    let config_content = r#"
default_format = "csv"
color = "never"

[table]
border_style = "rounded"
max_width = 60
"#;
    std::fs::write(dir.path().join(".dkit.toml"), config_content).unwrap();

    Command::cargo_bin("dkit")
        .unwrap()
        .args(["config", "show"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("csv"))
        .stdout(predicate::str::contains("never"))
        .stdout(predicate::str::contains("rounded"));
}
