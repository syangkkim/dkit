# dkit Tutorial

A step-by-step guide to getting started with dkit.

## 1. Installation

```bash
cargo install dkit
```

Verify the installation:

```bash
dkit --version
```

## 2. Your First Conversion

Create a sample JSON file:

```bash
echo '[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]' > users.json
```

Convert it to different formats:

```bash
# JSON → CSV
dkit convert users.json --to csv
# Output:
# name,age
# Alice,30
# Bob,25

# JSON → YAML
dkit convert users.json --to yaml

# JSON → TOML (save to file)
dkit convert users.json --to toml -o users.toml
```

dkit auto-detects the input format from the file extension. To convert back:

```bash
dkit convert users.toml --to json
```

## 3. Viewing Data as a Table

Preview any data file as a formatted table:

```bash
dkit view users.json
```

Customize the table output:

```bash
# Rounded borders with colored types
dkit view users.json --border rounded --color

# Show only specific columns
dkit view users.json --columns name

# Add row numbers and limit rows
dkit view users.json --row-numbers --limit 10
```

## 4. Querying Data

### Field Access

Access fields using dot notation:

```bash
# Single field
dkit query users.json '.[0].name'
# Output: "Alice"

# All names
dkit query users.json '.[].name'
# Output: ["Alice", "Bob"]
```

### Filtering

Use `where` to filter records:

```bash
dkit query users.json '.[] | where age > 25'
# Output: [{"name": "Alice", "age": 30}]
```

### Selecting Fields

Pick only the fields you need:

```bash
dkit query users.json '.[] | select name'
```

### Sorting and Limiting

```bash
dkit query users.json '.[] | sort age desc | limit 1'
```

### Chaining Operations

Combine operations with the pipe (`|`) operator:

```bash
dkit query users.json '.[] | where age > 20 | sort name | select name, age'
```

## 5. Aggregation

Compute summaries over your data:

```bash
# Count records
dkit query users.json '.[] | count'

# Average age
dkit query users.json '.[] | avg age'

# Group by and aggregate
dkit query data.csv '.[] | group_by department count(), avg(salary)'
```

## 6. Using Built-in Functions

Transform data with functions in `select`:

```bash
# String functions
dkit query users.json '.[] | select upper(name), age'

# Math functions
dkit query prices.csv '.[] | select name, round(price, 2)'

# Coalesce nulls
dkit query data.json '.[] | select name, coalesce(email, "N/A")'
```

## 7. Inspecting Data

### Schema

See the structure of your data:

```bash
dkit schema config.yaml
# Output (tree format):
# (object)
# ├─ database (object)
# │  ├─ host (string)
# │  ├─ port (integer)
# │  └─ name (string)
# └─ server (object)
#    └─ port (integer)
```

### Statistics

Get statistical summaries:

```bash
# Overall stats
dkit stats data.csv

# Detailed stats for a specific column
dkit stats data.csv --column revenue

# Histogram
dkit stats data.csv --column age --histogram
```

## 8. Comparing Files

Compare two data files, even in different formats:

```bash
dkit diff config_dev.yaml config_prod.yaml
```

Focus on a specific section:

```bash
dkit diff old.json new.json --path '.database'
```

Use different output formats:

```bash
dkit diff a.json b.json --diff-format side-by-side
dkit diff a.json b.json --diff-format json -o diff_result.json
```

## 9. Working with Special Formats

### Excel

```bash
# List sheets
dkit view data.xlsx --list-sheets

# Convert a specific sheet
dkit convert data.xlsx --to json --sheet "Sales"
```

### SQLite

```bash
# List tables
dkit view data.db --list-tables

# Convert with custom SQL
dkit convert data.db --to csv --sql "SELECT name, age FROM users WHERE age > 30"
```

### Parquet

```bash
# Read Parquet
dkit view data.parquet --limit 10

# Write with compression
dkit convert data.csv --to parquet --compression zstd -o data.parquet
```

## 10. Piping and Batch Processing

### Unix Pipes

dkit reads from stdin and writes to stdout:

```bash
# Pipe between commands
cat data.json | dkit query '.users[] | where active == true' | dkit convert --from json --to csv

# Combine with other Unix tools
dkit query data.json '.users[].email' | sort | uniq
```

### Batch Conversion

Convert multiple files at once:

```bash
dkit convert *.csv --to json --outdir ./json_files/
```

## 11. Configuration

Set defaults to avoid repeating options:

```bash
# Create a user-level config
dkit config init

# Create a project-level config
dkit config init --project

# View effective configuration
dkit config show
```

Example `.dkit.toml`:

```toml
default_format = "json"
color = true

[table]
border_style = "rounded"
max_width = 120
```

## 12. Command Aliases

Create shortcuts for frequently used commands:

```bash
# Define an alias
dkit alias set pretty "convert --to json --pretty"

# Use it
dkit pretty data.csv

# List all aliases
dkit alias list

# Built-in aliases: j2c, c2j, j2y, y2j, j2t, t2j, c2y, y2c
dkit j2c data.json    # JSON → CSV
```

## Next Steps

- [Cookbook](cookbook.md) — Practical recipes for real-world tasks
- [Migration Guide](migration.md) — Coming from jq, csvkit, or yq?
- [Query Syntax Reference](query-syntax.md) — Full query language documentation
- [CLI Specification](cli-spec.md) — Complete option reference for every command
