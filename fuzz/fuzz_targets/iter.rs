#![no_main]

use libfuzzer_sys::fuzz_target;
use rtap::create_radiotap_iterator;

fuzz_target!(|data: &[u8]| {
    let _ = create_radiotap_iterator(&mut data.iter().copied());
});
