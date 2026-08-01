//! Native ArkTS typed-array conversions.

use std::marker::PhantomData;

use crate::env::Env;
use crate::error::{Error, Result, Status};
use crate::sys;
use crate::types::{AniArrayBuffer, AniObject, ani_value_int, ani_value_ref};

use super::{ArrayBuffer, Boxable, FromAni, ToAni, TypeInfo};

/// A fixed-width numeric element that can be encoded into an ArrayBuffer.
/// # Safety
///
/// Implementations must be plain fixed-width values with no padding or invalid
/// bit patterns. Their alignment must match the associated ArkTS typed array.
pub unsafe trait TypedArrayElement: Copy + Send + Sync + 'static {
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
            unsafe impl TypedArrayElement for $ty {
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

/// One byte in an ArkTS `Uint8ClampedArray`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClampedU8(pub u8);

impl From<u8> for ClampedU8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ClampedU8> for u8 {
    fn from(value: ClampedU8) -> Self {
        value.0
    }
}

unsafe impl TypedArrayElement for ClampedU8 {
    const WIDTH: usize = 1;
    const CLASS_NAME: &'static str = "std.core.Uint8ClampedArray";
    const TYPE_SIGNATURE: &'static str = "Lstd/core/Uint8ClampedArray;";

    fn encode(self, output: &mut Vec<u8>) {
        output.push(self.0);
    }

    fn decode(input: &[u8]) -> Self {
        Self(input[0])
    }
}

/// An owned typed numeric array transported as its native ArkTS typed-array class.
#[derive(Debug)]
pub struct TypedArray<T: TypedArrayElement> {
    buffer: ArrayBuffer,
    byte_offset: usize,
    length: usize,
    marker: PhantomData<T>,
}

impl<T: TypedArrayElement> TypedArray<T> {
    /// Creates a typed array from owned elements.
    pub fn new(values: Vec<T>) -> Self {
        let mut bytes = Vec::with_capacity(values.len().saturating_mul(T::WIDTH));
        for value in &values {
            value.encode(&mut bytes);
        }
        Self {
            buffer: ArrayBuffer::new(bytes),
            byte_offset: 0,
            length: values.len(),
            marker: PhantomData,
        }
    }

    fn from_buffer(buffer: ArrayBuffer, byte_offset: usize, length: usize) -> Result<Self> {
        let byte_length = length
            .checked_mul(T::WIDTH)
            .ok_or_else(|| Error::new(Status::OutOfRange, "typed-array byte length overflow"))?;
        let end = byte_offset
            .checked_add(byte_length)
            .ok_or_else(|| Error::new(Status::OutOfRange, "typed-array byte range overflow"))?;
        if end > buffer.len() {
            return Err(Error::new(
                Status::OutOfRange,
                "typed-array view exceeds its backing ArrayBuffer",
            ));
        }
        let ptr = unsafe { buffer.as_ptr().add(byte_offset) };
        if byte_length != 0 && !(ptr as usize).is_multiple_of(std::mem::align_of::<T>()) {
            return Err(Error::new(
                Status::InvalidArgs,
                "typed-array data is misaligned",
            ));
        }
        Ok(Self {
            buffer,
            byte_offset,
            length,
            marker: PhantomData,
        })
    }

    /// Returns the element slice.
    pub fn as_slice(&self) -> &[T] {
        if self.length == 0 {
            return &[];
        }
        unsafe {
            std::slice::from_raw_parts(
                self.buffer.as_ptr().add(self.byte_offset).cast::<T>(),
                self.length,
            )
        }
    }

    /// Returns the mutable element slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.length == 0 {
            return &mut [];
        }
        unsafe {
            std::slice::from_raw_parts_mut(
                self.buffer.as_mut_ptr().add(self.byte_offset).cast::<T>(),
                self.length,
            )
        }
    }

    /// Consumes the wrapper.
    pub fn into_vec(self) -> Vec<T> {
        self.as_slice().to_vec()
    }

    /// Number of elements, not bytes.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Whether no elements are present.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn encode_bytes(&self) -> Vec<u8> {
        let byte_length = self.length.saturating_mul(T::WIDTH);
        self.buffer.as_slice()[self.byte_offset..self.byte_offset + byte_length].to_vec()
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

impl<T: TypedArrayElement> Clone for TypedArray<T> {
    fn clone(&self) -> Self {
        Self::new(self.as_slice().to_vec())
    }
}

impl<T: TypedArrayElement> Default for TypedArray<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<T: TypedArrayElement + PartialEq> PartialEq for TypedArray<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
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
        let length = i32::try_from(self.length).map_err(|_| {
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
        let object = validate_typed_array::<T>(env, value)?;
        let (byte_offset, byte_length, buffer_ref) = typed_array_view_info::<T>(env, &object)?;
        let buffer =
            unsafe { ArrayBuffer::from_ani(env, buffer_ref.as_raw() as sys::ani_arraybuffer) }?;
        Self::from_buffer(buffer, byte_offset, byte_length / T::WIDTH)
    }
}

