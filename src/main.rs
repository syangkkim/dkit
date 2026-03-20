mod cli;
mod commands;
mod error;
mod format;
mod output;
mod query;
mod value;

use std::process;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run_command(cli) {
        print_error(&err);
        process::exit(1);
    }
}

/// 에러를 색상 강조와 함께 stderr에 출력
fn print_error(err: &anyhow::Error) {
    let msg = format!("{err:#}");
    let mut lines = msg.lines();

    // 첫 줄은 "error:" 접두사와 빨간색
    if let Some(first) = lines.next() {
        eprintln!("{} {}", "error:".red().bold(), first);
    }

    // 나머지 줄 (힌트 등)은 노란색
    for line in lines {
        let line = line.trim();
        if line.starts_with("Hint:") || line.starts_with("Supported formats:") {
            eprintln!("  {}", line.yellow());
        } else if !line.is_empty() {
            eprintln!("  {line}");
        }
    }
}

fn run_command(cli: Cli) -> anyhow::Result<()> {
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
        Commands::Query {
            input,
            query,
            from,
            to,
            output,
        } => {
            commands::query::run(&commands::query::QueryArgs {
                input: &input,
                query: &query,
                from: from.as_deref(),
                to: to.as_deref(),
                output: output.as_deref(),
            })?;
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
        Commands::Stats {
            input,
            from,
            path,
            column,
            delimiter,
            no_header,
        } => {
            commands::stats::run(&commands::stats::StatsArgs {
                input: &input,
                from: from.as_deref(),
                path: path.as_deref(),
                column: column.as_deref(),
                delimiter,
                no_header,
            })?;
        }
        Commands::Schema { input, from } => {
            commands::schema::run(&commands::schema::SchemaArgs {
                input: &input,
                from: from.as_deref(),
            })?;
        }
    }

    Ok(())
}
