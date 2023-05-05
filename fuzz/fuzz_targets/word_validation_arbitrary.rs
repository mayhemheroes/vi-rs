#![no_main]
use libfuzzer_sys::fuzz_target;
use vi::validation;

fuzz_target!(|value: String| {
    validation::is_valid_word(&value);
});
