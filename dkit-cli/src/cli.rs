use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    name = "dkit",
    version,
    about = "Swiss army knife for data format conversion and querying",
    long_about = "dkit (Data Kit) — Convert and query data across JSON, CSV, YAML, TOML, and XML.\n\nExamples:\n  dkit convert data.json --format csv\n  dkit convert data.csv --format yaml -o output.yaml\n  dkit query data.json '.users[0].name'\n  dkit view data.csv --limit 10\n  cat data.json | dkit convert --from json --format toml",
    after_help = "Supported formats: json, jsonl, csv, yaml (yml), toml, xml, md, html, table\nUse 'dkit <command> --help' for more information about a command.\nUse 'dkit --list-formats' to see all supported output formats."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// List all supported output formats
    #[arg(long)]
    pub list_formats: bool,

    /// Show frequently used examples
    #[arg(long)]
    pub examples: bool,

    /// Show verbose error output (error chain, debug info)
    #[arg(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert between data formats (JSON, CSV, YAML, TOML)
    #[command(
        after_help = "Examples:\n  dkit convert data.json --format csv\n  dkit convert data.csv --format yaml --pretty\n  dkit convert a.json b.json --format csv --outdir ./output\n  dkit convert '*.json' --format csv --outdir ./output\n  dkit convert data_dir/ --format yaml --outdir ./output\n  dkit convert '*.json' -f csv --outdir out --rename '{name}.converted.{ext}'\n  dkit convert dir/ -f csv --outdir out --continue-on-error\n  cat data.json | dkit convert --from json --format toml\n  dkit convert - --from json --format csv < data.json\n  dkit convert data.json -f csv | dkit query - '.items[]' --from csv"
    )]
    Convert {
        /// Input file path(s). Use '-' or omit for stdin (auto-detects format or use --from)
        #[arg(value_name = "INPUT")]
        input: Vec<PathBuf>,

        /// Output format (json, jsonl, csv, yaml, toml, xml, md, html, table)
        #[arg(short = 'f', long, alias = "to", value_name = "FORMAT")]
        format: String,

        /// Input format override (auto-detected from file extension)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Output directory for batch conversion
        #[arg(long, value_name = "DIR")]
        outdir: Option<PathBuf>,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Pretty-print output
        #[arg(long)]
        pretty: bool,

        /// Compact single-line output (JSON)
        #[arg(long, conflicts_with = "pretty")]
        compact: bool,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Use YAML inline/flow style
        #[arg(long)]
        flow: bool,

        /// XML root element name (default: "root")
        #[arg(long, value_name = "NAME")]
        root_element: Option<String>,

        /// Include inline CSS styles (HTML output)
        #[arg(long)]
        styled: bool,

        /// Output a complete HTML document (HTML output)
        #[arg(long)]
        full_html: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,

        /// Output filename pattern for batch conversion (e.g. '{name}.converted.{ext}')
        #[arg(long, value_name = "PATTERN")]
        rename: Option<String>,

        /// Continue processing remaining files when an error occurs
        #[arg(long)]
        continue_on_error: bool,

        /// Sort by field name
        #[arg(long, value_name = "FIELD")]
        sort_by: Option<String>,

        /// Sort order (asc or desc, default: asc)
        #[arg(long, value_name = "ORDER", default_value = "asc")]
        sort_order: String,

        /// Show only the first N records
        #[arg(long, value_name = "N")]
        head: Option<usize>,

        /// Show only the last N records
        #[arg(long, value_name = "N")]
        tail: Option<usize>,

        /// Filter expression (e.g. 'age > 30 and city == "Seoul"')
        #[arg(long, value_name = "EXPR", alias = "where")]
        filter: Option<String>,

        /// Select specific fields (comma-separated, e.g. 'name, city, age')
        #[arg(long, value_name = "FIELDS")]
        select: Option<String>,

        /// Group by field(s) for aggregation (comma-separated, e.g. 'category' or 'category, region')
        #[arg(long, value_name = "FIELDS")]
        group_by: Option<String>,

        /// Aggregation functions (e.g. 'count(), sum(amount), avg(price)')
        #[arg(long, value_name = "EXPR")]
        agg: Option<String>,

        /// Remove duplicate records (based on entire record equality)
        #[arg(long)]
        unique: bool,

        /// Remove duplicate records based on a specific field (keeps first occurrence)
        #[arg(long, value_name = "FIELD")]
        unique_by: Option<String>,

        /// Parquet compression codec (none, snappy, gzip, zstd)
        #[arg(long, value_name = "CODEC", default_value = "none")]
        compression: String,

        /// Parquet row group size (number of rows per row group)
        #[arg(long, value_name = "N")]
        row_group_size: Option<usize>,

        /// Enable streaming mode for large files (chunk-based read/write)
        #[arg(long, value_name = "N", value_parser = clap::value_parser!(usize))]
        chunk_size: Option<usize>,

        /// Show progress during streaming conversion
        #[arg(long)]
        progress: bool,

        /// Watch input file(s) for changes and auto re-run
        #[arg(long)]
        watch: bool,

        /// Additional paths to watch for changes
        #[arg(long = "watch-path", value_name = "PATH")]
        watch_paths: Vec<std::path::PathBuf>,

        /// Preview conversion result without writing output file (prints first N records to stdout)
        #[arg(long)]
        dry_run: bool,

        /// Number of records to show in dry-run preview (default: 10)
        #[arg(long, value_name = "N", default_value = "10")]
        dry_run_limit: usize,
    },

    /// Query data using path expressions
    #[command(
        after_help = "Examples:\n  dkit query data.json '.users[0].name'\n  dkit query data.yaml '.config.database.host'\n  cat data.json | dkit query - '.items[]' --from json"
    )]
    Query {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Query expression (e.g. '.users[0].name')
        #[arg(value_name = "EXPR")]
        query: String,

        /// Input format (required for stdin)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (default: json). Supports: json, jsonl, csv, yaml, toml, xml, md, html, table
        #[arg(short = 'f', long, alias = "to", value_name = "FORMAT")]
        format: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// View data in a formatted table
    #[command(
        after_help = "Examples:\n  dkit view data.csv\n  dkit view data.json --path .users --limit 5\n  dkit view data.json --columns name,email\n  dkit view data.csv --border rounded --color\n  dkit view data.json --format json\n  dkit view data.json --row-numbers --max-width 30"
    )]
    View {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Input format (required for stdin)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (default: table). Supports: json, jsonl, csv, yaml, toml, xml, md, html, table
        #[arg(short = 'f', long, value_name = "FORMAT")]
        format: Option<String>,

        /// Path to nested data (e.g. '.users' or '.config.db')
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Maximum number of rows to display
        #[arg(short = 'n', long, value_name = "N")]
        limit: Option<usize>,

        /// Columns to display (comma-separated)
        #[arg(long, value_delimiter = ',', value_name = "COLS")]
        columns: Option<Vec<String>>,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Maximum column width (truncate longer values)
        #[arg(long, value_name = "N")]
        max_width: Option<u16>,

        /// Hide header row in table output
        #[arg(long)]
        hide_header: bool,

        /// Show row numbers
        #[arg(long)]
        row_numbers: bool,

        /// Table border style
        #[arg(long, value_name = "STYLE", default_value = "simple")]
        border: String,

        /// Colorize output by data type (numbers=blue, null=gray)
        #[arg(long)]
        color: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// List sheet names in an Excel file
        #[arg(long)]
        list_sheets: bool,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,

        /// List table names in a SQLite database
        #[arg(long)]
        list_tables: bool,

        /// Sort by field name
        #[arg(long, value_name = "FIELD")]
        sort_by: Option<String>,

        /// Sort order (asc or desc, default: asc)
        #[arg(long, value_name = "ORDER", default_value = "asc")]
        sort_order: String,

        /// Show only the first N records
        #[arg(long, value_name = "N")]
        head: Option<usize>,

        /// Show only the last N records
        #[arg(long, value_name = "N")]
        tail: Option<usize>,

        /// Filter expression (e.g. 'age > 30 and city == "Seoul"')
        #[arg(long, value_name = "EXPR", alias = "where")]
        filter: Option<String>,

        /// Select specific fields (comma-separated, e.g. 'name, city, age')
        #[arg(long, value_name = "FIELDS")]
        select: Option<String>,

        /// Group by field(s) for aggregation (comma-separated, e.g. 'category' or 'category, region')
        #[arg(long, value_name = "FIELDS")]
        group_by: Option<String>,

        /// Aggregation functions (e.g. 'count(), sum(amount), avg(price)')
        #[arg(long, value_name = "EXPR")]
        agg: Option<String>,

        /// Remove duplicate records (based on entire record equality)
        #[arg(long)]
        unique: bool,

        /// Remove duplicate records based on a specific field (keeps first occurrence)
        #[arg(long, value_name = "FIELD")]
        unique_by: Option<String>,

        /// Watch input file for changes and auto re-run
        #[arg(long)]
        watch: bool,

        /// Additional paths to watch for changes
        #[arg(long = "watch-path", value_name = "PATH")]
        watch_paths: Vec<std::path::PathBuf>,
    },

    /// Show statistics about data
    #[command(
        after_help = "Examples:\n  dkit stats data.csv\n  dkit stats data.json --path .users\n  dkit stats data.csv --column revenue\n  dkit stats data.csv --field revenue --format json\n  dkit stats data.csv --histogram"
    )]
    Stats {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Input format (required for stdin)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (json, yaml, table, md)
        #[arg(
            short = 'O',
            long = "output-format",
            alias = "format",
            short_alias = 'f',
            value_name = "FORMAT"
        )]
        output_format: Option<String>,

        /// Navigate to nested data path (e.g. '.users')
        #[arg(long, value_name = "QUERY")]
        path: Option<String>,

        /// Get statistics for a specific column
        #[arg(long, value_name = "NAME")]
        column: Option<String>,

        /// Get detailed statistics for a specific field (alias for --column)
        #[arg(long, value_name = "NAME")]
        field: Option<String>,

        /// Show text histogram for numeric fields
        #[arg(long)]
        histogram: bool,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Merge multiple data files into one
    #[command(
        after_help = "Examples:\n  dkit merge a.json b.json --format json\n  dkit merge users1.csv users2.csv --format json -o merged.json\n  dkit merge config1.yaml config2.yaml --format yaml"
    )]
    Merge {
        /// Input file paths (at least 2 required)
        #[arg(value_name = "INPUT", required = true)]
        input: Vec<PathBuf>,

        /// Output format (json, jsonl, csv, yaml, toml, xml, md, html). Defaults to first input's format
        #[arg(short = 'f', long, alias = "to", value_name = "FORMAT")]
        format: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Pretty-print output
        #[arg(long)]
        pretty: bool,

        /// Compact single-line output (JSON)
        #[arg(long, conflicts_with = "pretty")]
        compact: bool,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Use YAML inline/flow style
        #[arg(long)]
        flow: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Show schema/structure of data
    #[command(
        after_help = "Examples:\n  dkit schema config.yaml\n  dkit schema data.json\n  cat data.json | dkit schema - --from json"
    )]
    Schema {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Input format (required for stdin)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (json, yaml, table)
        #[arg(short = 'O', long = "output-format", value_name = "FORMAT")]
        output_format: Option<String>,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Validate data against a JSON Schema
    #[command(
        after_help = "Examples:\n  dkit validate data.json --schema schema.json\n  dkit validate data.yaml --schema schema.json\n  dkit validate data.csv --schema schema.json --from csv\n  dkit validate data.json --schema schema.json --quiet && echo 'valid' || echo 'invalid'"
    )]
    Validate {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// JSON Schema file path
        #[arg(long, value_name = "FILE")]
        schema: PathBuf,

        /// Input format (auto-detected from file extension)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Suppress detailed error output (print only valid/invalid summary)
        #[arg(short, long)]
        quiet: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Sample records from data
    #[command(
        after_help = "Examples:\n  dkit sample data.csv -n 100\n  dkit sample data.json --ratio 0.1\n  dkit sample data.csv -n 50 --seed 42\n  dkit sample data.csv -n 100 --method systematic\n  dkit sample data.csv -n 50 --method stratified --stratify-by category\n  dkit sample data.csv -n 100 -o sample.json -f json"
    )]
    Sample {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Number of records to sample
        #[arg(short = 'n', long, value_name = "N")]
        count: Option<usize>,

        /// Ratio of records to sample (0.0 to 1.0)
        #[arg(long, value_name = "RATIO")]
        ratio: Option<f64>,

        /// Random seed for reproducible sampling
        #[arg(long, value_name = "SEED")]
        seed: Option<u64>,

        /// Sampling method: random, systematic, stratified
        #[arg(long, value_name = "METHOD", default_value = "random")]
        method: String,

        /// Field to stratify by (required for stratified sampling)
        #[arg(long, value_name = "FIELD")]
        stratify_by: Option<String>,

        /// Input format (auto-detected from file extension)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (default: same as input)
        #[arg(short = 'f', long, alias = "to", value_name = "FORMAT")]
        format: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Pretty-print output
        #[arg(long)]
        pretty: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Flatten nested structures into dot-notation keys
    #[command(
        after_help = "Examples:\n  dkit flatten data.json\n  dkit flatten data.json --separator '/'\n  dkit flatten data.json --array-format bracket\n  dkit flatten data.json --max-depth 2\n  dkit flatten data.json -f yaml -o flat.yaml\n  cat data.json | dkit flatten - --from json"
    )]
    Flatten {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Key separator (default: '.')
        #[arg(long, value_name = "SEP", default_value = ".")]
        separator: String,

        /// Array flattening format: index (items.0.name) or bracket (items[0].name)
        #[arg(long, value_name = "FORMAT", default_value = "index")]
        array_format: String,

        /// Maximum depth to flatten (default: unlimited)
        #[arg(long, value_name = "N")]
        max_depth: Option<usize>,

        /// Input format (auto-detected from file extension)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (default: same as input)
        #[arg(short = 'f', long, alias = "to", value_name = "FORMAT")]
        format: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Pretty-print output
        #[arg(long)]
        pretty: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Unflatten dot-notation keys back into nested structures
    #[command(
        after_help = "Examples:\n  dkit unflatten flat.json\n  dkit unflatten flat.json --separator '/'\n  dkit unflatten flat.json -f yaml -o nested.yaml\n  cat flat.json | dkit unflatten - --from json"
    )]
    Unflatten {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Key separator (default: '.')
        #[arg(long, value_name = "SEP", default_value = ".")]
        separator: String,

        /// Input format (auto-detected from file extension)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (default: same as input)
        #[arg(short = 'f', long, alias = "to", value_name = "FORMAT")]
        format: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// CSV delimiter character (default: ',')
        #[arg(long, value_name = "CHAR")]
        delimiter: Option<char>,

        /// Treat CSV as having no header row
        #[arg(long)]
        no_header: bool,

        /// Pretty-print output
        #[arg(long)]
        pretty: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },

    /// Manage dkit configuration
    #[command(
        after_help = "Examples:\n  dkit config show              # Show current effective configuration\n  dkit config init              # Create user config at XDG/home location\n  dkit config init --project    # Create project config (.dkit.toml)"
    )]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Manage command aliases
    #[command(
        after_help = "Examples:\n  dkit alias list                              # List all aliases\n  dkit alias set j2c 'convert --from json --to csv'  # Set an alias\n  dkit alias set peek 'view --head 5'         # Set an alias\n  dkit alias remove myalias                   # Remove a user alias\n  dkit j2c data.json                          # Use an alias"
    )]
    Alias {
        #[command(subcommand)]
        action: AliasAction,
    },

    /// Generate shell completion scripts
    #[command(
        after_help = "Examples:\n  dkit completions bash > ~/.bash_completion.d/dkit\n  dkit completions zsh > ~/.zfunc/_dkit\n  dkit completions fish > ~/.config/fish/completions/dkit.fish\n  dkit completions powershell > dkit.ps1\n\nInstallation:\n  Bash:       dkit completions bash > ~/.bash_completion.d/dkit && source ~/.bash_completion.d/dkit\n  Zsh:        dkit completions zsh > ~/.zfunc/_dkit  (ensure ~/.zfunc is in $fpath)\n  Fish:       dkit completions fish > ~/.config/fish/completions/dkit.fish\n  PowerShell: dkit completions powershell > dkit.ps1 && . ./dkit.ps1"
    )]
    Completions {
        /// Target shell (bash, zsh, fish, powershell)
        #[arg(value_name = "SHELL")]
        shell: Shell,
    },

    /// Compare two data files and show differences
    #[command(
        after_help = "Examples:\n  dkit diff old.json new.json\n  dkit diff config_dev.yaml config_prod.yaml\n  dkit diff data.json data.yaml\n  dkit diff old.json new.json --path '.database'\n  dkit diff a.json b.json --quiet && echo 'same' || echo 'different'\n  dkit diff a.json b.json --mode value --diff-format json\n  dkit diff a.json b.json --array-diff key=id --ignore-order"
    )]
    Diff {
        /// First input file
        #[arg(value_name = "FILE1")]
        file1: PathBuf,

        /// Second input file
        #[arg(value_name = "FILE2")]
        file2: PathBuf,

        /// Compare only a nested data path (e.g. '.database')
        #[arg(long, value_name = "QUERY")]
        path: Option<String>,

        /// Only report whether files differ (exit code: 0=same, 1=different)
        #[arg(long)]
        quiet: bool,

        /// Comparison mode: structural (added/removed/changed), value (value changes only), key (key existence only)
        #[arg(long, value_name = "MODE", default_value = "structural")]
        mode: String,

        /// Diff output format: unified, side-by-side, json, summary
        #[arg(long, value_name = "FORMAT", default_value = "unified")]
        diff_format: String,

        /// Array comparison strategy: index (by position), value (by value), key=<field> (by key field)
        #[arg(long, value_name = "STRATEGY", default_value = "index")]
        array_diff: String,

        /// Ignore array element order when comparing
        #[arg(long)]
        ignore_order: bool,

        /// Ignore string case when comparing
        #[arg(long)]
        ignore_case: bool,

        /// Input file encoding (e.g. euc-kr, shift_jis, latin1)
        #[arg(long, value_name = "ENCODING")]
        encoding: Option<String>,

        /// Auto-detect input file encoding
        #[arg(long)]
        detect_encoding: bool,

        /// Excel sheet name or index (default: first sheet)
        #[arg(long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Excel header row number, 1-based (default: 1)
        #[arg(long, value_name = "N")]
        header_row: Option<usize>,

        /// SQLite table name to read from
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// SQL query to execute on SQLite database
        #[arg(long, value_name = "SQL")]
        sql: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current effective configuration
    Show,
    /// Create a default configuration file
    Init {
        /// Create project-level config (.dkit.toml) instead of user-level
        #[arg(long)]
        project: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum AliasAction {
    /// Register or update a command alias
    Set(AliasSetArgs),
    /// List all aliases (builtin and user-defined)
    List,
    /// Remove a user-defined alias
    Remove {
        /// Alias name to remove
        #[arg(value_name = "NAME")]
        name: String,
    },
}

#[derive(Args, Debug)]
pub struct AliasSetArgs {
    /// Alias name (e.g. j2c)
    #[arg(value_name = "NAME")]
    pub name: String,
    /// Command to expand to (e.g. 'convert --from json --to csv')
    #[arg(value_name = "COMMAND")]
    pub command: String,
}
