//! Array Type Conversion
//!
//! Implements conversion between Rust array/vector types and ANI array types
//! - Vec<T> <-> ani_array
//! - [T; N] -> ani_array
//! - &[T] -> ani_array

use crate::env::Env;
use crate::error::{Error, Result};
use crate::sys;

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

// Vec<i32> -> ani_array_int
impl<'env> ToAni<'env> for Vec<i32> {
    type Output = sys::ani_array_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        unsafe {
            let api = &*(*env.as_raw());
            let mut array: sys::ani_array_int = std::ptr::null_mut();

            let status = (api.Array_New_Int.unwrap())(env.as_raw(), self.len(), &mut array);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            // Set elements
            if !self.is_empty() {
                let status = (api.Array_SetRegion_Int.unwrap())(
                    env.as_raw(),
                    array,
                    0,
                    self.len(),
                    self.as_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(array)
        }
    }
}

impl<'env> FromAni<'env> for Vec<i32> {
    type Input = sys::ani_array_int;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe {
            let api = &*(*env.as_raw());

            // Get length
            let mut len: usize = 0;
            let status =
                (api.Array_GetLength.unwrap())(env.as_raw(), value as sys::ani_array, &mut len);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            // Allocate buffer
            let mut buffer = vec![0i32; len];

            if len > 0 {
                let status = (api.Array_GetRegion_Int.unwrap())(
                    env.as_raw(),
                    value,
                    0,
                    len,
                    buffer.as_mut_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(buffer)
        }
    }
}

// Vec<i64> -> ani_array_long
impl<'env> ToAni<'env> for Vec<i64> {
    type Output = sys::ani_array_long;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        unsafe {
            let api = &*(*env.as_raw());
            let mut array: sys::ani_array_long = std::ptr::null_mut();

            let status = (api.Array_New_Long.unwrap())(env.as_raw(), self.len(), &mut array);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            if !self.is_empty() {
                let status = (api.Array_SetRegion_Long.unwrap())(
                    env.as_raw(),
                    array,
                    0,
                    self.len(),
                    self.as_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(array)
        }
    }
}

impl<'env> FromAni<'env> for Vec<i64> {
    type Input = sys::ani_array_long;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe {
            let api = &*(*env.as_raw());

            let mut len: usize = 0;
            let status =
                (api.Array_GetLength.unwrap())(env.as_raw(), value as sys::ani_array, &mut len);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            let mut buffer = vec![0i64; len];

            if len > 0 {
                let status = (api.Array_GetRegion_Long.unwrap())(
                    env.as_raw(),
                    value,
                    0,
                    len,
                    buffer.as_mut_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(buffer)
        }
    }
}

// Vec<f64> -> ani_array_double
impl<'env> ToAni<'env> for Vec<f64> {
    type Output = sys::ani_array_double;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        unsafe {
            let api = &*(*env.as_raw());
            let mut array: sys::ani_array_double = std::ptr::null_mut();

            let status = (api.Array_New_Double.unwrap())(env.as_raw(), self.len(), &mut array);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            if !self.is_empty() {
                let status = (api.Array_SetRegion_Double.unwrap())(
                    env.as_raw(),
                    array,
                    0,
                    self.len(),
                    self.as_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(array)
        }
    }
}

impl<'env> FromAni<'env> for Vec<f64> {
    type Input = sys::ani_array_double;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe {
            let api = &*(*env.as_raw());

            let mut len: usize = 0;
            let status =
                (api.Array_GetLength.unwrap())(env.as_raw(), value as sys::ani_array, &mut len);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            let mut buffer = vec![0f64; len];

            if len > 0 {
                let status = (api.Array_GetRegion_Double.unwrap())(
                    env.as_raw(),
                    value,
                    0,
                    len,
                    buffer.as_mut_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(buffer)
        }
    }
}

// Vec<f32> -> ani_array_float
impl<'env> ToAni<'env> for Vec<f32> {
    type Output = sys::ani_array_float;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        unsafe {
            let api = &*(*env.as_raw());
            let mut array: sys::ani_array_float = std::ptr::null_mut();

            let status = (api.Array_New_Float.unwrap())(env.as_raw(), self.len(), &mut array);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            if !self.is_empty() {
                let status = (api.Array_SetRegion_Float.unwrap())(
                    env.as_raw(),
                    array,
                    0,
                    self.len(),
                    self.as_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(array)
        }
    }
}

impl<'env> FromAni<'env> for Vec<f32> {
    type Input = sys::ani_array_float;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe {
            let api = &*(*env.as_raw());

            let mut len: usize = 0;
            let status =
                (api.Array_GetLength.unwrap())(env.as_raw(), value as sys::ani_array, &mut len);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            let mut buffer = vec![0f32; len];

            if len > 0 {
                let status = (api.Array_GetRegion_Float.unwrap())(
                    env.as_raw(),
                    value,
                    0,
                    len,
                    buffer.as_mut_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(buffer)
        }
    }
}

// Vec<bool> -> ani_array_boolean
impl<'env> ToAni<'env> for Vec<bool> {
    type Output = sys::ani_array_boolean;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        unsafe {
            let api = &*(*env.as_raw());
            let mut array: sys::ani_array_boolean = std::ptr::null_mut();

            let status = (api.Array_New_Boolean.unwrap())(env.as_raw(), self.len(), &mut array);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            if !self.is_empty() {
                // 转换 bool 到 ani_boolean
                let ani_bools: Vec<sys::ani_boolean> =
                    self.iter().map(|&b| if b { 1 } else { 0 }).collect();

                let status = (api.Array_SetRegion_Boolean.unwrap())(
                    env.as_raw(),
                    array,
                    0,
                    self.len(),
                    ani_bools.as_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(array)
        }
    }
}

impl<'env> FromAni<'env> for Vec<bool> {
    type Input = sys::ani_array_boolean;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe {
            let api = &*(*env.as_raw());

            let mut len: usize = 0;
            let status =
                (api.Array_GetLength.unwrap())(env.as_raw(), value as sys::ani_array, &mut len);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            let mut buffer: Vec<sys::ani_boolean> = vec![0; len];

            if len > 0 {
                let status = (api.Array_GetRegion_Boolean.unwrap())(
                    env.as_raw(),
                    value,
                    0,
                    len,
                    buffer.as_mut_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(buffer.into_iter().map(|b| b != 0).collect())
        }
    }
}

// Vec<u8> / bytes -> ani_array_byte
impl<'env> ToAni<'env> for Vec<u8> {
    type Output = sys::ani_array_byte;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        unsafe {
            let api = &*(*env.as_raw());
            let mut array: sys::ani_array_byte = std::ptr::null_mut();

            let status = (api.Array_New_Byte.unwrap())(env.as_raw(), self.len(), &mut array);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            if !self.is_empty() {
                let status = (api.Array_SetRegion_Byte.unwrap())(
                    env.as_raw(),
                    array,
                    0,
                    self.len(),
                    self.as_ptr() as *const i8,
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(array)
        }
    }
}

impl<'env> FromAni<'env> for Vec<u8> {
    type Input = sys::ani_array_byte;

    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        unsafe {
            let api = &*(*env.as_raw());

            let mut len: usize = 0;
            let status =
                (api.Array_GetLength.unwrap())(env.as_raw(), value as sys::ani_array, &mut len);

            if status != sys::ani_status_ANI_OK {
                return Err(Error::from_status(crate::error::Status::from(status)));
            }

            let mut buffer: Vec<i8> = vec![0; len];

            if len > 0 {
                let status = (api.Array_GetRegion_Byte.unwrap())(
                    env.as_raw(),
                    value,
                    0,
                    len,
                    buffer.as_mut_ptr(),
                );

                if status != sys::ani_status_ANI_OK {
                    return Err(Error::from_status(crate::error::Status::from(status)));
                }
            }

            Ok(buffer.into_iter().map(|b| b as u8).collect())
        }
    }
}

// ============================================================================
// Slice Types (only supports ToAni, as references cannot be returned)
// ============================================================================

impl<'env> ToAni<'env> for &[i32] {
    type Output = sys::ani_array_int;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

impl<'env> ToAni<'env> for &[i64] {
    type Output = sys::ani_array_long;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

impl<'env> ToAni<'env> for &[f64] {
    type Output = sys::ani_array_double;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        self.to_vec().to_ani(env)
    }
}

impl<'env> ToAni<'env> for &[u8] {
    type Output = sys::ani_array_byte;

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
