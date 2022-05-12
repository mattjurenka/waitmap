#![no_main]
#![feature(array_chunks)]
use libfuzzer_sys::fuzz_target;

use waitmap::WaitMap;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mapping = WaitMap::new();
    let mut count = 0;
    for [key, val] in data.array_chunks() {
        if mapping.get(key).is_none() {
            count = count + 1;
        }
        mapping.insert(key, val);
    }
});
