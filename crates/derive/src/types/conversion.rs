//! Type Conversion Code Generation
//!
//! Generates code for converting between Rust and ANI types.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{FnArg, Pat, ReturnType, Type};

use super::signature::is_either_type;

// ============================================================================
// ANI Type Mapping
// ============================================================================

/// Map Rust type to ANI C type
pub fn rust_type_to_ani_type(ty: &Type) -> TokenStream {
    let type_str = quote!(#ty).to_string().replace(" ", "");

    match type_str.as_str() {
        "bool" => quote! { ani::sys::ani_boolean },
        "i8" | "u8" => quote! { ani::sys::ani_byte },
        "i16" => quote! { ani::sys::ani_short },
        "u16" | "char" => quote! { ani::sys::ani_char },
        "i32" | "u32" => quote! { ani::sys::ani_int },
        "i64" | "u64" => quote! { ani::sys::ani_long },
        "f32" => quote! { ani::sys::ani_float },
        "f64" => quote! { ani::sys::ani_double },
        "String" | "&str" => quote! { ani::sys::ani_string },
        "()" => quote! { () },
        _ => resolve_complex_ani_type(ty, &type_str),
    }
}

/// Resolve ANI type for complex Rust types
fn resolve_complex_ani_type(ty: &Type, type_str: &str) -> TokenStream {
    if type_str.starts_with("Vec<") {
        return quote! { ani::sys::ani_array };
    }

    if type_str.starts_with("Option<") {
        let inner_type_str = extract_generic_inner(type_str, "Option");
        return match inner_type_str.as_str() {
            "String" => quote! { ani::sys::ani_string },
            _ => quote! { ani::sys::ani_object },
        };
    }

    if type_str.starts_with("Result<") {
        if let Type::Path(type_path) = ty {
            if let Some(segment) = type_path.path.segments.last() {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(ok_type)) = args.args.first() {
                        return rust_type_to_ani_type(ok_type);
                    }
                }
            }
        }
        return quote! { ani::sys::ani_object };
    }

    if is_either_type(type_str) {
        return quote! { ani::sys::ani_object };
    }

    // PromiseRaw returns ani_object
    if type_str.starts_with("PromiseRaw") {
        return quote! { ani::sys::ani_object };
    }

    quote! { ani::sys::ani_object }
}

// ============================================================================
// Parameter Conversion
// ============================================================================

/// Generate parameter conversion code for function arguments
pub fn generate_param_conversions(params: &[&FnArg]) -> TokenStream {
    let conversions: Vec<TokenStream> = params
        .iter()
        .enumerate()
        .map(|(i, param)| generate_single_param_conversion(i, param))
        .collect();

    quote! { #(#conversions)* }
}

/// Generate conversion code for a single parameter
fn generate_single_param_conversion(index: usize, param: &FnArg) -> TokenStream {
    let FnArg::Typed(pat_type) = param else {
        return quote! {};
    };

    let param_name = extract_param_name(&pat_type.pat, index);
    let converted_name = format_ident!("{}_converted", param_name);
    let ty = &pat_type.ty;
    let type_str = quote!(#ty).to_string().replace(" ", "");

    generate_type_conversion(&param_name, &converted_name, ty, &type_str)
}

/// Extract parameter name from pattern
fn extract_param_name(pat: &Pat, index: usize) -> Ident {
    if let Pat::Ident(pat_ident) = pat {
        pat_ident.ident.clone()
    } else {
        format_ident!("arg{}", index)
    }
}

/// Generate type-specific conversion code
fn generate_type_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    type_str: &str,
) -> TokenStream {
    match type_str {
        // Primitives - direct pass-through
        "i32" | "i8" | "i16" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => {
            quote! { let #converted_name = #param_name; }
        }
        // Boolean - convert from ani_boolean
        "bool" => {
            quote! { let #converted_name = #param_name != 0; }
        }
        // String - convert from ani_string
        "String" => generate_string_conversion(param_name, converted_name),
        "&str" => generate_str_conversion(param_name, converted_name),
        // Option<T>
        _ if type_str.starts_with("Option<") => {
            let inner = extract_generic_inner(type_str, "Option");
            generate_option_conversion(param_name, converted_name, &inner)
        }
        // Either types
        _ if is_either_type(type_str) => generate_either_conversion(param_name, converted_name, ty),
        // Default - pass-through
        _ => quote! { let #converted_name = #param_name; },
    }
}

/// Generate String conversion code
fn generate_string_conversion(param_name: &Ident, converted_name: &Ident) -> TokenStream {
    quote! {
        let #converted_name = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            let ani_str = ani::types::AniString::from_raw(#param_name);
            env_wrapper.get_string(&ani_str).unwrap_or_default()
        };
    }
}

/// Generate &str conversion code
fn generate_str_conversion(param_name: &Ident, converted_name: &Ident) -> TokenStream {
    quote! {
        let #converted_name = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            let ani_str = ani::types::AniString::from_raw(#param_name);
            env_wrapper.get_string(&ani_str).unwrap_or_default()
        };
        let #converted_name = #converted_name.as_str();
    }
}

/// Generate Option<T> conversion code
fn generate_option_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    inner_type: &str,
) -> TokenStream {
    match inner_type {
        "String" => quote! {
            let #converted_name: Option<String> = {
                if #param_name.is_null() {
                    None
                } else {
                    let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                    let ani_str = ani::types::AniString::from_raw(#param_name);
                    env_wrapper.get_string(&ani_str).ok()
                }
            };
        },
        "i32" | "i64" | "i8" | "i16" | "u16" | "f32" | "f64" | "bool" => {
            generate_option_unbox_conversion(param_name, converted_name, inner_type)
        }
        _ => quote! {
            let #converted_name = {
                if #param_name.is_null() {
                    None
                } else {
                    Some(#param_name)
                }
            };
        },
    }
}

