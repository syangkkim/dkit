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
use commands::{EncodingOptions, ExcelOptions, ParquetWriteOptions, SqliteOptions};

fn main() {
    // Spawn a thread with a larger stack to prevent stack overflows on Windows,
    // which has a smaller default stack size (1MB) than Linux/macOS.
    let builder = std::thread::Builder::new().stack_size(32 * 1024 * 1024);
    let handler = builder
        .spawn(run_main)
        .expect("failed to spawn main thread");
    process::exit(handler.join().unwrap_or(1));
}

fn run_main() -> i32 {
    let cli = Cli::parse();

    if cli.list_formats {
        print_formats();
        return 0;
    }

    match cli.command {
        Some(_) => {}
        None => {
            // No subcommand and no --list-formats: show help
            use clap::CommandFactory;
            Cli::command().print_help().ok();
            println!();
            return 2;
        }
    }

    if let Err(err) = run_command(cli) {
        print_error(&err);
        return 1;
    }
    0
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
            sort_by,
            sort_order,
            head,
            tail,
            filter,
            compression,
            row_group_size,
            chunk_size,
            progress,
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
                data_filter: commands::DataFilterOptions {
                    sort_by,
                    descending: sort_order.eq_ignore_ascii_case("desc"),
                    head,
                    tail,
                    filter,
                },
                parquet_opts: ParquetWriteOptions {
                    compression,
                    row_group_size,
                },
                streaming_opts: chunk_size.map(|cs| commands::streaming::StreamingOptions {
                    chunk_size: cs,
                    progress,
                }),
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
            sort_by,
            sort_order,
            head,
            tail,
            filter,
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
                data_filter: commands::DataFilterOptions {
                    sort_by,
                    descending: sort_order.eq_ignore_ascii_case("desc"),
                    head,
                    tail,
                    filter,
                },
            })?;
        }
        Commands::Stats {
            input,
            from,
            format,
            path,
            column,
            field,
            histogram,
            delimiter,
            no_header,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            let effective_column = column.or(field);
            commands::stats::run(&commands::stats::StatsArgs {
                input: &input,
                from: from.as_deref(),
                format: format.as_deref(),
                path: path.as_deref(),
                column: effective_column.as_deref(),
                histogram,
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
        Commands::Validate {
            input,
            schema,
            from,
            quiet,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            let is_invalid = commands::validate::run(&commands::validate::ValidateArgs {
                input: &input,
                schema: &schema,
                from: from.as_deref(),
                quiet,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
            if is_invalid {
                process::exit(1);
            }
        }
        Commands::Diff {
            file1,
            file2,
            path,
            quiet,
            mode,
            diff_format,
            array_diff,
            ignore_order,
            ignore_case,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            let diff_mode = commands::diff::DiffMode::from_str(&mode)?;
            let diff_output_format = commands::diff::DiffOutputFormat::from_str(&diff_format)?;
            let array_diff_strategy = commands::diff::ArrayDiffStrategy::from_str(&array_diff)?;
            let has_diff = commands::diff::run(&commands::diff::DiffArgs {
                file1: &file1,
                file2: &file2,
                path: path.as_deref(),
                quiet,
                mode: diff_mode,
                diff_format: diff_output_format,
                array_diff: array_diff_strategy,
                ignore_order,
                ignore_case,
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
