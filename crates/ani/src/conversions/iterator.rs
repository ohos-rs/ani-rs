//! ArkTS synchronous iterator protocol.
//!
//! [`AniIterator`] consumes any ArkTS 1.2 `Iterable<T>` or `Iterator<T>`
//! lazily from Rust, without materializing the whole sequence the way
//! `Vec<T>` / `HashSet<T>` conversions do:
//!
//! - **Iterable** objects (Array, Set, Map, custom classes) are resolved via
//!   their `$_iterator()` method, falling back to `values()` and finally to
//!   treating the object itself as an iterator when it exposes `next()`.
//! - **Iterator** objects are driven through `next()`, reading the standard
//!   `done` / `value` result fields.
//!
//! Element decoding reuses the container-element conversion used by
//! Record/Map/Set ([`RecordValue`]), so strings, numbers, and reference
//! handles work out of the box.
//!
//! For the production direction (exposing a Rust iterator to ArkTS), bind a
//! class with a native `next()` method; see `examples/iterator`.
//!
//! # Examples
//!
//! ```rust,ignore
//! use ani::conversions::AniIterator;
//! use ani::prelude::*;
//!
//! #[ani]
//! pub fn join_words(env: &Env<'_>, mut words: AniIterator<'_, String>) -> Result<String> {
//!     let mut out = Vec::new();
//!     while let Some(word) = words.next(env)? {
//!         out.push(word);
//!     }
//!     Ok(out.join(","))
//! }
//! ```

use std::marker::PhantomData;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniClass, AniMethod, AniObject};

use super::collections::{RecordValue, find_method_no_signature};
use super::{FromAni, ToAni, ToAniArg, TypeInfo};

/// Lazily consumes an ArkTS `Iterable<T>` / `Iterator<T>` from Rust.
///
/// See the [module documentation](self) for protocol details.
pub struct AniIterator<'env, T> {
    iterator: AniObject<'env>,
    next_method: AniMethod,
    finished: bool,
    _marker: PhantomData<fn() -> T>,
}

fn class_of<'env>(env: &Env<'env>, object: &AniObject<'env>) -> Result<AniClass<'env>> {
    let object_type = env.get_object_type(object)?;
    Ok(unsafe { AniClass::from_raw(object_type.as_raw() as sys::ani_class) })
}

impl<'env, T> AniIterator<'env, T> {
    /// Wraps an object that already follows the iterator protocol
    /// (exposes `next(): IteratorResult<T>`).
    pub fn from_iterator(env: &Env<'env>, iterator: AniObject<'env>) -> Result<Self> {
        if iterator.is_null() {
            return Err(Error::new(Status::InvalidArgs, "iterator is null"));
        }
        let class = class_of(env, &iterator)?;
        let next_method = find_method_no_signature(env, &class, "next").map_err(|_| {
            Error::new(
                Status::InvalidType,
                "value does not expose an iterator `next()` method",
            )
        })?;
        Ok(Self {
            iterator,
            next_method,
            finished: false,
            _marker: PhantomData,
        })
    }

    /// Resolves an iterator from an iterable object.
    ///
    /// Tries `$_iterator()` (the ArkTS 1.2 lowering of `Symbol.iterator`),
    /// then `values()` (escompat containers), and finally falls back to
    /// treating the object itself as an iterator.
    pub fn from_iterable(env: &Env<'env>, iterable: AniObject<'env>) -> Result<Self> {
        if iterable.is_null() {
            return Err(Error::new(Status::InvalidArgs, "iterable is null"));
        }
        let class = class_of(env, &iterable)?;
        for method_name in ["$_iterator", "values"] {
            if let Ok(method) = find_method_no_signature(env, &class, method_name) {
                let iterator_ref = env.call_method_ref(&iterable, &method, &[])?;
                let iterator =
                    unsafe { AniObject::from_raw(iterator_ref.as_raw() as sys::ani_object) };
                return Self::from_iterator(env, iterator);
            }
        }
        Self::from_iterator(env, iterable).map_err(|_| {
            Error::new(
                Status::InvalidType,
                "value is neither an ArkTS Iterable nor an Iterator",
            )
        })
    }

