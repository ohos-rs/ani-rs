//! Dynamic `Any_*` wrapper built on existing conversion traits.

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::types::AniRef;

use super::{FromAni, ToAni, ToAniArg, ToAniArgs, TypeInfo};

/// Rust-side wrapper for dynamic values (`ani_ref`) using `Any_*` APIs.
pub struct AnyValue<'env>(AniRef<'env>);

impl<'env> AnyValue<'env> {
    /// Wrap an existing reference as dynamic value.
    #[inline]
    pub fn from_ref(value: AniRef<'env>) -> Self {
        Self(value)
    }

    /// Wrap a borrowed `AniRef` without forcing unsafe at call sites.
    #[inline]
    pub fn from_borrowed_ref(value: &AniRef<'env>) -> Self {
        Self(unsafe { AniRef::from_raw(value.as_raw()) })
    }

    /// Consume and return underlying `AniRef`.
    #[inline]
    pub fn into_ref(self) -> AniRef<'env> {
        self.0
    }

    /// Check runtime `instanceof`.
    #[inline]
    pub fn instance_of(&self, env: &Env<'env>, ty: &AnyValue<'_>) -> Result<bool> {
        env.any_instance_of(self.as_ref(), ty.as_ref())
    }

    /// Get property by name.
    #[inline]
    pub fn get_property(&self, env: &Env<'env>, name: &str) -> Result<Self> {
        Ok(Self(env.any_get_property(self.as_ref(), name)?))
    }

    /// Set property by name with another dynamic value.
    #[inline]
    pub fn set_property_ref(
        &self,
        env: &Env<'env>,
        name: &str,
        value: &AnyValue<'_>,
    ) -> Result<()> {
        env.any_set_property(self.as_ref(), name, value.as_ref())
    }

    /// Set property by name with a Rust value using `ToAniArg`.
    #[inline]
    pub fn set_property_arg<T: ToAniArg>(
        &self,
        env: &Env<'env>,
        name: &str,
        value: T,
    ) -> Result<()> {
        let raw = value.to_ani_arg(env)?;
        let as_ref = unsafe { AniRef::from_raw(raw) };
        env.any_set_property(self.as_ref(), name, &as_ref)
    }

    /// Get value by index.
    #[inline]
    pub fn get_index(&self, env: &Env<'env>, index: usize) -> Result<Self> {
        Ok(Self(env.any_get_by_index(self.as_ref(), index)?))
    }

    /// Set value by index with another dynamic value.
    #[inline]
    pub fn set_index_ref(&self, env: &Env<'env>, index: usize, value: &AnyValue<'_>) -> Result<()> {
        env.any_set_by_index(self.as_ref(), index, value.as_ref())
    }

    /// Set value by index with a Rust value using `ToAniArg`.
    #[inline]
    pub fn set_index_arg<T: ToAniArg>(
        &self,
        env: &Env<'env>,
        index: usize,
        value: T,
    ) -> Result<()> {
        let raw = value.to_ani_arg(env)?;
        let as_ref = unsafe { AniRef::from_raw(raw) };
        env.any_set_by_index(self.as_ref(), index, &as_ref)
    }

    /// Get value by dynamic key.
    #[inline]
    pub fn get_by_value(&self, env: &Env<'env>, key: &AnyValue<'_>) -> Result<Self> {
        Ok(Self(env.any_get_by_value(self.as_ref(), key.as_ref())?))
    }

    /// Set value by dynamic key.
    #[inline]
    pub fn set_by_value(
        &self,
        env: &Env<'env>,
        key: &AnyValue<'_>,
        value: &AnyValue<'_>,
    ) -> Result<()> {
        env.any_set_by_value(self.as_ref(), key.as_ref(), value.as_ref())
    }

    /// Call value as function using `FnArgs/ToAniArgs`.
    pub fn call<Args>(&self, env: &Env<'env>, args: Args) -> Result<Self>
    where
        Args: ToAniArgs,
    {
        let raw_args = args.to_ani_args(env)?;
        let refs: Vec<AniRef<'env>> = raw_args
            .iter()
            .map(|raw| unsafe { AniRef::from_raw(*raw) })
            .collect();
        Ok(Self(env.any_call(self.as_ref(), refs.as_slice())?))
    }

    /// Call object method by name using `FnArgs/ToAniArgs`.
    pub fn call_method<Args>(&self, env: &Env<'env>, name: &str, args: Args) -> Result<Self>
    where
        Args: ToAniArgs,
    {
        let raw_args = args.to_ani_args(env)?;
        let refs: Vec<AniRef<'env>> = raw_args
            .iter()
            .map(|raw| unsafe { AniRef::from_raw(*raw) })
            .collect();
        Ok(Self(env.any_call_method(
            self.as_ref(),
            name,
            refs.as_slice(),
        )?))
    }

    /// Construct value as ctor using `FnArgs/ToAniArgs`.
    pub fn construct<Args>(&self, env: &Env<'env>, args: Args) -> Result<Self>
    where
        Args: ToAniArgs,
    {
        let raw_args = args.to_ani_args(env)?;
        let refs: Vec<AniRef<'env>> = raw_args
            .iter()
            .map(|raw| unsafe { AniRef::from_raw(*raw) })
            .collect();
        Ok(Self(env.any_new(self.as_ref(), refs.as_slice())?))
    }
}

impl<'env> AsRef<AniRef<'env>> for AnyValue<'env> {
    #[inline]
    fn as_ref(&self) -> &AniRef<'env> {
        &self.0
    }
}

impl<'env> From<AniRef<'env>> for AnyValue<'env> {
    #[inline]
    fn from(value: AniRef<'env>) -> Self {
        Self(value)
    }
}

impl<'env> From<AnyValue<'env>> for AniRef<'env> {
    #[inline]
    fn from(value: AnyValue<'env>) -> Self {
        value.0
    }
}

impl TypeInfo for AnyValue<'_> {
    fn type_signature() -> &'static str {
        "Lstd/core/Object;"
    }

    fn ani_c_type() -> &'static str {
        "ani_ref"
    }
}

impl<'env> ToAni<'env> for AnyValue<'env> {
    type Output = sys::ani_ref;

    #[inline]
    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self.0.into_raw())
    }
}

impl<'env> FromAni<'env> for AnyValue<'env> {
    type Input = sys::ani_ref;

    #[inline]
    unsafe fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(Self(unsafe { AniRef::from_raw(value) }))
    }
}
