# dkit

**A unified CLI to convert, query, and explore data across formats.**

Convert between JSON, CSV, YAML, TOML, XML, Parquet, and more with a single CLI. Query nested data, compare files, preview as tables, and pipe everything together.

## Quick Start

```bash
# Convert JSON to CSV
dkit convert data.json --to csv

# Query nested data
dkit query config.yaml '.database.host'

# Filter and aggregate
dkit query sales.csv '.[] | where region == "US" | sum revenue'

# Preview as a table
dkit view users.csv --limit 10 --border rounded --color

# Compare two configs
dkit diff config_dev.yaml config_prod.yaml
```

## Installation

### From crates.io

```bash
cargo install dkit
```

### From source

```bash
git clone https://github.com/syang0531/dkit.git
cd dkit
cargo install --path dkit-cli
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/syang0531/dkit/releases).

### With cargo-binstall

```bash
cargo binstall dkit
```

### Shell completions

After installing, generate shell completions for tab-completion support:

```bash
# Bash
dkit completions bash > ~/.bash_completion.d/dkit && source ~/.bash_completion.d/dkit

# Zsh
dkit completions zsh > ~/.zfunc/_dkit

# Fish
dkit completions fish > ~/.config/fish/completions/dkit.fish

# PowerShell
dkit completions powershell > dkit.ps1 && . ./dkit.ps1
```

## Supported Formats

| Format      | Extensions             | Read | Write | Notes                          |
|-------------|------------------------|:----:|:-----:|--------------------------------|
| JSON        | `.json`                |  ✓   |   ✓   | Pretty / compact output        |
| JSONL       | `.jsonl`, `.ndjson`    |  ✓   |   ✓   | One JSON object per line       |
| CSV         | `.csv`                 |  ✓   |   ✓   | Configurable delimiter         |
| TSV         | `.tsv`                 |  ✓   |   ✓   | Tab-separated values           |
| YAML        | `.yaml`, `.yml`        |  ✓   |   ✓   |                                |
| TOML        | `.toml`                |  ✓   |   ✓   |                                |
| XML         | `.xml`                 |  ✓   |   ✓   | Namespace handling, attributes |
| MessagePack | `.msgpack`             |  ✓   |   ✓   | Binary format                  |
| Parquet     | `.parquet`             |  ✓   |   ✓   | Snappy / Gzip / Zstd compression |
| Excel       | `.xlsx`                |  ✓   |   —   | Sheet selection, input-only    |
| SQLite      | `.db`, `.sqlite`       |  ✓   |   —   | Custom SQL queries, input-only |
| INI/CFG     | `.ini`, `.cfg`         |  ✓   |   ✓   | Section-based config files     |
| .properties | `.properties`          |  ✓   |   ✓   | Java properties files          |
| .env        | `.env`                 |  ✓   |   ✓   | Environment variable files     |
| HCL         | `.hcl`, `.tf`, `.tfvars` |  ✓   |   ✓   | Terraform / HashiCorp configs  |
| plist       | `.plist`               |  ✓   |   ✓   | macOS Property List (XML)      |
| Markdown    | `.md`                  |  —   |   ✓   | GFM table, output-only         |
| HTML        | `.html`                |  —   |   ✓   | Styled tables, output-only     |

All conversion paths between readable and writable formats are supported. Format is auto-detected from file extension, or via content sniffing for stdin.

## Commands Overview

| Command       | Description                                      |
|---------------|--------------------------------------------------|
| `convert`     | Convert between any supported formats             |
| `query`       | Query and transform data with expressions         |
| `join`        | Join two files on a common key (inner/left/right/full) |
| `profile`     | Profile data quality (types, nulls, cardinality)  |
| `view`        | Preview data as a formatted table                 |
| `stats`       | Show statistics (count, avg, percentiles, etc.)   |
| `schema`      | Inspect data structure as a tree                  |
| `diff`        | Compare two data files (cross-format supported)   |
| `merge`       | Combine multiple files into one                   |
| `validate`    | Validate data against JSON Schema                 |
| `sample`      | Random / systematic / stratified sampling         |
| `flatten`     | Flatten nested structures (`a.b.c` keys)          |
| `unflatten`   | Restore flattened structures                      |
| `config`      | Manage user / project configuration               |
| `alias`       | Define and use command shortcuts                  |
| `completions` | Generate shell completion scripts                 |

## Usage

### `convert` — Format conversion

```bash
# Basic conversion (format detected from extension)
dkit convert data.json --to yaml
dkit convert users.csv --to json -o users.json

