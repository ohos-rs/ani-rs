//! ANI Type System
//!
//! Structured type representation for ANI FFI code generation.
//! This module provides a type-safe way to handle Rust-to-ANI type conversions
//! instead of string-based pattern matching.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, PathArguments, Type, TypePath};

// ============================================================================
// Core Type Definitions
// ============================================================================

/// Primitive types that map directly to ANI C types
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    Char,
}

/// String-like types
#[derive(Debug, Clone, PartialEq)]
pub enum StringType {
    String,
    Str,
}

/// Generic wrapper types with inner type
#[derive(Debug, Clone)]
pub enum WrapperType {
    /// Option<T>
    Option(Box<AniType>),
    /// Vec<T>
    Vec(Box<AniType>),
    /// Result<T, E> - we only care about the Ok type
    Result(Box<AniType>),
    /// Ref<T> - typed global reference
    Ref(Box<AniType>),
}

/// Function-related types
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FunctionType {
    /// Function<Args, Return>
    Function { args: Box<Type>, ret: Box<Type> },
    /// FunctionRef<Args, Return>
    FunctionRef { args: Box<Type>, ret: Box<Type> },
}

/// FnArgs wrapper type for multiple function arguments
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FnArgsType {
    /// The inner tuple type
    pub inner: Box<Type>,
    /// The parsed element types (if inner is a tuple)
    pub elements: Vec<AniType>,
}

/// Either type variants (union types)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EitherType {
    /// Number of variants (2 for Either, 3 for Either3, etc.)
    pub variant_count: usize,
    /// The inner types
    pub types: Vec<Box<Type>>,
}

/// Promise type
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PromiseType {
    /// The inner type for the promise value
    pub inner: Option<Box<AniType>>,
}

/// Record type (`Record<string, V>`)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RecordType {
    /// Value type `V`
    pub value: Box<AniType>,
}

/// The main ANI type enum
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AniType {
    /// Primitive types (bool, i32, f64, etc.)
    Primitive(PrimitiveType),
    /// String types (String, &str)
    String(StringType),
    /// Unit type ()
    Unit,
    /// Null literal type
    Null,
    /// Undefined literal type
    Undefined,
    /// Wrapper types (Option, Vec, Result, Ref)
    Wrapper(WrapperType),
    /// Function types
    Function(FunctionType),
    /// FnArgs wrapper for function arguments
    FnArgs(FnArgsType),
    /// Either types (union)
    Either(EitherType),
    /// Promise type
    Promise(PromiseType),
    /// Record type
    Record(RecordType),
    /// AniObject - raw object type
    AniObject,
    /// ArrayBuffer - binary data buffer type
    ArrayBuffer,
    /// Tuple type (for function arguments)
    Tuple(Vec<AniType>),
    /// Unknown/custom type - fallback to object
    Unknown(Box<Type>),
}

static OBJECT_TYPE_ALIASES: OnceLock<Mutex<BTreeMap<String, String>>> = OnceLock::new();

fn object_type_aliases() -> &'static Mutex<BTreeMap<String, String>> {
    OBJECT_TYPE_ALIASES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn normalize_object_alias_key(name: &str) -> String {
    name.split("::")
        .flat_map(|segment| segment.split("."))
        .filter(|segment| !segment.is_empty() && !matches!(*segment, "crate" | "self" | "super"))
        .collect::<Vec<_>>()
        .join(".")
}

pub fn register_object_type_alias(rust_name: &str, arkts_name: &str) {
    let rust_name = normalize_object_alias_key(rust_name);
    let arkts_name = arkts_name.trim();
    if rust_name.is_empty() || arkts_name.is_empty() {
        return;
    }

    object_type_aliases()
        .lock()
        .expect("failed to lock object type aliases")
        .insert(rust_name, arkts_name.to_string());
}

pub fn resolve_object_type_alias(name: &str) -> Option<String> {
    let key = normalize_object_alias_key(name);
    if key.is_empty() {
        return None;
    }

    object_type_aliases()
        .lock()
        .expect("failed to lock object type aliases")
        .get(&key)
        .cloned()
}

// ============================================================================
// Type Parsing
// ============================================================================

