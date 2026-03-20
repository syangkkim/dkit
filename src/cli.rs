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
        /// Input file path (use - for stdin)
        input: String,

        /// Output format
        #[arg(short, long)]
        to: String,

        /// Output file path (default: stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Pretty-print output
        #[arg(long)]
        pretty: bool,

        /// Compact output
        #[arg(long)]
        compact: bool,
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

        /// Maximum number of rows to display
        #[arg(short = 'n', long)]
        max_rows: Option<usize>,
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
