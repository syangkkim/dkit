#![no_main]
use dkit_core::format::csv::CsvReader;
use dkit_core::format::FormatReader;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // Default options (comma delimiter, with header)
    let reader = CsvReader::new(Default::default());
    let _ = reader.read(data);
});
