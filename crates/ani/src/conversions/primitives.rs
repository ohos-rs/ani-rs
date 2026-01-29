//! Primitive Type Conversion
//!
//! Implements conversion between Rust primitive types and ANI primitive types
//! - bool <-> ani_boolean
//! - i8 <-> ani_byte
//! - i16 <-> ani_short
//! - i32 <-> ani_int
//! - i64 <-> ani_long
//! - f32 <-> ani_float
//! - f64 <-> ani_double
//! - u8, u16, u32, u64 etc.

use crate::env::Env;
use crate::error::Result;
use crate::sys;

use super::traits::{FromAni, FromAniDirect, ToAni, ToAniDirect, TypeInfo};

// ============================================================================
// bool - Z
// ============================================================================

impl TypeInfo for bool {
    fn type_signature() -> &'static str {
        "Z"
    }
    fn ani_c_type() -> &'static str {
        "ani_boolean"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for bool {
    type Output = sys::ani_boolean;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(if self { 1 } else { 0 })
    }
}

impl ToAniDirect for bool {
    type Output = sys::ani_boolean;

    fn to_ani_direct(self) -> Self::Output {
        if self { 1 } else { 0 }
    }
}

impl<'env> FromAni<'env> for bool {
    type Input = sys::ani_boolean;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value != 0)
    }
}

impl FromAniDirect for bool {
    type Input = sys::ani_boolean;

    fn from_ani_direct(value: Self::Input) -> Self {
        value != 0
    }
}

// ============================================================================
// i8 (byte) - B
// ============================================================================

impl TypeInfo for i8 {
    fn type_signature() -> &'static str {
        "B"
    }
    fn ani_c_type() -> &'static str {
        "ani_byte"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for i8 {
    type Output = sys::ani_byte;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for i8 {
    type Output = sys::ani_byte;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for i8 {
    type Input = sys::ani_byte;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for i8 {
    type Input = sys::ani_byte;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// u8 - 作为 byte 处理
// ============================================================================

impl TypeInfo for u8 {
    fn type_signature() -> &'static str {
        "B"
    }
    fn ani_c_type() -> &'static str {
        "ani_byte"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for u8 {
    type Output = sys::ani_byte;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self as i8)
    }
}

impl ToAniDirect for u8 {
    type Output = sys::ani_byte;

    fn to_ani_direct(self) -> Self::Output {
        self as i8
    }
}

impl<'env> FromAni<'env> for u8 {
    type Input = sys::ani_byte;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value as u8)
    }
}

impl FromAniDirect for u8 {
    type Input = sys::ani_byte;

    fn from_ani_direct(value: Self::Input) -> Self {
        value as u8
    }
}

// ============================================================================
// i16 (short) - S
// ============================================================================

impl TypeInfo for i16 {
    fn type_signature() -> &'static str {
        "S"
    }
    fn ani_c_type() -> &'static str {
        "ani_short"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for i16 {
    type Output = sys::ani_short;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for i16 {
    type Output = sys::ani_short;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for i16 {
    type Input = sys::ani_short;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for i16 {
    type Input = sys::ani_short;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// u16 (char) - C
// ============================================================================

impl TypeInfo for u16 {
    fn type_signature() -> &'static str {
        "C"
    }
    fn ani_c_type() -> &'static str {
        "ani_char"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for u16 {
    type Output = sys::ani_char;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for u16 {
    type Output = sys::ani_char;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for u16 {
    type Input = sys::ani_char;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for u16 {
    type Input = sys::ani_char;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// char - 作为 u16 (ANI char) 处理
// ============================================================================

impl TypeInfo for char {
    fn type_signature() -> &'static str {
        "C"
    }
    fn ani_c_type() -> &'static str {
        "ani_char"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for char {
    type Output = sys::ani_char;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self as u16)
    }
}

impl ToAniDirect for char {
    type Output = sys::ani_char;

    fn to_ani_direct(self) -> Self::Output {
        self as u16
    }
}

impl<'env> FromAni<'env> for char {
    type Input = sys::ani_char;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(char::from_u32(value as u32).unwrap_or('\0'))
    }
}

// ============================================================================
// i32 (int) - I
// ============================================================================

impl TypeInfo for i32 {
    fn type_signature() -> &'static str {
        "I"
    }
    fn ani_c_type() -> &'static str {
        "ani_int"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for i32 {
    type Output = sys::ani_int;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for i32 {
    type Output = sys::ani_int;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for i32 {
    type Input = sys::ani_int;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for i32 {
    type Input = sys::ani_int;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// u32 - 作为 int 处理（需要注意溢出）
// ============================================================================

impl TypeInfo for u32 {
    fn type_signature() -> &'static str {
        "I"
    }
    fn ani_c_type() -> &'static str {
        "ani_int"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for u32 {
    type Output = sys::ani_int;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self as i32)
    }
}

impl ToAniDirect for u32 {
    type Output = sys::ani_int;

    fn to_ani_direct(self) -> Self::Output {
        self as i32
    }
}

impl<'env> FromAni<'env> for u32 {
    type Input = sys::ani_int;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value as u32)
    }
}

impl FromAniDirect for u32 {
    type Input = sys::ani_int;

    fn from_ani_direct(value: Self::Input) -> Self {
        value as u32
    }
}

// ============================================================================
// i64 (long) - J
// ============================================================================

impl TypeInfo for i64 {
    fn type_signature() -> &'static str {
        "J"
    }
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for i64 {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for i64 {
    type Output = sys::ani_long;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for i64 {
    type Input = sys::ani_long;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for i64 {
    type Input = sys::ani_long;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// u64 - 作为 long 处理
// ============================================================================

impl TypeInfo for u64 {
    fn type_signature() -> &'static str {
        "J"
    }
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for u64 {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self as i64)
    }
}

impl ToAniDirect for u64 {
    type Output = sys::ani_long;

    fn to_ani_direct(self) -> Self::Output {
        self as i64
    }
}

impl<'env> FromAni<'env> for u64 {
    type Input = sys::ani_long;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value as u64)
    }
}

impl FromAniDirect for u64 {
    type Input = sys::ani_long;

    fn from_ani_direct(value: Self::Input) -> Self {
        value as u64
    }
}

// ============================================================================
// isize / usize - 平台相关大小
// ============================================================================

impl TypeInfo for isize {
    fn type_signature() -> &'static str {
        "J"
    }
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for isize {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self as i64)
    }
}

impl ToAniDirect for isize {
    type Output = sys::ani_long;

    fn to_ani_direct(self) -> Self::Output {
        self as i64
    }
}

impl<'env> FromAni<'env> for isize {
    type Input = sys::ani_long;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value as isize)
    }
}

impl TypeInfo for usize {
    fn type_signature() -> &'static str {
        "J"
    }
    fn ani_c_type() -> &'static str {
        "ani_long"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for usize {
    type Output = sys::ani_long;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self as i64)
    }
}