impl AniType {
    /// Parse a syn::Type into an AniType
    pub fn from_syn_type(ty: &Type) -> Self {
        match ty {
            Type::Path(type_path) => Self::parse_type_path(type_path),
            Type::Reference(type_ref) => {
                // Handle &str
                if let Type::Path(inner_path) = type_ref.elem.as_ref() {
                    if is_path_ident(inner_path, "str") {
                        return AniType::String(StringType::Str);
                    }
                }
                AniType::Unknown(Box::new(ty.clone()))
            }
            Type::Tuple(tuple) => {
                if tuple.elems.is_empty() {
                    AniType::Unit
                } else {
                    // Parse each element of the tuple
                    let elements: Vec<AniType> = tuple
                        .elems
                        .iter()
                        .map(|elem| AniType::from_syn_type(elem))
                        .collect();
                    AniType::Tuple(elements)
                }
            }
            _ => AniType::Unknown(Box::new(ty.clone())),
        }
    }

    /// Parse a type path into AniType
    fn parse_type_path(type_path: &TypePath) -> Self {
        let segment = match type_path.path.segments.last() {
            Some(seg) => seg,
            None => return AniType::Unknown(Box::new(Type::Path(type_path.clone()))),
        };

        let ident = segment.ident.to_string();

        // Check for primitive types first
        if let Some(primitive) = parse_primitive(&ident) {
            return AniType::Primitive(primitive);
        }

        // Check for string types
        if ident == "String" {
            return AniType::String(StringType::String);
        }

        // Check for unit type represented as path
        if ident == "()" {
            return AniType::Unit;
        }

        if ident == "Null" {
            return AniType::Null;
        }

        if ident == "Undefined" {
            return AniType::Undefined;
        }

        // Check for AniObject
        if ident == "AniObject" {
            return AniType::AniObject;
        }

        // Check for ArrayBuffer types
        if ident == "ArrayBuffer" || ident == "ArrayBufferSlice" {
            return AniType::ArrayBuffer;
        }

        // Check for Either types
        if let Some(either) = parse_either_type(&ident, &segment.arguments) {
            return AniType::Either(either);
        }

        // Check for Promise types
        if ident == "PromiseRaw" {
            return AniType::Promise(PromiseType {
                inner: extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type(&t))),
            });
        }

        // HashMap<String, V> maps to ArkTS Record<string, V>
        if ident == "HashMap" {
            if let Some(record) = parse_record_type(&segment.arguments) {
                return AniType::Record(record);
            }
        }

        // Check for generic wrapper types
        match ident.as_str() {
            "Option" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type(&t)))
                    .unwrap_or_else(|| {
                        Box::new(AniType::Unknown(Box::new(Type::Path(type_path.clone()))))
                    });
                AniType::Wrapper(WrapperType::Option(inner))
            }
            "Vec" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type(&t)))
                    .unwrap_or_else(|| {
                        Box::new(AniType::Unknown(Box::new(Type::Path(type_path.clone()))))
                    });
                AniType::Wrapper(WrapperType::Vec(inner))
            }
            "Result" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type(&t)))
                    .unwrap_or_else(|| Box::new(AniType::Unit));
                AniType::Wrapper(WrapperType::Result(inner))
            }
            "Ref" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type(&t)))
                    .unwrap_or_else(|| Box::new(AniType::AniObject));
                AniType::Wrapper(WrapperType::Ref(inner))
            }
            "Function" => {
                let (args, ret) = extract_function_generics(&segment.arguments);
                AniType::Function(FunctionType::Function { args, ret })
            }
            "FunctionRef" => {
                let (args, ret) = extract_function_generics(&segment.arguments);
                AniType::Function(FunctionType::FunctionRef { args, ret })
            }
            "FnArgs" => {
                // FnArgs<T> where T is typically a tuple
                let inner = extract_first_generic_type(&segment.arguments);
                let elements = inner
                    .as_ref()
                    .map(|t| {
                        let parsed = AniType::from_syn_type(t);
                        match parsed {
                            AniType::Tuple(elems) => elems,
                            AniType::Unit => vec![],
                            other => vec![other],
                        }
                    })
                    .unwrap_or_default();

                AniType::FnArgs(FnArgsType {
                    inner: inner
                        .map(Box::new)
                        .unwrap_or_else(|| Box::new(syn::parse_quote!(()))),
                    elements,
                })
            }
            _ => AniType::Unknown(Box::new(Type::Path(type_path.clone()))),
        }
    }

    /// Check if this type is an Either variant
    #[allow(dead_code)]
    pub fn is_either(&self) -> bool {
        matches!(self, AniType::Either(_))
    }

    /// Check if this type is a primitive
    #[allow(dead_code)]
    pub fn is_primitive(&self) -> bool {
        matches!(self, AniType::Primitive(_))
    }

    /// Check if this type is a Result wrapping a Promise
    #[allow(dead_code)]
    pub fn is_result_promise(&self) -> bool {
        if let AniType::Wrapper(WrapperType::Result(inner)) = self {
            matches!(inner.as_ref(), AniType::Promise(_))
        } else {
            false
        }
    }

    /// Get the inner type for Result<T, E>
    #[allow(dead_code)]
    pub fn result_ok_type(&self) -> Option<&AniType> {
        if let AniType::Wrapper(WrapperType::Result(inner)) = self {
            Some(inner.as_ref())
        } else {
            None
        }
    }

    /// Get the inner type for Option<T>
    #[allow(dead_code)]
    pub fn option_inner_type(&self) -> Option<&AniType> {
        if let AniType::Wrapper(WrapperType::Option(inner)) = self {
            Some(inner.as_ref())
        } else {
            None
        }
    }

    /// Check if this type is a FnArgs type
    #[allow(dead_code)]
    pub fn is_fn_args(&self) -> bool {
        matches!(self, AniType::FnArgs(_))
    }

    /// Get the FnArgs element types
    #[allow(dead_code)]
    pub fn fn_args_elements(&self) -> Option<&[AniType]> {
        if let AniType::FnArgs(fn_args) = self {
            Some(&fn_args.elements)
        } else {
            None
        }
    }

    /// Check if this type is a Tuple type
    #[allow(dead_code)]
    pub fn is_tuple(&self) -> bool {
        matches!(self, AniType::Tuple(_))
    }

    /// Get the Tuple element types
    #[allow(dead_code)]
    pub fn tuple_elements(&self) -> Option<&[AniType]> {
        if let AniType::Tuple(elements) = self {
            Some(elements)
        } else {
            None
        }
    }
}

