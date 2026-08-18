//! ArkTS `Date` (`escompat.Date`) values.
//!
//! Maps `std::time::SystemTime` to ArkTS `Date` in both directions with
//! millisecond precision, mirroring napi-rs' `Date` support:
//!
//! - **Rust → ArkTS**: constructs `new Date(milliseconds)`.
//! - **ArkTS → Rust**: validates the object is an `escompat.Date` and reads
//!   `getTime()`.
//!
//! Sub-millisecond precision is truncated because ArkTS `Date` stores whole
//! milliseconds. Times before the Unix epoch map to negative millisecond
//! values, matching JavaScript/ArkTS `Date` semantics.
//!
//! With the `chrono` cargo feature enabled, `chrono::DateTime<Utc>` converts
//! through the same millisecond representation.
//!
//! # Examples
//!
//! ```rust,ignore
//! use std::time::SystemTime;
//! use ani::prelude::*;
//!
//! #[ani]
//! pub fn echo_date(value: SystemTime) -> SystemTime {
//!     value
//! }
//! ```

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniClass, AniObject, ani_value_double};

use super::either::ValidateFromAni;
use super::{FromAni, ToAni, ToAniArg, TypeInfo};

/// Largest absolute millisecond value an ArkTS/JS `Date` can represent
/// (±100,000,000 days around the epoch, per ECMA-262).
const MAX_DATE_MILLIS: f64 = 8.64e15;

fn find_date_class<'env>(env: &Env<'env>) -> Result<AniClass<'env>> {
    env.find_class("escompat.Date")
        .or_else(|_| env.find_class("Lescompat/Date;"))
}

/// Converts a [`SystemTime`] into signed milliseconds since the Unix epoch,
/// truncating sub-millisecond precision.
pub fn system_time_to_millis(time: SystemTime) -> f64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(after) => after.as_millis() as f64,
        // Pre-epoch times: negative offset. `unwrap` cannot fail because
        // `duration_since` erring means `UNIX_EPOCH > time`.
        Err(_) => -(UNIX_EPOCH
            .duration_since(time)
            .expect("time is before the Unix epoch")
            .as_millis() as f64),
    }
}

/// Converts signed milliseconds since the Unix epoch into a [`SystemTime`].
///
/// Rejects non-finite values and values outside the ArkTS `Date` range
/// (±8.64e15 ms), which also protects the `Duration` construction from
/// overflow.
pub fn millis_to_system_time(millis: f64) -> Result<SystemTime> {
    if !millis.is_finite() {
        return Err(Error::new(
            Status::InvalidArgs,
            format!("Date milliseconds is not finite: {millis}"),
        ));
    }
    if millis.abs() > MAX_DATE_MILLIS {
        return Err(Error::new(
            Status::OutOfRange,
            format!("Date milliseconds out of ArkTS Date range: {millis}"),
        ));
    }
    // Split into whole milliseconds plus fractional nanoseconds instead of
    // dividing by 1000.0, which would round through f64 seconds and lose
    // exactness for large timestamps.
    let magnitude = millis.abs();
    let whole_millis = magnitude.trunc();
    let frac_nanos = ((magnitude - whole_millis) * 1e6).round() as u32;
    let offset =
        Duration::from_millis(whole_millis as u64) + Duration::from_nanos(u64::from(frac_nanos));
    if millis >= 0.0 {
        UNIX_EPOCH.checked_add(offset)
    } else {
        UNIX_EPOCH.checked_sub(offset)
    }
    .ok_or_else(|| {
        Error::new(
            Status::OutOfRange,
            format!("Date milliseconds does not fit in SystemTime: {millis}"),
        )
    })
}

fn date_object_from_millis<'env>(env: &Env<'env>, millis: f64) -> Result<sys::ani_object> {
    let class = find_date_class(env)?;
    let constructor = env
        .find_constructor(&class, "d:")
        .or_else(|_| env.find_constructor(&class, "D:V"))?;
    let args = [ani_value_double(millis)];
    Ok(env.new_object(&class, &constructor, &args)?.into_raw())
}

fn millis_from_date_object<'env>(env: &Env<'env>, value: sys::ani_object) -> Result<f64> {
    if value.is_null() {
        return Err(Error::new(Status::InvalidArgs, "Date value is null"));
    }
    let object = unsafe { AniObject::from_raw(value) };
    let class = find_date_class(env)?;
    if !env.is_instance_of(&object, &class)? {
        return Err(Error::new(
            Status::InvalidType,
            "value is not an ArkTS Date",
        ));
    }
    let method = env
        .find_method(&class, "getTime", ":d")
        .or_else(|_| env.find_method(&class, "getTime", ":D"))?;
    env.call_method_double(&object, &method, &[])
}

