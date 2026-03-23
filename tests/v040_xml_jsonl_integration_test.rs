use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// JSONL 포맷 통합 테스트
// ============================================================

mod jsonl_format {
    use super::*;

    // --- convert: JSONL → other formats ---

    #[test]
    fn convert_jsonl_to_json() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_jsonl_to_csv() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "csv"])
            .assert()
            .success()
            .stdout(predicate::str::contains("name"))
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_jsonl_to_yaml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("name: Alice"))
            .stdout(predicate::str::contains("age: 30"));
    }

    #[test]
    fn convert_jsonl_to_toml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "toml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn convert_jsonl_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Alice</name>"))
            .stdout(predicate::str::contains("<name>Bob</name>"));
    }

    // --- convert: other formats → JSONL ---

    #[test]
    fn convert_json_to_jsonl() {
        dkit()
            .args(&["convert", "tests/fixtures/users.json", "--to", "jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_csv_to_jsonl() {
        dkit()
            .args(&["convert", "tests/fixtures/users.csv", "--to", "jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_yaml_to_jsonl() {
        // config.yaml is an object, so JSONL writes a single line
        dkit()
            .args(&["convert", "tests/fixtures/config.yaml", "--to", "jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn convert_xml_to_jsonl() {
        dkit()
            .args(&["convert", "tests/fixtures/users.xml", "--to", "jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("user"));
    }

    // --- roundtrip ---

    #[test]
    fn convert_jsonl_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let jsonl_path = tmp.path().join("roundtrip.jsonl");

        // JSON → JSONL
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "jsonl",
                "-o",
                jsonl_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // JSONL → JSON
        dkit()
            .args(&["convert", jsonl_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    // --- stdin with --from jsonl ---

    #[test]
    fn convert_stdin_jsonl_to_json() {
        let input = "{\"name\":\"Alice\",\"age\":30}\n{\"name\":\"Bob\",\"age\":25}\n";
        dkit()
            .args(&["convert", "--from", "jsonl", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_stdin_jsonl_to_csv() {
        let input = "{\"name\":\"Alice\",\"age\":30}\n{\"name\":\"Bob\",\"age\":25}\n";
        dkit()
            .args(&["convert", "--from", "jsonl", "--to", "csv"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn convert_stdin_auto_detect_jsonl() {
        // Content sniffing should detect JSONL (two JSON objects on separate lines)
        let input = "{\"name\":\"Alice\"}\n{\"name\":\"Bob\"}\n";
        dkit()
            .args(&["convert", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    // --- output file ---

    #[test]
    fn convert_jsonl_to_json_output_file() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("users.json");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.jsonl",
                "--to",
                "json",
                "-o",
                out.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&out).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Bob"));
    }

    // --- batch conversion ---

    #[test]
    fn batch_convert_to_jsonl() {
        let tmp = TempDir::new().unwrap();
        let outdir = tmp.path().join("out");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "tests/fixtures/users.csv",
                "--to",
                "jsonl",
                "--outdir",
                outdir.to_str().unwrap(),
            ])
            .assert()
            .success();

        assert!(outdir.join("users.jsonl").exists());
        // Second file also exists
        let content = fs::read_to_string(outdir.join("users.jsonl")).unwrap();
        assert!(content.contains("Alice"));
    }

    // --- view ---

    #[test]
    fn view_jsonl() {
        dkit()
            .args(&["view", "tests/fixtures/users.jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn view_jsonl_with_limit() {
        dkit()
            .args(&["view", "tests/fixtures/users.jsonl", "--limit", "1"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    // --- query ---

    #[test]
    fn query_jsonl_first_element() {
        dkit()
            .args(&["query", "tests/fixtures/users.jsonl", ".[0].name"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn query_jsonl_second_element() {
        dkit()
            .args(&["query", "tests/fixtures/users.jsonl", ".[1].name"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn query_jsonl_output_to_csv() {
        dkit()
            .args(&["query", "tests/fixtures/users.jsonl", ".[]", "--to", "csv"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    // --- stats ---

    #[test]
    fn stats_jsonl() {
        dkit()
            .args(&["stats", "tests/fixtures/users.jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("rows: 2"));
    }

    #[test]
    fn stats_jsonl_column() {
        dkit()
            .args(&["stats", "tests/fixtures/users.jsonl", "--column", "age"])
            .assert()
            .success()
            .stdout(predicate::str::contains("type: numeric"));
    }

    // --- schema ---

    #[test]
    fn schema_jsonl() {
        dkit()
            .args(&["schema", "tests/fixtures/users.jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("array"))
            .stdout(predicate::str::contains("name"));
    }

    // --- diff ---

    #[test]
    fn diff_jsonl_identical() {
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.jsonl",
                "tests/fixtures/users.jsonl",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_jsonl_vs_json_same_data() {
        // users.jsonl and users.json have equivalent data
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.jsonl",
                "tests/fixtures/users.json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_jsonl_files_different() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.jsonl");
        let f2 = tmp.path().join("b.jsonl");

        fs::write(&f1, "{\"name\":\"Alice\"}\n{\"name\":\"Bob\"}\n").unwrap();
        fs::write(&f2, "{\"name\":\"Alice\"}\n{\"name\":\"Charlie\"}\n").unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("name"));
    }

    // --- merge ---

    #[test]
    fn merge_jsonl_files() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.jsonl");
        let f2 = tmp.path().join("b.jsonl");

        fs::write(&f1, "{\"name\":\"Alice\"}\n").unwrap();
        fs::write(&f2, "{\"name\":\"Bob\"}\n").unwrap();

        dkit()
            .args(&[
                "merge",
                f1.to_str().unwrap(),
                f2.to_str().unwrap(),
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn merge_jsonl_output_as_jsonl() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.jsonl");
        let f2 = tmp.path().join("b.jsonl");

        fs::write(&f1, "{\"name\":\"Alice\"}\n").unwrap();
        fs::write(&f2, "{\"name\":\"Bob\"}\n").unwrap();

        dkit()
            .args(&[
                "merge",
                f1.to_str().unwrap(),
                f2.to_str().unwrap(),
                "--to",
                "jsonl",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }
}

// ============================================================
// XML ↔ JSONL 크로스 포맷 변환 테스트
// ============================================================

mod xml_jsonl_cross {
    use super::*;

    #[test]
    fn convert_xml_to_jsonl() {
        dkit()
            .args(&["convert", "tests/fixtures/config.xml", "--to", "jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn convert_jsonl_to_xml_to_json() {
        let tmp = TempDir::new().unwrap();
        let xml_path = tmp.path().join("users.xml");

        // JSONL → XML
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.jsonl",
                "--to",
                "xml",
                "-o",
                xml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // XML → JSON
        dkit()
            .args(&["convert", xml_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn convert_jsonl_to_msgpack_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let mp_path = tmp.path().join("users.msgpack");

        // JSONL → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.jsonl",
                "--to",
                "msgpack",
                "-o",
                mp_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → JSONL
        dkit()
            .args(&["convert", mp_path.to_str().unwrap(), "--to", "jsonl"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn diff_jsonl_vs_csv_same_data() {
        // users.jsonl and users.csv have equivalent data
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.jsonl",
                "tests/fixtures/users.csv",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn merge_jsonl_and_json() {
        dkit()
            .args(&[
                "merge",
                "tests/fixtures/users.jsonl",
                "tests/fixtures/users.json",
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }
}
