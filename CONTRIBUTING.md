# Contributing to dkit

Thank you for your interest in contributing to dkit! This guide will help you get started.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.75.0 or later
- Git

### Setting Up the Development Environment

```bash
git clone https://github.com/syang0531/dkit.git
cd dkit
cargo build
cargo test
```

### Project Structure

```
dkit/
├── dkit-core/          # Core library (Value types, format readers/writers, query engine)
│   ├── src/
│   │   ├── value.rs    # Unified Value enum
│   │   ├── error.rs    # Error types (DkitError)
│   │   ├── format/     # Format modules (JSON, CSV, YAML, TOML, XML, ...)
│   │   └── query/      # Query parser and evaluator
│   └── benches/        # Criterion benchmarks
├── dkit-cli/           # CLI application
│   ├── src/
│   │   ├── main.rs     # Entry point
│   │   ├── cli.rs      # Clap argument definitions
│   │   ├── commands/   # Subcommand implementations
│   │   └── output/     # Table and pretty-print formatting
│   └── tests/          # Integration tests
├── docs/               # Documentation
└── CHANGELOG.md
```

See [docs/architecture.md](docs/architecture.md) for a detailed overview.

## Development Workflow

### 1. Find or Create an Issue

- Check [open issues](https://github.com/syang0531/dkit/issues) for tasks to work on.
- For new features or bugs, open an issue first to discuss the approach.

### 2. Create a Branch

```bash
git checkout -b feat/my-feature    # For features
git checkout -b fix/my-bugfix      # For bug fixes
```

### 3. Make Changes

- Write code following the style guidelines below.
- Add tests for new functionality.
- Update documentation if the user-facing behavior changes.

### 4. Verify Your Changes

```bash
cargo test                      # Run all tests
cargo clippy -- -D warnings     # Lint (must pass with zero warnings)
cargo fmt -- --check            # Format check
```

All three checks must pass before submitting a PR.

### 5. Commit and Push

Write clear commit messages describing **why** the change was made:

```bash
git commit -m "Add XML namespace stripping option for convert"
git push -u origin feat/my-feature
```

### 6. Open a Pull Request

- Target the `main` branch.
- Include `Closes #N` in the PR body to auto-close the related issue.
- Describe what changed and why.
- PRs will be reviewed and merged once all checks pass.

## Code Style

### Formatting

- Run `cargo fmt` before committing. The CI enforces `rustfmt`.

### Linting

- All code must pass `cargo clippy -- -D warnings` with zero warnings.

### Error Handling

- **dkit-core** (library): Use `thiserror` to define error types in `DkitError`.
- **dkit-cli** (application): Use `anyhow` for error propagation and `miette` for user-friendly display.
- Avoid `.unwrap()` in library code; use `?` operator or return `Result`.

### Testing

- **Unit tests**: Add `#[cfg(test)]` module in the same file for internal logic.
- **Integration tests**: Add tests in `dkit-cli/tests/` for end-to-end CLI behavior using `assert_cmd`.
- Test fixtures go in `dkit-cli/tests/fixtures/`.

### Naming

- Use Rust standard naming conventions (snake_case for functions/variables, CamelCase for types).
- Module file names match the format they implement (e.g., `json.rs`, `csv.rs`, `parquet.rs`).

## Adding a New Format

1. Create a new module in `dkit-core/src/format/` (e.g., `newformat.rs`).
2. Implement `FormatReader` and/or `FormatWriter` traits.
3. Register the format in `dkit-core/src/format/mod.rs` (detection and dispatch).
4. Add CLI support in `dkit-cli/src/cli.rs` and `dkit-cli/src/commands/convert.rs`.
5. Add integration tests in `dkit-cli/tests/`.
6. Update `README.md` and `docs/cli-spec.md`.

## Adding a New Subcommand

1. Create a new file in `dkit-cli/src/commands/` (e.g., `mycommand.rs`).
2. Define CLI arguments in `dkit-cli/src/cli.rs` using clap derive macros.
3. Wire the command in `dkit-cli/src/commands/mod.rs` and `dkit-cli/src/main.rs`.
4. Add integration tests.
5. Update `README.md` and `docs/cli-spec.md`.

## Running Benchmarks

```bash
cd dkit-core
cargo bench
```

Benchmark results are saved to `target/criterion/` with HTML reports.

## Questions?

Open an issue on [GitHub](https://github.com/syang0531/dkit/issues) for any questions about contributing.