impl ToAniDirect for usize {
    type Output = sys::ani_long;

    fn to_ani_direct(self) -> Self::Output {
        self as i64
    }
}

impl<'env> FromAni<'env> for usize {
    type Input = sys::ani_long;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value as usize)
    }
}

// ============================================================================
// f32 (float) - F
// ============================================================================

impl TypeInfo for f32 {
    fn type_signature() -> &'static str {
        "F"
    }
    fn ani_c_type() -> &'static str {
        "ani_float"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for f32 {
    type Output = sys::ani_float;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for f32 {
    type Output = sys::ani_float;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for f32 {
    type Input = sys::ani_float;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for f32 {
    type Input = sys::ani_float;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// f64 (double) - D
// ============================================================================

impl TypeInfo for f64 {
    fn type_signature() -> &'static str {
        "D"
    }
    fn ani_c_type() -> &'static str {
        "ani_double"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for f64 {
    type Output = sys::ani_double;

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl ToAniDirect for f64 {
    type Output = sys::ani_double;

    fn to_ani_direct(self) -> Self::Output {
        self
    }
}

impl<'env> FromAni<'env> for f64 {
    type Input = sys::ani_double;

    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl FromAniDirect for f64 {
    type Input = sys::ani_double;

    fn from_ani_direct(value: Self::Input) -> Self {
        value
    }
}

// ============================================================================
// () (void) - V
// ============================================================================

impl TypeInfo for () {
    fn type_signature() -> &'static str {
        "V"
    }
    fn ani_c_type() -> &'static str {
        "void"
    }
    fn is_primitive() -> bool {
        true
    }
}

impl<'env> ToAni<'env> for () {
    type Output = ();

    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_conversion() {
        assert_eq!(true.to_ani_direct(), 1);
        assert_eq!(false.to_ani_direct(), 0);
        assert!(bool::from_ani_direct(1));
        assert!(!bool::from_ani_direct(0));
    }

    #[test]
    fn test_integer_conversion() {
        assert_eq!(42i32.to_ani_direct(), 42);
        assert_eq!(i32::from_ani_direct(42), 42);

        assert_eq!(100i64.to_ani_direct(), 100);
        assert_eq!(i64::from_ani_direct(100), 100);
    }

    #[test]
    fn test_float_conversion() {
        assert!((3.14f32.to_ani_direct() - 3.14f32).abs() < f32::EPSILON);
        assert!((f64::from_ani_direct(2.718) - 2.718).abs() < f64::EPSILON);
    }

    #[test]
    fn test_type_signatures() {
        assert_eq!(bool::type_signature(), "Z");
        assert_eq!(i8::type_signature(), "B");
        assert_eq!(i16::type_signature(), "S");
        assert_eq!(i32::type_signature(), "I");
        assert_eq!(i64::type_signature(), "J");
        assert_eq!(f32::type_signature(), "F");
        assert_eq!(f64::type_signature(), "D");
    }
}