/// Owned global-reference/COW typed-array model.
///
/// Input is zero-copy and keeps the backing ArrayBuffer alive. The first
/// mutable access detaches into Rust-owned memory because ANI does not expose
/// an external ArrayBuffer API for a symmetric Rust-to-ArkTS zero-copy path.
pub type TypedArrayRef<T> = TypedArray<T>;

fn validate_typed_array<'env, T: TypedArrayElement>(
    env: &Env<'env>,
    value: sys::ani_object,
) -> Result<AniObject<'env>> {
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
    Ok(object)
}

fn typed_array_view_info<'env, T: TypedArrayElement>(
    env: &Env<'env>,
    object: &AniObject<'_>,
) -> Result<(usize, usize, crate::types::AniRef<'env>)> {
    let byte_offset = usize::try_from(env.get_property_by_name_int(object, "byteOffset")?)
        .map_err(|_| Error::new(Status::OutOfRange, "negative typed-array byteOffset"))?;
    let byte_length = usize::try_from(env.get_property_by_name_int(object, "byteLength")?)
        .map_err(|_| Error::new(Status::OutOfRange, "negative typed-array byteLength"))?;
    if !byte_length.is_multiple_of(T::WIDTH) {
        return Err(Error::new(
            Status::InvalidArgs,
            "typed-array byteLength is not a multiple of its element width",
        ));
    }
    Ok((
        byte_offset,
        byte_length,
        env.get_property_by_name_ref(object, "buffer")?,
    ))
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
        let object = validate_typed_array::<T>(env, value)?;
        let (byte_offset, byte_length, buffer_ref) = typed_array_view_info::<T>(env, &object)?;
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

/// Explicitly scoped mutable zero-copy view.
///
/// This is intentionally not a safe `FromAni` argument conversion: ArkTS can
/// alias a typed array, so the caller must guarantee exclusive access for the
/// lifetime of the view.
pub struct TypedArraySliceMut<'env, T: TypedArrayElement> {
    data: &'env mut [T],
    marker: PhantomData<&'env mut AniObject<'env>>,
}

impl<'env, T: TypedArrayElement> TypedArraySliceMut<'env, T> {
    /// Create a mutable view after proving that no ArkTS or Rust alias can
    /// access the typed array for `'env`.
    ///
    /// # Safety
    ///
    /// `value` must be a live `T` typed array and uniquely borrowed for the
    /// returned view's entire lifetime.
    pub unsafe fn from_ani_unchecked(env: &Env<'env>, value: sys::ani_object) -> Result<Self> {
        let object = validate_typed_array::<T>(env, value)?;
        let (byte_offset, byte_length, buffer_ref) = typed_array_view_info::<T>(env, &object)?;
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
            data: unsafe { std::slice::from_raw_parts_mut(data, byte_length / T::WIDTH) },
            marker: PhantomData,
        })
    }

    /// Mutable element slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data
    }

    /// Read-only element slice.
    pub fn as_slice(&self) -> &[T] {
        self.data
    }
}

impl<T: TypedArrayElement> TypeInfo for TypedArraySliceMut<'_, T> {
    fn type_signature() -> &'static str {
        T::TYPE_SIGNATURE
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

/// Owned DataView with the same global-reference/COW backing model as
/// [`TypedArrayRef`].
#[derive(Debug)]
pub struct DataView {
    buffer: ArrayBuffer,
    byte_offset: usize,
    byte_length: usize,
}

impl DataView {
    /// Create a view over an owned buffer range.
    pub fn new(buffer: ArrayBuffer, byte_offset: usize, byte_length: usize) -> Result<Self> {
        let end = byte_offset
            .checked_add(byte_length)
            .ok_or_else(|| Error::new(Status::OutOfRange, "DataView byte range overflow"))?;
        if end > buffer.len() {
            return Err(Error::new(
                Status::OutOfRange,
                "DataView exceeds its backing ArrayBuffer",
            ));
        }
        Ok(Self {
            buffer,
            byte_offset,
            byte_length,
        })
    }

    /// Read-only bytes covered by the view.
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer.as_slice()[self.byte_offset..self.byte_offset + self.byte_length]
    }

    /// Mutable bytes. ANI-backed inputs detach using copy-on-write first.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer.as_mut_slice()[self.byte_offset..self.byte_offset + self.byte_length]
    }

    /// View byte offset in the backing buffer.
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    /// View byte length.
    pub fn byte_length(&self) -> usize {
        self.byte_length
    }
}

impl Clone for DataView {
    fn clone(&self) -> Self {
        Self::new(ArrayBuffer::from(self.as_slice()), 0, self.byte_length)
            .expect("copied DataView range is valid")
    }
}

impl PartialEq for DataView {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for DataView {}

impl TypeInfo for DataView {
    fn type_signature() -> &'static str {
        "Lstd/core/DataView;"
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env> FromAni<'env> for DataView {
    type Input = sys::ani_object;

