# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-20

### Added

- **Core data model**: `Value` enum for unified representation of JSON, CSV, YAML, and TOML data, backed by `IndexMap` for key-order preservation.
- **Error handling**: `DkitError` type using `thiserror` with structured error variants for parse, format, IO, query, and CLI errors.
- **Format support**: `FormatReader` and `FormatWriter` traits with implementations for:
  - JSON (via `serde_json`)
  - CSV (via `csv` crate, with `--delimiter` and `--no-header` options)
  - YAML (via `serde_yaml`)
  - TOML (via `toml` crate)
- **`convert` subcommand**: Convert between any combination of JSON, CSV, YAML, and TOML (12 conversion paths). Supports stdin/stdout piping, output file (`-o`), batch mode (`--outdir`), `--compact`, and `--pretty` options.
- **`query` subcommand**: Query data using path expressions with field access (`.field`), nested paths (`.a.b.c`), array indexing (`.[0]`, `.[-1]`), and array iteration (`.[].field`). Supports `--to` for output format conversion.
- **`view` subcommand**: Display data as a formatted table in the terminal using `comfy-table`.
- **Automatic format detection**: Detect input format from file extension; require `--from` flag for stdin.
- **User-friendly error messages**: Colored output with contextual hints using `colored`.
- **CI pipeline**: GitHub Actions workflow with test (Linux/macOS/Windows), clippy, and rustfmt checks.
- **Test suite**: Integration tests covering all conversion paths, query operations, table view, fixture data, edge cases (unicode, empty input, quoted CSV fields).

[0.1.0]: https://github.com/syangkkim/dkit/releases/tag/v0.1.0
