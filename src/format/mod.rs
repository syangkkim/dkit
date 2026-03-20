pub mod csv;
pub mod json;
pub mod toml;
pub mod yaml;

use std::io::{Read, Write};
use std::path::Path;

use crate::error::DkitError;
use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Json,
    Csv,
    Yaml,
    Toml,
}

impl Format {
    pub fn from_str(s: &str) -> Result<Self, DkitError> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "csv" => Ok(Format::Csv),
            "yaml" | "yml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            _ => Err(DkitError::UnknownFormat(s.to_string())),
        }
    }
}

pub fn detect_format(path: &Path) -> Result<Format, DkitError> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Ok(Format::Json),
        Some("csv" | "tsv") => Ok(Format::Csv),
        Some("yaml" | "yml") => Ok(Format::Yaml),
        Some("toml") => Ok(Format::Toml),
        Some(ext) => Err(DkitError::UnknownFormat(ext.to_string())),
        None => Err(DkitError::UnknownFormat("(no extension)".to_string())),
    }
}

pub trait FormatReader {
    fn read(&self, input: &str) -> anyhow::Result<Value>;
    fn read_from_reader(&self, reader: impl Read) -> anyhow::Result<Value>;
}

pub trait FormatWriter {
    fn write(&self, value: &Value) -> anyhow::Result<String>;
    fn write_to_writer(&self, value: &Value, writer: impl Write) -> anyhow::Result<()>;
}