    unsafe fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        if value.is_null() {
            return Err(Error::new(Status::InvalidArgs, "DataView cannot be null"));
        }
        let object = unsafe { AniObject::from_raw(value) };
        let class = env.find_class("std.core.DataView")?;
        if !env.object_instance_of(&object, &class)? {
            return Err(Error::new(
                Status::InvalidType,
                "expected std.core.DataView",
            ));
        }
        let byte_offset = usize::try_from(env.get_property_by_name_int(&object, "byteOffset")?)
            .map_err(|_| Error::new(Status::OutOfRange, "negative DataView byteOffset"))?;
        let byte_length = usize::try_from(env.get_property_by_name_int(&object, "byteLength")?)
            .map_err(|_| Error::new(Status::OutOfRange, "negative DataView byteLength"))?;
        let buffer = env.get_property_by_name_ref(&object, "buffer")?;
        let buffer =
            unsafe { ArrayBuffer::from_ani(env, buffer.as_raw() as sys::ani_arraybuffer) }?;
        Self::new(buffer, byte_offset, byte_length)
    }
}

impl<'env> ToAni<'env> for DataView {
    type Output = sys::ani_object;

    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        // ANI has no external ArrayBuffer constructor. Preserve the view range
        // semantically by copying the complete backing buffer into a VM-owned
        // ArrayBuffer, then constructing DataView over that object.
        let byte_offset = i32::try_from(self.byte_offset)
            .map_err(|_| Error::new(Status::OutOfRange, "DataView offset exceeds int range"))?;
        let byte_length = i32::try_from(self.byte_length)
            .map_err(|_| Error::new(Status::OutOfRange, "DataView length exceeds int range"))?;
        let buffer = self.buffer.to_ani(env)?;
        let class = env.find_class("std.core.DataView")?;
        let offset = byte_offset.box_value(env)?;
        let length = byte_length.box_value(env)?;
        // Optional ArkTS primitive parameters use the union/reference ABI in
        // current OpenHarmony stdlib. Retain the primitive signature fallback
        // for older API levels whose DataView constructor was lowered that
        // way.
        let object = if let Ok(constructor) = env.find_constructor(
            &class,
            "C{std.core.ArrayBuffer}X{C{std.core.Int}U}X{C{std.core.Int}U}:",
        ) {
            env.new_object(
                &class,
                &constructor,
                &[
                    ani_value_ref(buffer as sys::ani_ref),
                    ani_value_ref(offset.as_raw() as sys::ani_ref),
                    ani_value_ref(length.as_raw() as sys::ani_ref),
                ],
            )?
        } else if let Ok(constructor) = env.find_constructor_by_name(&class) {
            env.new_object(
                &class,
                &constructor,
                &[
                    ani_value_ref(buffer as sys::ani_ref),
                    ani_value_ref(offset.as_raw() as sys::ani_ref),
                    ani_value_ref(length.as_raw() as sys::ani_ref),
                ],
            )?
        } else {
            let constructor = env.find_constructor(&class, "C{std.core.ArrayBuffer}ii:")?;
            env.new_object(
                &class,
                &constructor,
                &[
                    ani_value_ref(buffer as sys::ani_ref),
                    ani_value_int(byte_offset),
                    ani_value_int(byte_length),
                ],
            )?
        };
        Ok(object.into_raw())
    }
}

/// Signed 8-bit typed array.
pub type Int8Array = TypedArray<i8>;
/// Unsigned 8-bit typed array.
pub type Uint8Array = TypedArray<u8>;
/// Clamped unsigned 8-bit typed array.
pub type Uint8ClampedArray = TypedArray<ClampedU8>;
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
        assert_eq!(
            <ClampedU8 as TypedArrayElement>::CLASS_NAME,
            "std.core.Uint8ClampedArray"
        );
        assert_eq!(DataView::type_signature(), "Lstd/core/DataView;");
    }

    #[test]
    fn typed_array_mutation_and_clone_are_owned_cow_values() {
        let original = Uint16Array::new(vec![1, 2, 3]);
        let mut changed = original.clone();
        changed.as_mut_slice()[0] = 9;
        assert_eq!(original.as_slice(), [1, 2, 3]);
        assert_eq!(changed.as_slice(), [9, 2, 3]);
    }

    #[test]
    fn data_view_validates_range_and_mutates_only_view_bytes() {
        let mut view = DataView::new(ArrayBuffer::new(vec![1, 2, 3, 4]), 1, 2).unwrap();
        assert_eq!(view.as_slice(), [2, 3]);
        view.as_mut_slice().copy_from_slice(&[8, 9]);
        assert_eq!(view.as_slice(), [8, 9]);
        assert_eq!(view.byte_offset(), 1);
        assert_eq!(view.byte_length(), 2);
        assert_eq!(
            DataView::new(ArrayBuffer::new(vec![1]), 1, 1)
                .unwrap_err()
                .status,
            Status::OutOfRange
        );
    }

    #[test]
    fn uint8_clamped_array_uses_exact_byte_representation() {
        let values = Uint8ClampedArray::new(vec![ClampedU8(0), ClampedU8(128), ClampedU8(255)]);
        assert_eq!(values.to_le_bytes(), [0, 128, 255]);
    }
}
