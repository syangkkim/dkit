#![no_main]
use dkit_core::query::parser::parse_query;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = parse_query(data);
});
