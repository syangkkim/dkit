#![no_main]
use dkit_core::format::yaml::YamlReader;
use dkit_core::format::FormatReader;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let reader = YamlReader;
    let _ = reader.read(data);
});