// ============================================================================
// ANI C Type Generation
// ============================================================================

impl AniType {
    /// Generate the ANI C type for this Rust type
    pub fn to_ani_c_type(&self) -> TokenStream {
        match self {
            AniType::Primitive(p) => p.to_ani_c_type(),
            AniType::String(_) => quote! { ani::sys::ani_string },
            AniType::Unit => quote! { () },
            AniType::Null | AniType::Undefined => quote! { ani::sys::ani_object },
            AniType::Wrapper(w) => w.to_ani_c_type(),
            AniType::Function(_) => quote! { ani::sys::ani_fn_object },
            AniType::FnArgs(_) => quote! { ani::sys::ani_object },
            AniType::Either(_) => quote! { ani::sys::ani_object },
            AniType::Promise(_) => quote! { ani::sys::ani_object },
            AniType::Record(_) => quote! { ani::sys::ani_object },
            AniType::AniObject => quote! { ani::sys::ani_object },
            AniType::ArrayBuffer => quote! { ani::sys::ani_arraybuffer },
            AniType::Tuple(_) => quote! { ani::sys::ani_object },
            AniType::Unknown(_) => quote! { ani::sys::ani_object },
        }
    }
}

impl PrimitiveType {
    /// Generate the ANI C type for this primitive
    pub fn to_ani_c_type(&self) -> TokenStream {
        match self {
            PrimitiveType::Bool => quote! { ani::sys::ani_boolean },
            PrimitiveType::I8 | PrimitiveType::U8 => quote! { ani::sys::ani_byte },
            PrimitiveType::I16 => quote! { ani::sys::ani_short },
            PrimitiveType::U16 | PrimitiveType::Char => quote! { ani::sys::ani_char },
            PrimitiveType::I32 | PrimitiveType::U32 => quote! { ani::sys::ani_int },
            PrimitiveType::I64 | PrimitiveType::U64 => quote! { ani::sys::ani_long },
            PrimitiveType::F32 => quote! { ani::sys::ani_float },
            PrimitiveType::F64 => quote! { ani::sys::ani_double },
        }
    }
}

impl WrapperType {
    /// Generate the ANI C type for this wrapper
    pub fn to_ani_c_type(&self) -> TokenStream {
        match self {
            WrapperType::Vec(_) => quote! { ani::sys::ani_array },
            WrapperType::Option(_) => {
                // Nullable values are union objects at the ANI ABI boundary.
                quote! { ani::sys::ani_object }
            }
            WrapperType::Result(inner) => inner.to_ani_c_type(),
            WrapperType::Ref(_) => quote! { ani::sys::ani_object },
        }
    }
}

// ============================================================================
// ANI Signature Generation
// ============================================================================

