//! Array Type Conversion
//!
//! Implements conversion between Rust array/vector types and ANI array types
//! - Vec<T> <-> ani_array
//! - [T; N] -> ani_array
//! - &[T] -> ani_array

use crate::env::Env;
use crate::error::Result;
use crate::sys;
use crate::{ani_call, ani_call_ret};

use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// Vec<T> - [T
// ============================================================================

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn type_signature() -> &'static str {
        // Simplified implementation, should actually be generated based on T
        "[Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_array"
    }
}

// Vec<i32> -> ani_fixedarray_int
impl<'env> ToAni<'env> for Vec<i32> {
    type Output = sys::ani_fixedarray_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let array = ani_call_ret!(
            env,
            FixedArray_New_Int,
            sys::ani_fixedarray_int,
            std::ptr::null_mut(),
            self.len()
        )?;
        if !self.is_empty() {
            ani_call!(
                env,
                FixedArray_SetRegion_Int,
                array,
                0,
                self.len(),
                self.as_ptr()
            )?;
        }
        Ok(array)
    }
}

impl<'env> FromAni<'env> for Vec<i32> {
    type Input = sys::ani_fixedarray_int;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let len = ani_call_ret!(
            env,
            FixedArray_GetLength,
            usize,
            0,
            value as sys::ani_fixedarray
        )?;
        let mut buffer = vec![0i32; len];
        if len > 0 {
            ani_call!(
                env,
                FixedArray_GetRegion_Int,
                value,
                0,
                len,
                buffer.as_mut_ptr()
            )?;
        }
        Ok(buffer)
    }
}

// Vec<i64> -> ani_fixedarray_long
impl<'env> ToAni<'env> for Vec<i64> {
    type Output = sys::ani_fixedarray_long;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let array = ani_call_ret!(
            env,
            FixedArray_New_Long,
            sys::ani_fixedarray_long,
            std::ptr::null_mut(),
            self.len()
        )?;
        if !self.is_empty() {
            ani_call!(
                env,
                FixedArray_SetRegion_Long,
                array,
                0,
                self.len(),
                self.as_ptr()
            )?;
        }
        Ok(array)
    }
}

impl<'env> FromAni<'env> for Vec<i64> {
    type Input = sys::ani_fixedarray_long;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let len = ani_call_ret!(
            env,
            FixedArray_GetLength,
            usize,
            0,
            value as sys::ani_fixedarray
        )?;
        let mut buffer = vec![0i64; len];
        if len > 0 {
            ani_call!(
                env,
                FixedArray_GetRegion_Long,
                value,
                0,
                len,
                buffer.as_mut_ptr()
            )?;
        }
        Ok(buffer)
    }
}

// Vec<f64> -> ani_fixedarray_double
impl<'env> ToAni<'env> for Vec<f64> {
    type Output = sys::ani_fixedarray_double;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let array = ani_call_ret!(
            env,
            FixedArray_New_Double,
            sys::ani_fixedarray_double,
            std::ptr::null_mut(),
            self.len()
        )?;
        if !self.is_empty() {
            ani_call!(
                env,
                FixedArray_SetRegion_Double,
                array,
                0,
                self.len(),
                self.as_ptr()
            )?;
        }
        Ok(array)
    }
}

impl<'env> FromAni<'env> for Vec<f64> {
    type Input = sys::ani_fixedarray_double;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let len = ani_call_ret!(
            env,
            FixedArray_GetLength,
            usize,
            0,
            value as sys::ani_fixedarray
        )?;
        let mut buffer = vec![0f64; len];
        if len > 0 {
            ani_call!(
                env,
                FixedArray_GetRegion_Double,
                value,
                0,
                len,
                buffer.as_mut_ptr()
            )?;
        }
        Ok(buffer)
    }
}

