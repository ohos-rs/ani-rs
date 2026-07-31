#![no_main]

mod ani_host_stubs;

use ani::conversions::Uint32Array;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    let decoded = Uint32Array::from_le_bytes(bytes);
    if bytes.len() % 4 == 0 {
        let values = decoded.expect("aligned typed-array bytes must decode");
        assert_eq!(values.to_le_bytes(), bytes);
    } else {
        assert!(decoded.is_err());
    }
});
