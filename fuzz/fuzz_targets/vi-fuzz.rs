#![no_main]

use libfuzzer_sys::fuzz_target;

use vi::vni;
use vi::validation;
use vi::parsing;

fuzz_target!(|data: (u8, &str)| {
    let (opt, in_string) = data;
    let mut result = String::new();
    match opt {
        0 => {
            parsing::parse_word(in_string);
        },
        1 => {
            vni::transform_buffer(in_string.chars(), &mut result);
        },
        2 => {
            validation::is_valid_word(in_string);
        },
        3 => {
            parsing::parse_vowel(in_string);
        }
        _ => ()
    }
});
