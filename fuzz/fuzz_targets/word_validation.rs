#![no_main]
use libfuzzer_sys::fuzz_target;
use vi::validation;

const SINGLE_INITIAL_CONSONANTS: &[&'static str] =
    &["b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",];

const DIGRAPHS_INITIAL_CONSONANTS: &[&'static str] =
    &["ch", "gh", "gi", "kh", "nh", "ng", "ph", "th", "tr", "qu"];

const TRIGRAPHS_INITIAL_CONSONANT: &'static str = "ngh";

const FINAL_CONSONANTS: &[&'static str] = &["c", "ch", "m", "n", "nh", "ng", "p", "t"];

const VOWELS: &[&'static str] = &[
    "ia", "ai", "ieu", "io", "ua", "ao", "au", "oi", "a", "i", "o", "e", "u", "oai", "uou", "uo",
    "uu", "ie", "ay", "oa", "eo", "oeo", "iu", "oao", "oay", "oe", "oo", "ui", "uy", "uya", "uyu",
    "uye", "uoi", "ye", "yeu", "y", "eu", "ue", "uay"
];

fuzz_target!(|value: (usize, usize, usize, bool, bool)| {
    let mut s = String::new();

    if value.3 {
        s += SINGLE_INITIAL_CONSONANTS[value.0 % SINGLE_INITIAL_CONSONANTS.len()];
    } else {
        if value.4 {
            s += DIGRAPHS_INITIAL_CONSONANTS[value.0 % DIGRAPHS_INITIAL_CONSONANTS.len()];
        } else {
            s += TRIGRAPHS_INITIAL_CONSONANT;
        }
    }

    s += VOWELS[value.1 % VOWELS.len()];

    s += FINAL_CONSONANTS[value.2 % FINAL_CONSONANTS.len()];

    assert!(validation::is_valid_word(&s));
});