impl AniType {
    /// Generate the ANI type signature string
    pub fn to_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => p.to_signature(),
            AniType::String(_) => "Lstd/core/String;".to_string(),
            AniType::Unit => "V".to_string(),
            AniType::Null => "C{std.core.Null}".to_string(),
            AniType::Undefined => "U".to_string(),
            AniType::Wrapper(w) => w.to_signature(),
            AniType::Function(_) => "Lstd/core/Function;".to_string(),
            AniType::FnArgs(fn_args) => {
                // FnArgs<(A, B, ...)> generates signature for each element
                fn_args
                    .elements
                    .iter()
                    .map(|e| e.to_signature())
                    .collect::<Vec<_>>()
                    .join("")
            }
            AniType::Either(either) => {
                let mut variants = String::new();
                for ty in &either.types {
                    let variant = AniType::from_syn_type(ty.as_ref()).to_union_variant_signature();
                    variants.push_str(&variant);
                }
                if variants.is_empty() {
                    "C{std.core.Object}".to_string()
                } else {
                    format!("X{{{variants}}}")
                }
            }
            AniType::Promise(_) => "Lstd/core/Object;".to_string(),
            AniType::Record(_) => "Lstd/core/Record;".to_string(),
            AniType::AniObject => "Lstd/core/Object;".to_string(),
            AniType::ArrayBuffer => "Lstd/core/ArrayBuffer;".to_string(),
            AniType::Tuple(elements) => {
                // Tuple generates signature for each element
                elements
                    .iter()
                    .map(|e| e.to_signature())
                    .collect::<Vec<_>>()
                    .join("")
            }
            AniType::Unknown(ty) => {
                unknown_type_to_signature(ty).unwrap_or_else(|| "Lstd/core/Object;".to_string())
            }
        }
    }

    /// Generate the boxed type signature (for Option inner types)
    pub fn to_boxed_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => p.to_boxed_signature(),
            _ => self.to_signature(),
        }
    }

    fn to_union_variant_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => p.to_boxed_new_signature().to_string(),
            AniType::String(_) => "C{std.core.String}".to_string(),
            AniType::Null => "C{std.core.Null}".to_string(),
            AniType::Undefined => "U".to_string(),
            AniType::AniObject => "C{std.core.Object}".to_string(),
            AniType::ArrayBuffer => "C{std.core.ArrayBuffer}".to_string(),
            AniType::Record(_) => "C{std.core.Record}".to_string(),
            AniType::Function(_) => "C{std.core.Function}".to_string(),
            AniType::Wrapper(WrapperType::Vec(inner)) => {
                format!("A{{{}}}", inner.to_fixed_array_elem_signature())
            }
            AniType::Wrapper(WrapperType::Option(inner)) => inner.to_union_variant_signature(),
            AniType::Wrapper(WrapperType::Result(inner)) => inner.to_union_variant_signature(),
            AniType::Wrapper(WrapperType::Ref(inner)) => inner.to_union_variant_signature(),
            AniType::Either(either) => {
                let mut variants = String::new();
                for ty in &either.types {
                    variants.push_str(
                        &AniType::from_syn_type(ty.as_ref()).to_union_variant_signature(),
                    );
                }
                if variants.is_empty() {
                    "C{std.core.Object}".to_string()
                } else {
                    format!("X{{{variants}}}")
                }
            }
            AniType::Promise(_) => "C{std.core.Object}".to_string(),
            AniType::FnArgs(_) | AniType::Tuple(_) | AniType::Unit => {
                "C{std.core.Object}".to_string()
            }
            AniType::Unknown(ty) => unknown_type_to_signature(ty)
                .map(|sig| to_new_style_ref_signature(&sig))
                .unwrap_or_else(|| "C{std.core.Object}".to_string()),
        }
    }

    fn to_fixed_array_elem_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => p.to_new_primitive_signature().to_string(),
            _ => self.to_union_variant_signature(),
        }
    }
}

impl PrimitiveType {
    /// Generate the primitive type signature
    pub fn to_signature(&self) -> String {
        match self {
            PrimitiveType::Bool => "Z".to_string(),
            PrimitiveType::I8 | PrimitiveType::U8 => "B".to_string(),
            PrimitiveType::I16 => "S".to_string(),
            PrimitiveType::U16 | PrimitiveType::Char => "C".to_string(),
            PrimitiveType::I32 | PrimitiveType::U32 => "I".to_string(),
            PrimitiveType::I64 | PrimitiveType::U64 => "J".to_string(),
            PrimitiveType::F32 => "F".to_string(),
            PrimitiveType::F64 => "D".to_string(),
        }
    }