# Batch conversion
dkit convert *.csv --to json --outdir ./converted/

# Pipe from stdin
cat data.json | dkit convert --from json --to csv

# Format-specific options
dkit convert data.json --to json --compact           # Minified JSON
dkit convert data.json --to json --indent 4          # 4-space indentation
dkit convert data.json --to json --sort-keys         # Alphabetically sorted keys
dkit convert data.csv --to json --no-header           # CSV without header row
dkit convert data.json --to xml --root-element items  # Custom XML root
dkit convert data.json --to html --styled --full-html # Styled HTML document

# Excel / SQLite input
dkit convert data.xlsx --to json --sheet Products
dkit convert data.db --to csv --sql "SELECT * FROM users WHERE age > 25"

# Parquet with compression
dkit convert data.csv --to parquet --compression zstd -o out.parquet

# Encoding support
dkit convert data.csv --to json --encoding euc-kr
dkit convert data.csv --to json --detect-encoding

# Streaming for large files
dkit convert large.jsonl --from jsonl -f csv --chunk-size 1000 -o out.csv

# Deduplication
dkit convert data.json --to csv --unique             # Remove duplicate records
dkit convert data.json --to csv --unique-by email    # Deduplicate by field

# Computed fields and transformations
dkit convert data.json --to json --add-field 'total = price * qty'
dkit convert data.json --to json --map 'name = upper(name)'

# Column selection and aggregation
dkit convert data.json --to csv --select 'name, email'
dkit convert sales.csv --to json --group-by category --agg 'count(), sum(amount)'
dkit convert data.json --to csv --select 'name, age' --filter 'age > 30' --sort-by age

# Dry-run (preview without writing)
dkit convert huge.json --to csv -o output.csv --dry-run

# INI / .properties format
dkit convert config.ini --to json                    # INI → JSON
dkit convert app.properties --to yaml                # Properties → YAML

# .env format
dkit convert .env --to json                         # .env → JSON
dkit convert config.json --to env -o .env            # JSON → .env
dkit diff .env.dev .env.prod                         # Compare environments

# HCL / Terraform format
dkit convert main.tf --to json                      # Terraform → JSON
dkit convert variables.json --to hcl -o vars.tf     # JSON → HCL
dkit query main.tf '.resource.aws_instance'         # Query Terraform config

# plist (macOS Property List) format
dkit convert Info.plist --to json                   # plist → JSON
dkit convert config.json --to plist -o Info.plist   # JSON → plist
dkit query Info.plist '.CFBundleVersion'             # Query plist value

# Explode (unnest arrays into rows)
dkit convert data.json --to csv --explode tags      # Unnest array field

# Pivot / Unpivot (reshape data)
dkit convert wide.csv --to json --unpivot 'jan,feb,mar' --key month --value sales
dkit convert long.csv --to json --pivot --index name --columns month --values sales

# Watch mode (re-convert on file change)
dkit convert data.json --to csv --watch

# Log file parsing
dkit convert access.log --to json --log-format apache        # Apache log → JSON
dkit convert app.log --to csv --log-format nginx             # nginx log → CSV
dkit convert app.log --to json --log-format syslog           # syslog → JSON
dkit convert app.log --to json --log-format '{timestamp} [{level}] {message}'  # Custom pattern
dkit convert app.log --to json --log-format apache --log-error raw  # Include unparseable lines

# Parallel batch conversion
dkit convert *.json --to csv --outdir ./out/ --parallel 4    # 4 threads
dkit convert *.json --to csv --outdir ./out/ --parallel auto # Auto-detect CPU cores

