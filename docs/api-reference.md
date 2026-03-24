# dkit-core Public API Reference

This document lists the public API surface of the `dkit-core` crate (v0.9.0+).

All items marked with `#[non_exhaustive]` may gain new variants or fields in future minor versions without a breaking change.

## MSRV

Minimum Supported Rust Version: **1.75.0**

---

## `value` module

### `Value` enum (`#[non_exhaustive]`)

Unified data model for all supported formats.

| Variant | Description |
|---------|-------------|
| `Null` | JSON null / missing value |
| `Bool(bool)` | Boolean |
| `Integer(i64)` | 64-bit signed integer |
| `Float(f64)` | 64-bit floating-point |
| `String(String)` | UTF-8 string |
| `Array(Vec<Value>)` | Ordered sequence |
| `Object(IndexMap<String, Value>)` | Ordered key-value map |

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `as_bool()` | `Option<bool>` | Extract boolean value |
| `as_i64()` | `Option<i64>` | Extract integer value |
| `as_f64()` | `Option<f64>` | Extract as f64 (works for Integer too) |
| `as_str()` | `Option<&str>` | Extract string slice |
| `as_array()` | `Option<&Vec<Value>>` | Extract array reference |
| `as_object()` | `Option<&IndexMap<String, Value>>` | Extract object reference |
| `is_null()` | `bool` | Check if null |

**Trait implementations:** `Debug`, `Clone`, `PartialEq`, `Serialize`, `Deserialize`, `Display`

---

## `error` module

### `DkitError` enum (`#[non_exhaustive]`)

| Variant | Description |
|---------|-------------|
| `UnknownFormat(String)` | Unrecognized format name |
| `ParseError { format, source }` | Format parsing failure |
| `ParseErrorAt { format, source, line, column, line_text }` | Parse error with location |
| `WriteError { format, source }` | Format writing failure |
| `FormatDetectionFailed(String)` | Could not auto-detect format |
| `QueryError(String)` | Invalid query or evaluation error |
| `IoError(io::Error)` | IO error (auto-converted via `From`) |
| `PathNotFound(String)` | Path navigation failed |

### `Result<T>` type alias

`type Result<T> = std::result::Result<T, DkitError>`

### Functions

| Function | Description |
|----------|-------------|
| `suggest_format(input: &str) -> Option<&'static str>` | Suggest closest format name for typo correction |

### Constants

| Constant | Description |
|----------|-------------|
| `SUPPORTED_FORMATS: &[&str]` | List of supported format name strings |

---

## `format` module

### `Format` enum (`#[non_exhaustive]`)

| Variant | Extension(s) | Direction |
|---------|-------------|-----------|
| `Json` | `.json` | Read/Write |
| `Jsonl` | `.jsonl`, `.ndjson` | Read/Write |
| `Csv` | `.csv`, `.tsv` | Read/Write |
| `Yaml` | `.yaml`, `.yml` | Read/Write |
| `Toml` | `.toml` | Read/Write |
| `Xml` | `.xml` | Read/Write |
| `Msgpack` | `.msgpack` | Read/Write |
| `Xlsx` | `.xlsx` | Read only |
| `Sqlite` | `.sqlite`, `.db` | Read only |
| `Parquet` | `.parquet` | Read/Write |
| `Markdown` | `.md` | Write only |
| `Html` | `.html` | Write only |
| `Table` | — | Write only (terminal) |

**Methods:**

| Method | Description |
|--------|-------------|
| `from_str(s) -> Result<Format, DkitError>` | Parse format name string |
| `list_output_formats() -> &[(&str, &str)]` | List all formats with descriptions |

### `FormatOptions` struct (`#[non_exhaustive]`)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `delimiter` | `Option<char>` | `None` | CSV delimiter |
| `no_header` | `bool` | `false` | CSV headerless mode |
| `pretty` | `bool` | `true` | Pretty-print output |
| `compact` | `bool` | `false` | Compact output (JSON) |
| `flow_style` | `bool` | `false` | YAML flow style |
| `root_element` | `Option<String>` | `None` | XML root element name |
| `styled` | `bool` | `false` | HTML inline CSS |
| `full_html` | `bool` | `false` | Full HTML document |

### `FormatReader` trait

| Method | Description |
|--------|-------------|
| `read(&self, input: &str) -> Result<Value>` | Parse string content |
| `read_from_reader(&self, reader: impl Read) -> Result<Value>` | Parse from reader |

### `FormatWriter` trait

