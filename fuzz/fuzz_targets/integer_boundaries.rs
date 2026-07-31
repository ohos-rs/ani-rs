#![no_main]

mod ani_host_stubs;

use ani::conversions::BigInt;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (u64, i128, u128)| {
    let unsigned = BigInt::from(input.0);
    assert_eq!(unsigned.to_u64().ok(), Some(input.0));

    let signed128 = BigInt::from(input.1);
    assert_eq!(signed128.to_i128().ok(), Some(input.1));

    let unsigned128 = BigInt::from(input.2);
    assert_eq!(unsigned128.to_u128().ok(), Some(input.2));
});
