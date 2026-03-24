# dkit

**Swiss army knife for data format conversion and querying.**

Convert between JSON, CSV, YAML, TOML, XML, TSV, and MessagePack with a single CLI. Query nested data, compare files, preview as tables, and pipe everything together.

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

| Format      | Extensions              | Read | Write |
|-------------|------------------------|------|-------|
| JSON        | `.json`                | O    | O     |
| JSONL       | `.jsonl`, `.ndjson`    | O    | O     |
| CSV         | `.csv`                 | O    | O     |
| TSV         | `.tsv`                 | O    | O     |
| YAML        | `.yaml`, `.yml`        | O    | O     |
| TOML        | `.toml`                | O    | O     |
| XML         | `.xml`                 | O    | O     |
| MessagePack | `.msgpack`             | O    | O     |
| Parquet     | `.parquet`             | O    | O     |
| Excel       | `.xlsx`                | O    | -     |
| SQLite      | `.db`, `.sqlite`       | O    | -     |
| Markdown    | `.md`                  | -    | O     |
| HTML        |                        | -    | O     |

All conversion paths between supported read/write formats are available. Excel and SQLite are input-only formats. Markdown and HTML are output-only formats for table rendering.

## Commands

### `convert` — Format conversion

```bash
# Basic conversion
dkit convert data.json --to yaml
dkit convert users.csv --to json
dkit convert config.yaml --to toml
dkit convert config.toml --to json

# XML conversion
dkit convert config.xml --to json
dkit convert data.json --to xml
dkit convert config.xml --to yaml

# JSONL (JSON Lines) conversion
dkit convert users.json --to jsonl              # JSON array → one object per line
dkit convert users.jsonl --to json              # JSONL → JSON array
dkit convert logs.jsonl --to csv                # JSONL → CSV

# Output to file
dkit convert data.json --to csv -o output.csv

# Batch conversion
dkit convert *.csv --to json --outdir ./converted/

# Pipe from stdin
cat data.json | dkit convert --from json --to csv
cat logs.jsonl | dkit convert --from jsonl --to json

# Options
dkit convert data.json --to json --compact     # Minified JSON
dkit convert data.tsv --to json --delimiter '\t'  # TSV input
dkit convert data.csv --to json --no-header    # CSV without header
dkit convert data.json --to xml --root-element users  # Custom XML root element

# Markdown/HTML table output
dkit convert data.json --to md                 # GFM Markdown table
dkit convert data.csv --to html                # HTML table
dkit convert data.json --to html --styled      # HTML with inline CSS
dkit convert data.json --to html --full-html   # Complete HTML document
dkit convert data.json --to html --styled --full-html  # Styled full document

# Excel (.xlsx) input
dkit convert data.xlsx --to json                         # Convert Excel to JSON
dkit convert data.xlsx --to csv --sheet Products         # Specific sheet by name
dkit convert data.xlsx --to yaml --sheet 1               # Specific sheet by index
dkit view data.xlsx --list-sheets                        # List available sheets

# SQLite (.db, .sqlite) input
dkit convert data.db --to json                           # Convert SQLite to JSON
dkit convert data.db --to csv --table users              # Specific table
dkit convert data.db --to json --sql "SELECT * FROM users WHERE age > 25"  # Custom SQL
dkit view data.db --list-tables                          # List available tables

# Encoding support
dkit convert data.csv --to json --encoding euc-kr       # EUC-KR input
dkit convert data.csv --to json --encoding shift_jis     # Shift-JIS input
dkit convert data.csv --to json --detect-encoding        # Auto-detect encoding

# Parquet (.parquet) input/output
dkit convert data.parquet --to json                      # Parquet → JSON
dkit convert data.parquet --to csv                       # Parquet → CSV
dkit convert data.json --to parquet -o out.parquet       # JSON → Parquet
dkit convert data.csv --to parquet --compression snappy  # Parquet with Snappy compression
dkit convert data.csv --to parquet --compression zstd    # Parquet with Zstd compression

# Streaming mode for large files (chunk-based processing)
dkit convert large.jsonl --from jsonl -f csv --chunk-size 1000 -o out.csv
dkit convert large.csv --from csv -f jsonl --chunk-size 500 -o out.jsonl
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
dkit query data.json '.users[] | where age > 20 | select name, email'
dkit query data.json '.items[] | sort price desc | limit 5'
dkit query data.json '.users[] | where name contains "Kim"'

# Output query results in different formats
dkit query data.json '.users[]' --to csv -o users.csv
```