    /// Returns the underlying iterator object.
    pub fn as_object(&self) -> &AniObject<'env> {
        &self.iterator
    }

    /// Consumes the wrapper and returns the underlying iterator object.
    pub fn into_object(self) -> AniObject<'env> {
        self.iterator
    }
}

impl<'env, T> AniIterator<'env, T> {
    /// Calls `next()`, reads `done`, and returns the raw `value` reference
    /// when the sequence is not exhausted.
    fn step<'call>(&mut self, env: &Env<'call>) -> Result<Option<crate::types::AniRef<'call>>> {
        if self.finished {
            return Ok(None);
        }
        let step = (|| {
            let result_ref = env.call_method_ref(&self.iterator, &self.next_method, &[])?;
            let result = unsafe { AniObject::from_raw(result_ref.as_raw() as sys::ani_object) };
            if env.get_property_by_name_boolean(&result, "done")? {
                return Ok(None);
            }
            env.get_property_by_name_ref(&result, "value").map(Some)
        })();
        match &step {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => self.finished = true,
        }
        step
    }

    /// Advances without decoding the element; returns whether one was
    /// consumed.
    pub fn advance(&mut self, env: &Env<'_>) -> Result<bool> {
        self.step(env).map(|item| item.is_some())
    }

    /// Advances the iterator, decoding the next element.
    ///
    /// Returns `Ok(None)` once the sequence reports `done`. After an error
    /// or completion the iterator stays finished.
    pub fn next<'call>(&mut self, env: &Env<'call>) -> Result<Option<T>>
    where
        T: RecordValue<'call>,
    {
        match self.step(env)? {
            Some(value_ref) => T::from_record_ref(env, &value_ref).map(Some),
            None => Ok(None),
        }
    }

    /// Drains the remaining elements into a `Vec`.
    pub fn into_vec<'call>(mut self, env: &Env<'call>) -> Result<Vec<T>>
    where
        T: RecordValue<'call>,
    {
        let mut out = Vec::new();
        while let Some(item) = self.next(env)? {
            out.push(item);
        }
        Ok(out)
    }

    /// Borrows the iterator as a std [`Iterator`] yielding `Result<T>`.
    pub fn iter<'borrow, 'call>(
        &'borrow mut self,
        env: &'borrow Env<'call>,
    ) -> AniIteratorAdapter<'borrow, 'call, 'env, T>
    where
        T: RecordValue<'call>,
    {
        AniIteratorAdapter { inner: self, env }
    }
}

/// std [`Iterator`] adapter borrowed from [`AniIterator::iter`].
pub struct AniIteratorAdapter<'borrow, 'call, 'env, T> {
    inner: &'borrow mut AniIterator<'env, T>,
    env: &'borrow Env<'call>,
}

impl<'call, T> Iterator for AniIteratorAdapter<'_, 'call, '_, T>
where
    T: RecordValue<'call>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        self.inner.next(self.env).transpose()
    }
}

impl<T> TypeInfo for AniIterator<'_, T> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T> FromAni<'env> for AniIterator<'env, T> {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let object = unsafe { AniObject::from_raw(value) };
        Self::from_iterable(env, object)
    }
}

impl<'env, T> ToAni<'env> for AniIterator<'env, T> {
    type Output = sys::ani_object;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.iterator.into_raw())
    }
}

impl<T> ToAniArg for AniIterator<'_, T> {
    fn to_ani_arg<'env>(&self, _env: &Env<'env>) -> Result<sys::ani_ref> {
        Ok(self.iterator.as_raw() as sys::ani_ref)
    }

    fn arg_signature() -> &'static str {
        "Lstd/core/Object;"
    }
}
