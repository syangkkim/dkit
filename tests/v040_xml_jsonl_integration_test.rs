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

// ============================================================
// XML 포맷 통합 테스트
// ============================================================

mod xml_format {
    use super::*;

    // --- convert: XML → other formats ---

    #[test]
    fn convert_xml_to_json() {
        dkit()
            .args(&["convert", "tests/fixtures/users.xml", "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"))
            .stdout(predicate::str::contains("Charlie"));
    }

    #[test]
    fn convert_xml_to_yaml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.xml", "--to", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Charlie"));
    }

    #[test]
    fn convert_xml_to_csv() {
        // users.xml has array-like structure suitable for CSV
        dkit()
            .args(&["convert", "tests/fixtures/users.csv", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Alice</name>"));
    }

    #[test]
    fn convert_xml_to_toml() {
        dkit()
            .args(&["convert", "tests/fixtures/config.xml", "--to", "toml"])
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

    #[test]
    fn convert_xml_to_msgpack_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let mp_path = tmp.path().join("config.msgpack");

        // XML → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "msgpack",
                "-o",
                mp_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → JSON (verify data preserved)
        dkit()
            .args(&["convert", mp_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    // --- convert: other formats → XML ---

    #[test]
    fn convert_json_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.json", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Alice</name>"))
            .stdout(predicate::str::contains("<name>Bob</name>"));
    }

    #[test]
    fn convert_yaml_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/config.yaml", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<host>localhost</host>"));
    }

    #[test]
    fn convert_toml_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/config.toml", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<host>localhost</host>"));
    }

    #[test]
    fn convert_csv_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.csv", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Alice</name>"));
    }

    // --- roundtrip ---

    #[test]
    fn convert_xml_json_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let json_path = tmp.path().join("config.json");
        let xml_path = tmp.path().join("config_back.xml");

        // XML → JSON
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "json",
                "-o",
                json_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let json_content = fs::read_to_string(&json_path).unwrap();
        assert!(json_content.contains("localhost"));
        assert!(json_content.contains("5432"));

        // JSON → XML
        dkit()
            .args(&[
                "convert",
                json_path.to_str().unwrap(),
                "--to",
                "xml",
                "-o",
                xml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let xml_content = fs::read_to_string(&xml_path).unwrap();
        assert!(xml_content.contains("localhost"));
        assert!(xml_content.contains("5432"));
    }

    #[test]
    fn convert_xml_yaml_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let yaml_path = tmp.path().join("config.yaml");

        // XML → YAML
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "yaml",
                "-o",
                yaml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // YAML → XML
        dkit()
            .args(&["convert", yaml_path.to_str().unwrap(), "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    // --- stdin ---

    #[test]
    fn convert_stdin_xml_to_json() {
        let xml_input = r#"<?xml version="1.0"?><root><name>Test</name><value>42</value></root>"#;
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin(xml_input)
            .assert()
            .success()
            .stdout(predicate::str::contains("Test"))
            .stdout(predicate::str::contains("42"));
    }

    #[test]
    fn convert_stdin_json_to_xml() {
        let json_input = r#"{"name":"Test","value":42}"#;
        dkit()
            .args(&["convert", "--from", "json", "--to", "xml"])
            .write_stdin(json_input)
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Test</name>"))
            .stdout(predicate::str::contains("<value>42</value>"));
    }

    // --- output file ---

    #[test]
    fn convert_xml_to_json_output_file() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("users.json");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.xml",
                "--to",
                "json",
                "-o",
                out.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&out).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Charlie"));
    }

    // --- batch conversion ---

    #[test]
    fn batch_convert_to_xml() {
        let tmp = TempDir::new().unwrap();
        let outdir = tmp.path().join("out");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "tests/fixtures/config.yaml",
                "--to",
                "xml",
                "--outdir",
                outdir.to_str().unwrap(),
            ])
            .assert()
            .success();

        assert!(outdir.join("users.xml").exists());
        assert!(outdir.join("config.xml").exists());
    }

    // --- view ---

    #[test]
    fn view_xml() {
        // XML root object is shown as table with key/value columns
        dkit()
            .args(&["view", "tests/fixtures/users.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("users"));
    }

    #[test]
    fn view_xml_config() {
        dkit()
            .args(&["view", "tests/fixtures/config.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("config"));
    }

    // --- query ---

    #[test]
    fn query_xml_field() {
        dkit()
            .args(&[
                "query",
                "tests/fixtures/config.xml",
                ".config.database.host",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn query_xml_output_to_json() {
        dkit()
            .args(&[
                "query",
                "tests/fixtures/config.xml",
                ".config.database",
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    // --- stats ---

    #[test]
    fn stats_xml() {
        dkit()
            .args(&["stats", "tests/fixtures/config.xml"])
            .assert()
            .success();
    }

    // --- schema ---

    #[test]
    fn schema_xml() {
        dkit()
            .args(&["schema", "tests/fixtures/config.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("object"));
    }

    #[test]
    fn schema_xml_users() {
        dkit()
            .args(&["schema", "tests/fixtures/users.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("object"));
    }

    // --- diff ---

    #[test]
    fn diff_xml_identical() {
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/config.xml",
                "tests/fixtures/config.xml",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_xml_files_different() {
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.xml",
                "tests/fixtures/config.xml",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn diff_xml_vs_yaml_detects_root_wrapper() {
        // XML wraps data under root element, so structure differs from YAML
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/config.xml",
                "tests/fixtures/config.yaml",
            ])
            .assert()
            .failure();
    }

    #[test]
    fn diff_xml_vs_xml_same_data() {
        // Same XML file compared to itself
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.xml",
                "tests/fixtures/users.xml",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    // --- merge ---

    #[test]
    fn merge_xml_files_to_json() {
        dkit()
            .args(&[
                "merge",
                "tests/fixtures/config.xml",
                "tests/fixtures/users.xml",
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"))
            .stdout(predicate::str::contains("user"));
    }

    #[test]
    fn merge_xml_and_yaml_to_json() {
        dkit()
            .args(&[
                "merge",
                "tests/fixtures/config.xml",
                "tests/fixtures/config.yaml",
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    // --- custom root element ---

    #[test]
    fn convert_json_to_xml_with_root_element() {
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "xml",
                "--root-element",
                "people",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("<people>"));
    }

    // --- compact/pretty ---

    #[test]
    fn convert_xml_to_json_compact() {
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "json",
                "--compact",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }
}

// ============================================================
// 포맷 간 라운드트립 테스트
// ============================================================

mod roundtrip_tests {
    use super::*;

    #[test]
    fn roundtrip_json_jsonl_json() {
        let tmp = TempDir::new().unwrap();
        let jsonl_path = tmp.path().join("data.jsonl");
        let json_path = tmp.path().join("data.json");

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
            .args(&[
                "convert",
                jsonl_path.to_str().unwrap(),
                "--to",
                "json",
                "-o",
                json_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // Verify data preserved
        let content = fs::read_to_string(&json_path).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Bob"));
        assert!(content.contains("alice@example.com"));
    }

    #[test]
    fn roundtrip_xml_yaml_xml() {
        let tmp = TempDir::new().unwrap();
        let yaml_path = tmp.path().join("config.yaml");
        let xml_path = tmp.path().join("config.xml");

        // XML → YAML
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "yaml",
                "-o",
                yaml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // YAML → XML
        dkit()
            .args(&[
                "convert",
                yaml_path.to_str().unwrap(),
                "--to",
                "xml",
                "-o",
                xml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&xml_path).unwrap();
        assert!(content.contains("localhost"));
        assert!(content.contains("5432"));
    }

    #[test]
    fn roundtrip_xml_toml_xml() {
        let tmp = TempDir::new().unwrap();
        let toml_path = tmp.path().join("config.toml");
        let xml_path = tmp.path().join("config.xml");

        // XML → TOML
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "toml",
                "-o",
                toml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // TOML → XML
        dkit()
            .args(&[
                "convert",
                toml_path.to_str().unwrap(),
                "--to",
                "xml",
                "-o",
                xml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&xml_path).unwrap();
        assert!(content.contains("localhost"));
    }

    #[test]
    fn roundtrip_jsonl_csv_jsonl() {
        let tmp = TempDir::new().unwrap();
        let csv_path = tmp.path().join("users.csv");
        let jsonl_path = tmp.path().join("users.jsonl");

        // JSONL → CSV
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.jsonl",
                "--to",
                "csv",
                "-o",
                csv_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // CSV → JSONL
        dkit()
            .args(&[
                "convert",
                csv_path.to_str().unwrap(),
                "--to",
                "jsonl",
                "-o",
                jsonl_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&jsonl_path).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Bob"));
    }

    #[test]
    fn roundtrip_jsonl_xml_jsonl() {
        let tmp = TempDir::new().unwrap();
        let xml_path = tmp.path().join("users.xml");
        let jsonl_path = tmp.path().join("users.jsonl");

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

        // XML → JSONL
        dkit()
            .args(&[
                "convert",
                xml_path.to_str().unwrap(),
                "--to",
                "jsonl",
                "-o",
                jsonl_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&jsonl_path).unwrap();
        assert!(content.contains("Alice"));
    }

    #[test]
    fn roundtrip_csv_json_csv() {
        let tmp = TempDir::new().unwrap();
        let json_path = tmp.path().join("users.json");
        let csv_path = tmp.path().join("users.csv");

        // CSV → JSON
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.csv",
                "--to",
                "json",
                "-o",
                json_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // JSON → CSV
        dkit()
            .args(&[
                "convert",
                json_path.to_str().unwrap(),
                "--to",
                "csv",
                "-o",
                csv_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&csv_path).unwrap();
        assert!(content.contains("Alice"));
        assert!(content.contains("Bob"));
    }
}

// ============================================================
// 엣지 케이스 테스트
// ============================================================

mod edge_cases {
    use super::*;

    // --- 빈 데이터 ---

    #[test]
    fn convert_empty_json_to_xml() {
        let input = "null";
        dkit()
            .args(&["convert", "--from", "json", "--to", "xml"])
            .write_stdin(input)
            .assert()
            .success();
    }

    #[test]
    fn convert_empty_array_json_to_jsonl() {
        let input = "[]";
        dkit()
            .args(&["convert", "--from", "json", "--to", "jsonl"])
            .write_stdin(input)
            .assert()
            .success();
    }

    #[test]
    fn convert_empty_object_json_to_xml() {
        let input = "{}";
        dkit()
            .args(&["convert", "--from", "json", "--to", "xml"])
            .write_stdin(input)
            .assert()
            .success();
    }

    #[test]
    fn convert_empty_jsonl_to_json() {
        let input = "";
        dkit()
            .args(&["convert", "--from", "jsonl", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success();
    }

    // --- 특수문자 / 유니코드 ---

    #[test]
    fn convert_unicode_jsonl_to_json() {
        let input =
            "{\"name\":\"홍길동\",\"city\":\"서울\"}\n{\"name\":\"田中太郎\",\"city\":\"東京\"}\n";
        dkit()
            .args(&["convert", "--from", "jsonl", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("홍길동"))
            .stdout(predicate::str::contains("田中太郎"));
    }

    #[test]
    fn convert_unicode_json_to_xml() {
        let input = r#"{"name":"홍길동","city":"서울"}"#;
        dkit()
            .args(&["convert", "--from", "json", "--to", "xml"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("홍길동"))
            .stdout(predicate::str::contains("서울"));
    }

    #[test]
    fn convert_unicode_xml_to_json() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?><root><name>홍길동</name><city>서울</city></root>"#;
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("홍길동"))
            .stdout(predicate::str::contains("서울"));
    }

    #[test]
    fn convert_special_chars_json_to_jsonl() {
        let input = r#"[{"text":"line1\nline2","quote":"she said \"hello\""},{"text":"tab\there","value":null}]"#;
        dkit()
            .args(&["convert", "--from", "json", "--to", "jsonl"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("line1"));
    }

    #[test]
    fn convert_xml_with_special_chars_to_json() {
        let input = r#"<?xml version="1.0"?><root><text>Hello &amp; World</text><math>5 &lt; 10</math></root>"#;
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("Hello & World"))
            .stdout(predicate::str::contains("5 < 10"));
    }

    // --- 대용량 데이터 ---

    #[test]
    fn convert_large_jsonl_to_json() {
        // Generate 100 lines of JSONL
        let mut input = String::new();
        for i in 0..100 {
            input.push_str(&format!("{{\"id\":{},\"name\":\"user_{}\"}}\n", i, i));
        }

        dkit()
            .args(&["convert", "--from", "jsonl", "--to", "json"])
            .write_stdin(input.as_str())
            .assert()
            .success()
            .stdout(predicate::str::contains("user_0"))
            .stdout(predicate::str::contains("user_99"));
    }

    #[test]
    fn convert_large_json_array_to_xml() {
        // Generate a JSON array with 50 elements
        let mut items: Vec<String> = Vec::new();
        for i in 0..50 {
            items.push(format!(
                "{{\"id\":{},\"name\":\"item_{}\",\"value\":{}}}",
                i,
                i,
                i * 10
            ));
        }
        let input = format!("[{}]", items.join(","));

        dkit()
            .args(&["convert", "--from", "json", "--to", "xml"])
            .write_stdin(input.as_str())
            .assert()
            .success()
            .stdout(predicate::str::contains("item_0"))
            .stdout(predicate::str::contains("item_49"));
    }

    #[test]
    fn convert_large_json_to_jsonl() {
        let mut items: Vec<String> = Vec::new();
        for i in 0..100 {
            items.push(format!("{{\"id\":{},\"val\":{}}}", i, i * 2));
        }
        let input = format!("[{}]", items.join(","));

        dkit()
            .args(&["convert", "--from", "json", "--to", "jsonl"])
            .write_stdin(input.as_str())
            .assert()
            .success()
            .stdout(predicate::str::contains("\"id\":0"))
            .stdout(predicate::str::contains("\"id\":99"));
    }

    // --- 단일 객체 / 비배열 데이터 ---

    #[test]
    fn convert_single_object_to_jsonl() {
        dkit()
            .args(&["convert", "tests/fixtures/single.json", "--to", "jsonl"])
            .assert()
            .success();
    }

    #[test]
    fn convert_single_object_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/single.json", "--to", "xml"])
            .assert()
            .success();
    }

    #[test]
    fn convert_nested_json_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/nested.json", "--to", "xml"])
            .assert()
            .success();
    }

    #[test]
    fn convert_nested_json_to_jsonl() {
        dkit()
            .args(&["convert", "tests/fixtures/nested.json", "--to", "jsonl"])
            .assert()
            .success();
    }

    // --- 잘못된 입력 에러 처리 ---

    #[test]
    fn convert_invalid_xml_shows_error() {
        // Mismatched closing tag is truly invalid XML
        let input = "<root><a></b></root>";
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin(input)
            .assert()
            .failure();
    }

    #[test]
    fn convert_invalid_jsonl_shows_error() {
        let input = "{\"valid\":true}\n{invalid json}\n";
        dkit()
            .args(&["convert", "--from", "jsonl", "--to", "json"])
            .write_stdin(input)
            .assert()
            .failure();
    }

    // --- 포맷 자동 감지 ---

    #[test]
    fn auto_detect_xml_format() {
        dkit()
            .args(&["convert", "tests/fixtures/users.xml", "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn auto_detect_jsonl_format() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn auto_detect_xml_from_content() {
        let input = r#"<?xml version="1.0"?><data><item>test</item></data>"#;
        dkit()
            .args(&["convert", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::contains("test"));
    }

    #[test]
    fn auto_detect_jsonl_from_content() {
        let input = "{\"a\":1}\n{\"a\":2}\n";
        dkit()
            .args(&["convert", "--to", "json"])
            .write_stdin(input)
            .assert()
            .success();
    }

    // --- mixed types ---

    #[test]
    fn convert_mixed_types_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/mixed_types.json", "--to", "xml"])
            .assert()
            .success();
    }

    #[test]
    fn convert_mixed_types_to_jsonl() {
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/mixed_types.json",
                "--to",
                "jsonl",
            ])
            .assert()
            .success();
    }
}
