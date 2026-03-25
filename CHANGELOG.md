# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-03-25

### Added

- Comprehensive user documentation: README overhaul, tutorial, cookbook, migration guide, and contributing guide.
- **v1.0.0 release**: First stable release of dkit with all planned features.

## [0.9.0] - 2026-03-24

### Added

- **Config file support**: User-level (`~/.dkit.toml`) and project-level (`.dkit.toml`) configuration with priority: CLI > project > user > defaults. Settings include `default_format`, `table.border_style`, `table.max_width`, `color`, and `encoding`. `dkit config show` and `dkit config init` subcommands. (#107)
- **Shell completion scripts**: Generate shell autocompletion scripts via `dkit completions <bash|zsh|fish|powershell>` using `clap_complete`. Supports subcommand, option, format name, and file path completion. (#108)
- **`--watch` mode**: File change detection for automatic re-execution — `dkit convert data.json -f csv --watch` and `dkit view data.csv --watch`. Built on `notify` crate with debouncing and Ctrl+C exit. (#109)
- **Enhanced error messages**: Line/column numbers in parse errors, code snippet with arrow pointer, "Did you mean?" suggestions for field names/subcommands/options, format mismatch hints, `--verbose` flag for stack trace output. Powered by `miette`. (#110)
- **Alias system**: Define and use command aliases — `dkit alias set j2c "convert --from json --to csv"`, `dkit j2c data.json`. Aliases stored in `~/.dkit.toml` under `[aliases]`. Built-in aliases: `j2c`, `c2j`, `j2y`, `y2j`, etc. `alias list` and `alias remove` subcommands. (#111)
- **Criterion benchmark suite**: Benchmarks for format I/O (1MB/10MB/100MB), query execution (filter/aggregate/sort), and format conversion (JSON↔CSV, JSON↔Parquet). Three benchmark binaries: `format_benchmarks`, `query_benchmarks`, `conversion_benchmarks`. (#112)

### Testing & Docs

- **Comprehensive v0.9.0 integration tests**: End-to-end tests for config file loading/priority, shell completion generation, watch mode, error messages, and alias system. README and docs updated. (#113)

## [0.8.0] - 2026-03-24

### Added

- **diff subcommand enhancements**: Structural comparison modes (`--mode structural|value|key`), multiple output formats (`--diff-format unified|side-by-side|json|summary`), array diff strategies (`--array-diff index|value|key=<field>`), `--ignore-order`, `--ignore-case` options, and script-friendly exit codes. (#100)
- **validate subcommand**: JSON Schema Draft 7 validation via `jsonschema` crate. Validates any supported format by converting to Value internally. Multi-error collection, `--quiet` mode, and exit code 0/1 for valid/invalid. (#101)
- **stats subcommand extensions**: Per-field detailed statistics — numeric fields get std/percentiles, string fields get length stats/unique counts/top values, null ratio calculation, type consistency checks, `--histogram` for text-based histograms. (#102)
- **sample subcommand**: Data sampling with `--method random|systematic|stratified`, count (`-n`) or ratio (`--ratio`), reproducible `--seed`, and stratified sampling via `--stratify-by`. (#103)
- **flatten/unflatten subcommands**: Flatten nested JSON structures (`{"a":{"b":1}}` → `{"a.b":1}`) and restore. Configurable separator (`--separator`), array format (`--array-format index|bracket`), and max depth (`--max-depth`). (#104)

### Testing & Docs

- **Comprehensive v0.8.0 integration tests**: End-to-end tests for diff enhancements, validate, stats extensions, sample, and flatten/unflatten. README and docs updated. (#105)

## [0.7.0] - 2026-03-24

### Added

- **Parquet Reader**: Read Apache Parquet files via `arrow` + `parquet` crates with column-to-row conversion, nested schema support (Struct, List, Map), and Row Group-based reading for memory efficiency. (#92)
- **Parquet Writer**: Write `Value` arrays to Parquet files with automatic schema inference, compression options (`--compression snappy|gzip|zstd|none`), and configurable Row Group size. (#93)
- **Query aggregate functions**: `count()`, `sum()`, `avg()`, `min()`, `max()`, `distinct()` for data summarization in query expressions. (#94)
- **Query GROUP BY clause**: Group-by aggregation with multi-key support, HAVING clause for group filtering, and result sorting. (#95)
- **Streaming chunk-based read/write**: Process large files without loading entirely into memory — line-based streaming for JSONL/CSV/TSV, Row Group streaming for Parquet, with `--chunk-size` and `--progress` options. (#96)
- **Extended query functions**: String functions (`upper`, `lower`, `trim`, `length`, `substr`, `concat`, `replace`), math functions (`round`, `ceil`, `floor`, `abs`), date functions (`now`, `date`, `year`, `month`, `day`), and type conversion (`to_int`, `to_float`, `to_string`, `to_bool`) with nested call support. (#97)

### Testing & Docs

- **Comprehensive v0.7.0 integration tests**: End-to-end tests for Parquet read/write, aggregate functions, GROUP BY, streaming, and extended query functions. README updated. (#98)

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

[Unreleased]: https://github.com/syangkkim/dkit/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/syangkkim/dkit/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/syangkkim/dkit/releases/tag/v0.9.0
[0.8.0]: https://github.com/syangkkim/dkit/releases/tag/v0.8.0
[0.7.0]: https://github.com/syangkkim/dkit/releases/tag/v0.7.0
[0.6.0]: https://github.com/syangkkim/dkit/releases/tag/v0.6.0
[0.5.0]: https://github.com/syangkkim/dkit/releases/tag/v0.5.0
[0.4.0]: https://github.com/syangkkim/dkit/releases/tag/v0.4.0
[0.3.0]: https://github.com/syangkkim/dkit/releases/tag/v0.3.0
[0.2.0]: https://github.com/syangkkim/dkit/releases/tag/v0.2.0
[0.1.0]: https://github.com/syangkkim/dkit/releases/tag/v0.1.0
