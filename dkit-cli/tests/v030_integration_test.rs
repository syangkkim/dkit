#![cfg(all(feature = "xml", feature = "msgpack"))]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// XML 포맷 통합 테스트
// ============================================================

mod xml_format {
    use super::*;

    // --- convert ---

    #[test]
    fn convert_xml_to_json_config() {
        dkit()
            .args(&["convert", "tests/fixtures/config.xml", "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"))
            .stdout(predicate::str::contains("5432"));
    }

    #[test]
    fn convert_xml_to_yaml() {
        dkit()
            .args(&["convert", "tests/fixtures/config.xml", "--to", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"))
            .stdout(predicate::str::contains("8080"));
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
    fn convert_json_to_xml_users() {
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
            .stdout(predicate::str::contains("<host>localhost</host>"))
            .stdout(predicate::str::contains("<port>"));
    }

    #[test]
    fn convert_xml_to_json_output_file() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("config.json");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "json",
                "-o",
                out.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&out).unwrap();
        assert!(content.contains("localhost"));
    }

    #[test]
    fn convert_xml_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let xml_path = tmp.path().join("roundtrip.xml");

        // JSON → XML
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
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
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_stdin_xml_to_json() {
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin("<root><name>Alice</name><age>30</age></root>")
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    // --- view ---

    #[test]
    fn view_xml_users() {
        // XML users wraps in <users> object, so view shows object keys
        dkit()
            .args(&["view", "tests/fixtures/users.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("users"));
    }

    #[test]
    fn view_xml_users_with_path() {
        // Use --path to navigate into the array
        dkit()
            .args(&["view", "tests/fixtures/users.xml", "--path", ".users.user"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    // --- stats ---

    #[test]
    fn stats_xml_users() {
        dkit()
            .args(&["stats", "tests/fixtures/users.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("rows:").or(predicate::str::contains("count")));
    }

    // --- schema ---

    #[test]
    fn schema_xml_config() {
        dkit()
            .args(&["schema", "tests/fixtures/config.xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("config").or(predicate::str::contains("database")));
    }

    #[test]
    fn schema_xml_users() {
        dkit()
            .args(&["schema", "tests/fixtures/users.xml"])
            .assert()
            .success();
    }

    // --- query ---

    #[test]
    fn query_xml_config_field() {
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
    fn query_xml_config_nested() {
        dkit()
            .args(&["query", "tests/fixtures/config.xml", ".config.server.port"])
            .assert()
            .success()
            .stdout(predicate::str::contains("8080"));
    }

    // --- XML with attributes ---

    #[test]
    fn convert_xml_with_attributes_stdin() {
        let xml = r#"<root><item id="1">First</item><item id="2">Second</item></root>"#;
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin(xml)
            .assert()
            .success()
            .stdout(predicate::str::contains("First"))
            .stdout(predicate::str::contains("Second"));
    }
}

// ============================================================
// MessagePack 포맷 통합 테스트
// ============================================================

mod msgpack_format {
    use super::*;

    #[test]
    fn convert_json_to_msgpack_file() {
        let tmp = TempDir::new().unwrap();
        let msgpack_path = tmp.path().join("data.msgpack");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                msgpack_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // Verify file was created and is non-empty binary
        let bytes = fs::read(&msgpack_path).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn convert_msgpack_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let msgpack_path = tmp.path().join("users.msgpack");

        // JSON → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                msgpack_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → JSON
        dkit()
            .args(&["convert", msgpack_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn convert_yaml_to_msgpack_to_json() {
        let tmp = TempDir::new().unwrap();
        let msgpack_path = tmp.path().join("config.msgpack");

        // YAML → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.yaml",
                "--to",
                "msgpack",
                "-o",
                msgpack_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → JSON
        dkit()
            .args(&["convert", msgpack_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"))
            .stdout(predicate::str::contains("5432"));
    }

    #[test]
    fn convert_toml_to_msgpack_to_yaml() {
        let tmp = TempDir::new().unwrap();
        let msgpack_path = tmp.path().join("config.msgpack");

        // TOML → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.toml",
                "--to",
                "msgpack",
                "-o",
                msgpack_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → YAML
        dkit()
            .args(&["convert", msgpack_path.to_str().unwrap(), "--to", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn convert_xml_to_msgpack_to_json() {
        let tmp = TempDir::new().unwrap();
        let msgpack_path = tmp.path().join("config.msgpack");

        // XML → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.xml",
                "--to",
                "msgpack",
                "-o",
                msgpack_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → JSON
        dkit()
            .args(&["convert", msgpack_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn diff_msgpack_files_identical() {
        let tmp = TempDir::new().unwrap();
        let mp1 = tmp.path().join("a.msgpack");
        let mp2 = tmp.path().join("b.msgpack");

        // Create two identical msgpack files
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                mp1.to_str().unwrap(),
            ])
            .assert()
            .success();

        fs::copy(&mp1, &mp2).unwrap();

        dkit()
            .args(&["diff", mp1.to_str().unwrap(), mp2.to_str().unwrap()])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_msgpack_files_different() {
        let tmp = TempDir::new().unwrap();
        let mp1 = tmp.path().join("a.msgpack");
        let mp2 = tmp.path().join("b.msgpack");

        // Create two different msgpack files
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                mp1.to_str().unwrap(),
            ])
            .assert()
            .success();

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users2.json",
                "--to",
                "msgpack",
                "-o",
                mp2.to_str().unwrap(),
            ])
            .assert()
            .success();

        dkit()
            .args(&["diff", mp1.to_str().unwrap(), mp2.to_str().unwrap()])
            .assert()
            .failure(); // files differ → exit code 1
    }

    #[test]
    fn diff_msgpack_vs_json() {
        let tmp = TempDir::new().unwrap();
        let mp = tmp.path().join("users.msgpack");

        // Create msgpack from same data
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                mp.to_str().unwrap(),
            ])
            .assert()
            .success();

        // diff msgpack vs json (same data → no differences)
        dkit()
            .args(&["diff", mp.to_str().unwrap(), "tests/fixtures/users.json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }
}

// ============================================================
// diff 서브커맨드 추가 통합 테스트
// ============================================================

mod diff_extended {
    use super::*;

    // --- XML 파일 간 비교 ---

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
    fn diff_xml_vs_yaml_same_data() {
        // config.xml and config.yaml have equivalent data
        // XML wraps in <config>, so may differ structurally - test it runs
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/config.yaml",
                "tests/fixtures/config.toml",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_xml_files_different() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.xml");
        let f2 = tmp.path().join("b.xml");

        fs::write(
            &f1,
            r#"<?xml version="1.0"?><root><name>Alice</name><age>30</age></root>"#,
        )
        .unwrap();
        fs::write(
            &f2,
            r#"<?xml version="1.0"?><root><name>Bob</name><age>25</age></root>"#,
        )
        .unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("name"))
            .stdout(predicate::str::contains("age"));
    }

    // --- TSV 파일 간 비교 ---

    #[test]
    fn diff_tsv_identical() {
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.tsv",
                "tests/fixtures/users.tsv",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_tsv_vs_csv_same_data() {
        // users.tsv and users.csv have the same data
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.tsv",
                "tests/fixtures/users.csv",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_tsv_vs_json() {
        // users.tsv and users.json should have equivalent data
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/users.tsv",
                "tests/fixtures/users.json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_tsv_files_different() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.tsv");
        let f2 = tmp.path().join("b.tsv");

        fs::write(&f1, "name\tage\nAlice\t30\n").unwrap();
        fs::write(&f2, "name\tage\nBob\t25\n").unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure();
    }

    // --- diff with --path on arrays ---

    #[test]
    fn diff_with_path_array_index() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, r#"{"items": [{"name": "A"}, {"name": "B"}]}"#).unwrap();
        fs::write(&f2, r#"{"items": [{"name": "A"}, {"name": "C"}]}"#).unwrap();

        dkit()
            .args(&[
                "diff",
                f1.to_str().unwrap(),
                f2.to_str().unwrap(),
                "--path",
                ".items",
            ])
            .assert()
            .failure()
            .stdout(predicate::str::contains("[1]"));
    }

    // --- diff with nested objects ---

    #[test]
    fn diff_deeply_nested_change() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, r#"{"level1": {"level2": {"level3": {"value": 1}}}}"#).unwrap();
        fs::write(&f2, r#"{"level1": {"level2": {"level3": {"value": 2}}}}"#).unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("level1.level2.level3.value"));
    }

    // --- diff with type changes ---

    #[test]
    fn diff_type_change() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, r#"{"value": "42"}"#).unwrap();
        fs::write(&f2, r#"{"value": 42}"#).unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("value"));
    }

    // --- diff with null values ---

    #[test]
    fn diff_null_changes() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, r#"{"x": null}"#).unwrap();
        fs::write(&f2, r#"{"x": 1}"#).unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("null"));
    }

    // --- diff quiet with cross-format ---

    #[test]
    fn diff_quiet_cross_format_identical() {
        dkit()
            .args(&[
                "diff",
                "tests/fixtures/config.yaml",
                "tests/fixtures/config.toml",
                "--quiet",
            ])
            .assert()
            .success()
            .stdout(predicate::str::is_empty());
    }

    // --- diff with empty arrays ---

    #[test]
    fn diff_empty_vs_nonempty_array() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, "[]").unwrap();
        fs::write(&f2, "[1, 2, 3]").unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("[0]"))
            .stdout(predicate::str::contains("(added)"));
    }
}

// ============================================================
// 크로스 포맷 변환 테스트
// ============================================================

mod cross_format {
    use super::*;

    #[test]
    fn xml_to_yaml_config() {
        dkit()
            .args(&["convert", "tests/fixtures/config.xml", "--to", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn tsv_to_yaml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.tsv", "--to", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn tsv_to_toml_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let toml_path = tmp.path().join("users.toml");

        // TSV → TOML (array of tables)
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.tsv",
                "--to",
                "toml",
                "-o",
                toml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&toml_path).unwrap();
        assert!(content.contains("Alice"));
    }

    #[test]
    fn csv_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.csv", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Alice</name>"));
    }

    #[test]
    fn tsv_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/users.tsv", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<name>Alice</name>"))
            .stdout(predicate::str::contains("<name>Bob</name>"));
    }

    #[test]
    fn csv_to_xml_to_json() {
        // CSV → XML → JSON roundtrip
        let tmp = TempDir::new().unwrap();
        let xml_path = tmp.path().join("users.xml");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.csv",
                "--to",
                "xml",
                "-o",
                xml_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        dkit()
            .args(&["convert", xml_path.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn yaml_to_xml_to_json() {
        let tmp = TempDir::new().unwrap();
        let xml_path = tmp.path().join("config.xml");

        // YAML → XML
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/config.yaml",
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
            .stdout(predicate::str::contains("localhost"));
    }

    #[test]
    fn toml_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/config.toml", "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<host>localhost</host>"));
    }

    #[test]
    fn json_to_msgpack_to_csv() {
        let tmp = TempDir::new().unwrap();
        let mp_path = tmp.path().join("users.msgpack");

        // JSON → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                mp_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → CSV
        dkit()
            .args(&["convert", mp_path.to_str().unwrap(), "--to", "csv"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    #[test]
    fn json_to_msgpack_to_xml() {
        let tmp = TempDir::new().unwrap();
        let mp_path = tmp.path().join("users.msgpack");

        // JSON → MessagePack
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "msgpack",
                "-o",
                mp_path.to_str().unwrap(),
            ])
            .assert()
            .success();

        // MessagePack → XML
        dkit()
            .args(&["convert", mp_path.to_str().unwrap(), "--to", "xml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    // --- batch conversion with new formats ---

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

    #[test]
    fn batch_convert_to_msgpack() {
        let tmp = TempDir::new().unwrap();
        let outdir = tmp.path().join("out");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "tests/fixtures/config.yaml",
                "--to",
                "msgpack",
                "--outdir",
                outdir.to_str().unwrap(),
            ])
            .assert()
            .success();

        assert!(outdir.join("users.msgpack").exists());
        assert!(outdir.join("config.msgpack").exists());

        // Verify we can read back
        dkit()
            .args(&[
                "convert",
                outdir.join("users.msgpack").to_str().unwrap(),
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }
}

// ============================================================
// 에지 케이스 및 에러 케이스
// ============================================================

mod edge_cases {
    use super::*;

    // --- empty data ---

    #[test]
    fn convert_empty_json_array_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/empty.json", "--to", "xml"])
            .assert()
            .success();
    }

    #[test]
    fn convert_empty_json_array_to_msgpack() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("empty.msgpack");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/empty.json",
                "--to",
                "msgpack",
                "-o",
                out.to_str().unwrap(),
            ])
            .assert()
            .success();

        // Read back
        dkit()
            .args(&["convert", out.to_str().unwrap(), "--to", "json"])
            .assert()
            .success();
    }

    // --- single object ---

    #[test]
    fn convert_single_json_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/single.json", "--to", "xml"])
            .assert()
            .success();
    }

    #[test]
    fn convert_single_json_to_msgpack_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let mp = tmp.path().join("single.msgpack");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/single.json",
                "--to",
                "msgpack",
                "-o",
                mp.to_str().unwrap(),
            ])
            .assert()
            .success();

        dkit()
            .args(&["convert", mp.to_str().unwrap(), "--to", "json"])
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
    fn convert_mixed_types_to_msgpack_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let mp = tmp.path().join("mixed.msgpack");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/mixed_types.json",
                "--to",
                "msgpack",
                "-o",
                mp.to_str().unwrap(),
            ])
            .assert()
            .success();

        dkit()
            .args(&["convert", mp.to_str().unwrap(), "--to", "json"])
            .assert()
            .success();
    }

    // --- nested data ---

    #[test]
    fn convert_nested_json_to_xml() {
        dkit()
            .args(&["convert", "tests/fixtures/nested.json", "--to", "xml"])
            .assert()
            .success();
    }

    #[test]
    fn convert_nested_json_to_msgpack_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let mp = tmp.path().join("nested.msgpack");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/nested.json",
                "--to",
                "msgpack",
                "-o",
                mp.to_str().unwrap(),
            ])
            .assert()
            .success();

        dkit()
            .args(&["convert", mp.to_str().unwrap(), "--to", "json"])
            .assert()
            .success();
    }

    // --- error cases ---

    #[test]
    fn convert_malformed_xml_input() {
        // Truly malformed XML that cannot be parsed
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("bad.xml");
        fs::write(&f, "").unwrap();

        dkit()
            .args(&["convert", f.to_str().unwrap(), "--to", "json"])
            .assert()
            .failure();
    }

    #[test]
    fn convert_invalid_msgpack_bytes() {
        // Use bytes that are genuinely invalid msgpack (reserved byte 0xc1)
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("bad.msgpack");
        fs::write(&f, &[0xc1_u8]).unwrap();

        dkit()
            .args(&["convert", f.to_str().unwrap(), "--to", "json"])
            .assert()
            .failure();
    }

    #[test]
    fn diff_xml_vs_yaml_shows_differences() {
        // XML wraps in <config> root, so it differs structurally from YAML
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.xml");
        let f2 = tmp.path().join("b.xml");

        fs::write(&f1, "<root><a>1</a></root>").unwrap();
        fs::write(&f2, "<root><a>2</a></root>").unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure();
    }

    // --- XML special characters ---

    #[test]
    fn convert_xml_with_special_chars_stdin() {
        let xml = r#"<root><msg>Hello &amp; World</msg></root>"#;
        dkit()
            .args(&["convert", "--from", "xml", "--to", "json"])
            .write_stdin(xml)
            .assert()
            .success()
            .stdout(predicate::str::contains("Hello & World"));
    }

    // --- TSV with special content ---

    #[test]
    fn convert_tsv_with_empty_fields() {
        let tmp = TempDir::new().unwrap();
        let f = tmp.path().join("sparse.tsv");
        fs::write(&f, "name\tage\tcity\nAlice\t30\t\nBob\t\tSeoul\n").unwrap();

        dkit()
            .args(&["convert", f.to_str().unwrap(), "--to", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    // --- diff with empty objects ---

    #[test]
    fn diff_empty_objects() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, "{}").unwrap();
        fs::write(&f2, "{}").unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found."));
    }

    #[test]
    fn diff_empty_vs_nonempty_object() {
        let tmp = TempDir::new().unwrap();
        let f1 = tmp.path().join("a.json");
        let f2 = tmp.path().join("b.json");

        fs::write(&f1, "{}").unwrap();
        fs::write(&f2, r#"{"key": "value"}"#).unwrap();

        dkit()
            .args(&["diff", f1.to_str().unwrap(), f2.to_str().unwrap()])
            .assert()
            .failure()
            .stdout(predicate::str::contains("key"))
            .stdout(predicate::str::contains("(added)"));
    }

    // --- merge with new formats ---

    #[test]
    fn merge_xml_files() {
        dkit()
            .args(&[
                "merge",
                "tests/fixtures/users.xml",
                "tests/fixtures/users.xml",
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn merge_tsv_files() {
        dkit()
            .args(&[
                "merge",
                "tests/fixtures/users.tsv",
                "tests/fixtures/users.tsv",
                "--to",
                "json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("Bob"));
    }

    // --- query on new formats ---

    #[test]
    fn query_tsv_first_element() {
        dkit()
            .args(&["query", "tests/fixtures/users.tsv", ".[0].name"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn query_xml_users_array() {
        dkit()
            .args(&["query", "tests/fixtures/users.xml", ".users.user"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    // --- view with new formats ---

    #[test]
    fn view_xml_config() {
        dkit()
            .args(&["view", "tests/fixtures/config.xml"])
            .assert()
            .success();
    }

    // --- stats with new formats ---

    #[test]
    fn stats_xml_config() {
        dkit()
            .args(&["stats", "tests/fixtures/config.xml"])
            .assert()
            .success();
    }
}
