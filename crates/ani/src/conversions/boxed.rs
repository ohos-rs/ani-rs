//! Boxing/Unboxing Support
//!
//! Implements boxing and unboxing for primitive types
//! Option<int> in ArkTS needs to use the Int wrapper class

use crate::env::Env;
use crate::error::Result;
use crate::types::*;

// ============================================================================
// Boxable Trait
// ============================================================================

/// Boxable trait - boxes a primitive type into an object
///
/// Used to convert Rust primitive types to ArkTS wrapper types
/// e.g., i32 -> std.core.Int
pub trait Boxable<'env> {
    /// Boxed type
    type Boxed;

    /// Box class descriptor
    fn box_class_descriptor() -> &'static str;

    /// Constructor signature
    fn box_constructor_signature() -> &'static str;

    /// Box the value
    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed>;
}

// ============================================================================
// Unboxable Trait
// ============================================================================

/// Unboxable trait - extracts a primitive type value from an object
pub trait Unboxable<'env>: Sized {
    /// Unbox method name
    fn unbox_method_name() -> &'static str {
        "unboxed"
    }

    /// Unbox method signature
    fn unbox_method_signature() -> &'static str;

    /// Unbox from a boxed object
    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self>;
}

// ============================================================================
// bool - Boolean
// ============================================================================

impl<'env> Boxable<'env> for bool {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Boolean;"
    }

    fn box_constructor_signature() -> &'static str {
        "z:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_boolean(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for bool {
    fn unbox_method_name() -> &'static str {
        "valueOf"
    }

    fn unbox_method_signature() -> &'static str {
        ":z"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_boolean(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// i8 - Byte
// ============================================================================

impl<'env> Boxable<'env> for i8 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Byte;"
    }

    fn box_constructor_signature() -> &'static str {
        "b:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_byte(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for i8 {
    fn unbox_method_name() -> &'static str {
        "toByte"
    }

    fn unbox_method_signature() -> &'static str {
        ":b"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_byte(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// i16 - Short
// ============================================================================

impl<'env> Boxable<'env> for i16 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Short;"
    }

    fn box_constructor_signature() -> &'static str {
        "s:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_short(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for i16 {
    fn unbox_method_name() -> &'static str {
        "toShort"
    }

    fn unbox_method_signature() -> &'static str {
        ":s"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_short(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// u16/char - Char
// ============================================================================

impl<'env> Boxable<'env> for u16 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Char;"
    }

    fn box_constructor_signature() -> &'static str {
        "c:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_char(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for u16 {
    fn unbox_method_name() -> &'static str {
        "toChar"
    }

    fn unbox_method_signature() -> &'static str {
        ":c"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_char(obj, Self::unbox_method_name(), None)
    }
}

impl<'env> Boxable<'env> for char {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Char;"
    }

    fn box_constructor_signature() -> &'static str {
        "c:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        (self as u16).box_value(env)
    }
}

// ============================================================================
// i32 - Int
// ============================================================================

impl<'env> Boxable<'env> for i32 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Int;"
    }

    fn box_constructor_signature() -> &'static str {
        "i:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_int(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for i32 {
    fn unbox_method_name() -> &'static str {
        "toInt"
    }

    fn unbox_method_signature() -> &'static str {
        ":i"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_int(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// i64 - Long
// ============================================================================

impl<'env> Boxable<'env> for i64 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Long;"
    }

    fn box_constructor_signature() -> &'static str {
        "l:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_long(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for i64 {
    fn unbox_method_name() -> &'static str {
        "toLong"
    }

    fn unbox_method_signature() -> &'static str {
        ":l"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_long(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// f32 - Float
// ============================================================================

impl<'env> Boxable<'env> for f32 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Float;"
    }

    fn box_constructor_signature() -> &'static str {
        "f:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_float(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for f32 {
    fn unbox_method_name() -> &'static str {
        "toFloat"
    }

    fn unbox_method_signature() -> &'static str {
        ":f"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_float(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// f64 - Double
// ============================================================================

impl<'env> Boxable<'env> for f64 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "Lstd/core/Double;"
    }

    fn box_constructor_signature() -> &'static str {
        "d:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        let class = env.find_class(Self::box_class_descriptor())?;
        let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
        let args = [ani_value_double(self)];
        env.new_object(&class, &ctor, &args[..])
    }
}

impl<'env> Unboxable<'env> for f64 {
    fn unbox_method_name() -> &'static str {
        "toDouble"
    }

    fn unbox_method_signature() -> &'static str {
        ":d"
    }

    fn unbox(env: &Env<'env>, obj: &AniObject<'env>) -> Result<Self> {
        env.call_method_by_name_double(obj, Self::unbox_method_name(), None)
    }
}

// ============================================================================
// Get Boxed Signature
// ============================================================================

/// Get the boxed signature for a primitive type
pub fn get_boxed_signature<T: Boxable<'static>>() -> &'static str {
    T::box_class_descriptor()
}

/// Mapping of primitive types to boxed type signatures
pub mod boxed_signatures {
    /// Boolean box class signature
    pub const BOOLEAN: &str = "Lstd/core/Boolean;";
    /// Byte box class signature
    pub const BYTE: &str = "Lstd/core/Byte;";
    /// Short box class signature
    pub const SHORT: &str = "Lstd/core/Short;";
    /// Char box class signature
    pub const CHAR: &str = "Lstd/core/Char;";
    /// Int box class signature
    pub const INT: &str = "Lstd/core/Int;";
    /// Long box class signature
    pub const LONG: &str = "Lstd/core/Long;";
    /// Float box class signature
    pub const FLOAT: &str = "Lstd/core/Float;";
    /// Double box class signature
    pub const DOUBLE: &str = "Lstd/core/Double;";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_descriptors() {
        assert_eq!(
            <bool as Boxable>::box_class_descriptor(),
            "Lstd/core/Boolean;"
        );
        assert_eq!(<i32 as Boxable>::box_class_descriptor(), "Lstd/core/Int;");
        assert_eq!(<i64 as Boxable>::box_class_descriptor(), "Lstd/core/Long;");
        assert_eq!(
            <f64 as Boxable>::box_class_descriptor(),
            "Lstd/core/Double;"
        );
    }

    #[test]
    fn test_boxed_signatures_module() {
        assert_eq!(boxed_signatures::INT, "Lstd/core/Int;");
        assert_eq!(boxed_signatures::LONG, "Lstd/core/Long;");
    }
}
