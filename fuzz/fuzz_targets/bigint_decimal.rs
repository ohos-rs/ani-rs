#![no_main]

mod ani_host_stubs;

use ani::conversions::BigInt;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: &str| {
    if let Ok(value) = BigInt::from_decimal(input) {
        let canonical = value.as_decimal();
        assert!(!canonical.is_empty());
        assert!(!canonical.starts_with('+'));
        assert!(!canonical.starts_with("-0"));
        assert_eq!(
            BigInt::from_decimal(canonical).expect("canonical bigint must parse"),
            value
        );
    }
});