# Execution timing (available on all commands)
dkit convert data.json --to csv --time
dkit query data.json '.users[]' --time
```

### `query` — Data querying

```bash
# Field access
dkit query config.yaml '.database.host'
dkit query data.json '.users[0].name'

# Array iteration and filtering
dkit query data.json '.users[] | where age > 20 | select name, email'
dkit query data.json '.items[] | sort price desc | limit 5'
dkit query data.json '.users[] | where name contains "Kim"'

# Array slicing and wildcards
dkit query data.json '.[0:3]'                        # First 3 elements
dkit query data.json '.[-2:]'                        # Last 2 elements
dkit query data.json '.[*].name'                     # All names (wildcard)

# IN / NOT IN, matches operators
dkit query data.json '.[] | where status in ("active", "pending")'
dkit query data.json '.[] | where name matches "^[A-C]"'

# Recursive descent (..) — find keys at any depth
dkit query nested.json '..email'                     # All 'email' fields
dkit query terraform.json '..instance_type'          # Deep key search

# Conditional expressions
dkit query data.json '.[] | select name, if(age < 18, "minor", "adult") as category'
dkit query data.json '.[] | select name, case when score >= 90 then "A" when score >= 70 then "B" else "C" end as grade'

# Aggregate functions
dkit query data.csv '.[] | count'
dkit query data.csv '.[] | sum price'
dkit query data.csv '.[] | avg score'

# GROUP BY
dkit query data.csv '.[] | group_by category count(), sum(price)'
dkit query data.csv '.[] | group_by category count() having count > 1'

# Built-in functions in select
dkit query data.csv '.[] | select upper(name), round(price, 2)'
dkit query data.json '.[] | select name, coalesce(email, "N/A")'

# Output in any format
dkit query data.json '.users[]' --to csv -o users.csv

# Query execution plan (--explain)
dkit query data.json '.[] | where age > 20 | select name | sort name desc | limit 5' --explain

# Query log files
dkit query access.log '.[] | where status == 404' --log-format apache
```

**Query syntax reference:**

| Syntax | Description |
|--------|-------------|
| `.field` | Object field access |
| `.field.sub` | Nested field access |
| `.[0]`, `.[-1]` | Array index (0-based, negative from end) |
| `.[]` | Iterate all elements |
| `.[*]` | Wildcard (same as `[]`) |
| `.[0:3]`, `.[-2:]`, `.[::2]` | Array slicing (start:end:step) |
| `..key` | Recursive descent (find key at any depth) |
| `where .field == value` | Filter (`==`, `!=`, `>`, `<`, `>=`, `<=`) |
| `where .field contains "str"` | String filter (`contains`, `starts_with`, `ends_with`) |
| `where .field in ("a", "b")` | Membership filter (`in`, `not in`) |
| `where .field matches "regex"` | Regex filter (`matches`, `not matches`) |
| `select .f1, .f2` | Select specific fields |
| `sort .field [desc]` | Sort ascending/descending |
| `limit N` | Limit results |
| `\|` | Pipeline chaining |

**Built-in functions:**

| Category | Functions |
|----------|-----------|
| String   | `upper`, `lower`, `trim`, `ltrim`, `rtrim`, `length`, `substr`, `concat`, `replace`, `split`, `index_of`, `rindex_of`, `starts_with`, `ends_with`, `reverse`, `repeat`, `pad_left`, `pad_right` |
| Math     | `round`, `ceil`, `floor`, `abs`, `sqrt`, `pow` |
| Date     | `now`, `date`, `year`, `month`, `day` |
| Type     | `to_int`, `to_float`, `to_string`, `to_bool` |
| Util     | `coalesce`, `if_null` |
| Conditional | `if(cond, then, else)`, `case when ... then ... end` |

**Aggregate functions:** `count`, `sum`, `avg`, `min`, `max`, `distinct`, `median`, `percentile`, `stddev`, `variance`, `mode`, `group_concat`

See [Query Syntax Reference](docs/query-syntax.md) for the full grammar and more examples.

### `join` — Cross-file data joining

```bash
# Inner join on same key name
dkit join users.json orders.json --on user_id -f json

