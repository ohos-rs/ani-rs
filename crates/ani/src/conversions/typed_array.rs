//! Native ArkTS typed-array conversions.

use std::marker::PhantomData;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniArrayBuffer, AniObject};

use super::{FromAni, ToAni, TypeInfo};

/// A fixed-width numeric element that can be encoded into an ArrayBuffer.
pub trait TypedArrayElement: Copy + Send + Sync + 'static {
    /// Number of bytes in one element.
    const WIDTH: usize;

    /// ArkTS typed-array class descriptor.
    const CLASS_NAME: &'static str;

    /// ANI type signature for the native typed-array class.
    const TYPE_SIGNATURE: &'static str;

    /// Appends one value using the platform-independent little-endian layout.
    fn encode(self, output: &mut Vec<u8>);

    /// Decodes one value from exactly [`WIDTH`](Self::WIDTH) bytes.
    fn decode(input: &[u8]) -> Self;
}

macro_rules! impl_typed_array_element {
    ($($ty:ty => ($class:literal, $signature:literal)),+ $(,)?) => {
        $(
            impl TypedArrayElement for $ty {
                const WIDTH: usize = std::mem::size_of::<Self>();
                const CLASS_NAME: &'static str = $class;
                const TYPE_SIGNATURE: &'static str = $signature;

                fn encode(self, output: &mut Vec<u8>) {
                    output.extend_from_slice(&self.to_le_bytes());
                }

                fn decode(input: &[u8]) -> Self {
                    Self::from_le_bytes(input.try_into().expect("typed chunk width validated"))
                }
            }
        )+
    };
}

impl_typed_array_element!(
    i8 => ("std.core.Int8Array", "Lstd/core/Int8Array;"),
    u8 => ("std.core.Uint8Array", "Lstd/core/Uint8Array;"),
    i16 => ("std.core.Int16Array", "Lstd/core/Int16Array;"),
    u16 => ("std.core.Uint16Array", "Lstd/core/Uint16Array;"),
    i32 => ("std.core.Int32Array", "Lstd/core/Int32Array;"),
    u32 => ("std.core.Uint32Array", "Lstd/core/Uint32Array;"),
    i64 => ("std.core.BigInt64Array", "Lstd/core/BigInt64Array;"),
    u64 => ("std.core.BigUint64Array", "Lstd/core/BigUint64Array;"),
    f32 => ("std.core.Float32Array", "Lstd/core/Float32Array;"),
    f64 => ("std.core.Float64Array", "Lstd/core/Float64Array;"),
);

/// An owned typed numeric array transported as its native ArkTS typed-array class.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypedArray<T: TypedArrayElement> {
    values: Vec<T>,
    marker: PhantomData<T>,
}

impl<T: TypedArrayElement> TypedArray<T> {
    /// Creates a typed array from owned elements.
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            marker: PhantomData,
        }
    }

    /// Returns the element slice.
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Returns the mutable element slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.values
    }

    /// Consumes the wrapper.
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }

    /// Number of elements, not bytes.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether no elements are present.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn encode_bytes(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.values.len().saturating_mul(T::WIDTH));
        for value in &self.values {
            value.encode(&mut output);
        }
        output
    }

    /// Encodes the elements using the stable little-endian ANI transport.
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.encode_bytes()
    }

    /// Decodes a stable little-endian ANI transport buffer.
    pub fn from_le_bytes(bytes: &[u8]) -> Result<Self> {
        Self::decode_bytes(bytes)
    }

    fn decode_bytes(bytes: &[u8]) -> Result<Self> {
        if T::WIDTH == 0 || !bytes.len().is_multiple_of(T::WIDTH) {
            return Err(Error::new(
                Status::InvalidArgs,
                format!(
                    "ArrayBuffer byte length {} is not divisible by typed element width {}",
                    bytes.len(),
                    T::WIDTH
                ),
            ));
        }
        Ok(Self::new(
            bytes.chunks_exact(T::WIDTH).map(T::decode).collect(),
        ))
    }
}

impl<T: TypedArrayElement> From<Vec<T>> for TypedArray<T> {
    fn from(values: Vec<T>) -> Self {
        Self::new(values)
    }
}

impl<T: TypedArrayElement> TypeInfo for TypedArray<T> {
    fn type_signature() -> &'static str {
        T::TYPE_SIGNATURE
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T: TypedArrayElement> ToAni<'env> for TypedArray<T> {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        let length = i32::try_from(self.values.len()).map_err(|_| {
            Error::new(
                Status::OutOfRange,
                "typed array length exceeds ArkTS int range",
            )
        })?;
        let class = env.find_class(T::CLASS_NAME).map_err(|error| {
            Error::with_cause(
                error.status,
                format!("failed to find typed-array class {}", T::CLASS_NAME),
                error,
            )
        })?;
        let constructor = env.find_constructor(&class, "i:").map_err(|error| {
            Error::with_cause(
                error.status,
                format!("failed to find {} length constructor", T::CLASS_NAME),
                error,
            )
        })?;
        let object = env
            .new_object(&class, &constructor, &[sys::ani_value { i: length }])
            .map_err(|error| {
                Error::with_cause(
                    error.status,
                    format!("failed to construct {}", T::CLASS_NAME),
                    error,
                )
            })?;
        let buffer_ref = env
            .get_property_by_name_ref(&object, "buffer")
            .map_err(|error| {
                Error::with_cause(
                    error.status,
                    format!("failed to read {}.buffer", T::CLASS_NAME),
                    error,
                )
            })?;
        let buffer =
            unsafe { AniArrayBuffer::from_raw(buffer_ref.as_raw() as sys::ani_arraybuffer) };
        let (data, byte_length) = env.get_arraybuffer_info(&buffer)?;
        let bytes = self.encode_bytes();
        if byte_length != bytes.len() {
            return Err(Error::new(
                Status::GenericFailure,
                format!(
                    "{} allocated {byte_length} bytes for {} Rust bytes",
                    T::CLASS_NAME,
                    bytes.len()
                ),
            ));
        }
        if !bytes.is_empty() {
            if data.is_null() {
                return Err(Error::new(
                    Status::GenericFailure,
                    "typed array buffer is null",
                ));
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), data.cast::<u8>(), bytes.len())
            };
        }
        Ok(object.into_raw())
    }
}

