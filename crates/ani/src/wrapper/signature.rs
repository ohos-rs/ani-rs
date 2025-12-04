use std::fmt;

use crate::errors::*;

/// A primitive type.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Primitive {
    Boolean, // Z
    Byte,    // B
    Char,    // C
    Double,  // D
    Float,   // F
    Int,     // I
    Long,    // J
    Short,   // S
    Void,    // V
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Primitive::Boolean => write!(f, "Z"),
            Primitive::Byte => write!(f, "B"),
            Primitive::Char => write!(f, "C"),
            Primitive::Double => write!(f, "D"),
            Primitive::Float => write!(f, "F"),
            Primitive::Int => write!(f, "I"),
            Primitive::Long => write!(f, "J"),
            Primitive::Short => write!(f, "S"),
            Primitive::Void => write!(f, "V"),
        }
    }
}

/// Enum representing any type in addition to method signatures.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum AniType {
    Primitive(Primitive),
    Object(String),
    Array(Box<AniType>),
    Method(Box<TypeSignature>),
}

impl fmt::Display for AniType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AniType::Primitive(ref ty) => ty.fmt(f),
            AniType::Object(ref name) => write!(f, "L{name};"),
            AniType::Array(ref ty) => write!(f, "[{ty}"),
            AniType::Method(ref m) => m.fmt(f),
        }
    }
}

/// Enum representing any type that may be used as a return value
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ReturnType {
    Primitive(Primitive),
    Object,
    Array,
}

/// A method type signature
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TypeSignature {
    /// The arguments of the method
    pub args: Vec<AniType>,
    /// The return type of the method
    pub ret: AniType,
}

impl TypeSignature {
    /// Create a new TypeSignature with the given arguments and return type
    pub fn new(args: Vec<AniType>, ret: AniType) -> Self {
        Self { args, ret }
    }
}

impl fmt::Display for TypeSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for arg in &self.args {
            write!(f, "{arg}")?;
        }
        write!(f, ")")?;
        self.ret.fmt(f)
    }
}

/// Parse a primitive type character
pub fn parse_primitive(c: char) -> Option<Primitive> {
    match c {
        'Z' => Some(Primitive::Boolean),
        'B' => Some(Primitive::Byte),
        'C' => Some(Primitive::Char),
        'D' => Some(Primitive::Double),
        'F' => Some(Primitive::Float),
        'I' => Some(Primitive::Int),
        'J' => Some(Primitive::Long),
        'S' => Some(Primitive::Short),
        'V' => Some(Primitive::Void),
        _ => None,
    }
}

/// Parse a type from a string (simple implementation)
pub fn parse_type(s: &str) -> Result<AniType> {
    if s.is_empty() {
        return Err(Error::ParseFailed("empty type string".to_string()));
    }
    
    let first = s.chars().next().unwrap();
    
    if let Some(p) = parse_primitive(first) {
        return Ok(AniType::Primitive(p));
    }
    
    match first {
        'L' => {
            // Object type: Lclass/name;
            if let Some(end) = s.find(';') {
                let class_name = &s[1..end];
                Ok(AniType::Object(class_name.to_string()))
            } else {
                Err(Error::ParseFailed(format!("missing semicolon in object type: {s}")))
            }
        }
        '[' => {
            // Array type
            let inner = parse_type(&s[1..])?;
            Ok(AniType::Array(Box::new(inner)))
        }
        _ => Err(Error::ParseFailed(format!("unknown type: {s}"))),
    }
}
