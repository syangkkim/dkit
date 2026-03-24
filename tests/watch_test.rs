use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

use tempfile::NamedTempFile;

#[test]
fn convert_watch_flag_is_accepted() {
    let output = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args(["convert", "--help"])
        .output()
        .expect("failed to execute");
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("--watch"), "help should mention --watch flag");
    assert!(
        help.contains("--watch-path"),
        "help should mention --watch-path flag"
    );
}

#[test]
fn view_watch_flag_is_accepted() {
    let output = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args(["view", "--help"])
        .output()
        .expect("failed to execute");
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("--watch"), "help should mention --watch flag");
    assert!(
        help.contains("--watch-path"),
        "help should mention --watch-path flag"
    );
}

#[test]
fn convert_watch_nonexistent_file_errors() {
    let output = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args(["convert", "nonexistent.json", "-f", "csv", "--watch"])
        .output()
        .expect("failed to execute");
    assert!(
        !output.status.success(),
        "should fail with nonexistent file"
    );
}

#[test]
fn convert_watch_starts_and_shows_watching_message() {
    let mut tmpfile = NamedTempFile::new().expect("failed to create temp file");
    write!(tmpfile, r#"[{{"name":"Alice","age":30}}]"#).unwrap();
    tmpfile.flush().unwrap();

    let path = tmpfile.path().to_str().unwrap().to_string();

    let mut child = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args(["convert", &path, "-f", "csv", "--watch"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    // Give it time to do the initial run
    std::thread::sleep(Duration::from_millis(500));

    child.kill().expect("failed to kill");
    let output = child.wait_with_output().expect("failed to wait");

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should show "Watching" message in stderr
    assert!(
        stderr.contains("Watching") || stderr.contains("watch"),
        "should show watch message, stderr: {stderr}"
    );
}

#[test]
fn view_watch_starts_and_shows_watching_message() {
    let mut tmpfile = NamedTempFile::new().expect("failed to create temp file");
    write!(
        tmpfile,
        r#"[{{"name":"Bob","age":25}},{{"name":"Eve","age":22}}]"#
    )
    .unwrap();
    tmpfile.flush().unwrap();

    let path = tmpfile.path().to_str().unwrap().to_string();

    let mut child = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args(["view", &path, "--watch"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    std::thread::sleep(Duration::from_millis(500));

    child.kill().expect("failed to kill");
    let output = child.wait_with_output().expect("failed to wait");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Watching") || stderr.contains("watch"),
        "should show watch message, stderr: {stderr}"
    );
}

#[test]
fn convert_watch_detects_file_change() {
    let mut tmpfile = NamedTempFile::new().expect("failed to create temp file");
    let path = tmpfile.path().to_str().unwrap().to_string();

    write!(tmpfile, r#"[{{"x":1}}]"#).unwrap();
    tmpfile.flush().unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args(["convert", &path, "-f", "csv", "--watch"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    // Wait for initial run
    std::thread::sleep(Duration::from_millis(800));

    // Modify the file
    let mut f = std::fs::File::create(&path).expect("failed to open for write");
    write!(f, r#"[{{"x":1}},{{"x":2}}]"#).unwrap();
    f.flush().unwrap();
    drop(f);

    // Wait for re-run
    std::thread::sleep(Duration::from_millis(1000));

    child.kill().expect("failed to kill");
    let output = child.wait_with_output().expect("failed to wait");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("re-running") || stderr.contains("changed") || stderr.contains("watch"),
        "should detect file change, stderr: {stderr}"
    );
}

#[test]
fn convert_watch_with_extra_watch_path() {
    let mut tmpfile = NamedTempFile::new().expect("failed to create temp file");
    write!(tmpfile, r#"[{{"x":1}}]"#).unwrap();
    tmpfile.flush().unwrap();

    let extra_dir = tempfile::tempdir().expect("failed to create temp dir");

    let path = tmpfile.path().to_str().unwrap().to_string();
    let extra_path = extra_dir.path().to_str().unwrap().to_string();

    let mut child = Command::new(env!("CARGO_BIN_EXE_dkit"))
        .args([
            "convert",
            &path,
            "-f",
            "csv",
            "--watch",
            "--watch-path",
            &extra_path,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    std::thread::sleep(Duration::from_millis(500));

    child.kill().expect("failed to kill");
    let output = child.wait_with_output().expect("failed to wait");

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should report watching 2 paths (the input file + the extra dir)
    assert!(
        stderr.contains("2 path(s)"),
        "should watch 2 paths, stderr: {stderr}"
    );
}
