# dkit

A unified CLI to convert, query, and explore data across formats.

## Features

- **Format conversion** — JSON, CSV, YAML, TOML, XML, MessagePack, Parquet, Excel, SQLite, and more
- **Query engine** — Filter, sort, aggregate, and transform with a built-in query language
- **Table preview** — View any data file as a formatted table in the terminal
- **File diff** — Compare two data files, even across different formats
- **Statistics** — Count, average, percentiles, histograms
- **Schema inspection** — Visualize data structure as a tree
- **Validation** — Validate data against JSON Schema
- **Sampling** — Random, systematic, or stratified sampling
- **Flatten / Unflatten** — Convert nested structures to flat keys and back
- **Streaming** — Chunk-based processing for large files
- **Watch mode** — Auto re-run on file changes

## Installation

```bash
# From crates.io
cargo install dkit

# With cargo-binstall
cargo binstall dkit

# From source
git clone https://github.com/syang0531/dkit.git
cd dkit && cargo install --path dkit-cli
```

Pre-built binaries are available on [GitHub Releases](https://github.com/syang0531/dkit/releases).

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

## Supported Formats

| Format      | Extensions             | Read | Write |
|-------------|------------------------|:----:|:-----:|
| JSON        | `.json`                |  ✓   |   ✓   |
| JSONL       | `.jsonl`, `.ndjson`    |  ✓   |   ✓   |
| CSV / TSV   | `.csv`, `.tsv`         |  ✓   |   ✓   |
| YAML        | `.yaml`, `.yml`        |  ✓   |   ✓   |
| TOML        | `.toml`                |  ✓   |   ✓   |
| XML         | `.xml`                 |  ✓   |   ✓   |
| MessagePack | `.msgpack`             |  ✓   |   ✓   |
| Parquet     | `.parquet`             |  ✓   |   ✓   |
| Excel       | `.xlsx`                |  ✓   |   —   |
| SQLite      | `.db`, `.sqlite`       |  ✓   |   —   |
| Markdown    | `.md`                  |  —   |   ✓   |
| HTML        | `.html`                |  —   |   ✓   |

## Documentation

See the [full README](https://github.com/syang0531/dkit) for detailed usage, query syntax, and more.

## License

[MIT](https://github.com/syang0531/dkit/blob/main/LICENSE)