// Vec<f32> -> ani_fixedarray_float
impl<'env> ToAni<'env> for Vec<f32> {
    type Output = sys::ani_fixedarray_float;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let array = ani_call_ret!(
            env,
            FixedArray_New_Float,
            sys::ani_fixedarray_float,
            std::ptr::null_mut(),
            self.len()
        )?;
        if !self.is_empty() {
            ani_call!(
                env,
                FixedArray_SetRegion_Float,
                array,
                0,
                self.len(),
                self.as_ptr()
            )?;
        }
        Ok(array)
    }
}

impl<'env> FromAni<'env> for Vec<f32> {
    type Input = sys::ani_fixedarray_float;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let len = ani_call_ret!(
            env,
            FixedArray_GetLength,
            usize,
            0,
            value as sys::ani_fixedarray
        )?;
        let mut buffer = vec![0f32; len];
        if len > 0 {
            ani_call!(
                env,
                FixedArray_GetRegion_Float,
                value,
                0,
                len,
                buffer.as_mut_ptr()
            )?;
        }
        Ok(buffer)
    }
}

// Vec<bool> -> ani_fixedarray_boolean
impl<'env> ToAni<'env> for Vec<bool> {
    type Output = sys::ani_fixedarray_boolean;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let array = ani_call_ret!(
            env,
            FixedArray_New_Boolean,
            sys::ani_fixedarray_boolean,
            std::ptr::null_mut(),
            self.len()
        )?;
        if !self.is_empty() {
            let ani_bools: Vec<sys::ani_boolean> =
                self.iter().map(|&b| if b { 1 } else { 0 }).collect();
            ani_call!(
                env,
                FixedArray_SetRegion_Boolean,
                array,
                0,
                self.len(),
                ani_bools.as_ptr()
            )?;
        }
        Ok(array)
    }
}

impl<'env> FromAni<'env> for Vec<bool> {
    type Input = sys::ani_fixedarray_boolean;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let len = ani_call_ret!(
            env,
            FixedArray_GetLength,
            usize,
            0,
            value as sys::ani_fixedarray
        )?;
        let mut buffer: Vec<sys::ani_boolean> = vec![0; len];
        if len > 0 {
            ani_call!(
                env,
                FixedArray_GetRegion_Boolean,
                value,
                0,
                len,
                buffer.as_mut_ptr()
            )?;
        }
        Ok(buffer.into_iter().map(|b| b != 0).collect())
    }
}

// Vec<u8> / bytes -> ani_fixedarray_byte
impl<'env> ToAni<'env> for Vec<u8> {
    type Output = sys::ani_fixedarray_byte;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let array = ani_call_ret!(
            env,
            FixedArray_New_Byte,
            sys::ani_fixedarray_byte,
            std::ptr::null_mut(),
            self.len()
        )?;
        if !self.is_empty() {
            ani_call!(
                env,
                FixedArray_SetRegion_Byte,
                array,
                0,
                self.len(),
                self.as_ptr() as *const i8
            )?;
        }
        Ok(array)
    }
}

impl<'env> FromAni<'env> for Vec<u8> {
    type Input = sys::ani_fixedarray_byte;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let len = ani_call_ret!(
            env,
            FixedArray_GetLength,
            usize,
            0,
            value as sys::ani_fixedarray
        )?;
        let mut buffer: Vec<i8> = vec![0; len];
        if len > 0 {
            ani_call!(
                env,
                FixedArray_GetRegion_Byte,
                value,
                0,
                len,
                buffer.as_mut_ptr()
            )?;
        }
        Ok(buffer.into_iter().map(|b| b as u8).collect())
    }
}

// ============================================================================
// Slice Types (only supports ToAni, as references cannot be returned)
// ============================================================================

impl<'env> ToAni<'env> for &[i32] {
    type Output = sys::ani_fixedarray_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

impl<'env> ToAni<'env> for &[i64] {
    type Output = sys::ani_fixedarray_long;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

impl<'env> ToAni<'env> for &[f64] {
    type Output = sys::ani_fixedarray_double;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

impl<'env> ToAni<'env> for &[u8] {
    type Output = sys::ani_fixedarray_byte;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_type_signature() {
        assert_eq!(<Vec<i32>>::type_signature(), "[Lstd/core/Object;");
    }
}
