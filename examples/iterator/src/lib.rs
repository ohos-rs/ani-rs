//! ArkTS synchronous iterator protocol in both directions.
//!
//! - **Consumption** (ArkTS → Rust): `AniIterator<T>` accepts any
//!   `Iterable<T>` or `Iterator<T>` and drives it lazily from Rust.
//! - **Production** (Rust → ArkTS): a bound class named `*Iterator` whose
//!   `next` method returns `Option<T>` becomes an ArkTS `Iterator<T>`, and
//!   `#[ani(name = "$_iterator")]` makes it iterable in `for..of`.

use ani::conversions::AniIterator;
use ani::prelude::*;
use ani_derive::{ani, AniClass};

// ============================================================================
// Consumption: ArkTS Iterable/Iterator -> Rust
// ============================================================================

/// Joins every string of an ArkTS Iterable (Array, Set, custom class...).
#[ani]
pub fn iter_join_strings(env: &Env<'_>, values: AniIterator<'_, String>) -> Result<String> {
    Ok(values.into_vec(env)?.join(","))
}

/// Sums numbers through the std `Iterator` adapter.
#[ani]
pub fn iter_sum_numbers(env: &Env<'_>, mut values: AniIterator<'_, f64>) -> Result<f64> {
    let mut total = 0.0;
    for value in values.iter(env) {
        total += value?;
    }
    Ok(total)
}

/// Consumes at most `limit` elements, demonstrating lazy iteration: the
/// remaining elements are never pulled from the ArkTS side.
#[ani]
pub fn iter_take_strings(
    env: &Env<'_>,
    mut values: AniIterator<'_, String>,
    limit: i32,
) -> Result<Vec<String>> {
    let mut out = Vec::new();
    while out.len() < limit.max(0) as usize {
        match values.next(env)? {
            Some(value) => out.push(value),
            None => break,
        }
    }
    Ok(out)
}

/// Counts the elements of any iterable without decoding them: `advance`
/// steps the protocol but skips value conversion.
#[ani]
pub fn iter_count(env: &Env<'_>, mut values: AniIterator<'_, AniObject<'_>>) -> Result<i32> {
    let mut count = 0;
    while values.advance(env)? {
        count += 1;
    }
    Ok(count)
}

// ============================================================================
// Production: Rust state -> ArkTS Iterator/Iterable
// ============================================================================

/// Iterable countdown: `for (const v of new CountdownIterator(3))` yields
/// 3, 2, 1. The class name must end with `Iterator` so `next` is bound to
/// the ArkTS iterator protocol.
#[derive(AniClass)]
#[ani(class = "CountdownIterator")]
pub struct CountdownIterator {
    pub current: i32,
}

#[ani(class = "CountdownIterator")]
impl CountdownIterator {
    #[ani(constructor)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(env: &Env<'_>, this: &AniObject<'_>, start: i32) -> Result<()> {
        CountdownIterator { current: start }.write_back_to_ani_object(env, this)
    }

    /// ArkTS `Iterator<int>` protocol: `Some` -> `{ done: false, value }`,
    /// `None` -> `{ done: true }`.
    #[ani]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<i32> {
        if self.current <= 0 {
            None
        } else {
            let value = self.current;
            self.current -= 1;
            Some(value)
        }
    }

    /// Makes the class iterable (`for..of`) by returning a fresh iterator.
    #[ani(name = "$_iterator")]
    pub fn iterator(&self) -> CountdownIterator {
        CountdownIterator {
            current: self.current,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown_yields_descending_values() {
        let mut countdown = CountdownIterator { current: 3 };
        assert_eq!(countdown.next(), Some(3));
        assert_eq!(countdown.next(), Some(2));
        assert_eq!(countdown.next(), Some(1));
        assert_eq!(countdown.next(), None);
        assert_eq!(countdown.next(), None);
    }
}
