# Migration Guide

Switching from **jq**, **csvkit**, or **yq** to dkit? This guide shows equivalent commands side-by-side.

---

## From jq

### Read a field

```bash
# jq
jq '.database.host' config.json

# dkit
dkit query config.json '.database.host'
```

### Iterate an array

```bash
# jq
jq '.users[] | .name' data.json

# dkit
dkit query data.json '.users[].name'
```

### Filter records

```bash
# jq
jq '[.users[] | select(.age > 30)]' data.json

# dkit
dkit query data.json '.users[] | where age > 30'
```

### Select fields

```bash
# jq
jq '.users[] | {name, email}' data.json

# dkit
dkit query data.json '.users[] | select name, email'
```

### Sort and limit

```bash
# jq
jq '[.items[] | .] | sort_by(.price) | reverse | .[0:5]' data.json

# dkit
dkit query data.json '.items[] | sort price desc | limit 5'
```

### Count elements

```bash
# jq
jq '.users | length' data.json

# dkit
dkit query data.json '.users[] | count'
```

### Convert JSON to CSV

```bash
# jq (manual — requires specifying columns)
jq -r '.[] | [.name, .age] | @csv' data.json

# dkit
dkit convert data.json --to csv
```

### Pretty-print / minify

```bash
# jq
jq '.' data.json          # pretty
jq -c '.' data.json       # compact

# dkit
dkit convert data.json --to json            # pretty (default)
dkit convert data.json --to json --compact  # compact
```

### Key differences from jq

| Area | jq | dkit |
|------|-----|------|
| Filter syntax | `select(.age > 30)` | `where age > 30` |
| Field selection | `{name, email}` | `select name, email` |
| Sorting | `sort_by(.price)` | `sort price` |
| Counting | `.users \| length` | `.users[] \| count` |
| Output formats | JSON only | JSON, CSV, YAML, TOML, XML, etc. |
| Multi-format input | JSON only | 12 formats |

---

## From csvkit (csvlook, csvsql, csvstat, csvgrep)

### Preview CSV as a table

```bash
# csvkit
csvlook data.csv

# dkit
dkit view data.csv
```

### Filter rows

```bash
# csvkit
csvgrep -c status -m "active" data.csv

# dkit
dkit query data.csv '.[] | where status == "active"'
```

### Column statistics

```bash
# csvkit
csvstat data.csv

# dkit
dkit stats data.csv
dkit stats data.csv --column revenue
```

### SQL-like queries on CSV

```bash
# csvkit
csvsql --query "SELECT name, age FROM data WHERE age > 30" data.csv

# dkit
dkit query data.csv '.[] | where age > 30 | select name, age'
```

### Convert CSV to JSON

```bash
# csvkit
csvjson data.csv

# dkit
dkit convert data.csv --to json
```

### Sort CSV

```bash
# csvkit
csvsort -c price -r data.csv

# dkit
dkit query data.csv '.[] | sort price desc' --to csv
```

### Key differences from csvkit

| Area | csvkit | dkit |
|------|--------|------|
| Input formats | CSV only | 12 formats |
| Output formats | CSV / JSON | 12 formats |
| Query syntax | SQL (csvsql) | Path + pipeline |
| Aggregation | csvsql | Built-in `count`, `sum`, `avg`, GROUP BY |
| File comparison | — | `dkit diff` |
| Schema inspection | — | `dkit schema` |
| Sampling | — | `dkit sample` |

---

## From yq

### Read a YAML field

```bash
# yq
yq '.database.host' config.yaml

# dkit
dkit query config.yaml '.database.host'
```

### Convert YAML to JSON

```bash
# yq
yq -o json config.yaml

# dkit
dkit convert config.yaml --to json
```

### Convert JSON to YAML

```bash
# yq
yq -P config.json

# dkit
dkit convert config.json --to yaml
```

### Filter array elements

```bash
# yq
yq '.users[] | select(.age > 30)' data.yaml

# dkit
dkit query data.yaml '.users[] | where age > 30'
```

### Merge YAML files

```bash
# yq
yq eval-all 'select(fileIndex == 0) * select(fileIndex == 1)' a.yaml b.yaml

# dkit
dkit merge a.yaml b.yaml --to yaml
```

### Key differences from yq

| Area | yq | dkit |
|------|-----|------|
| Input formats | YAML, JSON, XML | 12 formats |
| CSV support | — | Full read/write |
| TOML support | — | Full read/write |
| Parquet / Excel / SQLite | — | Supported |
| Aggregation | — | `count`, `sum`, `avg`, GROUP BY |
| Table preview | — | `dkit view` |
| Statistics | — | `dkit stats` |
| File diff | — | `dkit diff` |
| Validation | — | `dkit validate` |
| Sampling | — | `dkit sample` |

---

## Quick Reference: Common Equivalents

| Task | jq | csvkit | yq | dkit |
|------|-----|--------|-----|------|
| Read field | `jq '.key'` | — | `yq '.key'` | `dkit query f '.key'` |
| Filter | `select(.x>1)` | `csvgrep` | `select(.x>1)` | `where x > 1` |
| Sort | `sort_by(.x)` | `csvsort` | `sort_by(.x)` | `sort x` |
| Count | `length` | `csvstat` | `length` | `count` |
| To JSON | — | `csvjson` | `-o json` | `convert --to json` |
| To CSV | `@csv` | — | — | `convert --to csv` |
| To YAML | — | — | — | `convert --to yaml` |
| Preview | — | `csvlook` | — | `view` |
| Stats | — | `csvstat` | — | `stats` |
| Diff | — | — | — | `diff` |
| Merge | — | — | `eval-all` | `merge` |

---

## Why Switch?

1. **One tool for all formats** — No need to install separate tools for JSON, CSV, YAML, TOML, XML, Parquet, etc.
2. **Unified query syntax** — Learn one query language, use it across all formats.
3. **Cross-format operations** — Convert, diff, merge between any supported formats.
4. **Rich inspection tools** — `schema`, `stats`, `validate`, `sample`, `flatten` built in.
5. **Single static binary** — Easy to install and distribute.