| Method | Description |
|--------|-------------|
| `write(&self, value: &Value) -> Result<String>` | Serialize to string |
| `write_to_writer(&self, value: &Value, writer: impl Write) -> Result<()>` | Serialize to writer |

### Functions

| Function | Description |
|----------|-------------|
| `detect_format(path: &Path) -> Result<Format>` | Detect format from file extension |
| `detect_format_from_content(content: &str) -> Result<(Format, Option<char>)>` | Detect format from content sniffing |
| `default_delimiter(path: &Path) -> Option<char>` | Default delimiter for file extension |
| `default_delimiter_for_format(format_str: &str) -> Option<char>` | Default delimiter for format name |

### Format Readers & Writers

| Module | Reader | Writer |
|--------|--------|--------|
| `json` | `JsonReader` | `JsonWriter` |
| `jsonl` | `JsonlReader` | `JsonlWriter` |
| `csv` | `CsvReader` | `CsvWriter` |
| `yaml` | `YamlReader` | `YamlWriter` |
| `toml` | `TomlReader` | `TomlWriter` |
| `xml` | `XmlReader` | `XmlWriter` |
| `msgpack` | `MsgpackReader` | `MsgpackWriter` |
| `xlsx` | `XlsxReader` | — |
| `sqlite` | `SqliteReader` | — |
| `parquet` | `ParquetReader` | `ParquetWriter` |
| `markdown` | — | `MarkdownWriter` |
| `html` | — | `HtmlWriter` |

### Conversion Utilities

| Function | Module | Description |
|----------|--------|-------------|
| `from_json_value(v: serde_json::Value) -> Value` | `json` | Convert serde_json Value to dkit Value |
| `to_json_value(v: &Value) -> serde_json::Value` | `json` | Convert dkit Value to serde_json Value |

---

## `query` module

### Parser Functions

| Function | Description |
|----------|-------------|
| `parse_query(input: &str) -> Result<Query>` | Parse a query string into an AST |
| `parse_condition_expr(input: &str) -> Result<Condition>` | Parse a standalone condition expression |

### `Query` struct

| Field | Type | Description |
|-------|------|-------------|
| `path` | `Path` | Navigation path (`.users[0].name`) |
| `operations` | `Vec<Operation>` | Pipeline operations (`\| where ...`) |

### `Path` struct

| Field | Type | Description |
|-------|------|-------------|
| `segments` | `Vec<Segment>` | Path segments |

### `Segment` enum (`#[non_exhaustive]`)

| Variant | Description |
|---------|-------------|
| `Field(String)` | Field access (`.name`) |
| `Index(i64)` | Array index (`[0]`, `[-1]`) |
| `Iterate` | Array iteration (`[]`) |

### `Operation` enum (`#[non_exhaustive]`)

`Where`, `Select`, `Sort`, `Limit`, `Count`, `Sum`, `Avg`, `Min`, `Max`, `Distinct`, `GroupBy`

### `Condition` enum (`#[non_exhaustive]`)

`Comparison`, `And`, `Or`

### `CompareOp` enum (`#[non_exhaustive]`)

`Eq` (`==`), `Ne` (`!=`), `Gt` (`>`), `Lt` (`<`), `Ge` (`>=`), `Le` (`<=`), `Contains`, `StartsWith`, `EndsWith`

### `Expr` enum (`#[non_exhaustive]`)

`Field`, `Literal`, `FuncCall`

### `LiteralValue` enum (`#[non_exhaustive]`)

`String`, `Integer`, `Float`, `Bool`, `Null`

### `AggregateFunc` enum (`#[non_exhaustive]`)

`Count`, `Sum`, `Avg`, `Min`, `Max`

### Evaluator & Filter Functions

| Function | Description |
|----------|-------------|
| `evaluate_path(value: &Value, path: &Path) -> Result<Value>` | Navigate a Value using a parsed path |
| `apply_operations(value: Value, ops: &[Operation]) -> Result<Value>` | Apply pipeline operations to a Value |
| `evaluate_expr(row: &Value, expr: &Expr) -> Result<Value>` | Evaluate an expression against a record |
| `expr_default_key(expr: &Expr) -> String` | Generate a default output key for an expression |

---

## Deprecation Policy

- Public API items will be marked `#[deprecated]` for at least one minor version before removal.
- Deprecated items will include a message describing the replacement.
- Removals happen only in the next major version (`1.0.0` → `2.0.0`).
- `#[non_exhaustive]` enums and structs may gain new variants/fields in minor versions.
