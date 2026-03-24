use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- Multiple explicit files ---

#[test]
fn batch_convert_multiple_files() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/users.csv",
            "--format",
            "yaml",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "2 succeeded, 0 failed out of 2 files",
        ));

    assert!(outdir.path().join("users.yaml").exists());
}

// --- Glob pattern ---

#[test]
fn batch_convert_glob_pattern() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/*.json",
            "--format",
            "yaml",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("succeeded"));

    // At least users.json should be converted
    assert!(outdir.path().join("users.yaml").exists());
}

// --- Directory input ---

#[test]
fn batch_convert_directory_input() {
    // Create a temp dir with some json files
    let input_dir = TempDir::new().unwrap();
    let outdir = TempDir::new().unwrap();

    fs::write(input_dir.path().join("a.json"), r#"[{"x": 1}]"#).unwrap();
    fs::write(input_dir.path().join("b.json"), r#"[{"y": 2}]"#).unwrap();
    // Non-supported file should be ignored
    fs::write(input_dir.path().join("readme.txt"), "hello").unwrap();

    dkit()
        .args(&[
            "convert",
            input_dir.path().to_str().unwrap(),
            "--format",
            "csv",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "2 succeeded, 0 failed out of 2 files",
        ));

    assert!(outdir.path().join("a.csv").exists());
    assert!(outdir.path().join("b.csv").exists());
}

// --- Rename pattern ---

#[test]
fn batch_convert_rename_pattern() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/users.csv",
            "--format",
            "yaml",
            "--outdir",
            outdir.path().to_str().unwrap(),
            "--rename",
            "{name}.converted.{ext}",
        ])
        .assert()
        .success();

    assert!(outdir.path().join("users.converted.yaml").exists());
}

// --- Continue on error ---

#[test]
fn batch_convert_continue_on_error() {
    let input_dir = TempDir::new().unwrap();
    let outdir = TempDir::new().unwrap();

    // Valid file
    fs::write(input_dir.path().join("good.json"), r#"[{"a": 1}]"#).unwrap();
    // Invalid file
    fs::write(
        input_dir.path().join("bad.json"),
        "this is not valid json {{{",
    )
    .unwrap();

    dkit()
        .args(&[
            "convert",
            input_dir.path().to_str().unwrap(),
            "--format",
            "csv",
            "--outdir",
            outdir.path().to_str().unwrap(),
            "--continue-on-error",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("1 succeeded"))
        .stderr(predicate::str::contains("1 failed"))
        .stderr(predicate::str::contains("Failed files:"));

    assert!(outdir.path().join("good.csv").exists());
    assert!(!outdir.path().join("bad.csv").exists());
}

// --- Error without --continue-on-error stops immediately ---

#[test]
fn batch_convert_stops_on_error_by_default() {
    let input_dir = TempDir::new().unwrap();
    let outdir = TempDir::new().unwrap();

    fs::write(input_dir.path().join("bad.json"), "not json").unwrap();
    fs::write(input_dir.path().join("good.json"), r#"[{"a": 1}]"#).unwrap();

    dkit()
        .args(&[
            "convert",
            input_dir.path().to_str().unwrap(),
            "--format",
            "csv",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--continue-on-error"));
}

// --- Multiple files without --outdir should fail ---

#[test]
fn batch_convert_requires_outdir() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/users.csv",
            "--format",
            "yaml",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--outdir is required"));
}

// --- Single file still works normally ---

#[test]
fn single_file_convert_still_works() {
    dkit()
        .args(&["convert", "tests/fixtures/users.json", "--format", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"));
}

// --- Glob pattern with no matches ---

#[test]
fn glob_no_matches() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/*.nonexistent",
            "--format",
            "csv",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No files matched pattern"));
}

// --- Empty directory ---

#[test]
fn batch_convert_empty_directory() {
    let input_dir = TempDir::new().unwrap();
    let outdir = TempDir::new().unwrap();

    dkit()
        .args(&[
            "convert",
            input_dir.path().to_str().unwrap(),
            "--format",
            "csv",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No supported files found"));
}

// --- Progress output ---

#[test]
fn batch_convert_shows_progress() {
    let outdir = TempDir::new().unwrap();
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "tests/fixtures/users.csv",
            "--format",
            "yaml",
            "--outdir",
            outdir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("(1/2)"))
        .stderr(predicate::str::contains("(2/2)"))
        .stderr(predicate::str::contains("ok"));
}
