use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "dkit",
    version,
    about = "Swiss army knife for data format conversion and querying"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Convert between data formats (JSON, CSV, YAML, TOML)
    Convert {
        /// Input file path(s). Use stdin if not provided (requires --from)
        #[arg(value_name = "INPUT")]
        input: Vec<PathBuf>,

        /// Output format (json, csv, yaml, toml)
        #[arg(long)]
        to: String,

        /// Input format (required when reading from stdin)
        #[arg(long)]
        from: Option<String>,

        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output directory for multiple file conversion
        #[arg(long)]
        outdir: Option<PathBuf>,

        /// CSV delimiter character
        #[arg(long)]
        delimiter: Option<char>,

        /// Pretty-print output (default for JSON)
        #[arg(long)]
        pretty: bool,

        /// Compact output (single-line JSON)
        #[arg(long, conflicts_with = "pretty")]
        compact: bool,

        /// CSV without header row
        #[arg(long)]
        no_header: bool,

        /// YAML inline/flow style
        #[arg(long)]
        flow: bool,
    },

    /// Query data using dkit query syntax
    Query {
        /// Input file path (use - for stdin)
        input: String,

        /// Query expression
        #[arg(short, long)]
        query: String,

        /// Output format (default: same as input)
        #[arg(short, long)]
        to: Option<String>,
    },

    /// View data in a formatted table
    View {
        /// Input file path (use - for stdin)
        input: String,

        /// Input format (required when reading from stdin)
        #[arg(long)]
        from: Option<String>,

        /// Path to nested data (e.g. '.users' or '.config.db')
        #[arg(long)]
        path: Option<String>,

        /// Maximum number of rows to display
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Columns to display (comma-separated)
        #[arg(long, value_delimiter = ',')]
        columns: Option<Vec<String>>,

        /// CSV delimiter character
        #[arg(long)]
        delimiter: Option<char>,

        /// CSV without header row
        #[arg(long)]
        no_header: bool,
    },

    /// Show statistics about data
    Stats {
        /// Input file path (use - for stdin)
        input: String,
    },

    /// Show schema/structure of data
    Schema {
        /// Input file path (use - for stdin)
        input: String,
    },
}
