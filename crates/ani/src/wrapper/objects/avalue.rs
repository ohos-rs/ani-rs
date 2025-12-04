use std::char::{CharTryFromError, DecodeUtf16Error};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

use log::trace;

use crate::{errors::*, objects::AObject, signature::Primitive, sys::*};

#[cfg(doc)]
use crate::ANIEnv;

/// A owned local reference or primitive value.
///
/// This type is used for values returned from method calls. If the
/// method returns an object reference, it will take the form of an owned
/// [`AObject`].
///
/// See also [`AValue`], which is used for method call parameters. It is
/// different from this type in that it *borrows* an object reference instead
/// of owning one.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum AValueOwned<'local> {
    Object(AObject<'local>),
    Byte(abyte),
    Char(achar),
    Short(ashort),
    Int(aint),
    Long(along),
    Bool(aboolean),
    Float(afloat),
    Double(adouble),
    Void,
}

/// A borrowed local reference or primitive value.
///
/// This type is used for parameters passed to method calls. If the
/// method is to be passed an object reference, it takes the form of a borrowed
/// <code>&[AObject]</code>.
///
/// See also [`AValueOwned`], which is used for method return values. It is
/// different from this type in that it *owns* an object reference instead
/// of borrowing one.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum AValue<'obj_ref> {
    Object(&'obj_ref AObject<'obj_ref>),
    Byte(abyte),
    Char(achar),
    Short(ashort),
    Int(aint),
    Long(along),
    Bool(aboolean),
    Float(afloat),
    Double(adouble),
    Void,
}

impl<'local> AValueOwned<'local> {
    /// Convert the enum to its ani-compatible equivalent.
    pub fn as_ani(&self) -> avalue {
        self.borrow().as_ani()
    }

    /// Get the type name for the enum variant.
    pub fn type_name(&self) -> &'static str {
        self.borrow().type_name()
    }

    /// Get the primitive type for the enum variant. If it's not a primitive
    /// (i.e. an Object), returns None.
    pub fn primitive_type(&self) -> Option<Primitive> {
        self.borrow().primitive_type()
    }

    /// Try to unwrap to an Object.
    pub fn l(self) -> Result<AObject<'local>> {
        match self {
            Self::Object(obj) => Ok(obj),
            _ => Err(Error::WrongAValueType("object", self.type_name())),
        }
    }

    /// Try to unwrap to a boolean.
    pub fn z(self) -> Result<bool> {
        self.borrow().z()
    }

    /// Try to unwrap to a byte.
    pub fn b(self) -> Result<abyte> {
        self.borrow().b()
    }

    /// Try to unwrap to a char.
    pub fn c(self) -> Result<achar> {
        self.borrow().c()
    }

    /// Try to unwrap a `char` and then convert it to a Rust `char`.
    pub fn c_char(self) -> Result<char> {
        let char = self.c()?;
        char_from_ani(char).map_err(|source| Error::InvalidUtf16 { source })
    }

    /// Try to unwrap to a double.
    pub fn d(self) -> Result<adouble> {
        self.borrow().d()
    }

    /// Try to unwrap to a float.
    pub fn f(self) -> Result<afloat> {
        self.borrow().f()
    }

    /// Try to unwrap to an int.
    pub fn i(self) -> Result<aint> {
        self.borrow().i()
    }

    /// Try to unwrap a Rust `char` from an `int`. See [`char_from_ani_int`] for details.
    pub fn i_char(self) -> Result<char> {
        let char = self.i()?;
        char_from_ani_int(char).map_err(|source| Error::InvalidUtf32 { char, source })
    }

    /// Try to unwrap to a long.
    pub fn j(self) -> Result<along> {
        self.borrow().j()
    }

    /// Try to unwrap to a short.
    pub fn s(self) -> Result<ashort> {
        self.borrow().s()
    }

    /// Try to unwrap to a void.
    pub fn v(self) -> Result<()> {
        self.borrow().v()
    }

    /// Copies or borrows the value in this `AValueOwned`.
    pub fn borrow(&'_ self) -> AValue<'_> {
        match self {
            Self::Object(o) => AValue::Object(o),
            Self::Byte(v) => AValue::Byte(*v),
            Self::Char(v) => AValue::Char(*v),
            Self::Short(v) => AValue::Short(*v),
            Self::Int(v) => AValue::Int(*v),
            Self::Long(v) => AValue::Long(*v),
            Self::Bool(v) => AValue::Bool(*v),
            Self::Float(v) => AValue::Float(*v),
            Self::Double(v) => AValue::Double(*v),
            Self::Void => AValue::Void,
        }
    }
}

