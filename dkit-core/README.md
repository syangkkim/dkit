# dkit-core

Core library for dkit — a data format conversion and querying engine.

Use `dkit-core` to read, write, convert, and query structured data across multiple formats programmatically.

## Features

- **Multi-format I/O** — JSON, JSONL, CSV, TSV, YAML, TOML, and optionally XML, MessagePack, Parquet, Excel, SQLite
- **Unified `Value` type** — A common intermediate representation for all formats
- **Query engine** — Filter, sort, select, aggregate, and transform data with a built-in expression language
- **Format conversion** — Convert between any supported readable and writable formats
- **Schema inference** — Inspect and describe data structure
- **JSON Schema validation** — Validate data against JSON Schema

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
dkit-core = "1.0"
```

### Optional features

Enable additional format support via feature flags:

| Feature   | Formats added          |
|-----------|------------------------|
| `xml`     | XML read/write         |
| `msgpack` | MessagePack read/write |
| `excel`   | Excel (.xlsx) read     |
| `sqlite`  | SQLite (.db) read      |
| `parquet` | Parquet read/write     |
| `all`     | All of the above       |

```toml
[dependencies]
dkit-core = { version = "1.0", features = ["xml", "parquet"] }
```

### Example

```rust
use dkit_core::formats::{JsonFormat, CsvFormat, DataFormat};
use dkit_core::value::Value;

// Read JSON
let json_data = r#"[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]"#;
let value = JsonFormat::read(json_data.as_bytes())?;

// Write as CSV
let mut output = Vec::new();
CsvFormat::write(&mut output, &value, &Default::default())?;
```

## Documentation

- [API Reference](https://github.com/syangkkim/dkit/blob/main/docs/api-reference.md)
- [Technical Spec](https://github.com/syangkkim/dkit/blob/main/docs/technical-spec.md)
- [Query Syntax](https://github.com/syangkkim/dkit/blob/main/docs/query-syntax.md)

## License

[MIT](https://github.com/syangkkim/dkit/blob/main/LICENSE)