# Left join with different key names
dkit join users.csv orders.csv --on id=user_id --type left -f csv

# Cross-format join (YAML + CSV)
dkit join users.yaml transactions.csv --on email -f table

# Full outer join
dkit join a.json b.json --on id --type full -f json --pretty
```

### `profile` — Data profiling

```bash
dkit profile data.csv                            # Table output
dkit profile data.csv --output-format json        # JSON output for scripting
dkit profile data.csv --detailed                  # Detailed stats (histograms, outliers)
dkit profile data.json                            # JSON input
dkit profile data.csv --output-format md           # Markdown output
```

### `query` — Window functions

```bash
# Ranking
dkit query sales.json '.[] | select name, revenue, rank() over (order by revenue desc) as rank'

# Partition-based ranking
dkit query sales.json '.[] | select dept, name, revenue, row_number() over (partition by dept order by revenue desc) as dept_rank'

# Previous/next row reference
dkit query timeseries.json '.[] | select date, value, lag(value, 1) over (order by date) as prev_value'

# Running total
dkit query transactions.json '.[] | select date, amount, sum(amount) over (order by date) as running_total'
```

### `convert` — Template output

```bash
# Custom text output with Tera templates (requires --features template)
dkit convert users.json -f template --template '{{ name }} <{{ email }}>'
dkit convert data.json -f template --template-file report.hbs
dkit convert data.json -f template --template '{{ name | upper }}'   # Filters: upper, lower, default
```

### `view` — Table preview

```bash
dkit view users.csv
dkit view data.json --path '.users' --limit 20
dkit view data.csv --columns name,email --border rounded --color
dkit view data.json --row-numbers --max-width 30
dkit view data.json --select 'name, email'         # Select specific columns
dkit view sales.csv --group-by status --agg 'count()'  # Aggregation table
dkit view data.json --format md                   # Output as Markdown table
dkit view data.xlsx --list-sheets                 # List Excel sheets
dkit view data.db --list-tables                   # List SQLite tables
dkit view data.csv --watch                        # Auto-refresh on change
```

### `stats` — Data statistics

```bash
dkit stats data.csv                               # Overall statistics
dkit stats data.csv --column revenue               # Per-field details
dkit stats data.csv --column age --histogram       # Text histogram
dkit stats data.json --path .users --format json   # JSON output
dkit stats data.csv --output-format json            # Structured JSON for scripting
```

### `schema` — Structure inspection

```bash
dkit schema config.yaml
dkit schema data.json
cat data.json | dkit schema - --from json
```

### `diff` — Compare data files

```bash
dkit diff old.json new.json
dkit diff config_dev.yaml config_prod.yaml         # Cross-format OK
dkit diff a.json b.json --path '.database'         # Compare nested path
dkit diff a.json b.json --mode value               # Value changes only
dkit diff a.json b.json --diff-format side-by-side # Side-by-side view
dkit diff a.json b.json --array-diff key=id        # Match arrays by key
dkit diff a.json b.json --ignore-order --ignore-case
dkit diff a.json b.json --quiet                    # Exit code only
```

### `validate` — JSON Schema validation

```bash
dkit validate data.json --schema schema.json
dkit validate data.yaml --schema schema.json       # Any format works
dkit validate data.json --schema schema.json --quiet
```

### `sample` — Data sampling

```bash
dkit sample data.csv -n 100                         # 100 random records
dkit sample data.json --ratio 0.1                   # 10% sample
dkit sample data.csv -n 50 --seed 42                # Reproducible
dkit sample data.csv -n 100 --method systematic     # Every k-th element
dkit sample data.csv -n 50 --method stratified --stratify-by category
```

### `flatten` / `unflatten`

```bash
dkit flatten data.json                              # {"a.b.c": 1}
dkit flatten data.json --separator '/'              # {"a/b/c": 1}
dkit flatten data.json --array-format bracket       # {"items[0]": ...}
dkit unflatten flat.json                            # Restore nested structure
```

### `merge` — Combine files

```bash
dkit merge a.json b.json --to json
dkit merge users1.csv users2.csv --to json -o merged.json
```

### `config` — Configuration

```bash
dkit config show                    # Show effective config with sources
dkit config init                    # Create user-level config
dkit config init --project          # Create project-level .dkit.toml
```

Config priority: CLI options > `.dkit.toml` (project) > `~/.dkit.toml` (user) > defaults.

### `alias` — Command shortcuts

```bash
dkit alias list                     # Show all aliases
dkit alias set myjson "convert --to json --pretty"
dkit alias remove myjson

