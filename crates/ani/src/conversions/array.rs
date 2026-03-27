//! Array Type Conversion
//!
//! Implements conversion between Rust array/vector types and ANI array types
//! - Vec<T> <-> ani_array
//! - VecDeque<T> <-> ani_array
//! - LinkedList<T> <-> ani_array
//! - [T; N] -> ani_array
//! - &[T] -> ani_array

use std::collections::{LinkedList, VecDeque};

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{
    AniArray, AniArrayBuffer, AniClass, AniEnum, AniEnumItem, AniError, AniField, AniFnObject,
    AniFunction, AniMethod, AniModule, AniNamespace, AniObject, AniRef, AniResolver,
    AniStaticField, AniStaticMethod, AniString, AniTupleValue, AniType, AniVariable,
};
use crate::{ani_call, ani_call_ret};

use super::traits::{FromAni, ToAni, TypeInfo};

// ============================================================================
// Vec<T> - [T
// ============================================================================

trait IntoArrayElementRef<'env> {
    fn into_array_element_ref(self) -> AniRef<'env>;
}

trait FromArrayElementInput<'env>: Sized {
    fn from_array_element_ref(value: AniRef<'env>) -> Self;
}

impl<'env> IntoArrayElementRef<'env> for AniRef<'env> {
    fn into_array_element_ref(self) -> AniRef<'env> {
        self
    }
}

impl<'env> FromArrayElementInput<'env> for AniRef<'env> {
    fn from_array_element_ref(value: AniRef<'env>) -> Self {
        value
    }
}

impl<'env> IntoArrayElementRef<'env> for sys::ani_ref {
    fn into_array_element_ref(self) -> AniRef<'env> {
        unsafe { AniRef::from_raw(self) }
    }
}

impl<'env> FromArrayElementInput<'env> for sys::ani_ref {
    fn from_array_element_ref(value: AniRef<'env>) -> Self {
        value.into_raw()
    }
}

macro_rules! impl_array_element_ref_handle {
    ($wrapper:ty) => {
        impl<'env> IntoArrayElementRef<'env> for $wrapper {
            fn into_array_element_ref(self) -> AniRef<'env> {
                unsafe { AniRef::from_raw(self.as_raw() as sys::ani_ref) }
            }
        }

        impl<'env> FromArrayElementInput<'env> for $wrapper {
            fn from_array_element_ref(value: AniRef<'env>) -> Self {
                unsafe { <$wrapper>::from_raw(value.into_raw() as _) }
            }
        }
    };
}