impl TypeInfo for SystemTime {
    fn type_signature() -> &'static str {
        "Lescompat/Date;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> ToAni<'env> for SystemTime {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        date_object_from_millis(env, system_time_to_millis(self))
    }
}

impl<'env> FromAni<'env> for SystemTime {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        millis_to_system_time(millis_from_date_object(env, value)?)
    }
}

impl ToAniArg for SystemTime {
    fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
        (*self).to_ani(env).map(|value| value as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lescompat/Date;"
    }
}

impl<'env> ValidateFromAni<'env> for SystemTime {
    unsafe fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
        if value.is_null() {
            return false;
        }
        let object = unsafe { AniObject::from_raw(value) };
        match find_date_class(env) {
            Ok(class) => env.is_instance_of(&object, &class).unwrap_or(false),
            Err(_) => false,
        }
    }
}

#[cfg(feature = "chrono")]
mod chrono_impl {
    use chrono::{DateTime, Utc};

    use super::*;

    impl TypeInfo for DateTime<Utc> {
        fn type_signature() -> &'static str {
            "Lescompat/Date;"
        }

        fn ani_c_type() -> &'static str {
            "ani_object"
        }
    }

    impl<'env> ToAni<'env> for DateTime<Utc> {
        type Output = sys::ani_object;

        fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
            date_object_from_millis(env, self.timestamp_millis() as f64)
        }
    }

    impl<'env> FromAni<'env> for DateTime<Utc> {
        type Input = sys::ani_object;

        unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
            let millis = millis_from_date_object(env, value)?;
            if !millis.is_finite() || millis.abs() > MAX_DATE_MILLIS {
                return Err(Error::new(
                    Status::OutOfRange,
                    format!("Date milliseconds out of chrono range: {millis}"),
                ));
            }
            DateTime::<Utc>::from_timestamp_millis(millis as i64).ok_or_else(|| {
                Error::new(
                    Status::OutOfRange,
                    format!("Date milliseconds does not fit in chrono DateTime: {millis}"),
                )
            })
        }
    }

    impl ToAniArg for DateTime<Utc> {
        fn to_ani_arg<'env>(&self, env: &Env<'env>) -> Result<sys::ani_ref> {
            (*self).to_ani(env).map(|value| value as sys::ani_ref)
        }

        fn arg_signature() -> &'static str {
            "Lescompat/Date;"
        }
    }

    impl<'env> ValidateFromAni<'env> for DateTime<Utc> {
        unsafe fn validate(env: &Env<'env>, value: sys::ani_object) -> bool {
            unsafe { SystemTime::validate(env, value) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_maps_to_zero_millis() {
        assert_eq!(system_time_to_millis(UNIX_EPOCH), 0.0);
        assert_eq!(millis_to_system_time(0.0).unwrap(), UNIX_EPOCH);
    }

    #[test]
    fn positive_millis_roundtrip() {
        let time = UNIX_EPOCH + Duration::from_millis(1_700_000_000_123);
        let millis = system_time_to_millis(time);
        assert_eq!(millis, 1_700_000_000_123.0);
        assert_eq!(millis_to_system_time(millis).unwrap(), time);
    }

    #[test]
    fn pre_epoch_millis_roundtrip() {
        let time = UNIX_EPOCH - Duration::from_millis(86_400_000);
        let millis = system_time_to_millis(time);
        assert_eq!(millis, -86_400_000.0);
        assert_eq!(millis_to_system_time(millis).unwrap(), time);
    }

    #[test]
    fn sub_millisecond_precision_is_truncated() {
        let time = UNIX_EPOCH + Duration::new(1, 999_999);
        assert_eq!(system_time_to_millis(time), 1000.0);
    }

    #[test]
    fn non_finite_millis_are_rejected() {
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                millis_to_system_time(invalid).unwrap_err().status,
                Status::InvalidArgs
            );
        }
    }

    #[test]
    fn out_of_range_millis_are_rejected() {
        assert_eq!(
            millis_to_system_time(8.64e15 + 1e10).unwrap_err().status,
            Status::OutOfRange
        );
        assert_eq!(
            millis_to_system_time(-8.64e15 - 1e10).unwrap_err().status,
            Status::OutOfRange
        );
    }

    #[test]
    fn boundary_millis_are_accepted() {
        assert!(millis_to_system_time(8.64e15).is_ok());
        assert!(millis_to_system_time(-8.64e15).is_ok());
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn chrono_millis_are_consistent_with_system_time() {
        use chrono::{DateTime, Utc};

        let chrono_time = DateTime::<Utc>::from_timestamp_millis(1_700_000_000_123).unwrap();
        let system_time = millis_to_system_time(1_700_000_000_123.0).unwrap();
        assert_eq!(
            chrono_time.timestamp_millis() as f64,
            system_time_to_millis(system_time)
        );
    }
}
