#![no_main]

use libfuzzer_sys::fuzz_target;

use vi::vni;

fuzz_target!(|in_string: &str| {
    let mut result = String::new();
    vni::transform_buffer(in_string.chars(), &mut result);
});
