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
use commands::{EncodingOptions, ExcelOptions, SqliteOptions};

fn main() {
    let cli = Cli::parse();

    if cli.list_formats {
        print_formats();
        return;
    }

    match cli.command {
        Some(_) => {}
        None => {
            // No subcommand and no --list-formats: show help
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
            process::exit(2);
        }
    }

    if let Err(err) = run_command(cli) {
        print_error(&err);
        process::exit(1);
    }
}

/// 지원 포맷 목록 출력
fn print_formats() {
    println!("Supported output formats:");
    println!();
    for (name, desc) in format::Format::list_output_formats() {
        println!("  {:<10} {}", name, desc);
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
    match cli.command.unwrap() {
        Commands::Convert {
            input,
            format,
            from,
            output,
            outdir,
            delimiter,
            pretty,
            compact,
            no_header,
            flow,
            root_element,
            styled,
            full_html,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
            rename,
            continue_on_error,
        } => {
            commands::convert::run(&commands::convert::ConvertArgs {
                input: &input,
                to: &format,
                from: from.as_deref(),
                output: output.as_deref(),
                outdir: outdir.as_deref(),
                delimiter,
                pretty,
                compact,
                no_header,
                flow,
                root_element,
                styled,
                full_html,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
                rename: rename.as_deref(),
                continue_on_error,
            })?;
        }
        Commands::Query {
            input,
            query,
            from,
            format,
            output,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            commands::query::run(&commands::query::QueryArgs {
                input: &input,
                query: &query,
                from: from.as_deref(),
                to: format.as_deref(),
                output: output.as_deref(),
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::View {
            input,
            from,
            format,
            path,
            limit,
            columns,
            delimiter,
            no_header,
            max_width,
            hide_header,
            row_numbers,
            border,
            color,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            list_sheets,
            table,
            sql,
            list_tables,
        } => {
            commands::view::run(&commands::view::ViewArgs {
                input: &input,
                from: from.as_deref(),
                format: format.as_deref(),
                path: path.as_deref(),
                limit,
                columns,
                delimiter,
                no_header,
                max_width,
                hide_header,
                row_numbers,
                border: &border,
                color,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                list_sheets,
                sqlite_opts: SqliteOptions { table, sql },
                list_tables,
            })?;
        }
        Commands::Stats {
            input,
            from,
            format,
            path,
            column,
            delimiter,
            no_header,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            commands::stats::run(&commands::stats::StatsArgs {
                input: &input,
                from: from.as_deref(),
                format: format.as_deref(),
                path: path.as_deref(),
                column: column.as_deref(),
                delimiter,
                no_header,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::Merge {
            input,
            format,
            output,
            delimiter,
            pretty,
            compact,
            no_header,
            flow,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            commands::merge::run(&commands::merge::MergeArgs {
                input: &input,
                to: format.as_deref(),
                output: output.as_deref(),
                delimiter,
                no_header,
                pretty,
                compact,
                flow,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::Schema {
            input,
            from,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            commands::schema::run(&commands::schema::SchemaArgs {
                input: &input,
                from: from.as_deref(),
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::Diff {
            file1,
            file2,
            path,
            quiet,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            let has_diff = commands::diff::run(&commands::diff::DiffArgs {
                file1: &file1,
                file2: &file2,
                path: path.as_deref(),
                quiet,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
            if has_diff {
                process::exit(1);
            }
        }
    }

    Ok(())
}
