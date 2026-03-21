use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "dkit",
    version,
    about = "Swiss army knife for data format conversion and querying",
    long_about = "dkit (Data Kit) — Convert and query data across JSON, CSV, YAML, TOML, and XML.\n\nExamples:\n  dkit convert data.json --to csv\n  dkit convert data.csv --to yaml -o output.yaml\n  dkit query data.json '.users[0].name'\n  dkit view data.csv --limit 10\n  cat data.json | dkit convert --from json --to toml",
    after_help = "Supported formats: json, csv, yaml (yml), toml, xml\nUse 'dkit <command> --help' for more information about a command."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert between data formats (JSON, CSV, YAML, TOML)
    #[command(
        after_help = "Examples:\n  dkit convert data.json --to csv\n  dkit convert data.csv --to yaml --pretty\n  dkit convert a.json b.json --to csv --outdir ./output\n  cat data.json | dkit convert --from json --to toml"
    )]
    Convert {
        /// Input file path(s). Use stdin if not provided (requires --from)
        #[arg(value_name = "INPUT")]
        input: Vec<PathBuf>,

        /// Output format (json, csv, yaml, toml, xml)
        #[arg(long, value_name = "FORMAT")]
        to: String,

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

        /// Output format (default: json)
        #[arg(long, value_name = "FORMAT")]
        to: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// View data in a formatted table
    #[command(
        after_help = "Examples:\n  dkit view data.csv\n  dkit view data.json --path .users --limit 5\n  dkit view data.json --columns name,email"
    )]
    View {
        /// Input file path (use '-' for stdin)
        #[arg(value_name = "INPUT")]
        input: String,

        /// Input format (required for stdin)
        #[arg(long, value_name = "FORMAT")]
        from: Option<String>,

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
    },

    /// Merge multiple data files into one
    #[command(
        after_help = "Examples:\n  dkit merge a.json b.json --to json\n  dkit merge users1.csv users2.csv --to json -o merged.json\n  dkit merge config1.yaml config2.yaml --to yaml"
    )]
    Merge {
        /// Input file paths (at least 2 required)
        #[arg(value_name = "INPUT", required = true)]
        input: Vec<PathBuf>,

        /// Output format (json, csv, yaml, toml, xml). Defaults to first input's format
        #[arg(long, value_name = "FORMAT")]
        to: Option<String>,

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
    },
}
