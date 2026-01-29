//! Signature Generation
//!
//! Generates ANI type signatures from Rust types.

use quote::quote;
use syn::{FnArg, ReturnType, Signature, Type};

use crate::codegen::should_skip_in_signature;

/// Generate ANI signature from Rust type
pub fn rust_type_to_signature(ty: &Type) -> String {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        // Primitive types
        "bool" => "Z".to_string(),
        "i8" | "u8" => "B".to_string(),
        "i16" => "S".to_string(),
        "u16" | "char" => "C".to_string(),
        "i32" | "u32" => "I".to_string(),
        "i64" | "u64" => "J".to_string(),
        "f32" => "F".to_string(),
        "f64" => "D".to_string(),
        "()" => "V".to_string(),

        // String types
        "String" | "&str" | "&String" => "Lstd/core/String;".to_string(),

        // Complex types
        _ => resolve_complex_type_signature(ty, &type_str),
    }
}

/// Resolve signature for complex types (Option, Vec, Result, Either, PromiseRaw, Function, etc.)
fn resolve_complex_type_signature(ty: &Type, type_str: &str) -> String {
    // Option<T>
    if type_str.starts_with("Option<") {
        return resolve_option_signature(ty);
    }

    // Vec<T>
    if type_str.starts_with("Vec<") {
        return resolve_vec_signature(ty);
    }

    // Result<T, E>
    if type_str.starts_with("Result<") {
        return resolve_result_signature(ty);
    }

    // PromiseRaw - returns Promise object
    if type_str.starts_with("PromiseRaw") {
        return "Lstd/core/Object;".to_string();
    }

    // Function<Args, Return> - callback function type
    if type_str.starts_with("Function<") {
        return "Lstd/core/Function;".to_string();
    }

    // FunctionRef<Args, Return> - stored function reference
    if type_str.starts_with("FunctionRef<") {
        return "Lstd/core/Function;".to_string();
    }

    // FnArgs<T> - wrapper for multiple arguments (used inside Function<>)
    if type_str.starts_with("FnArgs<") {
        return "Lstd/core/Object;".to_string();
    }

    // Ref<T> - typed global reference (resolves to inner type signature)
    if type_str.starts_with("Ref<") {
        return resolve_ref_signature(ty);
    }

    // Either types (union types in ArkTS)
    if is_either_type(type_str) {
        return "Lstd/core/Object;".to_string();
    }

    // Default: Object type
    "Lstd/core/Object;".to_string()
}

/// All known Either type names (Either, Either3-Either26)
const EITHER_TYPES: &[&str] = &[
    "Either", "Either3", "Either4", "Either5", "Either6", "Either7", "Either8", "Either9",
    "Either10", "Either11", "Either12", "Either13", "Either14", "Either15", "Either16", "Either17",
    "Either18", "Either19", "Either20", "Either21", "Either22", "Either23", "Either24", "Either25",
    "Either26",
];

/// Check if type is an Either variant (Either, Either3-Either16)
pub fn is_either_type(type_str: &str) -> bool {
    // Extract type name before '<'
    let type_name = type_str.split('<').next().unwrap_or("");
    EITHER_TYPES.contains(&type_name)
}

/// Resolve Option<T> signature
fn resolve_option_signature(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                    return get_boxed_signature(inner);
                }
            }
        }
    }
    "Lstd/core/Object;".to_string()
}

/// Resolve Vec<T> signature
fn resolve_vec_signature(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                    let inner_sig = rust_type_to_signature(inner);
                    return format!("[{}", inner_sig);
                }
            }
        }
    }
    "[Lstd/core/Object;".to_string()
}

/// Resolve Result<T, E> signature (uses Ok type signature)
fn resolve_result_signature(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(ok_type)) = args.args.first() {
                    return rust_type_to_signature(ok_type);
                }
            }
        }
    }
    "V".to_string()
}

/// Resolve Ref<T> signature (uses inner type signature)
fn resolve_ref_signature(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                    // For Ref<AniObject<'static>>, we just return Object signature
                    let inner_str = quote!(#inner_type).to_string().replace(" ", "");
                    if inner_str.starts_with("AniObject") {
                        return "Lstd/core/Object;".to_string();
                    }
                    // For other inner types, try to resolve
                    return rust_type_to_signature(inner_type);
                }
            }
        }
    }
    "Lstd/core/Object;".to_string()
}

/// Get boxed type signature (for Option inner types)
fn get_boxed_signature(ty: &Type) -> String {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        "bool" => "Lstd/core/Boolean;".to_string(),
        "i8" => "Lstd/core/Byte;".to_string(),
        "i16" => "Lstd/core/Short;".to_string(),
        "i32" => "Lstd/core/Int;".to_string(),
        "i64" => "Lstd/core/Long;".to_string(),
        "f32" => "Lstd/core/Float;".to_string(),
        "f64" => "Lstd/core/Double;".to_string(),
        _ => rust_type_to_signature(ty),
    }
}

// ============================================================================
// Function Signature Generation
// ============================================================================

/// Generate ANI signature from function signature
///
/// This function automatically skips injected parameters (Env, This, Class)
/// since they are not part of the ArkTS function signature.
pub fn generate_fn_signature(sig: &Signature, skip_first: bool) -> String {
    generate_signature_from_inputs(&sig.inputs, &sig.output, skip_first)
}

/// Generate signature from parameter list
fn generate_signature_from_inputs(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
    output: &ReturnType,
    skip_first: bool,
) -> String {
    let mut sig = String::new();

    // Filter out:
    // 1. First parameter if skip_first is true (for class methods)
    // 2. All injected parameters (Env, This, Class)
    let params: Vec<_> = inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .collect();

    for input in params {
        if let FnArg::Typed(pat_type) = input {
            sig.push_str(&rust_type_to_signature(&pat_type.ty));
        }
    }

    sig.push(':');

    match output {
        ReturnType::Default => sig.push('V'),
        ReturnType::Type(_, ty) => {
            sig.push_str(&rust_type_to_signature(ty));
        }
    }

    sig
}

// ============================================================================
// Descriptor Conversion
// ============================================================================

/// Convert class name to ANI descriptor
/// Example: "MyModule.MyClass" -> "LMyModule/MyClass;"
pub fn class_to_descriptor(name: &str) -> String {
    let path = name.replace('.', "/");
    format!("L{};", path)
}

/// Convert namespace to ANI descriptor
pub fn namespace_to_descriptor(name: &str) -> String {
    let path = name.replace('.', "/");
    format!("L{};", path)
}

/// Convert module to ANI descriptor
pub fn module_to_descriptor(name: &str) -> String {
    format!("L{};", name)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_descriptor() {
        assert_eq!(class_to_descriptor("hello.Foo"), "Lhello/Foo;");
        assert_eq!(class_to_descriptor("std.core.String"), "Lstd/core/String;");
    }

    #[test]
    fn test_rust_type_to_signature() {
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(i32)), "I");
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(f64)), "D");
        assert_eq!(rust_type_to_signature(&syn::parse_quote!(bool)), "Z");
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(String)),
            "Lstd/core/String;"
        );
    }

    #[test]
    fn test_function_type_signature() {
        // Function<Args, Return> types
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<(), ()>)),
            "Lstd/core/Function;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<(i32,), String>)),
            "Lstd/core/Function;"
        );
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(Function<FnArgs<(i32, i32)>, i32>)),
            "Lstd/core/Function;"
        );

        // FunctionRef types
        assert_eq!(
            rust_type_to_signature(&syn::parse_quote!(FunctionRef<(i32,), String>)),
            "Lstd/core/Function;"
        );
    }
}
