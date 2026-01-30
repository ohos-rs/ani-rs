//! ArrayBuffer Example
//!
//! Demonstrates Rust ↔ ANI ArrayBuffer conversion using ani-rs.
//!
//! - **ArrayBufferSlice&lt;'env&gt;** — zero-copy borrowed view from ANI; use as argument when you
//!   only read in the current scope.
//! - **ArrayBuffer** — owned buffer; use when you need to own data or return a buffer to ANI
//!   (ToAni copies data into a new ANI ArrayBuffer).

use ani::prelude::{ArrayBuffer, ArrayBufferSlice};
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
}