impl<'env, T: TypedArrayElement> FromAni<'env> for TypedArray<T> {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let view = unsafe { TypedArraySlice::<T>::from_ani(env, value) }?;
        Ok(Self::new(view.as_slice().to_vec()))
    }
}

/// Zero-copy, read-only view of an ArkTS typed array valid for the ANI scope.
pub struct TypedArraySlice<'env, T: TypedArrayElement> {
    data: &'env [T],
    marker: PhantomData<&'env AniObject<'env>>,
}

impl<'env, T: TypedArrayElement> TypedArraySlice<'env, T> {
    /// Returns the borrowed element slice.
    pub fn as_slice(&self) -> &'env [T] {
        self.data
    }

    /// Copies this view into an owned typed array.
    pub fn to_owned(&self) -> TypedArray<T> {
        TypedArray::new(self.data.to_vec())
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether no elements are present.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: TypedArrayElement> TypeInfo for TypedArraySlice<'_, T> {
    fn type_signature() -> &'static str {
        T::TYPE_SIGNATURE
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T: TypedArrayElement> FromAni<'env> for TypedArraySlice<'env, T> {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(
                Status::InvalidArgs,
                "typed array cannot be null",
            ));
        }
        let object = unsafe { AniObject::from_raw(value) };
        let class = env.find_class(T::CLASS_NAME)?;
        if !env.object_instance_of(&object, &class)? {
            return Err(Error::new(
                Status::InvalidType,
                format!("expected {}", T::CLASS_NAME),
            ));
        }
        let byte_offset = usize::try_from(env.get_property_by_name_int(&object, "byteOffset")?)
            .map_err(|_| Error::new(Status::OutOfRange, "negative typed-array byteOffset"))?;
        let byte_length = usize::try_from(env.get_property_by_name_int(&object, "byteLength")?)
            .map_err(|_| Error::new(Status::OutOfRange, "negative typed-array byteLength"))?;
        if !byte_length.is_multiple_of(T::WIDTH) {
            return Err(Error::new(
                Status::InvalidArgs,
                "typed-array byteLength is not a multiple of its element width",
            ));
        }
        let buffer_ref = env.get_property_by_name_ref(&object, "buffer")?;
        let buffer =
            unsafe { AniArrayBuffer::from_raw(buffer_ref.as_raw() as sys::ani_arraybuffer) };
        let (base, buffer_length) = env.get_arraybuffer_info(&buffer)?;
        let end = byte_offset
            .checked_add(byte_length)
            .ok_or_else(|| Error::new(Status::OutOfRange, "typed-array byte range overflow"))?;
        if end > buffer_length || (byte_length != 0 && base.is_null()) {
            return Err(Error::new(
                Status::OutOfRange,
                "typed-array view exceeds its backing ArrayBuffer",
            ));
        }
        let data = unsafe { base.cast::<u8>().add(byte_offset).cast::<T>() };
        if byte_length != 0 && !(data as usize).is_multiple_of(std::mem::align_of::<T>()) {
            return Err(Error::new(
                Status::InvalidArgs,
                "typed-array data is misaligned",
            ));
        }
        Ok(Self {
            data: unsafe { std::slice::from_raw_parts(data, byte_length / T::WIDTH) },
            marker: PhantomData,
        })
    }
}

/// Signed 8-bit typed array.
pub type Int8Array = TypedArray<i8>;
/// Unsigned 8-bit typed array.
pub type Uint8Array = TypedArray<u8>;
/// Signed 16-bit typed array.
pub type Int16Array = TypedArray<i16>;
/// Unsigned 16-bit typed array.
pub type Uint16Array = TypedArray<u16>;
/// Signed 32-bit typed array.
pub type Int32Array = TypedArray<i32>;
/// Unsigned 32-bit typed array.
pub type Uint32Array = TypedArray<u32>;
/// Signed 64-bit typed array.
pub type BigInt64Array = TypedArray<i64>;
/// Unsigned 64-bit typed array.
pub type BigUint64Array = TypedArray<u64>;
/// 32-bit floating-point typed array.
pub type Float32Array = TypedArray<f32>;
/// 64-bit floating-point typed array.
pub type Float64Array = TypedArray<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_values_round_trip_through_little_endian_bytes() {
        let values = TypedArray::new(vec![0x0102_u16, u16::MAX]);
        assert_eq!(values.encode_bytes(), [0x02, 0x01, 0xff, 0xff]);
        assert_eq!(
            TypedArray::<u16>::decode_bytes(&values.encode_bytes())
                .unwrap()
                .into_vec(),
            vec![0x0102, u16::MAX]
        );
    }

    #[test]
    fn rejects_partial_elements() {
        assert_eq!(
            TypedArray::<u32>::decode_bytes(&[1, 2, 3])
                .unwrap_err()
                .status,
            Status::InvalidArgs
        );
    }

    #[test]
    fn aliases_expose_native_arkts_classes() {
        assert_eq!(Uint8Array::type_signature(), "Lstd/core/Uint8Array;");
        assert_eq!(Float64Array::ani_c_type(), "ani_object");
        assert_eq!(
            <u16 as TypedArrayElement>::CLASS_NAME,
            "std.core.Uint16Array"
        );
    }
}
