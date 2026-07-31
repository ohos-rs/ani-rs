//! ANI Type System
//!
//! Structured type representation for ANI FFI code generation.
//! This module provides a type-safe way to handle Rust-to-ANI type conversions
//! instead of string-based pattern matching.

use std::collections::{BTreeMap, HashSet};
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
    Isize,
    Usize,
    F32,
    F64,
    Char,
}

/// String-like types
#[derive(Debug, Clone, PartialEq)]
pub enum StringType {
    String,
    Str,
    CString,
    CStr,
    OsString,
    OsStr,
    PathBuf,
    Path,
    BoxStr,
    BoxPath,
    CowStr,
    CowPath,
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
    Function {
        args: Box<AniType>,
        ret: Box<AniType>,
        arity: usize,
    },
    /// FunctionRef<Args, Return>
    FunctionRef {
        args: Box<AniType>,
        ret: Box<AniType>,
        arity: usize,
    },
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
    pub types: Vec<AniType>,
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

/// Set type (`Set<T>`)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SetType {
    /// Element type `T`
    pub element: Box<AniType>,
}

/// Map type (`Map<K, V>`)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MapType {
    /// Key type `K`
    pub key: Box<AniType>,
    /// Value type `V`
    pub value: Box<AniType>,
}

/// Raw ANI runtime handles that should not fall back to `Unknown`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHandleType {
    Ref,
    Class,
    Type,
    Module,
    Namespace,
    String,
    Enum,
    Error,
    Method,
    StaticMethod,
    Field,
    StaticField,
    Function,
    FunctionObject,
    Variable,
    Resolver,
}

/// Raw ANI array handle families that should keep their surface container shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayHandleType {
    Array,
    ArrayRef,
    FixedArray,
    FixedArrayRef,
}