impl<'obj_ref> AValue<'obj_ref> {
    /// Convert the enum to its ani-compatible equivalent.
    pub fn as_ani(&self) -> avalue {
        let val: avalue = match self {
            Self::Object(obj) => avalue { l: obj.as_raw() },
            Self::Byte(byte) => avalue { b: *byte },
            Self::Char(char) => avalue { c: *char },
            Self::Short(short) => avalue { s: *short },
            Self::Int(int) => avalue { i: *int },
            Self::Long(long) => avalue { j: *long },
            Self::Bool(boolean) => avalue { b: *boolean as i8 },
            Self::Float(float) => avalue { f: *float },
            Self::Double(double) => avalue { d: *double },
            Self::Void => avalue {
                l: ::std::ptr::null_mut(),
            },
        };
        trace!("converted {:?} to avalue {:?}", self, unsafe {
            ::std::mem::transmute::<avalue, u64>(val)
        });
        val
    }

    /// Convert the enum to its ani-compatible equivalent.
    #[deprecated = "Use `as_ani` instead."]
    pub fn to_ani(self) -> avalue {
        self.as_ani()
    }

    /// Get the type name for the enum variant.
    pub fn type_name(&self) -> &'static str {
        match *self {
            Self::Void => "void",
            Self::Object(_) => "object",
            Self::Byte(_) => "byte",
            Self::Char(_) => "char",
            Self::Short(_) => "short",
            Self::Int(_) => "int",
            Self::Long(_) => "long",
            Self::Bool(_) => "bool",
            Self::Float(_) => "float",
            Self::Double(_) => "double",
        }
    }

    /// Get the primitive type for the enum variant. If it's not a primitive
    /// (i.e. an Object), returns None.
    pub fn primitive_type(&self) -> Option<Primitive> {
        Some(match *self {
            Self::Object(_) => return None,
            Self::Void => Primitive::Void,
            Self::Byte(_) => Primitive::Byte,
            Self::Char(_) => Primitive::Char,
            Self::Short(_) => Primitive::Short,
            Self::Int(_) => Primitive::Int,
            Self::Long(_) => Primitive::Long,
            Self::Bool(_) => Primitive::Boolean,
            Self::Float(_) => Primitive::Float,
            Self::Double(_) => Primitive::Double,
        })
    }

    /// Try to unwrap to an Object.
    pub fn l(self) -> Result<&'obj_ref AObject<'obj_ref>> {
        match self {
            Self::Object(obj) => Ok(obj),
            _ => Err(Error::WrongAValueType("object", self.type_name())),
        }
    }

    /// Try to unwrap to a boolean.
    pub fn z(self) -> Result<bool> {
        match self {
            Self::Bool(b) => Ok(b == ANI_TRUE_VAL),
            _ => Err(Error::WrongAValueType("bool", self.type_name())),
        }
    }

    /// Try to unwrap to a byte.
    pub fn b(self) -> Result<abyte> {
        match self {
            Self::Byte(b) => Ok(b),
            _ => Err(Error::WrongAValueType("abyte", self.type_name())),
        }
    }

    /// Try to unwrap to a char.
    pub fn c(self) -> Result<achar> {
        match self {
            Self::Char(b) => Ok(b),
            _ => Err(Error::WrongAValueType("achar", self.type_name())),
        }
    }

    /// Try to unwrap a `char` and then convert it to a Rust `char`.
    pub fn c_char(self) -> Result<char> {
        let char = self.c()?;
        char_from_ani(char).map_err(|source| Error::InvalidUtf16 { source })
    }

    /// Try to unwrap to a double.
    pub fn d(self) -> Result<adouble> {
        match self {
            Self::Double(b) => Ok(b),
            _ => Err(Error::WrongAValueType("adouble", self.type_name())),
        }
    }

    /// Try to unwrap to a float.
    pub fn f(self) -> Result<afloat> {
        match self {
            Self::Float(b) => Ok(b),
            _ => Err(Error::WrongAValueType("afloat", self.type_name())),
        }
    }

    /// Try to unwrap to an int.
    pub fn i(self) -> Result<aint> {
        match self {
            Self::Int(b) => Ok(b),
            _ => Err(Error::WrongAValueType("aint", self.type_name())),
        }
    }

    /// Try to unwrap a Rust `char` from an `int`. See [`char_from_ani_int`] for details.
    pub fn i_char(self) -> Result<char> {
        let char = self.i()?;
        char_from_ani_int(char).map_err(|source| Error::InvalidUtf32 { char, source })
    }

    /// Try to unwrap to a long.
    pub fn j(self) -> Result<along> {
        match self {
            Self::Long(b) => Ok(b),
            _ => Err(Error::WrongAValueType("along", self.type_name())),
        }
    }

    /// Try to unwrap to a short.
    pub fn s(self) -> Result<ashort> {
        match self {
            Self::Short(b) => Ok(b),
            _ => Err(Error::WrongAValueType("ashort", self.type_name())),
        }
    }

    /// Try to unwrap to a void.
    pub fn v(self) -> Result<()> {
        match self {
            Self::Void => Ok(()),
            _ => Err(Error::WrongAValueType("void", self.type_name())),
        }
    }

    /// Converts a Rust `char` to an `int`. See [`char_to_ani_int`] for details.
    pub fn int_from_char(char: char) -> Self {
        Self::Int(char_to_ani_int(char))
    }
}

