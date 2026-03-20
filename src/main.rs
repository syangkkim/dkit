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
        Commands::Convert {
            input,
            to,
            from,
            output,
            outdir,
            delimiter,
            pretty,
            compact,
            no_header,
            flow,
        } => {
            commands::convert::run(&commands::convert::ConvertArgs {
                input: &input,
                to: &to,
                from: from.as_deref(),
                output: output.as_deref(),
                outdir: outdir.as_deref(),
                delimiter,
                pretty,
                compact,
                no_header,
                flow,
            })?;
        }
        Commands::Query { .. } => {
            eprintln!("query command is not yet implemented");
        }
        Commands::View {
            input,
            from,
            path,
            limit,
            columns,
            delimiter,
            no_header,
        } => {
            commands::view::run(&commands::view::ViewArgs {
                input: &input,
                from: from.as_deref(),
                path: path.as_deref(),
                limit,
                columns,
                delimiter,
                no_header,
            })?;
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