/// The main ANI type enum

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AniType {
    /// Primitive types (bool, i32, f64, etc.)
    Primitive(PrimitiveType),
    /// String types (String, &str)
    String(StringType),
    /// Arbitrary-precision ArkTS bigint.
    BigInt,
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
    /// Set type
    Set(SetType),
    /// Map type
    Map(MapType),
    /// AniObject - raw object type
    AniObject,
    /// Low-level global reference handle.
    GlobalRef,
    /// Low-level weak reference handle.
    WeakRef,
    /// Explicit ANI runtime handle types (class/method/field/etc.)
    RuntimeHandle(RuntimeHandleType),
    /// Low-level ANI array / fixed-array handle types.
    ArrayHandle(ArrayHandleType),
    /// Dynamic `Any_*` value wrapper backed by `ani_ref`
    AnyValue,
    /// Tuple value wrapper backed by `ani_tuple_value`
    TupleValue,
    /// Enum item wrapper backed by `ani_enum_item`
    EnumItem,
    /// ArrayBuffer - binary data buffer type
    ArrayBuffer,
    /// Tuple type (for function arguments)
    Tuple(Vec<AniType>),
    /// NativePointer<T> - typed native pointer wrapper exposed as ArkTS `long`
    NativePointer(Box<Type>),
    /// Typed ANI fixed array wrapper exposed as ArkTS `FixedArray<T>`.
    FixedArray(PrimitiveType),
    /// Function-level generic type parameter `T`/`U`/etc.
    TypeParam(String),
    /// Nominal custom object path that should not collapse into `Unknown`.
    CustomObject(Box<TypePath>),
    /// Unknown/custom type - fallback to object
    Unknown(Box<Type>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectMemberAccessKind {
    Field,
    Property,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectMemberDescriptor {
    pub rust_name: String,
    pub arkts_name: String,
    pub access: ObjectMemberAccessKind,
}

static OBJECT_TYPE_ALIASES: OnceLock<Mutex<BTreeMap<String, String>>> = OnceLock::new();
static OBJECT_TYPE_MEMBERS: OnceLock<Mutex<BTreeMap<String, Vec<ObjectMemberDescriptor>>>> =
    OnceLock::new();

fn object_type_aliases() -> &'static Mutex<BTreeMap<String, String>> {
    OBJECT_TYPE_ALIASES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn object_type_members() -> &'static Mutex<BTreeMap<String, Vec<ObjectMemberDescriptor>>> {
    OBJECT_TYPE_MEMBERS.get_or_init(|| Mutex::new(BTreeMap::new()))
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

pub fn register_object_type_members(rust_name: &str, members: &[ObjectMemberDescriptor]) {
    let rust_name = normalize_object_alias_key(rust_name);
    if rust_name.is_empty() {
        return;
    }

    object_type_members()
        .lock()
        .expect("failed to lock object type members")
        .insert(rust_name, members.to_vec());
}

#[allow(dead_code)]
pub fn register_object_type_fields(rust_name: &str, fields: &[String]) {
    let members = fields
        .iter()
        .map(|field| ObjectMemberDescriptor {
            rust_name: field.clone(),
            arkts_name: field.clone(),
            access: ObjectMemberAccessKind::Field,
        })
        .collect::<Vec<_>>();
    register_object_type_members(rust_name, &members);
}

pub fn resolve_object_type_members(name: &str) -> Option<Vec<ObjectMemberDescriptor>> {
    let key = normalize_object_alias_key(name);
    if key.is_empty() {
        return None;
    }

    object_type_members()
        .lock()
        .expect("failed to lock object type members")
        .get(&key)
        .cloned()
}

pub fn resolve_object_type_member_names(name: &str) -> Option<Vec<String>> {
    resolve_object_type_members(name).map(|members| {
        members
            .into_iter()
            .map(|member| member.arkts_name)
            .collect::<Vec<_>>()
    })
}

pub fn resolve_object_type_fields(name: &str) -> Option<Vec<String>> {
    resolve_object_type_member_names(name)
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
        Self::from_syn_type_with_type_params(ty, &HashSet::new())
    }

    /// Parse a syn::Type into an AniType with function-level type parameter context.
    pub fn from_syn_type_with_type_params(ty: &Type, type_params: &HashSet<String>) -> Self {
        match ty {
            Type::Path(type_path) => Self::parse_type_path_with_type_params(type_path, type_params),
            Type::Reference(type_ref) => {
                // Handle &str specially, otherwise preserve the referenced surface type.
                if let Type::Path(inner_path) = type_ref.elem.as_ref()
                    && is_path_ident(inner_path, "str")
                {
                    return AniType::String(StringType::Str);
                }
                AniType::from_syn_type_with_type_params(type_ref.elem.as_ref(), type_params)
            }
            Type::Paren(type_paren) => {
                AniType::from_syn_type_with_type_params(type_paren.elem.as_ref(), type_params)
            }
            Type::Group(type_group) => {
                AniType::from_syn_type_with_type_params(type_group.elem.as_ref(), type_params)
            }
            Type::Tuple(tuple) => {
                if tuple.elems.is_empty() {
                    AniType::Unit
                } else {
                    let elements: Vec<AniType> = tuple
                        .elems
                        .iter()
                        .map(|elem| AniType::from_syn_type_with_type_params(elem, type_params))
                        .collect();
                    AniType::Tuple(elements)
                }
            }
            _ => AniType::Unknown(Box::new(ty.clone())),
        }
    }

    fn parse_type_path_with_type_params(
        type_path: &TypePath,
        type_params: &HashSet<String>,
    ) -> Self {
        let segment = match type_path.path.segments.last() {
            Some(seg) => seg,
            None => return AniType::Unknown(Box::new(Type::Path(type_path.clone()))),
        };

        let ident = segment.ident.to_string();

        if is_type_param_path(type_path, type_params) {
            return AniType::TypeParam(ident);
        }

        // Check for primitive types first
        if let Some(primitive) = parse_primitive(&ident) {
            return AniType::Primitive(primitive);
        }

        // Check for string types
        if ident == "String" {
            return AniType::String(StringType::String);
        }

        if ident == "str" {
            return AniType::String(StringType::Str);
        }

        if ident == "CString" {
            return AniType::String(StringType::CString);
        }

        if ident == "CStr" {
            return AniType::String(StringType::CStr);
        }

        if ident == "OsString" {
            return AniType::String(StringType::OsString);
        }

        if ident == "OsStr" {
            return AniType::String(StringType::OsStr);
        }

        if ident == "PathBuf" {
            return AniType::String(StringType::PathBuf);
        }

        if ident == "Path" {
            return AniType::String(StringType::Path);
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

        if ident == "GlobalRef" {
            return AniType::GlobalRef;
        }

        if ident == "WeakRef" {
            return AniType::WeakRef;
        }

        if let Some(handle) = RuntimeHandleType::from_ident(&ident) {
            return AniType::RuntimeHandle(handle);
        }

        if ident == "AnyValue" {
            return AniType::AnyValue;
        }

        if ident == "BigInt" {
            return AniType::BigInt;
        }

        if ident == "TupleValue" || ident == "AniTupleValue" {
            return AniType::TupleValue;
        }

        if ident == "EnumItem" || ident == "AniEnumItem" {
            return AniType::EnumItem;
        }

        // Check for ArrayBuffer types
        if ident == "ArrayBuffer" || ident == "ArrayBufferSlice" || ident == "AniArrayBuffer" {
            return AniType::ArrayBuffer;
        }

        if let Some(array_handle) = parse_array_handle_type(&ident) {
            return AniType::ArrayHandle(array_handle);
        }

        // Check for Either types
        if let Some(either) = parse_either_type(&ident, &segment.arguments, type_params) {
            return AniType::Either(either);
        }

        if ident == "NativePointer" || ident == "ManagedResource" {
            let inner = extract_first_generic_type(&segment.arguments)
                .unwrap_or_else(|| syn::parse_quote!(()));
            return AniType::NativePointer(Box::new(inner));
        }

        if let Some(elem) = parse_fixed_array_type(&ident) {
            return AniType::FixedArray(elem);
        }

        if ident == "Box"
            && let Some(inner) = extract_first_generic_type(&segment.arguments)
        {
            if matches!(&inner, Type::Path(inner_path) if is_path_ident(inner_path, "str")) {
                return AniType::String(StringType::BoxStr);
            }
            if matches!(&inner, Type::Path(inner_path) if is_path_ident(inner_path, "Path")) {
                return AniType::String(StringType::BoxPath);
            }
        }

        if ident == "Cow"
            && let Some(inner) = extract_first_generic_type(&segment.arguments)
        {
            if matches!(&inner, Type::Path(inner_path) if is_path_ident(inner_path, "str")) {
                return AniType::String(StringType::CowStr);
            }
            if matches!(&inner, Type::Path(inner_path) if is_path_ident(inner_path, "Path")) {
                return AniType::String(StringType::CowPath);
            }
        }

        if let Some(inner) = extract_transparent_wrapper_inner_type(&ident, &segment.arguments) {
            return AniType::from_syn_type_with_type_params(&inner, type_params);
        }

        // Check for Promise types
        if ident == "PromiseRaw" {
            return AniType::Promise(PromiseType {
                inner: extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type_with_type_params(&t, type_params))),
            });
        }

        if ident == "Deferred" {
            return AniType::RuntimeHandle(RuntimeHandleType::Resolver);
        }

        // HashMap<String, V> maps to ArkTS Record<string, V>
        if ident == "HashMap"
            && let Some(record) = parse_record_type(&segment.arguments, type_params)
        {
            return AniType::Record(record);
        }

        // HashSet<T> and BTreeSet<T> map to ArkTS Set<T>
        if (ident == "HashSet" || ident == "BTreeSet")
            && let Some(set) = parse_set_type(&segment.arguments, type_params)
        {
            return AniType::Set(set);
        }

        // BTreeMap<K, V> maps to ArkTS Map<K, V>
        if ident == "BTreeMap"
            && let Some(map) = parse_map_type(&segment.arguments, type_params)
        {
            return AniType::Map(map);
        }

        // Check for generic wrapper types

        match ident.as_str() {
            "Option" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type_with_type_params(&t, type_params)))
                    .unwrap_or_else(|| {
                        Box::new(AniType::Unknown(Box::new(Type::Path(type_path.clone()))))
                    });
                AniType::Wrapper(WrapperType::Option(inner))
            }
            "Vec" | "VecDeque" | "LinkedList" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type_with_type_params(&t, type_params)))
                    .unwrap_or_else(|| {
                        Box::new(AniType::Unknown(Box::new(Type::Path(type_path.clone()))))
                    });
                AniType::Wrapper(WrapperType::Vec(inner))
            }
            "Result" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type_with_type_params(&t, type_params)))
                    .unwrap_or_else(|| Box::new(AniType::Unit));
                AniType::Wrapper(WrapperType::Result(inner))
            }
            "Ref" => {
                let inner = extract_first_generic_type(&segment.arguments)
                    .map(|t| Box::new(AniType::from_syn_type_with_type_params(&t, type_params)))
                    .unwrap_or_else(|| Box::new(AniType::AniObject));
                AniType::Wrapper(WrapperType::Ref(inner))
            }
            "Function" => {
                let (args, ret, arity) = parse_function_generics(&segment.arguments, type_params);
                AniType::Function(FunctionType::Function { args, ret, arity })
            }
            "FunctionRef" | "ThreadsafeFunction" => {
                let (args, ret, arity) = parse_function_generics(&segment.arguments, type_params);
                AniType::Function(FunctionType::FunctionRef { args, ret, arity })
            }
            "FnArgs" => {
                // FnArgs<T> where T is typically a tuple
                let inner = extract_first_generic_type(&segment.arguments);
                let elements = inner
                    .as_ref()
                    .map(|t| {
                        let parsed = AniType::from_syn_type_with_type_params(t, type_params);
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
            _ if is_custom_object_type_path(type_path) => {
                AniType::CustomObject(Box::new(type_path.clone()))
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
            AniType::BigInt => quote! { ani::sys::ani_object },
            AniType::Unit => quote! { () },
            AniType::Null | AniType::Undefined => quote! { ani::sys::ani_object },
            AniType::Wrapper(w) => w.to_ani_c_type(),
            AniType::Function(_) => quote! { ani::sys::ani_fn_object },
            AniType::FnArgs(_) => quote! { ani::sys::ani_object },
            AniType::Either(_) => quote! { ani::sys::ani_object },
            AniType::Promise(_) => quote! { ani::sys::ani_object },
            AniType::Record(_) => quote! { ani::sys::ani_object },
            AniType::Set(_) => quote! { ani::sys::ani_object },
            AniType::Map(_) => quote! { ani::sys::ani_object },
            AniType::AniObject => quote! { ani::sys::ani_object },
            AniType::GlobalRef => quote! { ani::sys::ani_ref },
            AniType::WeakRef => quote! { ani::sys::ani_wref },
            AniType::RuntimeHandle(handle) => handle.to_ani_c_type(),
            AniType::ArrayHandle(handle) => handle.to_ani_c_type(),
            AniType::AnyValue => quote! { ani::sys::ani_ref },
            AniType::TupleValue => quote! { ani::sys::ani_tuple_value },
            AniType::EnumItem => quote! { ani::sys::ani_enum_item },

            AniType::ArrayBuffer => quote! { ani::sys::ani_arraybuffer },
            AniType::Tuple(_) => quote! { ani::sys::ani_object },
            AniType::NativePointer(_) => quote! { ani::sys::ani_long },
            AniType::FixedArray(p) => p.to_fixed_array_ani_c_type(),
            AniType::TypeParam(_) => quote! { ani::sys::ani_object },
            AniType::CustomObject(_) => quote! { ani::sys::ani_object },
            AniType::Unknown(_) => quote! { ani::sys::ani_object },
        }
    }
}

impl ArrayHandleType {
    fn to_ani_c_type(self) -> TokenStream {
        match self {
            Self::Array | Self::ArrayRef => quote! { ani::sys::ani_array },
            Self::FixedArray => quote! { ani::sys::ani_fixedarray },
            Self::FixedArrayRef => quote! { ani::sys::ani_fixedarray_ref },
        }
    }

    fn signature(&self) -> &'static str {
        "A{C{std.core.Object}}"
    }

    fn union_variant_signature(&self) -> String {
        self.signature().to_string()
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
            PrimitiveType::I64
            | PrimitiveType::U64
            | PrimitiveType::Isize
            | PrimitiveType::Usize => quote! { ani::sys::ani_long },
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

impl RuntimeHandleType {
    fn from_ident(ident: &str) -> Option<Self> {
        match ident {
            "AniRef" => Some(Self::Ref),
            "AniClass" => Some(Self::Class),
            "AniType" => Some(Self::Type),
            "AniModule" => Some(Self::Module),
            "AniNamespace" => Some(Self::Namespace),
            "AniString" => Some(Self::String),
            "AniEnum" => Some(Self::Enum),
            "AniError" => Some(Self::Error),
            "AniMethod" => Some(Self::Method),
            "AniStaticMethod" => Some(Self::StaticMethod),
            "AniField" => Some(Self::Field),
            "AniStaticField" => Some(Self::StaticField),
            "AniFunction" => Some(Self::Function),
            "AniFnObject" => Some(Self::FunctionObject),
            "AniVariable" => Some(Self::Variable),
            "AniResolver" => Some(Self::Resolver),
            _ => None,
        }
    }

    fn to_ani_c_type(self) -> TokenStream {
        match self {
            Self::Ref => quote! { ani::sys::ani_ref },
            Self::Class => quote! { ani::sys::ani_class },
            Self::Type => quote! { ani::sys::ani_type },
            Self::Module => quote! { ani::sys::ani_module },
            Self::Namespace => quote! { ani::sys::ani_namespace },
            Self::String => quote! { ani::sys::ani_string },
            Self::Enum => quote! { ani::sys::ani_enum },
            Self::Error => quote! { ani::sys::ani_error },
            Self::Method => quote! { ani::sys::ani_method },
            Self::StaticMethod => quote! { ani::sys::ani_static_method },
            Self::Field => quote! { ani::sys::ani_field },
            Self::StaticField => quote! { ani::sys::ani_static_field },
            Self::Function => quote! { ani::sys::ani_function },
            Self::FunctionObject => quote! { ani::sys::ani_fn_object },
            Self::Variable => quote! { ani::sys::ani_variable },
            Self::Resolver => quote! { ani::sys::ani_resolver },
        }
    }

    fn signature(self) -> &'static str {
        match self {
            Self::Class => "Lstd/core/Class;",
            Self::String => "Lstd/core/String;",
            Self::Function | Self::FunctionObject => "Lstd/core/Function;",
            Self::Ref
            | Self::Type
            | Self::Module
            | Self::Namespace
            | Self::Enum
            | Self::Error
            | Self::Method
            | Self::StaticMethod
            | Self::Field
            | Self::StaticField
            | Self::Variable
            | Self::Resolver => "Lstd/core/Object;",
        }
    }

    fn union_variant_signature(self) -> String {
        to_new_style_ref_signature(self.signature())
    }
}

// ============================================================================
// ANI Signature Generation
// ============================================================================

fn function_arity_from_ani_type(ty: &AniType) -> usize {
    match ty {
        AniType::Tuple(items) => items.len(),
        AniType::FnArgs(fn_args) => fn_args.elements.len(),
        AniType::Unit => 0,
        _ => 1,
    }
}

fn function_signature_name(function: &FunctionType) -> String {
    let arity = match function {
        FunctionType::Function { arity, .. } | FunctionType::FunctionRef { arity, .. } => *arity,
    };
    if arity <= 16 {
        format!("std.core.Function{arity}")
    } else {
        "std.core.Function".to_string()
    }
}

impl AniType {
    /// Generate the ANI type signature string
    pub fn to_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => p.to_signature(),
            AniType::String(_) => "Lstd/core/String;".to_string(),
            AniType::BigInt => "Lstd/core/BigInt;".to_string(),
            AniType::Unit => "V".to_string(),
            AniType::Null => "C{std.core.Null}".to_string(),
            AniType::Undefined => "U".to_string(),
            AniType::Wrapper(w) => w.to_signature(),
            AniType::Function(function) => {
                format!("L{};", function_signature_name(function).replace('.', "/"))
            }
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
                    let variant = ty.to_union_variant_signature();
                    variants.push_str(&variant);
                }
                if variants.is_empty() {
                    "C{std.core.Object}".to_string()
                } else {
                    format!("X{{{variants}}}")
                }
            }
            AniType::Promise(_) => "Lstd/core/Promise;".to_string(),
            AniType::Record(_) => "Lstd/core/Record;".to_string(),
            AniType::Set(_) => "Lstd/core/Set;".to_string(),
            AniType::Map(_) => "Lstd/core/Map;".to_string(),
            AniType::AniObject => "Lstd/core/Object;".to_string(),
            AniType::GlobalRef => "Lstd/core/Object;".to_string(),
            AniType::WeakRef => "Lstd/core/WeakRef;".to_string(),
            AniType::RuntimeHandle(handle) => handle.signature().to_string(),
            AniType::ArrayHandle(handle) => handle.signature().to_string(),
            AniType::AnyValue | AniType::TupleValue | AniType::EnumItem => {
                "Lstd/core/Object;".to_string()
            }

            AniType::ArrayBuffer => "Lstd/core/ArrayBuffer;".to_string(),
            AniType::NativePointer(_) => "J".to_string(),
            AniType::FixedArray(p) => format!("A{{{}}}", p.to_new_primitive_signature()),
            AniType::TypeParam(_) => "Lstd/core/Object;".to_string(),
            AniType::CustomObject(type_path) => custom_object_path_signature(type_path.as_ref())
                .unwrap_or_else(|| "Lstd/core/Object;".to_string()),
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
    fn to_union_variant_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => p.to_boxed_new_signature().to_string(),
            AniType::String(_) => "C{std.core.String}".to_string(),
            AniType::BigInt => "C{std.core.BigInt}".to_string(),
            AniType::Null => "C{std.core.Null}".to_string(),
            AniType::Undefined => "U".to_string(),
            AniType::AniObject => "C{std.core.Object}".to_string(),
            AniType::GlobalRef => "C{std.core.Object}".to_string(),
            AniType::WeakRef => "C{std.core.WeakRef}".to_string(),
            AniType::RuntimeHandle(handle) => handle.union_variant_signature(),
            AniType::ArrayHandle(handle) => handle.union_variant_signature(),
            AniType::AnyValue | AniType::TupleValue | AniType::EnumItem => {
                "C{std.core.Object}".to_string()
            }
            AniType::ArrayBuffer => "C{std.core.ArrayBuffer}".to_string(),
            AniType::NativePointer(_) => PrimitiveType::I64.to_boxed_new_signature().to_string(),
            AniType::FixedArray(p) => format!("A{{{}}}", p.to_new_primitive_signature()),
            AniType::TypeParam(_) => "C{std.core.Object}".to_string(),
            AniType::CustomObject(type_path) => custom_object_path_signature(type_path.as_ref())
                .map(|sig| to_new_style_ref_signature(&sig))
                .unwrap_or_else(|| "C{std.core.Object}".to_string()),
            AniType::Record(_) => "C{std.core.Record}".to_string(),
            AniType::Set(_) => to_new_style_ref_signature("Lstd/core/Set;"),
            AniType::Map(_) => to_new_style_ref_signature("Lstd/core/Map;"),
            AniType::Function(function) => format!("C{{{}}}", function_signature_name(function)),

            AniType::Wrapper(WrapperType::Vec(inner)) => inner.to_vec_bind_signature_variant(),
            AniType::Wrapper(WrapperType::Option(inner)) => inner.to_union_variant_signature(),
            AniType::Wrapper(WrapperType::Result(inner)) => inner.to_union_variant_signature(),
            AniType::Wrapper(WrapperType::Ref(inner)) => inner.to_union_variant_signature(),
            AniType::Either(either) => {
                let mut variants = String::new();
                for ty in &either.types {
                    variants.push_str(&ty.to_union_variant_signature());
                }
                if variants.is_empty() {
                    "C{std.core.Object}".to_string()
                } else {
                    format!("X{{{variants}}}")
                }
            }
            AniType::Promise(_) => "C{std.core.Promise}".to_string(),
            AniType::FnArgs(_) | AniType::Tuple(_) | AniType::Unit => {
                "C{std.core.Object}".to_string()
            }
            AniType::Unknown(ty) => unknown_type_to_signature(ty)
                .map(|sig| to_new_style_ref_signature(&sig))
                .unwrap_or_else(|| "C{std.core.Object}".to_string()),
        }
    }

    fn to_vec_bind_signature(&self) -> String {
        match self {
            AniType::Primitive(p) => format!("A{{{}}}", p.to_new_primitive_signature()),
            _ => "Lstd/core/Array;".to_string(),
        }
    }

    fn to_vec_bind_signature_variant(&self) -> String {
        match self {
            AniType::Primitive(p) => format!("A{{{}}}", p.to_new_primitive_signature()),
            _ => "C{std.core.Array}".to_string(),
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
            PrimitiveType::I64
            | PrimitiveType::U64
            | PrimitiveType::Isize
            | PrimitiveType::Usize => "J".to_string(),
            PrimitiveType::F32 => "F".to_string(),
            PrimitiveType::F64 => "D".to_string(),
        }
    }
    fn to_new_primitive_signature(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "z",
            PrimitiveType::I8 | PrimitiveType::U8 => "b",
            PrimitiveType::I16 => "s",
            PrimitiveType::U16 | PrimitiveType::Char => "c",
            PrimitiveType::I32 | PrimitiveType::U32 => "i",
            PrimitiveType::I64
            | PrimitiveType::U64
            | PrimitiveType::Isize
            | PrimitiveType::Usize => "l",
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
            PrimitiveType::I64
            | PrimitiveType::U64
            | PrimitiveType::Isize
            | PrimitiveType::Usize => "C{std.core.Long}",
            PrimitiveType::F32 => "C{std.core.Float}",
            PrimitiveType::F64 => "C{std.core.Double}",
        }
    }

    fn to_fixed_array_ani_c_type(&self) -> TokenStream {
        match self {
            PrimitiveType::Bool => quote! { ani::sys::ani_fixedarray_boolean },
            PrimitiveType::I8 | PrimitiveType::U8 => quote! { ani::sys::ani_fixedarray_byte },
            PrimitiveType::I16 => quote! { ani::sys::ani_fixedarray_short },
            PrimitiveType::U16 | PrimitiveType::Char => quote! { ani::sys::ani_fixedarray_char },
            PrimitiveType::I32 | PrimitiveType::U32 => quote! { ani::sys::ani_fixedarray_int },
            PrimitiveType::I64
            | PrimitiveType::U64
            | PrimitiveType::Isize
            | PrimitiveType::Usize => quote! { ani::sys::ani_fixedarray_long },
            PrimitiveType::F32 => quote! { ani::sys::ani_fixedarray_float },
            PrimitiveType::F64 => quote! { ani::sys::ani_fixedarray_double },
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
            WrapperType::Vec(inner) => inner.to_vec_bind_signature(),
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
        "isize" => Some(PrimitiveType::Isize),
        "usize" => Some(PrimitiveType::Usize),
        "f32" => Some(PrimitiveType::F32),
        "f64" => Some(PrimitiveType::F64),
        "char" => Some(PrimitiveType::Char),
        _ => None,
    }
}

