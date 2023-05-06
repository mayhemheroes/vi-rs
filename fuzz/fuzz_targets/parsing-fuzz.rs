#![no_main]

use libfuzzer_sys::fuzz_target;

use vi::parsing;

fuzz_target!(|data: (u8, &str)| {
    let (opt, in_string) = data;
    let mut result = String::new();
    match opt {
        0 => {
            parsing::parse_word(in_string);
        },
        1 => {
            parsing::parse_vowel(in_string);
        }
        _ => ()
    }
});