**Aggregate functions:**

| Function | Description | Example |
|----------|-------------|---------|
| `count` | Count elements | `.[] \| count` |
| `count field` | Count non-null values | `.[] \| count email` |
| `sum field` | Sum numeric field | `.[] \| sum price` |
| `avg field` | Average of numeric field | `.[] \| avg score` |
| `min field` | Minimum value | `.[] \| min price` |
| `max field` | Maximum value | `.[] \| max price` |
| `distinct field` | Unique values | `.[] \| distinct category` |

```bash
# Aggregate examples
dkit query data.csv '.[] | count'
dkit query data.csv '.[] | sum price'
dkit query data.json '.users[] | where age > 30 | avg score'
dkit query data.csv '.[] | distinct category'

# GROUP BY examples
dkit query data.csv '.[] | group_by category count(), sum(price)'
dkit query data.csv '.[] | group_by region min(price), max(price)'
dkit query data.csv '.[] | group_by category count() having count > 1'
dkit query data.csv '.[] | group_by category count() | sort count desc | limit 5'
```

**Built-in functions (usable in `select`):**

| Category | Functions |
|----------|-----------|
| String | `upper()`, `lower()`, `trim()`, `ltrim()`, `rtrim()`, `length()`, `substr()`, `concat()`, `replace()`, `split()` |
| Math | `round()`, `ceil()`, `floor()`, `abs()`, `sqrt()`, `pow()` |
| Date | `now()`, `date()`, `year()`, `month()`, `day()` |
| Type | `to_int()`, `to_float()`, `to_string()`, `to_bool()` |
| Util | `coalesce()`, `if_null()` |

```bash
# Function examples
dkit query data.csv '.[] | select upper(name), round(price, 2)'
dkit query data.json '.users[] | select upper(trim(name)) as NAME, year(created_at)'
dkit query data.csv '.[] | where score > 80 | select name, to_string(score)'
dkit query data.json '.[] | select name, coalesce(email, "N/A")'
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

# Table customization
dkit view data.csv --border rounded --color        # Rounded borders with type coloring
dkit view data.json --row-numbers --max-width 30   # Row numbers, truncate long values
dkit view data.json --hide-header --border none     # Minimal output
dkit view data.json --border heavy -n 10            # Heavy borders, limit 10 rows

# Output in different formats
dkit view data.json --format json                  # JSON output instead of table
dkit view data.json --format md                    # Markdown table
dkit view data.json --format html                  # HTML table
```

### `stats` — Data statistics

```bash
# Show overall statistics
dkit stats data.csv

# Navigate to nested data
dkit stats data.json --path .users

# Statistics for a specific column (numeric: sum, avg, median, std, p25, p75)
dkit stats data.csv --column revenue

# String column stats (unique count, length distribution, top values)
dkit stats data.csv --column category

# Histogram visualization
dkit stats data.csv --column age --histogram

# Output formats
dkit stats data.csv --format json
dkit stats data.csv --format md
```

### `schema` — Data structure inspection

```bash
# Show schema as a tree
dkit schema config.yaml
dkit schema data.json

# From stdin
cat data.json | dkit schema - --from json
```

### `diff` — Compare data files

```bash
# Compare same-format files
dkit diff old.json new.json
dkit diff config_dev.yaml config_prod.yaml

# Cross-format comparison
dkit diff data.json data.yaml

# Compare nested path only
dkit diff old.json new.json --path '.database'

# Quiet mode (exit code: 0=same, 1=different)
dkit diff a.json b.json --quiet && echo 'same' || echo 'different'

# Comparison modes
dkit diff a.json b.json --mode value          # Value changes only
dkit diff a.json b.json --mode key            # Key existence only

# Output formats
dkit diff a.json b.json --diff-format json           # JSON output
dkit diff a.json b.json --diff-format side-by-side    # Side-by-side view
dkit diff a.json b.json --diff-format summary         # Summary only

# Array comparison strategies
dkit diff a.json b.json --array-diff value            # Match by value
dkit diff a.json b.json --array-diff key=id           # Match by key field

# Ignore options
dkit diff a.json b.json --ignore-order                # Ignore array order
dkit diff a.json b.json --ignore-case                 # Ignore string case
```

### `validate` — JSON Schema validation

