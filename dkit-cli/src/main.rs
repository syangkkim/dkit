mod cli;
mod commands;
mod config;
mod output;

use std::process;

use clap::Parser;
use colored::Colorize;

use clap::CommandFactory;
use cli::{AliasAction, Cli, Commands, ConfigAction};
use commands::{EncodingOptions, ExcelOptions, ParquetWriteOptions, SqliteOptions};
use dkit_core::error::{suggest_format, DkitError, SUPPORTED_FORMATS};

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
    // Collect raw args and expand aliases before clap parsing
    let raw_args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    let expanded = expand_alias(raw_args);

    let cli = match Cli::try_parse_from(&expanded) {
        Ok(cli) => cli,
        Err(e) => {
            e.exit();
        }
    };
    let verbose = cli.verbose;

    if cli.list_formats {
        print_formats();
        return 0;
    }

    if cli.examples {
        print_examples();
        return 0;
    }

    match cli.command {
        Some(_) => {}
        None => {
            // No subcommand and no --list-formats: show help
            Cli::command().print_help().ok();
            println!();
            return 2;
        }
    }

    if let Err(err) = run_command(cli) {
        print_error(&err, verbose);
        return 1;
    }
    0
}

/// Split an alias command string into individual arguments,
/// handling single and double quoted strings.
fn split_alias_command(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;

    for ch in s.chars() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            ' ' | '\t' if !in_single && !in_double => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Expand aliases in the raw argument list.
/// If args[1] matches a known alias, replace it with the alias expansion.
fn expand_alias(args: Vec<std::ffi::OsString>) -> Vec<std::ffi::OsString> {
    if args.len() < 2 {
        return args;
    }

    let candidate = match args[1].to_str() {
        Some(s) if !s.starts_with('-') => s.to_string(),
        _ => return args,
    };

    // Check built-in aliases first
    let builtins = config::builtin_aliases();
    let alias_cmd = if let Some(cmd) = builtins.get(&candidate) {
        Some(cmd.clone())
    } else {
        // Check user-defined aliases from config
        let source = config::load_config();
        source.config.aliases.get(&candidate).cloned()
    };

    if let Some(cmd) = alias_cmd {
        let mut new_args = vec![args[0].clone()];
        for part in split_alias_command(&cmd) {
            new_args.push(std::ffi::OsString::from(part));
        }
        new_args.extend_from_slice(&args[2..]);
        new_args
    } else {
        args
    }
}

/// 자주 사용하는 예시 모음 출력
fn print_examples() {
    println!(
        "\
Frequently used examples:

  Format conversion:
    dkit convert data.json -f csv              # JSON → CSV
    dkit convert data.csv -f yaml -o out.yaml  # CSV → YAML (file output)
    dkit convert '*.json' -f csv --outdir out  # Batch convert all JSON files
    cat data.json | dkit convert --from json -f toml  # Pipe: JSON → TOML

  Querying:
    dkit query data.json '.users[0].name'      # Extract a field
    dkit query data.yaml '.config.database'    # Navigate nested data
    dkit query data.json '.items[*].price' -f csv  # Query with output format

  Viewing:
    dkit view data.csv                         # Pretty table output
    dkit view data.json --path .users -n 5     # Nested data, limit rows
    dkit view data.csv --columns name,email    # Select columns
    dkit view data.csv --sort-by age --head 10 # Sort and limit

  Statistics:
    dkit stats data.csv                        # Overview statistics
    dkit stats data.csv --column revenue       # Single column stats
    dkit stats data.csv --histogram            # Text histogram

  Comparing:
    dkit diff old.json new.json                # Show differences
    dkit diff a.yaml b.yaml --quiet            # Exit code only

  Merging:
    dkit merge a.json b.json -f json           # Merge files
    dkit merge users1.csv users2.csv -f csv    # Merge CSV files

  Sampling:
    dkit sample data.csv -n 100               # Random sample
    dkit sample data.csv -n 50 --seed 42      # Reproducible sample

  Schema & validation:
    dkit schema data.json                     # Show data structure
    dkit validate data.json --schema s.json   # Validate against schema

  Flatten / unflatten:
    dkit flatten nested.json                  # Flatten nested keys
    dkit unflatten flat.json                  # Restore nested structure

  Other:
    dkit completions bash > ~/.bash_completion.d/dkit  # Shell completions
    dkit config show                          # Show configuration
    dkit alias set j2c 'convert -f csv'       # Create alias

Use 'dkit <command> --help' for detailed help on each command."
    );
}

/// 지원 포맷 목록 출력
fn print_formats() {
    println!("Supported output formats:");
    println!();
    for (name, desc) in dkit_core::format::Format::list_output_formats() {
        println!("  {:<10} {}", name, desc);
    }
}

/// 에러를 색상 강조와 함께 stderr에 출력
/// - error=빨강, hint/warning=노랑, suggestion=청색
/// - ParseErrorAt: 줄/열 번호 + 코드 스니펫(화살표)
/// - UnknownFormat: "Did you mean?" 제안
/// - verbose=true: 전체 에러 체인 출력
fn print_error(err: &anyhow::Error, verbose: bool) {
    // DkitError로 다운캐스트하여 향상된 출력 시도
    if let Some(dkit_err) = err.downcast_ref::<DkitError>() {
        match dkit_err {
            DkitError::UnknownFormat(s) => {
                eprintln!("{} Unknown format: '{s}'", "error:".red().bold());
                eprintln!(
                    "  {}",
                    format!("Supported formats: {}", SUPPORTED_FORMATS.join(", ")).yellow()
                );
                if let Some(suggestion) = suggest_format(s) {
                    eprintln!(
                        "  {}",
                        format!("Did you mean '{suggestion}'?").cyan().bold()
                    );
                }
                return;
            }
            DkitError::ParseErrorAt {
                format,
                source,
                line,
                column,
                line_text,
            } => {
                eprintln!(
                    "{} Failed to parse {format}: {source}",
                    "error:".red().bold()
                );
                eprintln!("  {} line {line}, column {column}", "-->".blue().bold());
                if !line_text.is_empty() {
                    let line_num = line.to_string();
                    let pad = " ".repeat(line_num.len());
                    eprintln!("  {pad} {}", "|".blue());
                    eprintln!("  {line_num} {} {line_text}", "|".blue());
                    let arrow_pad = " ".repeat(column.saturating_sub(1));
                    eprintln!("  {pad} {} {arrow_pad}{}", "|".blue(), "^".red().bold());
                    eprintln!("  {pad} {}", "|".blue());
                }
                eprintln!(
                    "  {}",
                    format!("Hint: check that the input is valid {format}").yellow()
                );
                if verbose {
                    eprintln!("\n  {}", "verbose backtrace:".dimmed());
                    eprintln!("  {}", format!("{err:?}").dimmed());
                }
                return;
            }
            DkitError::FormatDetectionFailed(s) => {
                eprintln!("{} Failed to detect format: {s}", "error:".red().bold());
                eprintln!(
                    "  {}",
                    "Hint: specify the input format explicitly, e.g. --from json".yellow()
                );
                eprintln!(
                    "  {}",
                    format!("Supported formats: {}", SUPPORTED_FORMATS.join(", ")).yellow()
                );
                return;
            }
            _ => {} // fall through to generic display
        }
    }

    // 제네릭 에러 출력
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

    if verbose {
        eprintln!("\n  {}", "verbose backtrace:".dimmed());
        eprintln!("  {}", format!("{err:?}").dimmed());
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
            watch,
            watch_paths,
        } => {
            let run = || {
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
                    root_element: root_element.clone(),
                    styled,
                    full_html,
                    encoding_opts: EncodingOptions {
                        encoding: encoding.clone(),
                        detect_encoding,
                    },
                    excel_opts: ExcelOptions {
                        sheet: sheet.clone(),
                        header_row,
                    },
                    sqlite_opts: SqliteOptions {
                        table: table.clone(),
                        sql: sql.clone(),
                    },
                    rename: rename.as_deref(),
                    continue_on_error,
                    data_filter: commands::DataFilterOptions {
                        sort_by: sort_by.clone(),
                        descending: sort_order.eq_ignore_ascii_case("desc"),
                        head,
                        tail,
                        filter: filter.clone(),
                    },
                    parquet_opts: ParquetWriteOptions {
                        compression: compression.clone(),
                        row_group_size,
                    },
                    streaming_opts: chunk_size.map(|cs| commands::streaming::StreamingOptions {
                        chunk_size: cs,
                        progress,
                    }),
                })
            };

            if watch {
                let targets = commands::watch::collect_watch_targets(&input, &watch_paths);
                commands::watch::run_watch(&targets, run)?;
            } else {
                run()?;
            }
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
            watch,
            watch_paths,
        } => {
            let run = || {
                commands::view::run(&commands::view::ViewArgs {
                    input: &input,
                    from: from.as_deref(),
                    format: format.as_deref(),
                    path: path.as_deref(),
                    limit,
                    columns: columns.clone(),
                    delimiter,
                    no_header,
                    max_width,
                    hide_header,
                    row_numbers,
                    border: &border,
                    color,
                    encoding_opts: EncodingOptions {
                        encoding: encoding.clone(),
                        detect_encoding,
                    },
                    excel_opts: ExcelOptions {
                        sheet: sheet.clone(),
                        header_row,
                    },
                    list_sheets,
                    sqlite_opts: SqliteOptions {
                        table: table.clone(),
                        sql: sql.clone(),
                    },
                    list_tables,
                    data_filter: commands::DataFilterOptions {
                        sort_by: sort_by.clone(),
                        descending: sort_order.eq_ignore_ascii_case("desc"),
                        head,
                        tail,
                        filter: filter.clone(),
                    },
                })
            };

            if watch {
                let targets =
                    commands::watch::collect_watch_targets_from_input(&input, &watch_paths);
                commands::watch::run_watch(&targets, run)?;
            } else {
                run()?;
            }
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
        Commands::Sample {
            input,
            count,
            ratio,
            seed,
            method,
            stratify_by,
            from,
            format,
            output,
            delimiter,
            no_header,
            pretty,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            commands::sample::run(&commands::sample::SampleArgs {
                input: &input,
                count,
                ratio,
                seed,
                method: &method,
                stratify_by: stratify_by.as_deref(),
                from: from.as_deref(),
                format: format.as_deref(),
                output: output.as_deref(),
                delimiter,
                no_header,
                pretty,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::Flatten {
            input,
            separator,
            array_format,
            max_depth,
            from,
            format,
            output,
            delimiter,
            no_header,
            pretty,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            let af = commands::flatten::ArrayFormat::from_str(&array_format)?;
            commands::flatten::run_flatten(&commands::flatten::FlattenArgs {
                input: &input,
                separator: &separator,
                array_format: af,
                max_depth,
                from: from.as_deref(),
                format: format.as_deref(),
                output: output.as_deref(),
                delimiter,
                no_header,
                pretty,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::Unflatten {
            input,
            separator,
            from,
            format,
            output,
            delimiter,
            no_header,
            pretty,
            encoding,
            detect_encoding,
            sheet,
            header_row,
            table,
            sql,
        } => {
            commands::flatten::run_unflatten(&commands::flatten::UnflattenArgs {
                input: &input,
                separator: &separator,
                from: from.as_deref(),
                format: format.as_deref(),
                output: output.as_deref(),
                delimiter,
                no_header,
                pretty,
                encoding_opts: EncodingOptions {
                    encoding,
                    detect_encoding,
                },
                excel_opts: ExcelOptions { sheet, header_row },
                sqlite_opts: SqliteOptions { table, sql },
            })?;
        }
        Commands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "dkit", &mut std::io::stdout());
        }
        Commands::Config { action } => match action {
            ConfigAction::Show => config::run_show()?,
            ConfigAction::Init { project } => config::run_init(project)?,
        },
        Commands::Alias { action } => {
            let source = config::load_config();
            match action {
                AliasAction::Set(args) => config::run_alias_set(&args.name, &args.command)?,
                AliasAction::List => config::run_alias_list(&source.config.aliases)?,
                AliasAction::Remove { name } => config::run_alias_remove(&name)?,
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
