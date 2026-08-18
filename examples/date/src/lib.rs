//! ArkTS `Date` exchanged as Rust `std::time::SystemTime`.
//!
//! Build with `--features chrono_example` to also exchange
//! `chrono::DateTime<Utc>` values.

use std::time::{Duration, SystemTime};

use ani::conversions::{millis_to_system_time, system_time_to_millis};
use ani::error::Result;
use ani_derive::ani;

/// Returns the same instant through the ANI object ABI.
#[ani]
pub fn date_identity(value: SystemTime) -> SystemTime {
    value
}

/// Reads the signed milliseconds since the Unix epoch.
#[ani]
pub fn date_to_millis(value: SystemTime) -> f64 {
    system_time_to_millis(value)
}

/// Builds a Date from signed epoch milliseconds, rejecting non-finite and
/// out-of-range input.
#[ani]
pub fn date_from_millis(millis: f64) -> Result<SystemTime> {
    millis_to_system_time(millis)
}

/// Shifts an instant by whole days (may be negative).
#[ani]
pub fn date_add_days(value: SystemTime, days: i32) -> SystemTime {
    let offset = Duration::from_secs(60 * 60 * 24 * u64::from(days.unsigned_abs()));
    if days >= 0 {
        value + offset
    } else {
        value - offset
    }
}

/// Compares two instants.
#[ani]
pub fn date_is_before(first: SystemTime, second: SystemTime) -> bool {
    first < second
}

#[cfg(feature = "chrono_example")]
mod chrono_example {
    use chrono::{DateTime, Datelike, Utc};

    use super::ani;

    /// Returns the same instant through chrono's UTC representation.
    #[ani]
    pub fn chrono_identity(value: DateTime<Utc>) -> DateTime<Utc> {
        value
    }

    /// Reads the UTC calendar year of an ArkTS Date.
    #[ani]
    pub fn chrono_utc_year(value: DateTime<Utc>) -> i32 {
        value.year()
    }
}