impl_array_element_ref_handle!(AniObject<'env>);
impl_array_element_ref_handle!(AniString<'env>);
impl_array_element_ref_handle!(AniArray<'env>);
impl_array_element_ref_handle!(AniArrayBuffer<'env>);
impl_array_element_ref_handle!(AniClass<'env>);
impl_array_element_ref_handle!(AniType<'env>);
impl_array_element_ref_handle!(AniModule<'env>);
impl_array_element_ref_handle!(AniNamespace<'env>);
impl_array_element_ref_handle!(AniEnum<'env>);
impl_array_element_ref_handle!(AniError<'env>);
impl_array_element_ref_handle!(AniMethod);
impl_array_element_ref_handle!(AniStaticMethod);
impl_array_element_ref_handle!(AniField);
impl_array_element_ref_handle!(AniStaticField);
impl_array_element_ref_handle!(AniFunction);
impl_array_element_ref_handle!(AniFnObject<'env>);
impl_array_element_ref_handle!(AniVariable);
impl_array_element_ref_handle!(AniResolver);
impl_array_element_ref_handle!(AniTupleValue<'env>);
impl_array_element_ref_handle!(AniEnumItem<'env>);

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn type_signature() -> &'static str {
        // Simplified implementation, should actually be generated based on T
        "[Lstd/core/Object;"
    }
    fn ani_c_type() -> &'static str {
        "ani_array"
    }
}

impl<'env, T> ToAni<'env> for Vec<T>
where
    T: ToAni<'env>,
    <T as ToAni<'env>>::Output: IntoArrayElementRef<'env>,
{
    type Output = sys::ani_array;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let mut values = self.into_iter();
        let Some(first) = values.next() else {
            return Ok(env.create_array(0, None)?.into_raw());
        };

        let first_ref = first.to_ani(env)?.into_array_element_ref();
        let array = env.create_array(1, Some(&first_ref))?;

        for value in values {
            let element = value.to_ani(env)?.into_array_element_ref();
            env.push_array_element(&array, &element)?;
        }
        Ok(array.into_raw())
    }
}

impl<'env, T> FromAni<'env> for Vec<T>
where
    T: FromAni<'env>,
    <T as FromAni<'env>>::Input: FromArrayElementInput<'env>,
{
    type Input = sys::ani_array;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "Null pointer: array"));
        }

        let array = unsafe { AniArray::from_raw(value) };
        let len = env.get_array_length(&array)?;
        let mut out = Vec::with_capacity(len);

        for index in 0..len {
            let element = env.get_array_element(&array, index)?;
            let value = T::from_ani(
                env,
                <<T as FromAni<'env>>::Input as FromArrayElementInput<'env>>::from_array_element_ref(element),
            )?;
            out.push(value);
        }

        Ok(out)
    }
}

impl<T: TypeInfo> TypeInfo for VecDeque<T> {
    fn type_signature() -> &'static str {
        <Vec<T> as TypeInfo>::type_signature()
    }

    fn ani_c_type() -> &'static str {
        <Vec<T> as TypeInfo>::ani_c_type()
    }
}

impl<'env, T> ToAni<'env> for VecDeque<T>
where
    Vec<T>: ToAni<'env>,
{
    type Output = <Vec<T> as ToAni<'env>>::Output;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.into_iter().collect::<Vec<_>>().to_ani(env)
    }
}

impl<'env, T> FromAni<'env> for VecDeque<T>
where
    Vec<T>: FromAni<'env>,
{
    type Input = <Vec<T> as FromAni<'env>>::Input;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(Vec::<T>::from_ani(env, value)?.into())
    }
}

impl<T: TypeInfo> TypeInfo for LinkedList<T> {
    fn type_signature() -> &'static str {
        <Vec<T> as TypeInfo>::type_signature()
    }

    fn ani_c_type() -> &'static str {
        <Vec<T> as TypeInfo>::ani_c_type()
    }
}

impl<'env, T> ToAni<'env> for LinkedList<T>
where
    Vec<T>: ToAni<'env>,
{
    type Output = <Vec<T> as ToAni<'env>>::Output;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.into_iter().collect::<Vec<_>>().to_ani(env)
    }
}

impl<'env, T> FromAni<'env> for LinkedList<T>
where
    Vec<T>: FromAni<'env>,
{
    type Input = <Vec<T> as FromAni<'env>>::Input;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(Vec::<T>::from_ani(env, value)?.into_iter().collect())
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

    #[test]
    fn test_list_like_type_signatures_delegate_to_vec() {
        assert_eq!(
            <VecDeque<String>>::type_signature(),
            <Vec<String> as TypeInfo>::type_signature()
        );
        assert_eq!(
            <LinkedList<String>>::type_signature(),
            <Vec<String> as TypeInfo>::type_signature()
        );
    }

    #[test]
    fn test_object_array_and_list_traits_compile() {
        fn assert_to_ani<T>()
        where
            for<'env> T: ToAni<'env>,
        {
        }

        fn assert_from_ani<T>()
        where
            for<'env> T: FromAni<'env>,
        {
        }

        assert_to_ani::<Vec<String>>();
        assert_from_ani::<Vec<String>>();
        assert_to_ani::<VecDeque<String>>();
        assert_from_ani::<VecDeque<String>>();
        assert_to_ani::<LinkedList<String>>();
        assert_from_ani::<LinkedList<String>>();
    }
}
