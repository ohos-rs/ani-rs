//! Typed numeric arrays backed by ANI `ArrayBuffer` values.

use std::marker::PhantomData;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;

use super::{ArrayBuffer, FromAni, ToAni, TypeInfo};

/// A fixed-width numeric element that can be encoded into an ArrayBuffer.
pub trait TypedArrayElement: Copy + Send + Sync + 'static {
    /// Number of bytes in one element.
    const WIDTH: usize;

    /// Appends one value using the platform-independent little-endian layout.
    fn encode(self, output: &mut Vec<u8>);

    /// Decodes one value from exactly [`WIDTH`](Self::WIDTH) bytes.
    fn decode(input: &[u8]) -> Self;
}

macro_rules! impl_typed_array_element {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl TypedArrayElement for $ty {
                const WIDTH: usize = std::mem::size_of::<Self>();

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

impl_typed_array_element!(i8, u8, i16, u16, i32, u32, i64, u64, f32, f64);

/// An owned typed numeric view transported through ANI as an ArrayBuffer.
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
        "Lescompat/ArrayBuffer;"
    }

    fn ani_c_type() -> &'static str {
        "ani_arraybuffer"
    }
}

impl<'env, T: TypedArrayElement> ToAni<'env> for TypedArray<T> {
    type Output = sys::ani_arraybuffer;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        ArrayBuffer::new(self.encode_bytes()).to_ani(env)
    }
}

impl<'env, T: TypedArrayElement> FromAni<'env> for TypedArray<T> {
    type Input = sys::ani_arraybuffer;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        let buffer = unsafe { ArrayBuffer::from_ani(env, value) }?;
        Self::decode_bytes(buffer.as_slice())
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
}
