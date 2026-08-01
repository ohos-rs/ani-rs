//! ArrayBuffer Example
//!
//! Demonstrates Rust ↔ ANI ArrayBuffer conversion using ani-rs.
//!
//! - **ArrayBufferSlice&lt;'env&gt;** — zero-copy borrowed view from ANI; use as argument when you
//!   only read in the current scope.
//! - **ArrayBuffer** — owned buffer; use when you need to own data or return a buffer to ANI
//!   (ToAni copies data into a new ANI ArrayBuffer).

use ani::error::Result;
use ani::prelude::{
    ArrayBuffer, ArrayBufferSlice, ClampedU8, DataView, Uint16Array, Uint8ClampedArray,
};
use ani_derive::ani;

// ============================================================================
// Read-only view (ArrayBufferSlice) — zero-copy from ANI
// ============================================================================

/// Return the length of an ArrayBuffer in bytes.
///
/// ArkTS: `native function bufferLength(buf: ArrayBuffer): int;`
#[ani]
pub fn buffer_length(buf: ArrayBufferSlice<'_>) -> i32 {
    buf.len() as i32
}

/// Sum all bytes in an ArrayBuffer (e.g. simple checksum).
///
/// ArkTS: `native function sumBytes(buf: ArrayBuffer): long;`
#[ani]
pub fn sum_bytes(buf: ArrayBufferSlice<'_>) -> i64 {
    buf.iter().map(|&b| b as i64).sum()
}

// ============================================================================
// Owned buffer (ArrayBuffer) — as argument or return
// ============================================================================

/// Take an owned ArrayBuffer and return its length.
///
/// ArkTS: `native function processBuffer(buf: ArrayBuffer): int;`
#[ani]
pub fn process_buffer(buf: ArrayBuffer) -> i32 {
    buf.len() as i32
}

/// Create a new zero-initialized ArrayBuffer of the given size and return it to ANI.
///
/// ArkTS: `native function createBuffer(size: int): ArrayBuffer;`
#[ani]
pub fn create_buffer(size: i32) -> ArrayBuffer {
    let len = size.max(0) as usize;
    ArrayBuffer::zeroed(len)
}

/// Concatenate two ArrayBuffer slices into a new owned ArrayBuffer.
///
/// ArkTS: `native function concatBuffers(a: ArrayBuffer, b: ArrayBuffer): ArrayBuffer;`
#[ani]
pub fn concat_buffers(a: ArrayBufferSlice<'_>, b: ArrayBufferSlice<'_>) -> ArrayBuffer {
    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend_from_slice(a.as_ref());
    out.extend_from_slice(b.as_ref());
    ArrayBuffer::new(out)
}

/// Mutate an ANI-backed input. `ArrayBuffer` applies copy-on-write before the
/// Rust mutation and returns a runtime-owned buffer to ArkTS.
#[ani]
pub fn replace_first_byte(mut buffer: ArrayBuffer, value: u8) -> ArrayBuffer {
    if let Some(first) = buffer.as_mut_slice().first_mut() {
        *first = value;
    }
    buffer
}

#[ani]
pub fn first_byte(buffer: ArrayBufferSlice<'_>) -> u8 {
    buffer.first().copied().unwrap_or_default()
}

/// Create a typed numeric buffer with a stable little-endian representation.
#[ani]
pub fn make_u16_array() -> Uint16Array {
    Uint16Array::new(vec![1, 0x0102, u16::MAX])
}

#[ani]
pub fn sum_u16_array(values: Uint16Array) -> i64 {
    values
        .as_slice()
        .iter()
        .map(|value| i64::from(*value))
        .sum()
}

/// Mutate an owned/COW typed-array input. The ArkTS input remains unchanged.
#[ani]
pub fn replace_first_u16(mut values: Uint16Array) -> Uint16Array {
    if let Some(first) = values.as_mut_slice().first_mut() {
        *first = 99;
    }
    values
}

/// Create the native clamped typed-array class.
#[ani]
pub fn make_clamped_array() -> Uint8ClampedArray {
    Uint8ClampedArray::new(vec![ClampedU8(0), ClampedU8(128), ClampedU8(255)])
}

#[ani]
pub fn sum_clamped_array(values: Uint8ClampedArray) -> i64 {
    values
        .as_slice()
        .iter()
        .map(|value| i64::from(value.0))
        .sum()
}

/// Return a two-byte DataView over the middle of a four-byte buffer.
#[ani]
pub fn make_data_view() -> Result<DataView> {
    DataView::new(ArrayBuffer::new(vec![1, 2, 3, 4]), 1, 2)
}

#[ani]
pub fn sum_data_view(view: DataView) -> i64 {
    view.as_slice().iter().map(|value| i64::from(*value)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_length() {
        let buf = ArrayBuffer::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(buf.len(), 5);
    }

    #[test]
    fn test_sum_bytes() {
        let buf = ArrayBuffer::new(vec![1u8, 2, 3, 4, 5]);
        let sum: i64 = buf.iter().map(|&b| b as i64).sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_process_buffer() {
        let buf = ArrayBuffer::new(vec![0u8; 10]);
        assert_eq!(process_buffer(buf), 10);
    }

    #[test]
    fn test_create_buffer() {
        let buf = create_buffer(8);
        assert_eq!(buf.len(), 8);
        assert!(buf.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_create_buffer_negative() {
        let buf = create_buffer(-1);
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn typed_array_and_mutation_logic_work() {
        let mut buffer = replace_first_byte(ArrayBuffer::new(vec![1, 2, 3]), 255);
        assert_eq!(buffer.as_mut_slice(), [255, 2, 3]);
        assert_eq!(sum_u16_array(make_u16_array()), 1 + 0x0102 + 65_535);
        assert_eq!(
            replace_first_u16(Uint16Array::new(vec![1, 2])).into_vec(),
            vec![99, 2]
        );
        assert_eq!(sum_clamped_array(make_clamped_array()), 383);
        assert_eq!(sum_data_view(make_data_view().unwrap()), 5);
    }
}
