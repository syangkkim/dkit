use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// --- UTF-8 BOM ---

#[test]
fn convert_utf8_bom_json() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bom.json");
    // UTF-8 BOM + JSON content
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(b"{\"name\": \"Alice\", \"age\": 30}");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["convert", path.to_str().unwrap(), "--format", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("age: 30"));
}

#[test]
fn view_utf8_bom_csv() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bom.csv");
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(b"name,age\nAlice,30\nBob,25\n");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["view", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

// --- UTF-16LE BOM ---

#[test]
fn convert_utf16le_bom_json() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("utf16le.json");
    // UTF-16LE BOM + JSON content
    let content = "{\"name\": \"Alice\"}";
    let mut bytes = vec![0xFF, 0xFE]; // UTF-16LE BOM
    for ch in content.encode_utf16() {
        bytes.push((ch & 0xFF) as u8);
        bytes.push((ch >> 8) as u8);
    }
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["convert", path.to_str().unwrap(), "--format", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Alice"));
}

// --- UTF-16BE BOM ---

#[test]
fn convert_utf16be_bom_json() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("utf16be.json");
    let content = "{\"city\": \"Seoul\"}";
    let mut bytes = vec![0xFE, 0xFF]; // UTF-16BE BOM
    for ch in content.encode_utf16() {
        bytes.push((ch >> 8) as u8);
        bytes.push((ch & 0xFF) as u8);
    }
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["convert", path.to_str().unwrap(), "--format", "yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("city: Seoul"));
}

// --- EUC-KR encoding ---

#[test]
fn convert_euc_kr_csv_with_encoding_option() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("korean.csv");

    // "이름,나이\n홍길동,30\n" in EUC-KR
    let header_name: &[u8] = &[0xC0, 0xCC, 0xB8, 0xA7]; // "이름" in EUC-KR
    let header_age: &[u8] = &[0xB3, 0xAA, 0xC0, 0xCC]; // "나이" in EUC-KR
    let name_hong: &[u8] = &[0xC8, 0xAB, 0xB1, 0xE6, 0xB5, 0xBF]; // "홍길동" in EUC-KR

    let mut bytes = Vec::new();
    bytes.extend_from_slice(header_name);
    bytes.push(b',');
    bytes.extend_from_slice(header_age);
    bytes.push(b'\n');
    bytes.extend_from_slice(name_hong);
    bytes.extend_from_slice(b",30\n");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&[
            "convert",
            path.to_str().unwrap(),
            "--format",
            "json",
            "--encoding",
            "euc-kr",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("홍길동"))
        .stdout(predicate::str::contains("이름"))
        .stdout(predicate::str::contains("나이"));
}

// --- Shift-JIS encoding ---

#[test]
fn convert_shift_jis_csv_with_encoding_option() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("japanese.csv");

    // "名前,年齢\n太郎,25\n" in Shift-JIS
    let header_name: &[u8] = &[0x96, 0xBC, 0x91, 0x4F]; // "名前" in Shift-JIS
    let header_age: &[u8] = &[0x94, 0x4E, 0x97, 0xEE]; // "年齢" in Shift-JIS
    let name_taro: &[u8] = &[0x91, 0xBE, 0x98, 0x59]; // "太郎" in Shift-JIS

    let mut bytes = Vec::new();
    bytes.extend_from_slice(header_name);
    bytes.push(b',');
    bytes.extend_from_slice(header_age);
    bytes.push(b'\n');
    bytes.extend_from_slice(name_taro);
    bytes.extend_from_slice(b",25\n");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&[
            "convert",
            path.to_str().unwrap(),
            "--format",
            "json",
            "--encoding",
            "shift_jis",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("名前"))
        .stdout(predicate::str::contains("太郎"));
}

// --- Latin1 encoding ---

#[test]
fn convert_latin1_csv_with_encoding_option() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("latin1.csv");

    // "name,city\nJosé,München\n" in Latin1
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"name,city\n");
    bytes.extend_from_slice(b"Jos");
    bytes.push(0xE9); // é in Latin1
    bytes.push(b',');
    bytes.extend_from_slice(b"M");
    bytes.push(0xFC); // ü in Latin1
    bytes.extend_from_slice(b"nchen\n");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&[
            "convert",
            path.to_str().unwrap(),
            "--format",
            "json",
            "--encoding",
            "latin1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("José"))
        .stdout(predicate::str::contains("München"));
}

// --- --detect-encoding ---