/// Generate Option<T> unbox conversion for primitive types
fn generate_option_unbox_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    inner_type: &str,
) -> TokenStream {
    let rust_type: syn::Type =
        syn::parse_str(inner_type).unwrap_or_else(|_| syn::parse_quote!(i32));

    quote! {
        let #converted_name: Option<#rust_type> = {
            if #param_name.is_null() {
                None
            } else {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                let obj = ani::types::AniObject::from_raw(#param_name);
                ani::conversions::Unboxable::unbox(&env_wrapper, &obj).ok()
            }
        };
    }
}

/// Generate Either type conversion code
fn generate_either_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
) -> TokenStream {
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            ani::conversions::FromAni::from_ani(&env_wrapper, #param_name)
                .expect("Failed to convert Either type")
        };
    }
}

// ============================================================================
// Return Value Conversion
// ============================================================================

/// Generate return value conversion code
pub fn generate_return_conversion(return_type: &ReturnType) -> TokenStream {
    match return_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => {
            let type_str = quote!(#ty).to_string().replace(" ", "");

            // Handle Result<PromiseRaw, E> specially
            if is_result_promise_type(&type_str) {
                return generate_result_promise_return_conversion();
            }

            // Handle Result<T, E>
            if type_str.starts_with("Result<") {
                return generate_result_return_conversion(ty);
            }

            generate_simple_return_conversion(&type_str)
        }
    }
}

/// Generate return conversion for Result<PromiseRaw, E>
fn generate_result_promise_return_conversion() -> TokenStream {
    quote! {
        match result {
            Ok(promise) => promise.into_raw(),
            Err(e) => {
                let biz_err: ::ani::error::BusinessError = e.into();
                unsafe { biz_err.throw_into(env) };
                std::ptr::null_mut()
            }
        }
    }
}

/// Generate return conversion for simple types
fn generate_simple_return_conversion(type_str: &str) -> TokenStream {
    match type_str {
        "i32" | "i64" | "i8" | "i16" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => {
            quote! { result }
        }
        "bool" => quote! { if result { 1 } else { 0 } },
        "String" => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match env_wrapper.create_string(&result) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut()
            }
        },
        "()" => quote! {},
        _ if is_either_type(type_str) => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(result, &env_wrapper) {
                Ok(obj) => obj,
                Err(_) => std::ptr::null_mut()
            }
        },
        // PromiseRaw - extract raw ani_object
        _ if type_str.starts_with("PromiseRaw") => quote! {
            result.into_raw()
        },
        _ => quote! { result },
    }
}

/// Check if type is a Result containing PromiseRaw
fn is_result_promise_type(type_str: &str) -> bool {
    type_str.starts_with("Result<PromiseRaw")
}

/// Generate return conversion for Result<T, E>
fn generate_result_return_conversion(ty: &Type) -> TokenStream {
    let inner_type_str = extract_result_ok_type(ty);

    let ok_conversion = match inner_type_str.as_str() {
        "String" => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match env_wrapper.create_string(&val) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut()
            }
        },
        "bool" => quote! { if val { 1 } else { 0 } },
        "()" => quote! {},
        _ => quote! { val },
    };

    quote! {
        match result {
            Ok(val) => { #ok_conversion },
            Err(e) => {
                let biz_err: ::ani::error::BusinessError = e.into();
                unsafe { biz_err.throw_into(env) };
                Default::default()
            }
        }
    }
}

/// Extract Ok type from Result<T, E>
fn extract_result_ok_type(ty: &Type) -> String {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(ok_type)) = args.args.first() {
                    return quote!(#ok_type).to_string().replace(" ", "");
                }
            }
        }
    }
    String::new()
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Extract inner type from generic type string
/// e.g., "Option<i32>" -> "i32"
fn extract_generic_inner(type_str: &str, wrapper: &str) -> String {
    let prefix = format!("{}<", wrapper);
    if type_str.starts_with(&prefix) && type_str.ends_with('>') {
        type_str[prefix.len()..type_str.len() - 1].to_string()
    } else {
        type_str.to_string()
    }
}