impl<'obj_ref> From<&'obj_ref AValueOwned<'obj_ref>> for AValue<'obj_ref> {
    fn from(other: &'obj_ref AValueOwned) -> Self {
        other.borrow()
    }
}

impl<'local, T: Into<AObject<'local>>> From<T> for AValueOwned<'local> {
    fn from(other: T) -> Self {
        Self::Object(other.into())
    }
}

impl<'obj_ref, T: AsRef<AObject<'obj_ref>>> From<&'obj_ref T> for AValue<'obj_ref> {
    fn from(other: &'obj_ref T) -> Self {
        Self::Object(other.as_ref())
    }
}

impl<'local> TryFrom<AValueOwned<'local>> for AObject<'local> {
    type Error = Error;

    fn try_from(value: AValueOwned<'local>) -> Result<Self> {
        value.l()
    }
}

impl From<aboolean> for AValueOwned<'_> {
    fn from(other: aboolean) -> Self {
        Self::Bool(other)
    }
}

impl TryFrom<AValueOwned<'_>> for aboolean {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.borrow().try_into()
    }
}

impl From<aboolean> for AValue<'_> {
    fn from(other: aboolean) -> Self {
        Self::Bool(other)
    }
}

impl TryFrom<AValue<'_>> for aboolean {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        match value {
            AValue::Bool(b) => Ok(b),
            _ => Err(Error::WrongAValueType("bool", value.type_name())),
        }
    }
}

// achar
impl From<achar> for AValueOwned<'_> {
    fn from(other: achar) -> Self {
        Self::Char(other)
    }
}

impl TryFrom<AValueOwned<'_>> for achar {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.c()
    }
}

impl From<achar> for AValue<'_> {
    fn from(other: achar) -> Self {
        Self::Char(other)
    }
}

impl TryFrom<AValue<'_>> for achar {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.c()
    }
}

/// Converts a Rust `char` to a `char`, if possible.
impl TryFrom<char> for AValueOwned<'_> {
    type Error = CharToAniError;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(Self::Char(char_to_ani(value)?))
    }
}

/// Converts a Rust `char` to a `char`, if possible.
impl TryFrom<char> for AValue<'_> {
    type Error = CharToAniError;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(Self::Char(char_to_ani(value)?))
    }
}

