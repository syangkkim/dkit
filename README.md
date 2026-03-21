# dkit

**Swiss army knife for data format conversion and querying.**

Convert between JSON, CSV, YAML, and TOML with a single CLI. Query nested data, preview as tables, and pipe everything together.

## Quick Start

```bash
# Install
cargo install dkit

# Convert JSON to YAML
dkit convert data.json --to yaml

# Query nested data
dkit query config.yaml '.database.host'

# Preview CSV as a table
dkit view users.csv --limit 10
```

## Installation

### From crates.io

```bash
cargo install dkit
```

### From source

```bash
git clone https://github.com/syangkkim/dkit.git
cd dkit
cargo install --path .
```

## Supported Formats

| Format | Extensions | Read | Write |
|--------|-----------|------|-------|
| JSON   | `.json`   | O    | O     |
| CSV    | `.csv`    | O    | O     |
| YAML   | `.yaml`, `.yml` | O | O  |
| TOML   | `.toml`   | O    | O     |

All 12 conversion paths (4 x 3) are supported.

## Commands

### `convert` — Format conversion

```bash
# Basic conversion
dkit convert data.json --to yaml
dkit convert users.csv --to json
dkit convert config.yaml --to toml
dkit convert config.toml --to json

# Output to file
dkit convert data.json --to csv -o output.csv

# Batch conversion
dkit convert *.csv --to json --outdir ./converted/

# Pipe from stdin
cat data.json | dkit convert --from json --to csv

# Options
dkit convert data.json --to json --compact     # Minified JSON
dkit convert data.tsv --to json --delimiter '\t'  # TSV input
dkit convert data.csv --to json --no-header    # CSV without header
```

### `query` — Data querying

```bash
# Field access
dkit query config.yaml '.database.host'
dkit query config.toml '.server.port'

# Nested path
dkit query data.json '.users[0].name'

# Array iteration
dkit query data.json '.users[].email'

# Negative indexing
dkit query data.json '.items[-1]'
```

**Query syntax:**

| Syntax | Description |
|--------|-------------|
| `.field` | Object field access |
| `.field.sub` | Nested field access |
| `.[0]` | Array index (0-based) |
| `.[-1]` | Negative index (from end) |
| `.[]` | Iterate all elements |
| `where .field == value` | Filter with comparison (`==`, `!=`, `>`, `<`, `>=`, `<=`) |
| `where .field contains "str"` | Filter with string operators (`contains`, `starts_with`, `ends_with`) |
| `select .field1, .field2` | Select specific fields |
| `sort .field` / `sort .field desc` | Sort by field (ascending/descending) |
| `limit N` | Limit number of results |
| `\|` | Pipeline chaining (pass results between operations) |

```bash
# Advanced query examples
dkit query data.json '.users[] | where .age > 20 | select .name, .email'
dkit query data.json '.items[] | sort .price desc | limit 5'
dkit query data.json '.users[] | where .name contains "Kim"'

# Output query results in different formats
dkit query data.json '.users[]' --to csv -o users.csv
```

### `view` — Table preview

```bash
# View as table
dkit view users.csv

# Limit rows
dkit view large_data.csv --limit 20

# Navigate nested data
dkit view data.json --path '.users'

# Select columns
dkit view users.csv --columns name,email
```

### `stats` — Data statistics

```bash
# Show overall statistics
dkit stats data.csv

# Navigate to nested data
dkit stats data.json --path .users

# Statistics for a specific column
dkit stats data.csv --column revenue
```

### `schema` — Data structure inspection

```bash
# Show schema as a tree
dkit schema config.yaml
dkit schema data.json

# From stdin
cat data.json | dkit schema - --from json
```

### `merge` — Combine multiple files

```bash
# Merge JSON files
dkit merge a.json b.json --to json

# Merge CSV files and convert to JSON
dkit merge users1.csv users2.csv --to json -o merged.json

# Merge YAML configs
dkit merge config1.yaml config2.yaml --to yaml
```

## Comparison with Existing Tools

| Feature | dkit | jq | miller | yq |
|---------|------|-----|--------|----|
| JSON | O | O | O | O |
| CSV | O | X | O | X |
| YAML | O | X | X | O |
| TOML | O | X | X | X |
| Cross-format convert | O | X | Partial | Partial |
| Table output | O | X | O | X |
| Query (where/select/sort) | O | O | O | O |
| Pipeline chaining | O | O | O | X |
| Statistics | O | X | O | X |
| Schema inspection | O | X | X | X |
| File merging | O | X | O | X |
| Single binary | O | O | O | O |

dkit focuses on **seamless conversion between all supported formats** with a unified query syntax, eliminating the need for separate tools per format.

## Building from Source

```bash
cargo build              # Build
cargo test               # Run tests
cargo clippy -- -D warnings  # Lint
cargo fmt -- --check     # Format check
```

## Contributing

Contributions are welcome! Please see the [GitHub Issues](https://github.com/syangkkim/dkit/issues) for planned features and known issues.

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Commit your changes
4. Push to the branch and open a Pull Request

Please ensure `cargo test` and `cargo clippy -- -D warnings` pass before submitting.

## License

[MIT](LICENSE)
