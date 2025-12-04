use crate::{
    errors::*,
    objects::{AutoLocal, JObject},
    anienv::ANIEnv,
};

#[cfg(doc)]
use crate::objects::{JClass, JMethodID};

/// Trait for things that can be looked up via a descriptor.
///
/// # Safety
///
/// Implementations of this trait must return the correct value from the
/// `lookup` method.
pub unsafe trait Desc<'local, T> {
    /// The type that this `Desc` returns.
    type Output: AsRef<T>;

    /// Look up the concrete type from the VM.
    fn lookup(self, _: &mut ANIEnv<'local>) -> Result<Self::Output>;
}

unsafe impl<'local, T> Desc<'local, T> for T
where
    T: AsRef<T>,
{
    type Output = Self;

    fn lookup(self, _: &mut ANIEnv<'local>) -> Result<T> {
        Ok(self)
    }
}

unsafe impl<'local, T> Desc<'local, T> for &T
where
    T: AsRef<T>,
{
    type Output = Self;

    fn lookup(self, _: &mut ANIEnv<'local>) -> Result<Self::Output> {
        Ok(self)
    }
}

unsafe impl<'local, 'other_local, T> Desc<'local, T> for AutoLocal<'other_local, T>
where
    T: AsRef<T> + Into<JObject<'other_local>>,
{
    type Output = Self;

    fn lookup(self, _: &mut ANIEnv<'local>) -> Result<Self::Output> {
        Ok(self)
    }
}

unsafe impl<'local, 'other_local, T> Desc<'local, T> for &AutoLocal<'other_local, T>
where
    T: AsRef<T> + Into<JObject<'other_local>>,
{
    type Output = Self;

    fn lookup(self, _: &mut ANIEnv<'local>) -> Result<Self::Output> {
        Ok(self)
    }
}
