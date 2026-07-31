//! Boxing/Unboxing Support
//!
//! Implements boxing and unboxing for primitive types
//! `Option<int>` in ArkTS needs to use the Int wrapper class

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
        "std.core.Boolean"
    }

    fn box_constructor_signature() -> &'static str {
        "z:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_boolean(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_boolean(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_boolean(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_boolean(obj, "value")
        }
    }
}

// ============================================================================
// i8 - Byte
// ============================================================================

impl<'env> Boxable<'env> for i8 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Byte"
    }

    fn box_constructor_signature() -> &'static str {
        "b:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_byte(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_byte(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_byte(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_byte(obj, "value")
        }
    }
}

// ============================================================================
// i16 - Short
// ============================================================================

impl<'env> Boxable<'env> for i16 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Short"
    }

    fn box_constructor_signature() -> &'static str {
        "s:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_short(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_short(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_short(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_short(obj, "value")
        }
    }
}

// ============================================================================
// u16/char - Char
// ============================================================================

impl<'env> Boxable<'env> for u16 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Char"
    }

    fn box_constructor_signature() -> &'static str {
        "c:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_char(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_char(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_char(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_char(obj, "value")
        }
    }
}

impl<'env> Boxable<'env> for char {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Char"
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
        "std.core.Int"
    }

    fn box_constructor_signature() -> &'static str {
        "i:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_int(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_int(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_int(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_int(obj, "value")
        }
    }
}

// ============================================================================
// i64 - Long
// ============================================================================

impl<'env> Boxable<'env> for i64 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Long"
    }

    fn box_constructor_signature() -> &'static str {
        "l:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_long(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_long(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_long(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_long(obj, "value")
        }
    }
}

// ============================================================================
// f32 - Float
// ============================================================================

impl<'env> Boxable<'env> for f32 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Float"
    }

    fn box_constructor_signature() -> &'static str {
        "f:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_float(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_float(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_float(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_float(obj, "value")
        }
    }
}

// ============================================================================
// f64 - Double
// ============================================================================

impl<'env> Boxable<'env> for f64 {
    type Boxed = AniObject<'env>;

    fn box_class_descriptor() -> &'static str {
        "std.core.Double"
    }

    fn box_constructor_signature() -> &'static str {
        "d:"
    }

    fn box_value(self, env: &Env<'env>) -> Result<Self::Boxed> {
        #[cfg(feature = "api24")]
        {
            env.primitive_box_double(self)
        }

        #[cfg(not(feature = "api24"))]
        {
            let class = env.find_class(Self::box_class_descriptor())?;
            let ctor = env.find_constructor(&class, Self::box_constructor_signature())?;
            let args = [ani_value_double(self)];
            env.new_object(&class, &ctor, &args[..])
        }
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
        #[cfg(feature = "api24")]
        {
            env.primitive_unbox_double(obj)
        }

        #[cfg(not(feature = "api24"))]
        {
            env.get_field_by_name_double(obj, "value")
        }
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
    pub const BOOLEAN: &str = "std.core.Boolean";
    /// Byte box class signature
    pub const BYTE: &str = "std.core.Byte";
    /// Short box class signature
    pub const SHORT: &str = "std.core.Short";
    /// Char box class signature
    pub const CHAR: &str = "std.core.Char";
    /// Int box class signature
    pub const INT: &str = "std.core.Int";
    /// Long box class signature
    pub const LONG: &str = "std.core.Long";
    /// Float box class signature
    pub const FLOAT: &str = "std.core.Float";
    /// Double box class signature
    pub const DOUBLE: &str = "std.core.Double";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_descriptors() {
        assert_eq!(
            <bool as Boxable>::box_class_descriptor(),
            "std.core.Boolean"
        );
        assert_eq!(<i32 as Boxable>::box_class_descriptor(), "std.core.Int");
        assert_eq!(<i64 as Boxable>::box_class_descriptor(), "std.core.Long");
        assert_eq!(<f64 as Boxable>::box_class_descriptor(), "std.core.Double");
    }

    #[test]
    fn test_boxed_signatures_module() {
        assert_eq!(boxed_signatures::INT, "std.core.Int");
        assert_eq!(boxed_signatures::LONG, "std.core.Long");
    }
}
