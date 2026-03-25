#![no_main]
use dkit_core::format::toml::TomlReader;
use dkit_core::format::FormatReader;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let reader = TomlReader;
    let _ = reader.read(data);
});