    /// Generate the boxed primitive type signature
    pub fn to_boxed_signature(&self) -> String {
        match self {
            PrimitiveType::Bool => "Lstd/core/Boolean;".to_string(),
            PrimitiveType::I8 => "Lstd/core/Byte;".to_string(),
            PrimitiveType::I16 => "Lstd/core/Short;".to_string(),
            PrimitiveType::I32 => "Lstd/core/Int;".to_string(),
            PrimitiveType::I64 => "Lstd/core/Long;".to_string(),
            PrimitiveType::F32 => "Lstd/core/Float;".to_string(),
            PrimitiveType::F64 => "Lstd/core/Double;".to_string(),
            // Unsigned types use the same boxed types
            PrimitiveType::U8 => "Lstd/core/Byte;".to_string(),
            PrimitiveType::U16 | PrimitiveType::Char => "Lstd/core/Char;".to_string(),
            PrimitiveType::U32 => "Lstd/core/Int;".to_string(),
            PrimitiveType::U64 => "Lstd/core/Long;".to_string(),
        }
    }

    fn to_new_primitive_signature(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "z",
            PrimitiveType::I8 | PrimitiveType::U8 => "b",
            PrimitiveType::I16 => "s",
            PrimitiveType::U16 | PrimitiveType::Char => "c",
            PrimitiveType::I32 | PrimitiveType::U32 => "i",
            PrimitiveType::I64 | PrimitiveType::U64 => "l",
            PrimitiveType::F32 => "f",
            PrimitiveType::F64 => "d",
        }
    }

    fn to_boxed_new_signature(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "C{std.core.Boolean}",
            PrimitiveType::I8 | PrimitiveType::U8 => "C{std.core.Byte}",
            PrimitiveType::I16 => "C{std.core.Short}",
            PrimitiveType::U16 | PrimitiveType::Char => "C{std.core.Char}",
            PrimitiveType::I32 | PrimitiveType::U32 => "C{std.core.Int}",
            PrimitiveType::I64 | PrimitiveType::U64 => "C{std.core.Long}",
            PrimitiveType::F32 => "C{std.core.Float}",
            PrimitiveType::F64 => "C{std.core.Double}",
        }
    }

    /// Get the Rust type identifier for this primitive
    pub fn rust_type_str(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "bool",
            PrimitiveType::I8 => "i8",
            PrimitiveType::U8 => "u8",
            PrimitiveType::I16 => "i16",
            PrimitiveType::U16 => "u16",
            PrimitiveType::I32 => "i32",
            PrimitiveType::U32 => "u32",
            PrimitiveType::I64 => "i64",
            PrimitiveType::U64 => "u64",
            PrimitiveType::F32 => "f32",
            PrimitiveType::F64 => "f64",
            PrimitiveType::Char => "char",
        }
    }
}

