# dkit Cookbook

Practical recipes for common data tasks.

---

## Format Conversion

### Convert an entire directory of CSV files to JSON

```bash
dkit convert *.csv --to json --outdir ./json/
```

### Minify a JSON file

```bash
dkit convert data.json --to json --compact -o data.min.json
```

### Pretty-print a minified JSON file

```bash
cat data.min.json | dkit convert --from json --to json
```

### Convert JSON array to one-record-per-line JSONL

```bash
dkit convert users.json --to jsonl -o users.jsonl
```

### Convert CSV with non-UTF-8 encoding

```bash
# Specify encoding explicitly
dkit convert data_kr.csv --to json --encoding euc-kr

# Auto-detect encoding
dkit convert data.csv --to json --detect-encoding
```

### Convert CSV without a header row

```bash
dkit convert data.csv --to json --no-header
```

### Export data as a styled HTML report

```bash
dkit convert report.json --to html --styled --full-html -o report.html
```

### Export data as a Markdown table

```bash
dkit convert data.csv --to md
```

---

## Querying

### Extract a single value from a config file

```bash
dkit query config.yaml '.database.host'
```

### List all email addresses

```bash
dkit query users.json '.users[].email'
```

### Filter records by condition

```bash
dkit query orders.csv '.[] | where status == "shipped" | where total > 100'
```

### Find records matching a string pattern

```bash
dkit query data.json '.[] | where name starts_with "A"'
dkit query data.json '.[] | where description contains "urgent"'
```

### Select and rename fields

```bash
dkit query data.json '.[] | select upper(name) as NAME, round(price, 2) as PRICE'
```

### Top N by value

```bash
dkit query products.csv '.[] | sort price desc | limit 10'
```

### Get the last element of an array

```bash
dkit query data.json '.items[-1]'
```

### Count records matching a condition

```bash
dkit query data.csv '.[] | where status == "active" | count'
```

---

## Aggregation

### Sum, average, min, max

```bash
dkit query sales.csv '.[] | sum revenue'
dkit query sales.csv '.[] | avg revenue'
dkit query sales.csv '.[] | min price'
dkit query sales.csv '.[] | max price'
```

### Group by and aggregate

```bash
# Count per category
dkit query products.csv '.[] | group_by category count()'

# Revenue per region
dkit query sales.csv '.[] | group_by region sum(revenue), avg(revenue)'

# Filter groups with HAVING
dkit query data.csv '.[] | group_by department count() having count > 5'

# Top 3 categories by count
dkit query data.csv '.[] | group_by category count() | sort count desc | limit 3'
```

### Get distinct values

```bash
dkit query data.csv '.[] | distinct category'
```

---

## Data Inspection

### Quick preview of a large CSV

```bash
dkit view large.csv --limit 20 --border rounded --color
```

### Check the structure of a JSON file

```bash
dkit schema complex_data.json
```

### Get statistics for a numeric column

```bash
dkit stats sales.csv --column revenue
# Shows: count, sum, avg, min, max, median, std, p25, p75
```

### Visualize a distribution

```bash
dkit stats data.csv --column age --histogram
```

### Compare two config files

```bash
dkit diff config_dev.yaml config_prod.yaml --diff-format side-by-side
```

### Check only if files differ (for scripts)

```bash
dkit diff a.json b.json --quiet && echo "identical" || echo "different"
```

---

## Data Manipulation

### Merge multiple files

```bash
# Concatenate CSV files
dkit merge jan.csv feb.csv mar.csv --to csv -o q1.csv

# Merge JSON objects
dkit merge defaults.json overrides.json --to json
```

### Flatten nested JSON

```bash
dkit flatten config.json
# {"database.host": "localhost", "database.port": 5432}

# Use / as separator
dkit flatten config.json --separator '/'
```

### Restore flattened data

```bash
dkit unflatten flat.json
```

### Sample data for testing

```bash
# 100 random records
dkit sample data.csv -n 100

# 10% sample, reproducible
dkit sample data.csv --ratio 0.1 --seed 42

# Stratified sample by category
dkit sample data.csv -n 200 --method stratified --stratify-by region
```

### Validate data against a schema

```bash
dkit validate data.json --schema schema.json
```

---

## Piping and Integration

### Chain dkit commands

```bash
# Convert, then query
cat data.csv | dkit convert --from csv --to json | dkit query '.[] | where age > 30'
```

### Query and convert output

```bash
dkit query data.json '.users[] | where active == true | select name, email' --to csv -o active_users.csv
```

### Use with jq for post-processing

```bash
dkit convert data.yaml --to json | jq '.[] | .name'
```

### Use with other Unix tools

```bash
# Count unique categories
dkit query data.csv '.[] | distinct category' | dkit convert --from json --to jsonl | wc -l

# Sort emails alphabetically
dkit query users.json '.users[].email' | dkit convert --from json --to jsonl | sort
```

---

## Special Formats

### Excel: list sheets and convert

```bash
dkit view report.xlsx --list-sheets
dkit convert report.xlsx --to csv --sheet "Q1 Sales" -o q1.csv
```

### SQLite: explore and export

```bash
dkit view app.db --list-tables
dkit view app.db --table users --limit 10
dkit convert app.db --to json --sql "SELECT name, email FROM users WHERE active = 1"
```

### Parquet: read and write with compression

```bash
# Read
dkit view data.parquet --limit 5

# Write with Zstd compression
dkit convert data.csv --to parquet --compression zstd -o data.parquet

# Convert Parquet to CSV
dkit convert data.parquet --to csv -o data.csv
```

### Streaming large files

```bash
# Process in chunks of 1000 records
dkit convert large.jsonl --from jsonl -f csv --chunk-size 1000 -o out.csv
```

---

## Watch Mode

### Auto-convert on file change

```bash
dkit convert data.json --to csv --watch
```

### Live table preview

```bash
dkit view data.csv --watch
```

### Watch an additional directory

```bash
dkit convert data.json --to yaml --watch --watch-path ./templates/
```

---

## Configuration Tips

### Set default table style project-wide

Create `.dkit.toml` in your project root:

```toml
default_format = "json"
color = true

[table]
border_style = "rounded"
max_width = 100
```

### Create useful aliases

```bash
dkit alias set pretty "convert --to json --pretty"
dkit alias set preview "view --border rounded --color --limit 20"
dkit alias set topn "query '.[] | sort price desc | limit 10'"
```
