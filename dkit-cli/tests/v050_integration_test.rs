use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn dkit() -> Command {
    Command::cargo_bin("dkit").unwrap()
}

// ============================================================
// HTML 출력 포맷 통합 테스트
// ============================================================

mod html_output {
    use super::*;

    #[test]
    fn convert_json_to_html() {
        dkit()
            .args(&["convert", "tests/fixtures/users.json", "--to", "html"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<table>"))
            .stdout(predicate::str::contains("<th>name</th>"))
            .stdout(predicate::str::contains("<td>Alice</td>"));
    }

    #[test]
    fn convert_csv_to_html() {
        dkit()
            .args(&["convert", "tests/fixtures/users.csv", "--to", "html"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<table>"))
            .stdout(predicate::str::contains("<thead>"))
            .stdout(predicate::str::contains("<tbody>"));
    }

    #[test]
    fn convert_yaml_to_html() {
        dkit()
            .args(&["convert", "tests/fixtures/config.yaml", "--to", "html"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<th>key</th>"))
            .stdout(predicate::str::contains("<th>value</th>"));
    }

    #[test]
    fn convert_json_to_html_styled() {
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "html",
                "--styled",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("style="))
            .stdout(predicate::str::contains("border-collapse"));
    }

    #[test]
    fn convert_json_to_html_full_document() {
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "html",
                "--full-html",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("<!DOCTYPE html>"))
            .stdout(predicate::str::contains("<html>"))
            .stdout(predicate::str::contains("<meta charset=\"UTF-8\">"))
            .stdout(predicate::str::contains("</html>"));
    }

    #[test]
    fn convert_json_to_html_styled_full_document() {
        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "html",
                "--styled",
                "--full-html",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("<!DOCTYPE html>"))
            .stdout(predicate::str::contains("<style>"))
            .stdout(predicate::str::contains("border-collapse"));
    }

    #[test]
    fn convert_json_to_html_output_file() {
        let dir = TempDir::new().unwrap();
        let out = dir.path().join("output.html");

        dkit()
            .args(&[
                "convert",
                "tests/fixtures/users.json",
                "--to",
                "html",
                "--full-html",
                "-o",
                out.to_str().unwrap(),
            ])
            .assert()
            .success();

        let content = fs::read_to_string(&out).unwrap();
        assert!(content.contains("<!DOCTYPE html>"));
        assert!(content.contains("Alice"));
    }

    #[test]
    fn convert_stdin_to_html() {
        dkit()
            .args(&["convert", "--from", "json", "--to", "html"])
            .write_stdin(r#"[{"x": 1}, {"x": 2}]"#)
            .assert()
            .success()
            .stdout(predicate::str::contains("<th>x</th>"))
            .stdout(predicate::str::contains("<td>1</td>"))
            .stdout(predicate::str::contains("<td>2</td>"));
    }

    #[test]
    fn convert_html_escapes_special_chars() {
        dkit()
            .args(&["convert", "--from", "json", "--to", "html"])
            .write_stdin(r#"[{"text": "<b>bold</b> & \"quoted\""}]"#)
            .assert()
            .success()
            .stdout(predicate::str::contains("&lt;b&gt;bold&lt;/b&gt;"))
            .stdout(predicate::str::contains("&amp;"));
    }

    #[test]
    fn query_output_as_html() {
        dkit()
            .args(&["query", "tests/fixtures/users.json", ".", "--to", "html"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<table>"))
            .stdout(predicate::str::contains("<th>name</th>"));
    }
}

// ============================================================
// Markdown 출력 포맷 통합 테스트 (기존 테스트 보완)
// ============================================================

mod markdown_output {
    use super::*;

    #[test]
    fn convert_toml_to_md() {
        dkit()
            .args(&["convert", "tests/fixtures/config.toml", "--to", "md"])
            .assert()
            .success()
            .stdout(predicate::str::contains("| key | value |"))
            .stdout(predicate::str::contains("| --- | --- |"));
    }

    #[test]
    #[cfg(feature = "xml")]
    fn convert_xml_to_md() {
        dkit()
            .args(&["convert", "tests/fixtures/users.xml", "--to", "md"])
            .assert()
            .success()
            .stdout(predicate::str::contains("|"));
    }

    #[test]
    fn convert_jsonl_to_md() {
        dkit()
            .args(&["convert", "tests/fixtures/users.jsonl", "--to", "md"])
            .assert()
            .success()
            .stdout(predicate::str::contains("| name |"))
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn convert_md_null_value_display() {
        dkit()
            .args(&["convert", "--from", "json", "--to", "md"])
            .write_stdin(r#"[{"name": "Alice", "email": null}]"#)
            .assert()
            .success()
            .stdout(predicate::str::contains("null"));
    }

    #[test]
    fn convert_md_boolean_values() {
        dkit()
            .args(&["convert", "--from", "json", "--to", "md"])
            .write_stdin(r#"[{"flag": true, "other": false}]"#)
            .assert()
            .success()
            .stdout(predicate::str::contains("true"))
            .stdout(predicate::str::contains("false"));
    }

    #[test]
    fn convert_md_single_object() {
        dkit()
            .args(&["convert", "--from", "json", "--to", "md"])
            .write_stdin(r#"{"host": "localhost", "port": 3000}"#)
            .assert()
            .success()
            .stdout(predicate::str::contains("| key | value |"))
            .stdout(predicate::str::contains("| host | localhost |"))
            .stdout(predicate::str::contains("| port | 3000 |"));
    }
}

// ============================================================
// 테이블 커스터마이징 옵션 조합 테스트
// ============================================================

mod table_customization {
    use super::*;

    #[test]
    fn view_border_none_with_row_numbers() {
        dkit()
            .args(&[
                "view",
                "tests/fixtures/users.json",
                "--border",
                "none",
                "--row-numbers",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("|").not());
    }

    #[test]
    fn view_border_heavy_with_limit_and_columns() {
        dkit()
            .args(&[
                "view",
                "tests/fixtures/users.json",
                "--border",
                "heavy",
                "-n",
                "1",
                "--columns",
                "name",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn view_hide_header_with_row_numbers() {
        dkit()
            .args(&[
                "view",
                "tests/fixtures/users.json",
                "--hide-header",
                "--row-numbers",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("name").not());
    }

    #[test]
    fn view_max_width_with_color() {
        dkit()
            .args(&[
                "view",
                "-",
                "--from",
                "json",
                "--max-width",
                "10",
                "--color",
            ])
            .write_stdin(r#"[{"description": "This is a very long description that should be truncated", "count": 42}]"#)
            .assert()
            .success();
    }

    #[test]
    fn view_all_options_combined() {
        dkit()
            .args(&[
                "view",
                "tests/fixtures/users.json",
                "--border",
                "rounded",
                "--row-numbers",
                "--max-width",
                "20",
                "-n",
                "1",
                "--columns",
                "name,age",
                "--color",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn view_format_json_output() {
        dkit()
            .args(&["view", "tests/fixtures/users.json", "--format", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"name\""));
    }

    #[test]
    fn view_format_md_output() {
        dkit()
            .args(&["view", "tests/fixtures/users.json", "--format", "md"])
            .assert()
            .success()
            .stdout(predicate::str::contains("| name |"));
    }

    #[test]
    fn view_format_html_output() {
        dkit()
            .args(&["view", "tests/fixtures/users.json", "--format", "html"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<table>"));
    }

    #[test]
    fn view_format_csv_output() {
        dkit()
            .args(&["view", "tests/fixtures/users.json", "--format", "csv"])
            .assert()
            .success()
            .stdout(predicate::str::contains("name"))
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn view_format_yaml_output() {
        dkit()
            .args(&["view", "tests/fixtures/users.json", "--format", "yaml"])
            .assert()
            .success()
            .stdout(predicate::str::contains("name: Alice"));
    }

    #[test]
    fn query_output_as_table() {
        dkit()
            .args(&["query", "tests/fixtures/users.json", ".", "--to", "table"])
            .assert()
            .success()
            .stdout(predicate::str::contains("name"))
            .stdout(predicate::str::contains("Alice"));
    }

    #[test]
    fn stats_format_table_output() {
        dkit()
            .args(&["stats", "tests/fixtures/users.json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("rows:"))
            .stdout(predicate::str::contains("columns:"));
    }
}

// ============================================================
// 인코딩 변환 통합 테스트 (크로스 서브커맨드)
// ============================================================

mod encoding_cross_commands {
    use super::*;

    #[test]
    fn query_with_encoding_euc_kr() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("korean.csv");

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"name,value\n");
        bytes.extend_from_slice(&[0xC8, 0xAB, 0xB1, 0xE6, 0xB5, 0xBF]); // 홍길동 in EUC-KR
        bytes.extend_from_slice(b",100\n");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&["query", path.to_str().unwrap(), ".", "--encoding", "euc-kr"])
            .assert()
            .success()
            .stdout(predicate::str::contains("홍길동"));
    }

    #[test]
    fn convert_euc_kr_csv_to_md() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("korean.csv");

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"name,value\n");
        bytes.extend_from_slice(&[0xC8, 0xAB, 0xB1, 0xE6, 0xB5, 0xBF]); // 홍길동
        bytes.extend_from_slice(b",100\n");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&[
                "convert",
                path.to_str().unwrap(),
                "--to",
                "md",
                "--encoding",
                "euc-kr",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("| name |"))
            .stdout(predicate::str::contains("홍길동"));
    }

    #[test]
    fn convert_euc_kr_csv_to_html() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("korean.csv");

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"name,value\n");
        bytes.extend_from_slice(&[0xC8, 0xAB, 0xB1, 0xE6, 0xB5, 0xBF]); // 홍길동
        bytes.extend_from_slice(b",100\n");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&[
                "convert",
                path.to_str().unwrap(),
                "--to",
                "html",
                "--encoding",
                "euc-kr",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("<table>"))
            .stdout(predicate::str::contains("홍길동"));
    }

    #[test]
    fn convert_utf16le_csv_to_json() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("utf16le.csv");

        let content = "name,age\nAlice,30\n";
        let mut bytes = vec![0xFF, 0xFE]; // UTF-16LE BOM
        for ch in content.encode_utf16() {
            bytes.push((ch & 0xFF) as u8);
            bytes.push((ch >> 8) as u8);
        }
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&["convert", path.to_str().unwrap(), "--format", "json"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Alice"))
            .stdout(predicate::str::contains("30"));
    }

    #[test]
    fn convert_shift_jis_to_yaml() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("japanese.csv");

        // "名前,年齢\n太郎,25\n" in Shift-JIS
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[0x96, 0xBC, 0x91, 0x4F]); // 名前
        bytes.push(b',');
        bytes.extend_from_slice(&[0x94, 0x4E, 0x97, 0xEE]); // 年齢
        bytes.push(b'\n');
        bytes.extend_from_slice(&[0x91, 0xBE, 0x98, 0x59]); // 太郎
        bytes.extend_from_slice(b",25\n");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&[
                "convert",
                path.to_str().unwrap(),
                "--format",
                "yaml",
                "--encoding",
                "shift_jis",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("太郎"));
    }

    #[test]
    fn schema_with_encoding_euc_kr() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("korean.csv");

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"name,value\n");
        bytes.extend_from_slice(b"test,100\n");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&["schema", path.to_str().unwrap(), "--encoding", "euc-kr"])
            .assert()
            .success()
            .stdout(predicate::str::contains("name"));
    }

    #[test]
    fn merge_with_encoding() {
        let dir = TempDir::new().unwrap();
        let path1 = dir.path().join("a.csv");
        let path2 = dir.path().join("b.csv");

        // Both files as EUC-KR
        let mut bytes1 = Vec::new();
        bytes1.extend_from_slice(b"name,value\n");
        bytes1.extend_from_slice(b"test1,100\n");
        fs::write(&path1, &bytes1).unwrap();

        let mut bytes2 = Vec::new();
        bytes2.extend_from_slice(b"name,value\n");
        bytes2.extend_from_slice(b"test2,200\n");
        fs::write(&path2, &bytes2).unwrap();

        dkit()
            .args(&[
                "merge",
                path1.to_str().unwrap(),
                path2.to_str().unwrap(),
                "--format",
                "json",
                "--encoding",
                "euc-kr",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("test1"))
            .stdout(predicate::str::contains("test2"));
    }

    #[test]
    fn diff_with_encoding() {
        let dir = TempDir::new().unwrap();
        let path1 = dir.path().join("a.csv");
        let path2 = dir.path().join("b.csv");

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"name,value\ntest,100\n");
        fs::write(&path1, &bytes).unwrap();
        fs::write(&path2, &bytes).unwrap();

        dkit()
            .args(&[
                "diff",
                path1.to_str().unwrap(),
                path2.to_str().unwrap(),
                "--encoding",
                "euc-kr",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("No differences found"));
    }
}

// ============================================================
// Markdown/HTML + 인코딩 크로스 테스트
// ============================================================

mod cross_format_encoding {
    use super::*;

    #[test]
    fn convert_utf8_bom_json_to_md() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bom.json");
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(b"[{\"name\": \"Alice\", \"age\": 30}]");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&["convert", path.to_str().unwrap(), "--to", "md"])
            .assert()
            .success()
            .stdout(predicate::str::contains("| name |"))
            .stdout(predicate::str::contains("| Alice |"));
    }

    #[test]
    fn convert_utf8_bom_json_to_html() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("bom.json");
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(b"[{\"name\": \"Alice\", \"age\": 30}]");
        fs::write(&path, &bytes).unwrap();

        dkit()
            .args(&["convert", path.to_str().unwrap(), "--to", "html"])
            .assert()
            .success()
            .stdout(predicate::str::contains("<table>"))
            .stdout(predicate::str::contains("<td>Alice</td>"));
    }

    #[test]
    fn convert_latin1_csv_to_html_styled() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("latin1.csv");

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
                "--to",
                "html",
                "--styled",
                "--encoding",
                "latin1",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("José"))
            .stdout(predicate::str::contains("München"))
            .stdout(predicate::str::contains("style="));
    }
}

// ============================================================
// --list-formats 테스트
// ============================================================

mod list_formats {
    use super::*;

    #[test]
    fn list_formats_shows_md_and_html() {
        dkit()
            .args(&["--list-formats"])
            .assert()
            .success()
            .stdout(predicate::str::contains("md"))
            .stdout(predicate::str::contains("html"));
    }
}