```bash
# Validate data against JSON Schema
dkit validate data.json --schema schema.json
dkit validate data.yaml --schema schema.json
dkit validate data.toml --schema schema.json

# Quiet mode (only valid/invalid)
dkit validate data.json --schema schema.json --quiet

# From stdin
cat data.json | dkit validate - --schema schema.json --from json
```

### `sample` — Random/stratified sampling

```bash
# Random sampling
dkit sample data.csv -n 100                    # 100 random records
dkit sample data.json --ratio 0.1              # 10% sample
dkit sample data.csv -n 50 --seed 42           # Reproducible sampling

# Systematic sampling (every k-th element)
dkit sample data.csv -n 100 --method systematic

# Stratified sampling (proportional per group)
dkit sample data.csv -n 50 --method stratified --stratify-by category

# Output format
dkit sample data.csv -n 100 -f json -o sample.json
```

### `flatten` / `unflatten` — Flatten/restore nested structures

```bash
# Flatten nested JSON
dkit flatten data.json                         # {"a.b.c": 1}
dkit flatten data.json --separator '/'         # {"a/b/c": 1}
dkit flatten data.json --array-format bracket  # {"items[0].name": "Alice"}
dkit flatten data.json --max-depth 2           # Limit depth

# Unflatten (restore nested structure)
dkit unflatten flat.json                       # {"a": {"b": {"c": 1}}}
dkit unflatten flat.json --separator '/'

# Roundtrip
dkit flatten data.json -o flat.json && dkit unflatten flat.json
```

### `config` — Configuration management

```bash
# Show current effective configuration (with source information)
dkit config show

# Create a default user config file
dkit config init

# Create a project-level config file (.dkit.toml in current directory)
dkit config init --project
```

Config file priority (highest to lowest):
1. CLI options
2. Project config (`.dkit.toml` in current directory)
3. User config (`$XDG_CONFIG_HOME/dkit/config.toml` or `~/.dkit.toml`)
4. Defaults

### `alias` — Command aliases

```bash
# List all aliases (built-in + user-defined)
dkit alias list

# Register a user alias
dkit alias set <NAME> <COMMAND>

# Remove a user alias
dkit alias remove <NAME>

# Use a built-in alias (j2c, c2j, j2y, y2j, j2t, t2j, c2y, y2c)
dkit j2c data.json          # JSON → CSV
dkit c2j data.csv           # CSV → JSON
dkit y2j config.yaml        # YAML → JSON
```

### `completions` — Shell completion scripts

```bash
# Generate and install shell completions
dkit completions bash > ~/.bash_completion.d/dkit && source ~/.bash_completion.d/dkit
dkit completions zsh > ~/.zfunc/_dkit
dkit completions fish > ~/.config/fish/completions/dkit.fish
dkit completions powershell > dkit.ps1 && . ./dkit.ps1
```

### Watch mode

`convert` and `view` support `--watch` to automatically re-run on file changes:

```bash
dkit convert data.json --to csv --watch           # Re-convert on change
dkit view data.csv --watch                        # Refresh table on change
dkit convert data.json --to yaml --watch --watch-path ./templates/  # Watch extra path
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
| CSV/TSV | O | X | O | X |
| YAML | O | X | X | O |
| TOML | O | X | X | X |
| XML | O | X | X | O |
| MessagePack | O | X | X | X |
| Parquet | O | X | X | X |
| Excel (.xlsx) input | O | X | X | X |
| SQLite input | O | X | X | X |
| Markdown/HTML output | O | X | X | X |
| Cross-format convert | O | X | Partial | Partial |
| Table output | O | X | O | X |
| Query (where/select/sort) | O | O | O | O |
| Aggregate functions | O | O | O | X |
| GROUP BY | O | Partial | O | X |
| Built-in functions | O | O | O | X |
| Pipeline chaining | O | O | O | X |
| Streaming (large files) | O | X | O | X |
| Statistics | O | X | O | X |
| Schema inspection | O | X | X | X |
| File merging | O | X | O | X |
| File diff (modes/formats) | O | X | X | X |
| JSON Schema validation | O | X | X | X |
| Random/stratified sampling | O | X | X | X |
| Flatten/unflatten | O | X | X | X |
| Multi-encoding support | O | X | X | X |
| Watch mode (auto re-run) | O | X | X | X |
| Config file | O | X | X | X |
| Command aliases | O | X | X | X |
| Shell completions | O | O | O | O |
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