/// Converts an ANI `char` to a Rust `char`, if possible.
pub fn char_from_ani(char: achar) -> std::result::Result<char, DecodeUtf16Error> {
    char::decode_utf16([char]).next().unwrap()
}

/// Converts a Rust `char` to an ANI `char`, if possible.
pub fn char_to_ani(char: char) -> std::result::Result<achar, CharToAniError> {
    if char.len_utf16() != 1 {
        return Err(CharToAniError { char });
    }

    let mut buf = [0u16; 1];
    let buf: &mut [u16] = char.encode_utf16(&mut buf);
    Ok(buf[0])
}

// ashort
impl From<ashort> for AValueOwned<'_> {
    fn from(other: ashort) -> Self {
        Self::Short(other)
    }
}

impl TryFrom<AValueOwned<'_>> for ashort {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.s()
    }
}

impl From<ashort> for AValue<'_> {
    fn from(other: ashort) -> Self {
        Self::Short(other)
    }
}

impl TryFrom<AValue<'_>> for ashort {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.s()
    }
}

// afloat
impl From<afloat> for AValueOwned<'_> {
    fn from(other: afloat) -> Self {
        Self::Float(other)
    }
}

impl TryFrom<AValueOwned<'_>> for afloat {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.f()
    }
}

impl From<afloat> for AValue<'_> {
    fn from(other: afloat) -> Self {
        Self::Float(other)
    }
}

impl TryFrom<AValue<'_>> for afloat {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.f()
    }
}

// adouble
impl From<adouble> for AValueOwned<'_> {
    fn from(other: adouble) -> Self {
        Self::Double(other)
    }
}

impl TryFrom<AValueOwned<'_>> for adouble {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.d()
    }
}

impl From<adouble> for AValue<'_> {
    fn from(other: adouble) -> Self {
        Self::Double(other)
    }
}

impl TryFrom<AValue<'_>> for adouble {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.d()
    }
}

// aint
impl From<aint> for AValueOwned<'_> {
    fn from(other: aint) -> Self {
        Self::Int(other)
    }
}

impl TryFrom<AValueOwned<'_>> for aint {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.i()
    }
}

impl From<aint> for AValue<'_> {
    fn from(other: aint) -> Self {
        Self::Int(other)
    }
}

impl TryFrom<AValue<'_>> for aint {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.i()
    }
}

/// Converts a Rust `char` to an `int`.
pub fn char_to_ani_int(char: char) -> aint {
    u32::from(char) as aint
}

/// Converts an `int` to a Rust `char`.
pub fn char_from_ani_int(val: aint) -> std::result::Result<char, CharTryFromError> {
    char::try_from(val as u32)
}

// along
impl From<along> for AValueOwned<'_> {
    fn from(other: along) -> Self {
        Self::Long(other)
    }
}

impl TryFrom<AValueOwned<'_>> for along {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.j()
    }
}

impl From<along> for AValue<'_> {
    fn from(other: along) -> Self {
        Self::Long(other)
    }
}

impl TryFrom<AValue<'_>> for along {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.j()
    }
}

// abyte
impl From<abyte> for AValueOwned<'_> {
    fn from(other: abyte) -> Self {
        Self::Byte(other)
    }
}

impl TryFrom<AValueOwned<'_>> for abyte {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.b()
    }
}

impl From<abyte> for AValue<'_> {
    fn from(other: abyte) -> Self {
        Self::Byte(other)
    }
}

impl TryFrom<AValue<'_>> for abyte {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.b()
    }
}

// void
impl From<()> for AValueOwned<'_> {
    fn from(_: ()) -> Self {
        Self::Void
    }
}

impl TryFrom<AValueOwned<'_>> for () {
    type Error = Error;

    fn try_from(value: AValueOwned) -> Result<Self> {
        value.v()
    }
}

impl From<()> for AValue<'_> {
    fn from(_: ()) -> Self {
        Self::Void
    }
}

impl TryFrom<AValue<'_>> for () {
    type Error = Error;

    fn try_from(value: AValue) -> Result<Self> {
        value.v()
    }
}