impl WrapperType {
    /// Generate the wrapper type signature
    pub fn to_signature(&self) -> String {
        match self {
            WrapperType::Option(inner) => {
                format!(
                    "X{{{}{}}}",
                    inner.to_union_variant_signature(),
                    "C{std.core.Null}"
                )
            }
            WrapperType::Vec(inner) => format!("A{{{}}}", inner.to_fixed_array_elem_signature()),
            WrapperType::Result(inner) => inner.to_signature(),
            WrapperType::Ref(inner) => {
                // For Ref<AniObject>, return Object signature
                if matches!(inner.as_ref(), AniType::AniObject) {
                    "Lstd/core/Object;".to_string()
                } else {
                    inner.to_signature()
                }
            }
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Parse a primitive type from string identifier
fn parse_primitive(ident: &str) -> Option<PrimitiveType> {
    match ident {
        "bool" => Some(PrimitiveType::Bool),
        "i8" => Some(PrimitiveType::I8),
        "u8" => Some(PrimitiveType::U8),
        "i16" => Some(PrimitiveType::I16),
        "u16" => Some(PrimitiveType::U16),
        "i32" => Some(PrimitiveType::I32),
        "u32" => Some(PrimitiveType::U32),
        "i64" => Some(PrimitiveType::I64),
        "u64" => Some(PrimitiveType::U64),
        "f32" => Some(PrimitiveType::F32),
        "f64" => Some(PrimitiveType::F64),
        "char" => Some(PrimitiveType::Char),
        _ => None,
    }
}

/// All known Either type names with their variant counts
const EITHER_TYPES: &[(&str, usize)] = &[
    ("Either", 2),
    ("Either3", 3),
    ("Either4", 4),
    ("Either5", 5),
    ("Either6", 6),
    ("Either7", 7),
    ("Either8", 8),
    ("Either9", 9),
    ("Either10", 10),
    ("Either11", 11),
    ("Either12", 12),
    ("Either13", 13),
    ("Either14", 14),
    ("Either15", 15),
    ("Either16", 16),
    ("Either17", 17),
    ("Either18", 18),
    ("Either19", 19),
    ("Either20", 20),
    ("Either21", 21),
    ("Either22", 22),
    ("Either23", 23),
    ("Either24", 24),
    ("Either25", 25),
    ("Either26", 26),
];

/// Parse Either type from identifier and arguments
fn parse_either_type(ident: &str, args: &PathArguments) -> Option<EitherType> {
    let variant_count = EITHER_TYPES
        .iter()
        .find(|(name, _)| *name == ident)
        .map(|(_, count)| *count)?;

    let types = extract_all_generic_types(args)
        .into_iter()
        .map(Box::new)
        .collect();

    Some(EitherType {
        variant_count,
        types,
    })
}

/// Parse `HashMap<String, V>` as `Record<string, V>`.
fn parse_record_type(args: &PathArguments) -> Option<RecordType> {
    let types = extract_all_generic_types(args);
    if types.len() != 2 {
        return None;
    }

    let key_ty = AniType::from_syn_type(&types[0]);
    if !matches!(key_ty, AniType::String(_)) {
        return None;
    }

    Some(RecordType {
        value: Box::new(AniType::from_syn_type(&types[1])),
    })
}

fn unknown_type_to_signature(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => {
            let last = type_path.path.segments.last()?.ident.to_string();
            if let Some(sig) = known_ani_runtime_signature(&last) {
                return Some(sig.to_string());
            }

            let raw_path = type_path
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .filter(|seg| !matches!(seg.as_str(), "crate" | "self" | "super"))
                .collect::<Vec<_>>()
                .join(".");

            let alias =
                resolve_object_type_alias(&raw_path).or_else(|| resolve_object_type_alias(&last));
            let path = alias.as_deref().unwrap_or(&raw_path);
            if path.is_empty() {
                return None;
            }

            let qualified = qualify_custom_type_descriptor(path);
            Some(format!("L{};", qualified.replace('.', "/")))
        }
        Type::Reference(type_ref) => unknown_type_to_signature(type_ref.elem.as_ref()),
        Type::Paren(type_paren) => unknown_type_to_signature(type_paren.elem.as_ref()),
        Type::Group(type_group) => unknown_type_to_signature(type_group.elem.as_ref()),
        _ => None,
    }
}
fn qualify_custom_type_descriptor(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty()
        || trimmed.starts_with('@')
        || trimmed.starts_with("std.")
        || trimmed.starts_with("escompat.")
        || trimmed.starts_with("arkts.")
    {
        return trimmed.to_string();
    }

    let module_name = std::env::var("ANI_TEST_MODULE_NAME")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            std::env::var("CARGO_PKG_NAME")
                .unwrap_or_else(|_| String::from("entry"))
                .replace('-', "_")
        })
        .replace('-', "_");

    if trimmed.starts_with(&format!("{module_name}.")) {
        trimmed.to_string()
    } else {
        format!("{module_name}.{trimmed}")
    }
}

fn known_ani_runtime_signature(ident: &str) -> Option<&'static str> {
    match ident {
        "AniString" => Some("Lstd/core/String;"),
        "AniArrayBuffer" => Some("Lstd/core/ArrayBuffer;"),
        "FixedBooleanArray" | "AniFixedArrayBoolean" => Some("A{z}"),
        "FixedByteArray" | "AniFixedArrayByte" => Some("A{b}"),
        "FixedShortArray" | "AniFixedArrayShort" => Some("A{s}"),
        "FixedCharArray" | "AniFixedArrayChar" => Some("A{c}"),
        "FixedIntArray" | "AniArrayInt" | "AniFixedArrayInt" => Some("A{i}"),
        "FixedLongArray" | "AniArrayLong" | "AniFixedArrayLong" => Some("A{l}"),
        "FixedFloatArray" | "AniFixedArrayFloat" => Some("A{f}"),
        "FixedDoubleArray" | "AniArrayDouble" | "AniFixedArrayDouble" => Some("A{d}"),
        "AniFunction" | "AniFnObject" => Some("Lstd/core/Function;"),
        "Null" => Some("C{std.core.Null}"),
        "Undefined" => Some("U"),
        "AniRef" | "AniObject" | "AniClass" | "AniType" | "AniModule" | "AniNamespace"
        | "AniEnum" | "AniError" | "AniEnumItem" | "AniTupleValue" | "AniMethod"
        | "AniStaticMethod" | "AniField" | "AniStaticField" | "AniVariable" | "AniResolver"
        | "GlobalRef" | "WeakRef" => Some("Lstd/core/Object;"),
        _ => None,
    }
}