fn parse_array_handle_type(ident: &str) -> Option<ArrayHandleType> {
    match ident {
        "AniArray" => Some(ArrayHandleType::Array),
        "AniArrayRef" => Some(ArrayHandleType::ArrayRef),
        "AniFixedArray" => Some(ArrayHandleType::FixedArray),
        "AniFixedArrayRef" => Some(ArrayHandleType::FixedArrayRef),
        _ => None,
    }
}

fn parse_fixed_array_type(ident: &str) -> Option<PrimitiveType> {
    match ident {
        "FixedBooleanArray" | "AniFixedArrayBoolean" => Some(PrimitiveType::Bool),
        "FixedByteArray" | "AniFixedArrayByte" => Some(PrimitiveType::I8),
        "FixedShortArray" | "AniFixedArrayShort" => Some(PrimitiveType::I16),
        "FixedCharArray" | "AniFixedArrayChar" => Some(PrimitiveType::Char),
        "FixedIntArray" | "AniArrayInt" | "AniFixedArrayInt" => Some(PrimitiveType::I32),
        "FixedLongArray" | "AniArrayLong" | "AniFixedArrayLong" => Some(PrimitiveType::I64),
        "FixedFloatArray" | "AniFixedArrayFloat" => Some(PrimitiveType::F32),
        "FixedDoubleArray" | "AniArrayDouble" | "AniFixedArrayDouble" => Some(PrimitiveType::F64),
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
fn parse_either_type(
    ident: &str,
    args: &PathArguments,
    type_params: &HashSet<String>,
) -> Option<EitherType> {
    let variant_count = EITHER_TYPES
        .iter()
        .find(|(name, _)| *name == ident)
        .map(|(_, count)| *count)?;

    let types = extract_all_generic_types(args)
        .into_iter()
        .map(|ty| AniType::from_syn_type_with_type_params(&ty, type_params))
        .collect();

    Some(EitherType {
        variant_count,
        types,
    })
}

/// Parse `HashMap<String, V>` as `Record<string, V>`.
fn parse_record_type(args: &PathArguments, type_params: &HashSet<String>) -> Option<RecordType> {
    let types = extract_all_generic_types(args);
    if types.len() != 2 {
        return None;
    }

    let key_ty = AniType::from_syn_type_with_type_params(&types[0], type_params);
    if !matches!(key_ty, AniType::String(_)) {
        return None;
    }

    Some(RecordType {
        value: Box::new(AniType::from_syn_type_with_type_params(
            &types[1],
            type_params,
        )),
    })
}

fn parse_set_type(args: &PathArguments, type_params: &HashSet<String>) -> Option<SetType> {
    extract_first_generic_type(args).map(|element| SetType {
        element: Box::new(AniType::from_syn_type_with_type_params(
            &element,
            type_params,
        )),
    })
}

pub(crate) fn type_path_qualified_name(type_path: &TypePath) -> String {
    type_path
        .path
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .filter(|seg| !matches!(seg.as_str(), "crate" | "self" | "super"))
        .collect::<Vec<_>>()
        .join(".")
}

pub(crate) fn is_custom_object_name(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }

    let Some(first) = ident.chars().next() else {
        return false;
    };
    if !first.is_ascii_uppercase() {
        return false;
    }

    !matches!(
        ident,
        "String"
            | "Option"
            | "Result"
            | "Vec"
            | "Box"
            | "Rc"
            | "Arc"
            | "Cow"
            | "Pin"
            | "Mutex"
            | "RwLock"
            | "RefCell"
            | "Cell"
            | "UnsafeCell"
            | "ManuallyDrop"
            | "MaybeUninit"
            | "OnceLock"
            | "LazyLock"
            | "HashMap"
            | "BTreeMap"
            | "HashSet"
            | "BTreeSet"
            | "VecDeque"
            | "LinkedList"
            | "CStr"
            | "OsStr"
            | "OsString"
            | "Path"
            | "PathBuf"
            | "Function"
            | "FunctionRef"
            | "ThreadsafeFunction"
            | "FnArgs"
            | "BigInt"
            | "PromiseRaw"
            | "Deferred"
            | "NativePointer"
            | "ManagedResource"
    )
}

pub(crate) fn is_custom_object_type_path(type_path: &TypePath) -> bool {
    let Some(last) = type_path.path.segments.last() else {
        return false;
    };
    let ident = last.ident.to_string();
    if known_ani_runtime_signature(&ident).is_some()
        || RuntimeHandleType::from_ident(&ident).is_some()
    {
        return false;
    }

    let qualified = type_path_qualified_name(type_path);
    resolve_object_type_alias(&qualified)
        .or_else(|| resolve_object_type_alias(&ident))
        .is_some()
        || is_custom_object_name(&ident)
}

fn custom_object_path_signature(type_path: &TypePath) -> Option<String> {
    let last = type_path.path.segments.last()?.ident.to_string();
    let raw_path = type_path_qualified_name(type_path);
    let alias = resolve_object_type_alias(&raw_path).or_else(|| resolve_object_type_alias(&last));
    let path = alias.as_deref().unwrap_or(&raw_path);
    if path.is_empty() {
        return None;
    }

    let qualified = qualify_custom_type_descriptor(path);
    Some(format!("L{};", qualified.replace('.', "/")))
}

fn parse_map_type(args: &PathArguments, type_params: &HashSet<String>) -> Option<MapType> {
    let types = extract_all_generic_types(args);
    if types.len() != 2 {
        return None;
    }

    Some(MapType {
        key: Box::new(AniType::from_syn_type_with_type_params(
            &types[0],
            type_params,
        )),
        value: Box::new(AniType::from_syn_type_with_type_params(
            &types[1],
            type_params,
        )),
    })
}

pub(crate) fn extract_transparent_wrapper_inner_type(
    ident: &str,
    args: &PathArguments,
) -> Option<Type> {
    match ident {
        "Box" | "Rc" | "Arc" | "Cow" | "Pin" | "Mutex" | "RwLock" | "RefCell" | "Cell"
        | "UnsafeCell" | "ManuallyDrop" | "MaybeUninit" | "OnceLock" | "LazyLock" => {
            extract_first_generic_type(args)
        }
        _ => None,
    }
}

fn unknown_type_to_signature(ty: &Type) -> Option<String> {
    let reparsed = AniType::from_syn_type(ty);
    match reparsed {
        AniType::Unknown(inner) if inner.as_ref() != ty => {
            return unknown_type_to_signature(inner.as_ref());
        }
        AniType::Unknown(_) => {}
        other => return Some(other.to_signature()),
    }

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
        "AniArray" | "AniArrayRef" | "AniFixedArray" | "AniFixedArrayRef" => {
            Some("A{C{std.core.Object}}")
        }
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
        | "GlobalRef" => Some("Lstd/core/Object;"),
        "WeakRef" => Some("Lstd/core/WeakRef;"),
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
fn is_type_param_path(type_path: &TypePath, type_params: &HashSet<String>) -> bool {
    type_path.qself.is_none()
        && type_path.path.segments.len() == 1
        && type_path.path.segments.first().is_some_and(|segment| {
            segment.arguments.is_empty() && type_params.contains(&segment.ident.to_string())
        })
}

fn is_path_ident(type_path: &TypePath, ident: &str) -> bool {
    type_path.path.is_ident(ident)
        || type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == ident)
}

/// Extract the first generic type argument
fn extract_first_generic_type(args: &PathArguments) -> Option<Type> {
    if let PathArguments::AngleBracketed(angle_args) = args {
        return angle_args.args.iter().find_map(|arg| match arg {
            GenericArgument::Type(ty) => Some(ty.clone()),
            _ => None,
        });
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

/// Parse Function<Args, Return> generics with type-parameter context.
fn parse_function_generics(
    args: &PathArguments,
    type_params: &HashSet<String>,
) -> (Box<AniType>, Box<AniType>, usize) {
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

        let (args_ty, ret_ty) = if types.len() >= 2 {
            (types[0].clone(), types[1].clone())
        } else {
            (syn::parse_quote!(()), syn::parse_quote!(()))
        };

        let parsed_args = AniType::from_syn_type_with_type_params(&args_ty, type_params);
        let parsed_ret = AniType::from_syn_type_with_type_params(&ret_ty, type_params);
        let arity = function_arity_from_ani_type(&parsed_args);
        return (Box::new(parsed_args), Box::new(parsed_ret), arity);
    }

    (Box::new(AniType::Unit), Box::new(AniType::Unit), 0)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static TEST_ENV_LOCK: Mutex<()> = Mutex::new(());

    struct TestEnvVarGuard {
        previous: Option<String>,
        _lock: MutexGuard<'static, ()>,
    }

    impl TestEnvVarGuard {
        fn unset(key: &str) -> Self {
            let lock = TEST_ENV_LOCK.lock().expect("lock test env mutex");
            let previous = std::env::var(key).ok();
            // Safety: tests holding this guard serialize process-wide env mutation.
            unsafe {
                std::env::remove_var(key);
            }
            let _ = key;
            Self {
                previous,
                _lock: lock,
            }
        }
    }

    impl Drop for TestEnvVarGuard {
        fn drop(&mut self) {
            // Safety: tests holding this guard serialize process-wide env mutation.
            unsafe {
                match &self.previous {
                    Some(previous) => std::env::set_var("ANI_TEST_MODULE_NAME", previous),
                    None => std::env::remove_var("ANI_TEST_MODULE_NAME"),
                }
            }
        }
    }

    #[test]
    fn test_parse_primitive() {
        let ty: Type = syn::parse_quote!(i32);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::Primitive(PrimitiveType::I32)));

        let ty: Type = syn::parse_quote!(isize);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::Primitive(PrimitiveType::Isize)));

        let ty: Type = syn::parse_quote!(usize);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::Primitive(PrimitiveType::Usize)));
    }

    #[test]
    fn test_parse_string() {
        let ty: Type = syn::parse_quote!(String);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::String)));

        let ty: Type = syn::parse_quote!(std::ffi::CString);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::CString)));

        let ty: Type = syn::parse_quote!(&std::ffi::CStr);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::CStr)));

        let ty: Type = syn::parse_quote!(std::ffi::OsString);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::OsString)));

        let ty: Type = syn::parse_quote!(&std::ffi::OsStr);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::OsStr)));

        let ty: Type = syn::parse_quote!(std::path::PathBuf);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::PathBuf)));

        let ty: Type = syn::parse_quote!(&std::path::Path);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::Path)));

        let ty: Type = syn::parse_quote!(Box<str>);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::BoxStr)));

        let ty: Type = syn::parse_quote!(Box<std::path::Path>);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::BoxPath)));

        let ty: Type = syn::parse_quote!(std::borrow::Cow<'static, str>);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::CowStr)));

        let ty: Type = syn::parse_quote!(std::borrow::Cow<'static, std::path::Path>);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::String(StringType::CowPath)));
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
    fn test_parse_promise_raw_and_deferred() {
        let ty: Type = syn::parse_quote!(PromiseRaw<String>);
        let ani_type = AniType::from_syn_type(&ty);
        match ani_type {
            AniType::Promise(PromiseType { inner: Some(inner) }) => {
                assert!(matches!(
                    inner.as_ref(),
                    AniType::String(StringType::String)
                ));
            }
            other => panic!("Expected PromiseRaw type, got {other:?}"),
        }

        let ty: Type = syn::parse_quote!(Deferred<String>);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(
            ani_type,
            AniType::RuntimeHandle(RuntimeHandleType::Resolver)
        ));
    }

    #[test]
    fn test_parse_type_param_with_context() {
        let mut type_params = HashSet::new();
        type_params.insert("T".to_string());

        let ty: Type = syn::parse_quote!(Option<T>);
        let ani_type = AniType::from_syn_type_with_type_params(&ty, &type_params);
        if let AniType::Wrapper(WrapperType::Option(inner)) = ani_type {
            assert!(matches!(inner.as_ref(), AniType::TypeParam(name) if name == "T"));
        } else {
            panic!("Expected Option<T> to preserve T as type param");
        }
    }

    #[test]
    fn test_parse_function_type_param_with_context() {
        let mut type_params = HashSet::new();
        type_params.insert("T".to_string());

        let ty: Type = syn::parse_quote!(Function<(T,), T>);
        let ani_type = AniType::from_syn_type_with_type_params(&ty, &type_params);
        if let AniType::Function(FunctionType::Function { args, ret, arity }) = ani_type {
            assert_eq!(arity, 1);
            assert!(
                matches!(args.as_ref(), AniType::Tuple(items) if matches!(items.as_slice(), [AniType::TypeParam(name)] if name == "T"))
            );
            assert!(matches!(ret.as_ref(), AniType::TypeParam(name) if name == "T"));
        } else {
            panic!("Expected Function<(T,), T> to preserve T as type param");
        }
    }

    #[test]
    fn test_parse_nested_container_type_params_with_context() {
        let mut type_params = HashSet::new();
        type_params.insert("T".to_string());
        type_params.insert("U".to_string());

        let ty: Type = syn::parse_quote!(Either<T, HashMap<String, U>>);
        let ani_type = AniType::from_syn_type_with_type_params(&ty, &type_params);
        if let AniType::Either(either) = ani_type {
            assert!(
                matches!(either.types.as_slice(), [AniType::TypeParam(name), AniType::Record(_)] if name == "T")
            );
            match &either.types[1] {
                AniType::Record(record) => {
                    assert!(
                        matches!(record.value.as_ref(), AniType::TypeParam(name) if name == "U")
                    );
                }
                other => panic!("Expected Record<U> variant, got {:?}", other),
            }
        } else {
            panic!("Expected Either<T, HashMap<String, U>> to preserve nested type params");
        }
    }

    #[test]
    fn test_nested_container_type_param_signature_uses_type_param_erasure() {
        let mut type_params = HashSet::new();
        type_params.insert("T".to_string());

        let ty: Type = syn::parse_quote!(Either<T, String>);
        let ani_type = AniType::from_syn_type_with_type_params(&ty, &type_params);
        assert_eq!(
            ani_type.to_signature(),
            "X{C{std.core.Object}C{std.core.String}}"
        );

        let ty: Type = syn::parse_quote!(BTreeMap<String, T>);
        let ani_type = AniType::from_syn_type_with_type_params(&ty, &type_params);
        if let AniType::Map(map) = ani_type {
            assert!(matches!(map.value.as_ref(), AniType::TypeParam(name) if name == "T"));
        } else {
            panic!("Expected BTreeMap<String, T> to preserve T as type param");
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

        let ty: Type = syn::parse_quote!(std::ffi::CString);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;");

        let ty: Type = syn::parse_quote!(isize);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "J");

        let ty: Type = syn::parse_quote!(usize);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "J");

        let ty: Type = syn::parse_quote!(Vec<i32>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "A{i}");

        let ty: Type = syn::parse_quote!(FixedIntArray);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "A{i}");

        let ty: Type = syn::parse_quote!(HashMap<String, i32>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Record;");

        let ty: Type = syn::parse_quote!(HashSet<String>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Set;");

        let ty: Type = syn::parse_quote!(BTreeSet<String>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Set;");

        let ty: Type = syn::parse_quote!(BTreeMap<String, i32>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Map;");

        let ty: Type = syn::parse_quote!(ani::conversions::NativePointer<crate::NativeResource>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "J");

        let ty: Type = syn::parse_quote!(ani::conversions::ManagedResource<crate::NativeResource>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "J");

        let ty: Type = syn::parse_quote!(ani::conversions::BigInt);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/BigInt;");

        let ty: Type = syn::parse_quote!(ani::conversions::AnyValue);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Object;");

        let ty: Type = syn::parse_quote!(ani::conversions::TupleValue);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Object;");

        let ty: Type = syn::parse_quote!(ani::conversions::EnumItem);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Object;");

        let ty: Type = syn::parse_quote!(GlobalRef);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Object;");

        let ty: Type = syn::parse_quote!(WeakRef);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/WeakRef;");

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
    fn test_parse_set() {
        let ty: Type = syn::parse_quote!(HashSet<String>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Set(set) = ani_type {
            assert!(matches!(
                set.element.as_ref(),
                AniType::String(StringType::String)
            ));
        } else {
            panic!("Expected Set type");
        }

        let ty: Type = syn::parse_quote!(BTreeSet<String>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Set(set) = ani_type {
            assert!(matches!(
                set.element.as_ref(),
                AniType::String(StringType::String)
            ));
        } else {
            panic!("Expected Set type");
        }
    }

    #[test]
    fn test_parse_native_pointer() {
        let ty: Type = syn::parse_quote!(ani::conversions::NativePointer<crate::NativeResource>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::NativePointer(inner) = ani_type {
            assert_eq!(quote!(#inner).to_string(), "crate :: NativeResource");
        } else {
            panic!("Expected NativePointer type");
        }
    }

    #[test]
    fn test_parse_managed_resource() {
        let ty: Type = syn::parse_quote!(ani::conversions::ManagedResource<crate::NativeResource>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::NativePointer(inner) = ani_type {
            assert_eq!(quote!(#inner).to_string(), "crate :: NativeResource");
        } else {
            panic!("Expected managed resource to use the native handle ABI");
        }
    }

    #[test]
    fn test_parse_runtime_wrappers() {
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(ani::conversions::AnyValue)),
            AniType::AnyValue
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(FixedIntArray)),
            AniType::FixedArray(PrimitiveType::I32)
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(AniFixedArrayBoolean<'_>)),
            AniType::FixedArray(PrimitiveType::Bool)
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(GlobalRef)),
            AniType::GlobalRef
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(WeakRef)),
            AniType::WeakRef
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(ani::conversions::TupleValue)),
            AniType::TupleValue
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(ani::conversions::EnumItem)),
            AniType::EnumItem
        ));
    }

    #[test]
    fn test_parse_map() {
        let ty: Type = syn::parse_quote!(BTreeMap<String, i32>);
        let ani_type = AniType::from_syn_type(&ty);
        if let AniType::Map(map) = ani_type {
            assert!(matches!(
                map.key.as_ref(),
                AniType::String(StringType::String)
            ));
            assert!(matches!(
                map.value.as_ref(),
                AniType::Primitive(PrimitiveType::I32)
            ));
        } else {
            panic!("Expected Map type");
        }
    }

    #[test]
    fn test_object_containers_preserve_custom_inner_types() {
        let record_ty: Type = syn::parse_quote!(HashMap<String, crate::models::UserInfo>);
        let record = AniType::from_syn_type(&record_ty);
        if let AniType::Record(record) = record {
            assert!(matches!(record.value.as_ref(), AniType::CustomObject(_)));
        } else {
            panic!("Expected Record type");
        }

        let set_ty: Type = syn::parse_quote!(HashSet<crate::models::UserInfo>);
        let set = AniType::from_syn_type(&set_ty);
        if let AniType::Set(set) = set {
            assert!(matches!(set.element.as_ref(), AniType::CustomObject(_)));
        } else {
            panic!("Expected Set type");
        }

        let btree_set_ty: Type = syn::parse_quote!(BTreeSet<crate::models::UserInfo>);
        let btree_set = AniType::from_syn_type(&btree_set_ty);
        if let AniType::Set(set) = btree_set {
            assert!(matches!(set.element.as_ref(), AniType::CustomObject(_)));
        } else {
            panic!("Expected Set type");
        }

        let map_ty: Type = syn::parse_quote!(BTreeMap<String, crate::models::UserInfo>);
        let map = AniType::from_syn_type(&map_ty);
        if let AniType::Map(map) = map {
            assert!(matches!(
                map.key.as_ref(),
                AniType::String(StringType::String)
            ));
            assert!(matches!(map.value.as_ref(), AniType::CustomObject(_)));
        } else {
            panic!("Expected Map type");
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
        let _guard = TestEnvVarGuard::unset("ANI_TEST_MODULE_NAME");
        let ty: Type = syn::parse_quote!(crate::models::UserInfo);
        let ani_type = AniType::from_syn_type(&ty);
        assert!(matches!(ani_type, AniType::CustomObject(_)));
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");
    }

    #[test]
    fn test_unknown_custom_type_signature_local_type_is_module_qualified() {
        let _guard = TestEnvVarGuard::unset("ANI_TEST_MODULE_NAME");
        let ty: Type = syn::parse_quote!(UserProfile);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/UserProfile;");
    }

    #[test]
    fn test_unknown_custom_type_signature_uses_registered_object_alias() {
        let _guard = TestEnvVarGuard::unset("ANI_TEST_MODULE_NAME");
        register_object_type_alias("AliasedProfile", "models.AliasedProfile");
        let ty: Type = syn::parse_quote!(AliasedProfile);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(
            ani_type.to_signature(),
            "Lani_derive/models/AliasedProfile;"
        );
    }

    #[test]
    fn test_transparent_wrapper_custom_type_signature() {
        let _guard = TestEnvVarGuard::unset("ANI_TEST_MODULE_NAME");
        let ty: Type = syn::parse_quote!(Box<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::sync::Arc<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");
    }

    #[test]
    fn test_reference_and_cow_surface_types_preserve_inner_signature() {
        let _guard = TestEnvVarGuard::unset("ANI_TEST_MODULE_NAME");
        let ty: Type = syn::parse_quote!(&crate::models::UserInfo);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::borrow::Cow<'static, str>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;");
    }

    #[test]
    fn test_sync_and_cell_wrappers_preserve_inner_signature() {
        let _guard = TestEnvVarGuard::unset("ANI_TEST_MODULE_NAME");
        let ty: Type = syn::parse_quote!(std::sync::Mutex<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::sync::RwLock<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::cell::RefCell<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::cell::UnsafeCell<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::mem::ManuallyDrop<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::mem::MaybeUninit<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::sync::OnceLock<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");

        let ty: Type = syn::parse_quote!(std::sync::LazyLock<crate::models::UserInfo>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lani_derive/models/UserInfo;");
    }

    #[test]
    fn test_parse_raw_array_handle_types() {
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(AniArray<'_>)),
            AniType::ArrayHandle(ArrayHandleType::Array)
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(AniArrayRef<'_>)),
            AniType::ArrayHandle(ArrayHandleType::ArrayRef)
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(AniFixedArray<'_>)),
            AniType::ArrayHandle(ArrayHandleType::FixedArray)
        ));
        assert!(matches!(
            AniType::from_syn_type(&syn::parse_quote!(AniFixedArrayRef<'_>)),
            AniType::ArrayHandle(ArrayHandleType::FixedArrayRef)
        ));
    }

    #[test]
    fn test_raw_array_handle_signatures() {
        let array = AniType::from_syn_type(&syn::parse_quote!(AniArray<'_>));
        assert_eq!(array.to_signature(), "A{C{std.core.Object}}");

        let fixed = AniType::from_syn_type(&syn::parse_quote!(AniFixedArray<'_>));
        assert_eq!(fixed.to_signature(), "A{C{std.core.Object}}");

        let fixed_ref = AniType::from_syn_type(&syn::parse_quote!(AniFixedArrayRef<'_>));
        assert_eq!(fixed_ref.to_signature(), "A{C{std.core.Object}}");
    }

    #[test]
    fn test_parse_runtime_handle_types() {
        let ty: Type = syn::parse_quote!(AniRef<'_>);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Ref)
        ));

        let ty: Type = syn::parse_quote!(AniClass<'_>);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Class)
        ));

        let ty: Type = syn::parse_quote!(AniType<'_>);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Type)
        ));

        let ty: Type = syn::parse_quote!(AniString<'_>);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::String)
        ));

        let ty: Type = syn::parse_quote!(AniEnum<'_>);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Enum)
        ));

        let ty: Type = syn::parse_quote!(AniError<'_>);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Error)
        ));

        let ty: Type = syn::parse_quote!(AniMethod);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Method)
        ));

        let ty: Type = syn::parse_quote!(AniResolver);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::Resolver)
        ));

        let ty: Type = syn::parse_quote!(AniStaticField);
        assert!(matches!(
            AniType::from_syn_type(&ty),
            AniType::RuntimeHandle(RuntimeHandleType::StaticField)
        ));
    }

    #[test]
    fn test_runtime_handle_c_type_generation() {
        let ty: Type = syn::parse_quote!(AniRef<'_>);
        assert_eq!(
            AniType::from_syn_type(&ty).to_ani_c_type().to_string(),
            quote!(ani::sys::ani_ref).to_string()
        );

        let ty: Type = syn::parse_quote!(AniClass<'_>);
        assert_eq!(
            AniType::from_syn_type(&ty).to_ani_c_type().to_string(),
            quote!(ani::sys::ani_class).to_string()
        );

        let ty: Type = syn::parse_quote!(AniMethod);
        assert_eq!(
            AniType::from_syn_type(&ty).to_ani_c_type().to_string(),
            quote!(ani::sys::ani_method).to_string()
        );

        let ty: Type = syn::parse_quote!(AniError<'_>);
        assert_eq!(
            AniType::from_syn_type(&ty).to_ani_c_type().to_string(),
            quote!(ani::sys::ani_error).to_string()
        );

        let ty: Type = syn::parse_quote!(AniVariable);
        assert_eq!(
            AniType::from_syn_type(&ty).to_ani_c_type().to_string(),
            quote!(ani::sys::ani_variable).to_string()
        );
    }

    #[test]
    fn test_unknown_known_ani_wrapper_signature() {
        let ty: Type = syn::parse_quote!(AniString<'_>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/String;");

        let ty: Type = syn::parse_quote!(AniClass<'_>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Class;");

        let ty: Type = syn::parse_quote!(AniMethod);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Object;");

        let ty: Type = syn::parse_quote!(AniError<'_>);
        let ani_type = AniType::from_syn_type(&ty);
        assert_eq!(ani_type.to_signature(), "Lstd/core/Object;");
    }
}