#[test]
fn convert_with_detect_encoding() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("detect.csv");

    // Latin1 encoded CSV
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"name,city\n");
    bytes.extend_from_slice(b"Jos");
    bytes.push(0xE9); // é in Latin1
    bytes.push(b',');
    bytes.extend_from_slice(b"M");
    bytes.push(0xFC); // ü in Latin1
    bytes.extend_from_slice(b"nchen\n");
    fs::write(&path, &bytes).unwrap();

    // With --detect-encoding, it should detect the encoding and convert
    dkit()
        .args(&[
            "convert",
            path.to_str().unwrap(),
            "--format",
            "json",
            "--detect-encoding",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("city"));
}

// --- Error cases ---

#[test]
fn convert_unknown_encoding_error() {
    dkit()
        .args(&[
            "convert",
            "tests/fixtures/users.json",
            "--format",
            "csv",
            "--encoding",
            "invalid-encoding-xyz",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown encoding"));
}

#[test]
fn convert_non_utf8_without_encoding_option_error() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("nonutf8.csv");

    // Invalid UTF-8 bytes
    let bytes: &[u8] = &[0xC7, 0xD1, 0xB1, 0xDB, 0x0A]; // EUC-KR "한글\n"
    fs::write(&path, bytes).unwrap();

    dkit()
        .args(&["convert", path.to_str().unwrap(), "--format", "json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("UTF-8"));
}

// --- Encoding with different subcommands ---

#[test]
fn view_euc_kr_with_encoding() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("korean.csv");

    // Simple EUC-KR CSV
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"name,value\n");
    // "test" in ASCII (compatible with EUC-KR)
    bytes.extend_from_slice(b"test,100\n");
    // Add Korean: 홍길동
    bytes.extend_from_slice(&[0xC8, 0xAB, 0xB1, 0xE6, 0xB5, 0xBF]);
    bytes.extend_from_slice(b",200\n");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["view", path.to_str().unwrap(), "--encoding", "euc-kr"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("홍길동"));
}

#[test]
fn stats_euc_kr_with_encoding() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("korean.csv");

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"name,value\n");
    bytes.extend_from_slice(b"test,100\n");
    bytes.extend_from_slice(b"data,200\n");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["stats", path.to_str().unwrap(), "--encoding", "euc-kr"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rows: 2"));
}

#[test]
fn schema_utf8_bom_json() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bom.json");
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(b"{\"name\": \"Alice\", \"age\": 30}");
    fs::write(&path, &bytes).unwrap();

    dkit()
        .args(&["schema", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: string"))
        .stdout(predicate::str::contains("age: integer"));
}

#[test]
fn diff_utf8_bom_files() {
    let dir = TempDir::new().unwrap();
    let path1 = dir.path().join("a.json");
    let path2 = dir.path().join("b.json");

    let mut bytes1 = vec![0xEF, 0xBB, 0xBF];
    bytes1.extend_from_slice(b"{\"name\": \"Alice\"}");
    fs::write(&path1, &bytes1).unwrap();

    let mut bytes2 = vec![0xEF, 0xBB, 0xBF];
    bytes2.extend_from_slice(b"{\"name\": \"Alice\"}");
    fs::write(&path2, &bytes2).unwrap();

    dkit()
        .args(&["diff", path1.to_str().unwrap(), path2.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("No differences found"));
}

// --- BOM priority over --encoding ---

#[test]
fn bom_takes_priority_over_encoding_option() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("bom_priority.json");
    // UTF-8 BOM + valid UTF-8 JSON content
    let mut bytes = vec![0xEF, 0xBB, 0xBF];
    bytes.extend_from_slice(b"{\"key\": \"value\"}");
    fs::write(&path, &bytes).unwrap();

    // Even if --encoding latin1 is specified, BOM should be detected first
    dkit()
        .args(&[
            "convert",
            path.to_str().unwrap(),
            "--format",
            "yaml",
            "--encoding",
            "latin1",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("key: value"));
}

// --- Merge with encoding ---

#[test]
fn merge_utf8_bom_files() {
    let dir = TempDir::new().unwrap();
    let path1 = dir.path().join("a.json");
    let path2 = dir.path().join("b.json");

    let mut bytes1 = vec![0xEF, 0xBB, 0xBF];
    bytes1.extend_from_slice(b"[{\"name\": \"Alice\"}]");
    fs::write(&path1, &bytes1).unwrap();

    let mut bytes2 = vec![0xEF, 0xBB, 0xBF];
    bytes2.extend_from_slice(b"[{\"name\": \"Bob\"}]");
    fs::write(&path2, &bytes2).unwrap();

    dkit()
        .args(&[
            "merge",
            path1.to_str().unwrap(),
            path2.to_str().unwrap(),
            "--format",
            "json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}
