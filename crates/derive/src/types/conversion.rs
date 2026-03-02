//! Type Conversion Code Generation
//!
//! Generates code for converting between Rust and ANI types.
//! Uses the structured AniType system instead of string-based pattern matching.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{FnArg, Pat, ReturnType, Type};

use super::ani_type::{AniType, FunctionType, PrimitiveType, StringType, WrapperType};

// ============================================================================
// ANI Type Mapping
// ============================================================================

/// Map Rust type to ANI C type
pub fn rust_type_to_ani_type(ty: &Type) -> TokenStream {
    let ani_type = AniType::from_syn_type(ty);
    ani_type.to_ani_c_type()
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
    let ani_type = AniType::from_syn_type(ty);

    generate_type_conversion(&param_name, &converted_name, ty, &ani_type)
}

/// Extract parameter name from pattern
fn extract_param_name(pat: &Pat, index: usize) -> Ident {
    if let Pat::Ident(pat_ident) = pat {
        pat_ident.ident.clone()
    } else {
        format_ident!("arg{}", index)
    }
}

/// Generate type-specific conversion code using AniType
fn generate_type_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    ani_type: &AniType,
) -> TokenStream {
    match ani_type {
        // Primitives - direct pass-through (except bool)
        AniType::Primitive(p) => generate_primitive_conversion(param_name, converted_name, p),

        // String types
        AniType::String(s) => generate_string_type_conversion(param_name, converted_name, s),

        // AniObject and handle-like types - use FromAni for strong typing
        AniType::AniObject => generate_generic_from_ani_conversion(param_name, converted_name, ty),

        // Option<T>
        AniType::Wrapper(WrapperType::Option(inner)) => {
            generate_option_conversion(param_name, converted_name, inner.as_ref())
        }

        // Either types
        AniType::Either(_) => generate_either_conversion(param_name, converted_name, ty),

        // Function types
        AniType::Function(func_type) => {
            generate_function_type_conversion(param_name, converted_name, ty, func_type)
        }

        // Ref<T> types
        AniType::Wrapper(WrapperType::Ref(_)) => {
            generate_ref_conversion(param_name, converted_name, ty)
        }

        // ArrayBuffer / ArrayBufferSlice - from_ani(ani_arraybuffer)
        AniType::ArrayBuffer => {
            generate_arraybuffer_param_conversion(param_name, converted_name, ty)
        }

        // Record<string, V> - use FromAni on typed Rust container (e.g. HashMap<String, V>)
        AniType::Record(_) => generate_generic_from_ani_conversion(param_name, converted_name, ty),

        // Unknown/custom types - fallback to FromAni
        AniType::Unknown(_) => generate_generic_from_ani_conversion(param_name, converted_name, ty),

        // Default - pass-through
        _ => quote! { let #converted_name = #param_name; },
    }
}

/// Generate primitive type conversion
fn generate_primitive_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    primitive: &PrimitiveType,
) -> TokenStream {
    match primitive {
        PrimitiveType::Bool => {
            quote! { let #converted_name = #param_name != 0; }
        }
        // All other primitives are direct pass-through
        _ => quote! { let #converted_name = #param_name; },
    }
}

/// Generate String/&str conversion code
fn generate_string_type_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    string_type: &StringType,
) -> TokenStream {
    match string_type {
        StringType::String => quote! {
            let #converted_name = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                let ani_str = ani::types::AniString::from_raw(#param_name);
                env_wrapper.get_string(&ani_str).unwrap_or_default()
            };
        },
        StringType::Str => quote! {
            let #converted_name = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                let ani_str = ani::types::AniString::from_raw(#param_name);
                env_wrapper.get_string(&ani_str).unwrap_or_default()
            };
            let #converted_name = #converted_name.as_str();
        },
    }
}

/// Generate Option<T> conversion code
fn generate_option_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    inner_type: &AniType,
) -> TokenStream {
    match inner_type {
        AniType::String(StringType::String) => quote! {
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
        AniType::Primitive(p) => generate_option_unbox_conversion(param_name, converted_name, p),
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
    primitive: &PrimitiveType,
) -> TokenStream {
    let rust_type_str = primitive.rust_type_str();
    let rust_type: syn::Type =
        syn::parse_str(rust_type_str).unwrap_or_else(|_| syn::parse_quote!(i32));

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

/// Generate Function/FunctionRef type conversion code
fn generate_function_type_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    func_type: &FunctionType,
) -> TokenStream {
    match func_type {
        FunctionType::Function { .. } => quote! {
            let #converted_name: #ty = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                ani::conversions::FromAni::from_ani(&env_wrapper, #param_name as ani::sys::ani_fn_object)
                    .expect("Failed to convert Function type")
            };
        },
        FunctionType::FunctionRef { .. } => quote! {
            let #converted_name: #ty = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                ani::conversions::FromAni::from_ani(&env_wrapper, #param_name as ani::sys::ani_fn_object)
                    .expect("Failed to convert FunctionRef type")
            };
        },
    }
}