fn to_new_style_ref_signature(signature: &str) -> String {
    if signature == "U"
        || signature.starts_with("C{")
        || signature.starts_with("A{")
        || signature.starts_with("X{")
        || signature.starts_with("E{")
        || signature.starts_with("P{")
    {
        return signature.to_string();
    }
    if let Some(inner) = signature
        .strip_prefix('L')
        .and_then(|s| s.strip_suffix(';'))
    {
        return format!("C{{{}}}", inner.replace('/', "."));
    }
    match signature {
        "Z" | "z" => "C{std.core.Boolean}".to_string(),
        "B" | "b" => "C{std.core.Byte}".to_string(),
        "S" | "s" => "C{std.core.Short}".to_string(),
        "C" | "c" => "C{std.core.Char}".to_string(),
        "I" | "i" => "C{std.core.Int}".to_string(),
        "J" | "l" => "C{std.core.Long}".to_string(),
        "F" | "f" => "C{std.core.Float}".to_string(),
        "D" | "d" => "C{std.core.Double}".to_string(),
        _ => "C{std.core.Object}".to_string(),
    }
}

/// Check if a type path refers to a specific identifier
fn is_path_ident(type_path: &TypePath, ident: &str) -> bool {
    type_path.path.is_ident(ident)
}

/// Extract the first generic type argument
fn extract_first_generic_type(args: &PathArguments) -> Option<Type> {
    if let PathArguments::AngleBracketed(angle_args) = args {
        if let Some(GenericArgument::Type(ty)) = angle_args.args.first() {
            return Some(ty.clone());
        }
    }
    None
}

