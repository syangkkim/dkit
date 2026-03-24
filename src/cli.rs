use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    },

    /// Show statistics about data
    #[command(
        after_help = "Examples:\n  dkit stats data.csv\n  dkit stats data.json --path .users\n  dkit stats data.csv --column revenue"
    )]
    Stats {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Input format (required for stdin)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

        /// Output format (default: table). Supports: json, jsonl, csv, yaml, toml, xml, md, html, table
        #[arg(short = 'f', long, value_name = "FORMAT")]
        format: Option<String>,

        /// Navigate to nested data path (e.g. '.users')
        #[arg(long, value_name = "QUERY")]
        path: Option<String>,

        /// Get statistics for a specific column
        #[arg(long, value_name = "NAME")]
        column: Option<String>,

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

    /// Compare two data files and show differences
    #[command(
        after_help = "Examples:\n  dkit diff old.json new.json\n  dkit diff config_dev.yaml config_prod.yaml\n  dkit diff data.json data.yaml\n  dkit diff old.json new.json --path '.database'\n  dkit diff a.json b.json --quiet && echo 'same' || echo 'different'"
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
