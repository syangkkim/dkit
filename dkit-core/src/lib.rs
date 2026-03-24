//! # dkit-core
//!
//! Core library for **dkit** — a Swiss army knife for data format conversion and querying.
//!
//! This crate provides the foundational types and engines that power dkit:
//!
//! - [`value::Value`] — Unified data model representing JSON, CSV, YAML, TOML, and more
//! - [`error::DkitError`] — Structured error types for parsing, writing, and querying
//! - [`format`] — Readers and writers for 12+ data formats (JSON, CSV, YAML, TOML, XML, etc.)
//! - [`query`] — Query engine with path navigation, filtering, sorting, and built-in functions
//!
//! ## Quick Start
//!
//! ```rust
//! use dkit_core::format::FormatReader;
//! use dkit_core::format::json::JsonReader;
//!
//! let json = r#"{"name": "Alice", "age": 30}"#;
//! let reader = JsonReader;
//! let value = reader.read(json).unwrap();
//! ```

/// Core error types for format parsing, writing, and query evaluation.
pub mod error;

/// Data format readers and writers.
///
/// Supported formats: JSON, JSON Lines, CSV/TSV, YAML, TOML, XML, MessagePack,
/// Excel (xlsx, read-only), SQLite (read-only), Apache Parquet, Markdown (write-only),
/// HTML (write-only).
pub mod format;

/// Query engine: parser, evaluator, filter operations, and built-in functions.
pub mod query;

/// Unified data model — the `Value` enum that all formats convert to and from.
pub mod value;