/// Generate Ref<T> type conversion code
fn generate_ref_conversion(param_name: &Ident, converted_name: &Ident, ty: &Type) -> TokenStream {
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            ani::conversions::FromAni::from_ani(&env_wrapper, #param_name as ani::sys::ani_object)
                .expect("Failed to convert Ref type")
        };
    }
}

/// Generate ArrayBuffer / ArrayBufferSlice parameter conversion (ani_arraybuffer -> Rust)
fn generate_arraybuffer_param_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
) -> TokenStream {
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            ani::conversions::FromAni::from_ani(&env_wrapper, #param_name as ani::sys::ani_arraybuffer)
                .expect("Failed to convert ArrayBuffer type")
        };
    }
}

/// Generate generic parameter conversion using FromAni::from_ani.
fn generate_generic_from_ani_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
) -> TokenStream {
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            ani::conversions::FromAni::from_ani(
                &env_wrapper,
                #param_name as <#ty as ani::conversions::FromAni<'_>>::Input,
            )
                .expect("Failed to convert ANI value")
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
            let ani_type = AniType::from_syn_type(ty);
            generate_return_conversion_for_type(&ani_type, ty)
        }
    }
}

/// Generate return conversion based on AniType
fn generate_return_conversion_for_type(ani_type: &AniType, original_ty: &Type) -> TokenStream {
    match ani_type {
        // Result<PromiseRaw, E> - special handling
        AniType::Wrapper(WrapperType::Result(inner))
            if matches!(inner.as_ref(), AniType::Promise(_)) =>
        {
            generate_result_promise_return_conversion()
        }

        // Result<T, E>
        AniType::Wrapper(WrapperType::Result(inner)) => {
            generate_result_return_conversion(inner.as_ref())
        }

        // Simple types
        _ => generate_simple_return_conversion(ani_type, original_ty),
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
fn generate_simple_return_conversion(ani_type: &AniType, original_ty: &Type) -> TokenStream {
    match ani_type {
        // Numeric primitives - direct return
        AniType::Primitive(p) => match p {
            PrimitiveType::Bool => quote! { if result { 1 } else { 0 } },
            _ => quote! { result },
        },

        // String - convert to ani_string
        AniType::String(StringType::String) => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match env_wrapper.create_string(&result) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut()
            }
        },

        // Unit type - no return
        AniType::Unit => quote! {},

        // ArrayBuffer / Vec<u8> - use ToAni (Rust -> ani_arraybuffer)
        AniType::ArrayBuffer => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(result, &env_wrapper) {
                Ok(ab) => ab,
                Err(_) => std::ptr::null_mut()
            }
        },

        // Either types - use ToAni
        AniType::Either(_) => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(result, &env_wrapper) {
                Ok(obj) => obj,
                Err(_) => std::ptr::null_mut()
            }
        },

        // Record<string, V> - ToAni returns AniObject, convert to raw pointer.
        AniType::Record(_) => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(result, &env_wrapper) {
                Ok(obj) => obj.into_raw(),
                Err(_) => std::ptr::null_mut()
            }
        },

        // PromiseRaw - extract raw
        AniType::Promise(_) => quote! {
            result.into_raw()
        },

        // Default - direct return
        _ => {
            // Check if it's a custom object type that might use ToAni
            let ty_str = quote::quote!(#original_ty).to_string();
            // Skip types that look like raw pointers or ANI sys types
            if ty_str.contains("ani_") || ty_str.contains("* mut") || ty_str.contains("*mut") {
                quote! { result }
            } else if ty_str.contains("::")
                || ty_str
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
            {
                // Likely a custom type, try ToAni
                quote! {
                    let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                    match ani::conversions::ToAni::to_ani(result, &env_wrapper) {
                        Ok(obj) => obj,
                        Err(_) => std::ptr::null_mut()
                    }
                }
            } else {
                quote! { result }
            }
        }
    }
}

/// Generate return conversion for Result<T, E>
fn generate_result_return_conversion(ok_type: &AniType) -> TokenStream {
    let ok_conversion = match ok_type {
        AniType::String(StringType::String) => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match env_wrapper.create_string(&val) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut()
            }
        },
        AniType::Primitive(PrimitiveType::Bool) => quote! { if val { 1 } else { 0 } },
        AniType::Unit => quote! {},
        AniType::ArrayBuffer => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(val, &env_wrapper) {
                Ok(ab) => ab,
                Err(_) => std::ptr::null_mut()
            }
        },
        AniType::Record(_) => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(val, &env_wrapper) {
                Ok(obj) => obj.into_raw(),
                Err(_) => std::ptr::null_mut()
            }
        },
        _ => quote! {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::ToAni::to_ani(val, &env_wrapper) {
                Ok(v) => v,
                Err(_) => Default::default()
            }
        },
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
