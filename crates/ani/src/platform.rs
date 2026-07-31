//! Compile-time OpenHarmony API support contract.

/// The oldest OpenHarmony API level supported by ani-rs.
pub const MINIMUM_API_LEVEL: u32 = 23;

/// The OpenHarmony source/header level used to generate `ani-sys`.
pub const HEADER_API_LEVEL: u32 = 26;

/// The newest QEMU image level covered by the release validation suite.
pub const VALIDATED_API_LEVEL: u32 = 26;

/// The minimum runtime required by the selected Cargo feature profile.
#[cfg(feature = "api26")]
pub const SELECTED_API_LEVEL: u32 = 26;
/// The minimum runtime required by the selected Cargo feature profile.
#[cfg(all(not(feature = "api26"), feature = "api24"))]
pub const SELECTED_API_LEVEL: u32 = 24;
/// The minimum runtime required by the selected Cargo feature profile.
#[cfg(all(not(feature = "api26"), not(feature = "api24")))]
pub const SELECTED_API_LEVEL: u32 = 23;

/// Whether primitive boxing/unboxing ANI entry points are compiled in.
pub const HAS_PRIMITIVE_BOXING: bool = cfg!(feature = "api24");

const _: () = assert!(SELECTED_API_LEVEL >= MINIMUM_API_LEVEL);
const _: () = assert!(SELECTED_API_LEVEL <= HEADER_API_LEVEL);

/// A machine-readable support profile for diagnostics and build tooling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SupportProfile {
    /// Oldest runtime usable by this build.
    pub selected_api_level: u32,
    /// API level of the checked-in C header.
    pub header_api_level: u32,
    /// API level validated on the release QEMU image.
    pub validated_api_level: u32,
    /// Whether native primitive boxing is enabled.
    pub primitive_boxing: bool,
}

/// Returns this build's support profile.
pub const fn support_profile() -> SupportProfile {
    SupportProfile {
        selected_api_level: SELECTED_API_LEVEL,
        header_api_level: HEADER_API_LEVEL,
        validated_api_level: VALIDATED_API_LEVEL,
        primitive_boxing: HAS_PRIMITIVE_BOXING,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_contract_never_exceeds_header() {
        assert_eq!(support_profile().selected_api_level, SELECTED_API_LEVEL);
        assert_eq!(HAS_PRIMITIVE_BOXING, SELECTED_API_LEVEL >= 24);
    }
}
