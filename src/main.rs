mod cli;
mod commands;
mod error;
mod format;
mod output;
mod query;
mod value;

use clap::Parser;

use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { .. } => {
            eprintln!("convert command is not yet implemented");
        }
        Commands::Query { .. } => {
            eprintln!("query command is not yet implemented");
        }
        Commands::View { .. } => {
            eprintln!("view command is not yet implemented");
        }
        Commands::Stats { .. } => {
            eprintln!("stats command is not yet implemented");
        }
        Commands::Schema { .. } => {
            eprintln!("schema command is not yet implemented");
        }
    }

    Ok(())
}