# Built-in aliases
dkit j2c data.json                  # JSON → CSV
dkit c2j data.csv                   # CSV → JSON
dkit y2j config.yaml                # YAML → JSON
```

## Comparison with Other Tools

| Feature | dkit | jq | miller | yq |
|---------|:----:|:--:|:------:|:--:|
| JSON | ✓ | ✓ | ✓ | ✓ |
| CSV/TSV | ✓ | — | ✓ | — |
| YAML | ✓ | — | — | ✓ |
| TOML | ✓ | — | — | — |
| XML | ✓ | — | — | ✓ |
| MessagePack | ✓ | — | — | — |
| Parquet | ✓ | — | — | — |
| Excel (.xlsx) input | ✓ | — | — | — |
| SQLite input | ✓ | — | — | — |
| INI / .properties | ✓ | — | — | — |
| .env files | ✓ | — | — | — |
| HCL / Terraform | ✓ | — | — | — |
| plist (macOS) | ✓ | — | — | — |
| Cross-format convert | ✓ | — | Partial | Partial |
| Query (where/select/sort) | ✓ | ✓ | ✓ | ✓ |
| Aggregate / GROUP BY | ✓ | Partial | ✓ | — |
| Built-in functions | ✓ | ✓ | ✓ | — |
| Table preview | ✓ | — | ✓ | — |
| Statistics | ✓ | — | ✓ | — |
| Schema inspection | ✓ | — | — | — |
| File diff | ✓ | — | — | — |
| JSON Schema validation | ✓ | — | — | — |
| Sampling | ✓ | — | — | — |
| Flatten / unflatten | ✓ | — | — | — |
| Streaming (large files) | ✓ | — | ✓ | — |
| Log parsing | ✓ | — | — | — |
| Parallel batch conversion | ✓ | — | — | — |
| Execution timing | ✓ | — | — | — |
| Query explain plan | ✓ | — | — | — |
| Window functions | ✓ | — | — | — |
| Cross-file JOIN | ✓ | — | — | — |
| Data profiling | ✓ | — | — | — |
| Template output | ✓ | — | ✓ | — |
| Watch mode | ✓ | — | — | — |
| Config file | ✓ | — | — | — |
| Shell completions | ✓ | ✓ | ✓ | ✓ |

dkit focuses on **seamless conversion between all supported formats** with a unified query syntax, eliminating the need for separate tools per format.

## Documentation

- [Tutorial](docs/tutorial.md) — Step-by-step getting started guide
- [Cookbook](docs/cookbook.md) — Practical recipes for common tasks
- [Migration Guide](docs/migration.md) — Switching from jq / csvkit / yq
- [Query Syntax](docs/query-syntax.md) — Full query language reference
- [CLI Specification](docs/cli-spec.md) — Complete CLI reference
- [Architecture](docs/architecture.md) — Project internals
- [Technical Spec](docs/technical-spec.md) — Core types and traits
- [API Reference](docs/api-reference.md) — Library API documentation

## Building from Source

```bash
cargo build                     # Build
cargo test                      # Run tests
cargo clippy -- -D warnings     # Lint
cargo fmt -- --check            # Format check
```

Minimum supported Rust version: **1.75.0**

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[MIT](LICENSE)