/// Extract all generic type arguments
fn extract_all_generic_types(args: &PathArguments) -> Vec<Type> {
    if let PathArguments::AngleBracketed(angle_args) = args {
        angle_args
            .args
            .iter()
            .filter_map(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(ty.clone())
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}

/// Extract Function<Args, Return> generics
fn extract_function_generics(args: &PathArguments) -> (Box<Type>, Box<Type>) {
    if let PathArguments::AngleBracketed(angle_args) = args {
        let types: Vec<_> = angle_args
            .args
            .iter()
            .filter_map(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(ty.clone())
                } else {
                    None
                }
            })
            .collect();

        if types.len() >= 2 {
            return (Box::new(types[0].clone()), Box::new(types[1].clone()));
        }
    }

    // Default: () for both
    (
        Box::new(syn::parse_quote!(())),
        Box::new(syn::parse_quote!(())),
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_primitive() {
        let ty: Type = syn::parse_quote!(i32);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::Primitive(PrimitiveType::I32)));
    }

    #[test]
    fn test_parse_string() {
        let ty: Type = syn::parse_quote!(String);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::String)));
    }

    #[test]
    fn test_parse_option() {
        let ty: Type = syn::parse_quote!(Option<i32>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Wrapper(WrapperType::Option(inner)) = ani_type {
            assert!(matches!(
                inner.as_ref(),
                AniType::Primitive(PrimitiveType::I32)
            ));
        } else {
            panic!("Expected Option type");
        }
    }

    #[test]
    fn test_parse_vec() {
        let ty: Type = syn::parse_quote!(Vec<String>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Wrapper(WrapperType::Vec(inner)) = ani_type {
            assert!(matches!(
                inner.as_ref(),
                AniType::String(StringType::String)
            ));
        } else {
            panic!("Expected Vec type");
        }
    }

    #[test]
    fn test_parse_result() {
        let ty: Type = syn::parse_quote!(Result<String, Error>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Wrapper(WrapperType::Result(inner)) = ani_type {
            assert!(matches!(
                inner.as_ref(),
                AniType::String(StringType::String)
            ));
        } else {
            panic!("Expected Result type");
        }
    }

    #[test]
    fn test_parse_either() {
        let ty: Type = syn::parse_quote!(Either<i32, String>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Either(either) = ani_type {
            assert_eq!(either.variant_count, 2);
        } else {
            panic!("Expected Either type");
        }
    }

    #[test]
    fn test_signature_generation() {
        let ty: Type = syn::parse_quote!(i32);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "I");

        let ty: Type = syn::parse_quote!(String);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;");

        let ty: Type = syn::parse_quote!(Vec<i32>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "A{i}");

        let ty: Type = syn::parse_quote!(HashMap<String, i32>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Record;");

        let ty: Type = syn::parse_quote!(Undefined);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "U");

        let ty: Type = syn::parse_quote!(Null);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "C{std.core.Null}");
    }

    #[test]
    fn test_boxed_signature() {
        let ty: Type = syn::parse_quote!(Option<i32>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(
            ani_type.to_signature(),
            "X{C{std.core.Int}C{std.core.Null}}"
        );
    }

    #[test]
    fn test_union_signature_uses_object_slot_for_undefined() {
        let ty: Type = syn::parse_quote!(Either<String, Undefined>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "X{C{std.core.String}U}");

        let ty: Type = syn::parse_quote!(Either3<String, Null, Undefined>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(
            ani_type.to_signature(),
            "X{C{std.core.String}C{std.core.Null}U}"
        );
    }

    #[test]
    fn test_parse_tuple() {
        // Single element tuple
        let ty: Type = syn::parse_quote!((i32,));
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Tuple(elements) = ani_type {
            assert_eq!(elements.len(), 1);
            assert!(matches!(
                elements[0],
                AniType::Primitive(PrimitiveType::I32)
            ));
        } else {
            panic!("Expected Tuple type, got {:?}", ani_type);
        }

        // Multi element tuple
        let ty: Type = syn::parse_quote!((i32, String, bool));
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Tuple(elements) = ani_type {
            assert_eq!(elements.len(), 3);
            assert!(matches!(
                elements[0],
                AniType::Primitive(PrimitiveType::I32)
            ));
            assert!(matches!(elements[1], AniType::String(StringType::String)));
            assert!(matches!(
                elements[2],
                AniType::Primitive(PrimitiveType::Bool)
            ));
        } else {
            panic!("Expected Tuple type");
        }
    }

    #[test]
    fn test_parse_record() {
        let ty: Type = syn::parse_quote!(HashMap<String, i32>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Record(record) = ani_type {
            assert!(matches!(
                record.value.as_ref(),
                AniType::Primitive(PrimitiveType::I32)
            ));
        } else {
            panic!("Expected Record type");
        }
    }

    #[test]
    fn test_parse_fn_args() {
        // FnArgs<(i32, i32)>
        let ty: Type = syn::parse_quote!(FnArgs<(i32, i32)>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::FnArgs(fn_args) = &ani_type {
            assert_eq!(fn_args.elements.len(), 2);
            assert!(matches!(
                fn_args.elements[0],
                AniType::Primitive(PrimitiveType::I32)
            ));
            assert!(matches!(
                fn_args.elements[1],
                AniType::Primitive(PrimitiveType::I32)
            ));
        } else {
            panic!("Expected FnArgs type, got {:?}", ani_type);
        }

        // FnArgs<(String, i64, bool)>
        let ty: Type = syn::parse_quote!(FnArgs<(String, i64, bool)>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::FnArgs(fn_args) = &ani_type {
            assert_eq!(fn_args.elements.len(), 3);
            assert!(matches!(
                fn_args.elements[0],
                AniType::String(StringType::String)
            ));
            assert!(matches!(
                fn_args.elements[1],
                AniType::Primitive(PrimitiveType::I64)
            ));
            assert!(matches!(
                fn_args.elements[2],
                AniType::Primitive(PrimitiveType::Bool)
            ));
        } else {
            panic!("Expected FnArgs type");
        }
    }

    #[test]
    fn test_fn_args_signature() {
        // FnArgs<(i32, i32)> should generate "II"
        let ty: Type = syn::parse_quote!(FnArgs<(i32, i32)>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "II");

        // FnArgs<(String, i64)> should generate "Lstd/core/String;J"
        let ty: Type = syn::parse_quote!(FnArgs<(String, i64)>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;J");
    }

    #[test]
    fn test_tuple_signature() {
        // (i32, i32) should generate "II"
        let ty: Type = syn::parse_quote!((i32, i32));
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "II");

        // (String, bool, f64) should generate signatures concatenated
        let ty: Type = syn::parse_quote!((String, bool, f64));
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;ZD");
    }

    #[test]
    fn test_unknown_custom_type_signature() {
        let ty: Type = syn::parse_quote!(crate::models::UserInfo);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");
    }

    #[test]
    fn test_unknown_custom_type_signature_local_type_is_module_qualified() {
        let ty: Type = syn::parse_quote!(UserProfile);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/UserProfile;");
    }

    #[test]
    fn test_unknown_custom_type_signature_uses_registered_object_alias() {
        register_object_type_alias("AliasedProfile", "models.AliasedProfile");
        let ty: Type = syn::parse_quote!(AliasedProfile);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(
            ani_type.to_signature(),
            "Lani_derive/models/AliasedProfile;"
        );
    }

    #[test]
    fn test_unknown_known_ani_wrapper_signature() {
        let ty: Type = syn::parse_quote!(AniString<'_>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;");
    }
}
