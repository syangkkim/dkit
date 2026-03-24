# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-03-24

### Added

- **Excel (.xlsx) Reader**: Read Excel files via `calamine` crate with sheet selection (`--sheet <name|index>`), cell type auto-conversion (numbers, strings, dates, booleans), and sheet listing (`dkit view data.xlsx --list-sheets`). (#85)
- **SQLite Reader**: Read SQLite database tables with `--table` option, custom SQL queries (`--sql`), table listing (`dkit view data.db --list-tables`), and read-only mode for safety. (#86)
- **stdin/stdout pipeline streaming**: Support reading from stdin and writing to stdout for Unix-style piping (`cat data.json | dkit convert -f csv`), with `-` as explicit stdin marker and content sniffing auto-detection. (#87)
- **Batch convert (multi-file)**: Convert multiple files at once using glob patterns (`dkit convert *.json -f csv`) or directories, with output directory (`-o`), filename patterns (`--rename`), progress display, and `--continue-on-error` option. (#88)
- **Sort and filter options for convert/view**: `--sort-by`, `--sort-order`, `--head`, `--tail`, and `--where` options for quick data manipulation without a full query. (#89)

### Testing & Docs

- **Comprehensive v0.6.0 integration tests**: End-to-end tests for Excel reading, SQLite reading, pipeline chaining, batch conversion, and sort/filter options. (#90)

## [0.5.0] - 2026-03-23

### Added

- **Markdown table output**: GFM-compatible Markdown table format with automatic numeric column right-alignment and pipe character escaping. (`dkit convert data.json -f md`) (#78)
- **HTML table output**: HTML `<table>` output with optional inline CSS styling (`--styled`) and full HTML document mode (`--full-html`). (#79)
- **Table view customization**: New options for `view` subcommand — `--max-width`, `--no-header`, `--columns`, `--row-numbers`, `--border` (none/simple/rounded/heavy), and `--color` for type-based coloring. (#80)
- **Encoding support**: Handle non-UTF-8 input files with `--encoding` option (e.g., euc-kr, shift-jis, latin1), BOM auto-detection, and optional encoding auto-detection (`--detect-encoding`) via `encoding_rs` and `chardetng` crates. (#82)

### Changed

- **Unified `--format` / `-f` option**: Standardized output format selection across all subcommands. Supports json, csv, tsv, yaml, toml, xml, jsonl, md, html, and table formats. Added `dkit --list-formats` to show available formats. (#81)

### Testing & Docs

- **Comprehensive v0.5.0 integration tests**: End-to-end tests for Markdown/HTML output, table customization options, encoding conversion, and format selection. (#83)

## [0.4.0] - 2026-03-23

### Added

- **XML Reader**: Parse XML files into `Value` with namespace handling (`--ns-mode strip/preserve`) and attribute support. (#71)
- **XML Writer**: Serialize `Value` to XML with `--root-element` option for custom root element names. (#72)
- **JSONL (JSON Lines) format**: Read and write `.jsonl` files — one JSON object per line. (#73)
- **Content sniffing for format detection**: Auto-detect stdin format via content sniffing in addition to file extension detection. (#74)
- **`convert` subcommand XML/JSONL integration**: Full support for XML and JSONL in all conversion paths. (#75)
- **Comprehensive v0.4.0 integration tests**: End-to-end tests for XML, JSONL formats across all subcommands. (#76)

## [0.3.0] - 2026-03-21

### Added

- **XML format support**: Read and write XML files via `quick-xml` crate. Supports conversion between XML and all other formats. (#22)
- **TSV format support**: Tab-separated values with automatic `.tsv` extension detection. Leverages CSV Reader's delimiter option. (#23)
- **MessagePack format support**: Binary MessagePack format via `rmp-serde` crate. Read and write `.msgpack` files. (#24)
- **`diff` subcommand**: Compare two data files and display differences with colored output (added: green, removed: red, changed: yellow). Supports cross-format comparison, `--path` for nested data, and `--quiet` mode. (#35)
- **Comprehensive v0.3.0 integration tests**: End-to-end tests for XML, TSV, MessagePack formats, diff subcommand, and cross-format conversions. (#52)

## [0.2.0] - 2026-03-21

### Added

- **`stats` subcommand**: Display basic statistics for data columns — type, count, sum, avg, min, max, median. Supports `--path` and `--column` options. (#19)
- **`schema` subcommand**: Show data structure as a tree with type inference (├─/└─ format). (#20)
- **`merge` subcommand**: Combine multiple data files into one — supports array concatenation, object merging, and cross-format merging with `--to` output format. (#21)
- **Query `--to` and `-o` options**: Output query results in any supported format and write to file. (#18)
- **Query `where` clause**: Filter data with comparison operators (`==`, `!=`, `>`, `<`, `>=`, `<=`), string operators (`contains`, `starts_with`, `ends_with`), and logical operators (`and`, `or`). (#45, #46)
- **Query `select` clause**: Extract specific fields/columns from query results. (#47)
- **Query `sort`/`limit` clauses**: Sort results by field (ascending/descending) and limit output count. (#48)
- **Query pipeline chaining (`|`)**: Chain multiple query operations sequentially, passing results from left to right. (#49)
- **Comprehensive v0.2.0 integration tests**: End-to-end tests for all new query operations, subcommands, and edge cases. (#50)

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

[0.6.0]: https://github.com/syangkkim/dkit/releases/tag/v0.6.0
[0.5.0]: https://github.com/syangkkim/dkit/releases/tag/v0.5.0
[0.4.0]: https://github.com/syangkkim/dkit/releases/tag/v0.4.0
[0.3.0]: https://github.com/syangkkim/dkit/releases/tag/v0.3.0
[0.2.0]: https://github.com/syangkkim/dkit/releases/tag/v0.2.0
[0.1.0]: https://github.com/syangkkim/dkit/releases/tag/v0.1.0
