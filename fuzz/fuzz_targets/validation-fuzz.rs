#![no_main]

use libfuzzer_sys::fuzz_target;

use vi::validation;

fuzz_target!(|in_string: &str| {
    validation::is_valid_word(in_string);
});