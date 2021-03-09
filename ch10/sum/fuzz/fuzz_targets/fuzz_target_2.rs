#![no_main]
use libfuzzer_sys::fuzz_target;
use sum::sum_wrappiing;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    sum_wrappiing(data);
});
