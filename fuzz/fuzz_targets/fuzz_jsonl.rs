#![no_main]
use dkit_core::format::jsonl::JsonlReader;
use dkit_core::format::FormatReader;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let reader = JsonlReader;
    let _ = reader.read(data);
});
