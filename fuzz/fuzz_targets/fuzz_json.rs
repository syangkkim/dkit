#![no_main]
use dkit_core::format::json::JsonReader;
use dkit_core::format::FormatReader;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let reader = JsonReader;
    let _ = reader.read(data);
});
