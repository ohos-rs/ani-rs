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
pub fn generate_param_conversions(params: &[&FnArg], on_error_return: &TokenStream) -> TokenStream {
    let conversions: Vec<TokenStream> = params
        .iter()
        .enumerate()
        .map(|(i, param)| generate_single_param_conversion(i, param, on_error_return))
        .collect();

    quote! { #(#conversions)* }
}

/// Generate conversion code for a single parameter
fn generate_single_param_conversion(
    index: usize,
    param: &FnArg,
    on_error_return: &TokenStream,
) -> TokenStream {
    let FnArg::Typed(pat_type) = param else {
        return quote! {};
    };

    let param_name = extract_param_name(&pat_type.pat, index);
    let converted_name = format_ident!("{}_converted", param_name);
    let ty = &pat_type.ty;
    let ani_type = AniType::from_syn_type(ty);

    generate_type_conversion(&param_name, &converted_name, ty, &ani_type, on_error_return)
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
    on_error_return: &TokenStream,
) -> TokenStream {
    match ani_type {
        // Primitives - direct pass-through (except bool)
        AniType::Primitive(p) => generate_primitive_conversion(param_name, converted_name, p),

        // String types
        AniType::String(s) => {
            generate_string_type_conversion(param_name, converted_name, s, on_error_return)
        }

        // AniObject and nullish singleton handle-like types - use FromAni for strong typing
        AniType::AniObject | AniType::Null | AniType::Undefined => {
            generate_generic_from_ani_conversion(param_name, converted_name, ty, on_error_return)
        }

        // Option<T> - delegate to the typed runtime conversion instead of
        // synthesizing partially-typed nullable handling in the macro.
        AniType::Wrapper(WrapperType::Option(_)) => {
            generate_generic_from_ani_conversion(param_name, converted_name, ty, on_error_return)
        }

        // Either types
        AniType::Either(_) => {
            generate_either_conversion(param_name, converted_name, ty, on_error_return)
        }

        // Function types
        AniType::Function(func_type) => generate_function_type_conversion(
            param_name,
            converted_name,
            ty,
            func_type,
            on_error_return,
        ),

        // Ref<T> types
        AniType::Wrapper(WrapperType::Ref(_)) => {
            generate_ref_conversion(param_name, converted_name, ty, on_error_return)
        }

        // ArrayBuffer / ArrayBufferSlice - from_ani(ani_arraybuffer)
        AniType::ArrayBuffer => {
            generate_arraybuffer_param_conversion(param_name, converted_name, ty, on_error_return)
        }

        // Record/Set/Map containers - use FromAni on the typed Rust container.
        AniType::Record(_) | AniType::Set(_) | AniType::Map(_) => {
            generate_generic_from_ani_conversion(param_name, converted_name, ty, on_error_return)
        }

        // Unknown/custom types - fallback to FromAni
        AniType::Unknown(_) => {
            generate_generic_from_ani_conversion(param_name, converted_name, ty, on_error_return)
        }

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
        PrimitiveType::U8 => quote! { let #converted_name = #param_name as u8; },
        PrimitiveType::U16 => quote! { let #converted_name = #param_name as u16; },
        PrimitiveType::U32 => quote! { let #converted_name = #param_name as u32; },
        PrimitiveType::U64 => quote! { let #converted_name = #param_name as u64; },
        PrimitiveType::Char => quote! {
            let #converted_name = std::char::from_u32(#param_name as u32).unwrap_or('\0');
        },
        // Signed primitives are direct pass-through.
        _ => quote! { let #converted_name = #param_name; },
    }
}

/// Generate String/&str conversion code
fn generate_string_type_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    string_type: &StringType,
    on_error_return: &TokenStream,
) -> TokenStream {
    let on_error = generate_param_conversion_error(on_error_return);
    match string_type {
        StringType::String => quote! {
            let #converted_name = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                let ani_str = ani::types::AniString::from_raw(#param_name);
                match env_wrapper.get_string(&ani_str) {
                    Ok(value) => value,
                    Err(e) => { #on_error }
                }
            };
        },
        StringType::Str => quote! {
            let #converted_name = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                let ani_str = ani::types::AniString::from_raw(#param_name);
                match env_wrapper.get_string(&ani_str) {
                    Ok(value) => value,
                    Err(e) => { #on_error }
                }
            };
            let #converted_name = #converted_name.as_str();
        },
    }
}

/// Generate Either type conversion code
fn generate_either_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    on_error_return: &TokenStream,
) -> TokenStream {
    let on_error = generate_param_conversion_error(on_error_return);
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::FromAni::from_ani(&env_wrapper, #param_name) {
                Ok(v) => v,
                Err(e) => { #on_error }
            }
        };
    }
}

/// Generate Function/FunctionRef type conversion code
fn generate_function_type_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    func_type: &FunctionType,
    on_error_return: &TokenStream,
) -> TokenStream {
    let on_error = generate_param_conversion_error(on_error_return);
    match func_type {
        FunctionType::Function { .. } => quote! {
            let #converted_name: #ty = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                match ani::conversions::FromAni::from_ani(
                    &env_wrapper,
                    #param_name as ani::sys::ani_fn_object,
                ) {
                    Ok(v) => v,
                    Err(e) => { #on_error }
                }
            };
        },
        FunctionType::FunctionRef { .. } => quote! {
            let #converted_name: #ty = {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                match ani::conversions::FromAni::from_ani(
                    &env_wrapper,
                    #param_name as ani::sys::ani_fn_object,
                ) {
                    Ok(v) => v,
                    Err(e) => { #on_error }
                }
            };
        },
    }
}

/// Generate Ref<T> type conversion code
fn generate_ref_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    on_error_return: &TokenStream,
) -> TokenStream {
    let on_error = generate_param_conversion_error(on_error_return);
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::FromAni::from_ani(
                &env_wrapper,
                #param_name as ani::sys::ani_object,
            ) {
                Ok(v) => v,
                Err(e) => { #on_error }
            }
        };
    }
}

/// Generate ArrayBuffer / ArrayBufferSlice parameter conversion (ani_arraybuffer -> Rust)
fn generate_arraybuffer_param_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    on_error_return: &TokenStream,
) -> TokenStream {
    let on_error = generate_param_conversion_error(on_error_return);
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::FromAni::from_ani(
                &env_wrapper,
                #param_name as ani::sys::ani_arraybuffer,
            ) {
                Ok(v) => v,
                Err(e) => { #on_error }
            }
        };
    }
}

/// Generate generic parameter conversion using FromAni::from_ani.
fn generate_generic_from_ani_conversion(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    on_error_return: &TokenStream,
) -> TokenStream {
    let on_error = generate_param_conversion_error(on_error_return);
    quote! {
        let #converted_name: #ty = {
            let env_wrapper = ani::env::Env::from_raw_unchecked(env);
            match ani::conversions::FromAni::from_ani(
                &env_wrapper,
                #param_name as <#ty as ani::conversions::FromAni<'_>>::Input,
            ) {
                Ok(v) => v,
                Err(e) => { #on_error }
            }
        };
    }
}

fn generate_param_conversion_error(on_error_return: &TokenStream) -> TokenStream {
    quote! {
        let env_wrapper = ani::env::Env::from_raw_unchecked(env);
        let _ = ani::conversions::throw_error(&env_wrapper, &e.to_string());
        #on_error_return
    }
}

fn generate_return_conversion_error(on_error_return: TokenStream) -> TokenStream {
    quote! {
        let env_wrapper = ani::env::Env::from_raw_unchecked(env);
        let _ = ani::conversions::throw_error(&env_wrapper, &e.to_string());
        #on_error_return
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

/// Generate return conversion based on AniType.
fn generate_return_conversion_for_type(ani_type: &AniType, original_ty: &Type) -> TokenStream {
    match ani_type {
        AniType::Wrapper(WrapperType::Result(inner)) => {
            generate_result_return_conversion(inner.as_ref(), extract_result_ok_type(original_ty))
        }
        _ => generate_value_return_conversion(ani_type, quote! { result }, Some(original_ty)),
    }
}

fn extract_result_ok_type(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;
    if segment.ident != "Result" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };

    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(inner) => Some(inner),
        _ => None,
    })
}

fn is_direct_passthrough_type(ty: &Type) -> bool {
    let ty_str = quote::quote!(#ty).to_string();
    ty_str.contains("ani_") || ty_str.contains("* mut") || ty_str.contains("*mut")
}

fn looks_like_custom_object_type(ty: &Type) -> bool {
    let ty_str = quote::quote!(#ty).to_string();
    ty_str.contains("::")
        || ty_str
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
}

fn generate_to_ani_conversion(
    value_expr: TokenStream,
    on_error_return: TokenStream,
    into_raw_object: bool,
) -> TokenStream {
    if into_raw_object {
        quote! {
            {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                match ani::conversions::ToAni::to_ani(#value_expr, &env_wrapper) {
                    Ok(obj) => obj.into_raw(),
                    Err(e) => { #on_error_return }
                }
            }
        }
    } else {
        quote! {
            {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                match ani::conversions::ToAni::to_ani(#value_expr, &env_wrapper) {
                    Ok(v) => v,
                    Err(e) => { #on_error_return }
                }
            }
        }
    }
}

fn generate_value_return_conversion(
    ani_type: &AniType,
    value_expr: TokenStream,
    original_ty: Option<&Type>,
) -> TokenStream {
    let on_null_error = generate_return_conversion_error(quote! { std::ptr::null_mut() });

    match ani_type {
        AniType::Primitive(p) => match p {
            PrimitiveType::Bool => quote! { if #value_expr { 1 } else { 0 } },
            PrimitiveType::U8 => quote! { #value_expr as ani::sys::ani_byte },
            PrimitiveType::U32 => quote! { #value_expr as ani::sys::ani_int },
            PrimitiveType::U64 => quote! { #value_expr as ani::sys::ani_long },
            PrimitiveType::Char => quote! { (#value_expr as u32) as ani::sys::ani_char },
            _ => quote! { #value_expr },
        },
        AniType::String(StringType::String) => quote! {
            {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                match env_wrapper.create_string(&#value_expr) {
                    Ok(s) => s.into_raw(),
                    Err(e) => { #on_null_error }
                }
            }
        },
        AniType::Unit => quote! {},
        AniType::Promise(_) => quote! { #value_expr.into_raw() },
        AniType::Record(_) | AniType::Set(_) | AniType::Map(_) => {
            generate_to_ani_conversion(value_expr, on_null_error, true)
        }
        AniType::ArrayBuffer | AniType::Either(_) | AniType::Null | AniType::Undefined => {
            generate_to_ani_conversion(value_expr, on_null_error, false)
        }
        _ => {
            if let Some(original_ty) = original_ty {
                if is_direct_passthrough_type(original_ty) {
                    quote! { #value_expr }
                } else if looks_like_custom_object_type(original_ty) {
                    generate_to_ani_conversion(value_expr, on_null_error, false)
                } else {
                    quote! { #value_expr }
                }
            } else {
                generate_to_ani_conversion(value_expr, on_null_error, false)
            }
        }
    }
}

fn generate_result_error_fallback(ok_type: &AniType, original_ok_ty: Option<&Type>) -> TokenStream {
    match ok_type {
        AniType::Primitive(_) => quote! { Default::default() },
        AniType::Unit => quote! { return; },
        AniType::String(StringType::String)
        | AniType::Promise(_)
        | AniType::Record(_)
        | AniType::Set(_)
        | AniType::Map(_)
        | AniType::ArrayBuffer
        | AniType::Either(_)
        | AniType::Null
        | AniType::Undefined => quote! { std::ptr::null_mut() },
        _ => {
            if let Some(original_ok_ty) = original_ok_ty {
                if is_direct_passthrough_type(original_ok_ty)
                    || !looks_like_custom_object_type(original_ok_ty)
                {
                    quote! { Default::default() }
                } else {
                    quote! { std::ptr::null_mut() }
                }
            } else {
                quote! { std::ptr::null_mut() }
            }
        }
    }
}

/// Generate return conversion for Result<T, E> in the same local `match result`
/// style used by napi-rs, while still sharing the success-value conversion logic.
fn generate_result_return_conversion(
    ok_type: &AniType,
    original_ok_ty: Option<&Type>,
) -> TokenStream {
    let on_error_return =
        generate_return_conversion_error(generate_result_error_fallback(ok_type, original_ok_ty));
    let ok_conversion = generate_value_return_conversion(ok_type, quote! { val }, original_ok_ty);

    quote! {
        match result {
            Ok(val) => { #ok_conversion },
            Err(e) => {
                #on_error_return
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn param_conversion_either_uses_throw_not_expect() {
        let arg: FnArg = parse_quote!(value: Either<String, i32>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("throw_error"));
        assert!(!code.contains("expect"));
    }

    #[test]
    fn param_conversion_unknown_uses_throw_not_expect() {
        let arg: FnArg = parse_quote!(variable: AniVariable);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("throw_error"));
        assert!(!code.contains("expect"));
    }

    #[test]
    fn param_conversion_string_uses_throw_not_default() {
        let arg: FnArg = parse_quote!(name: String);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("throw_error"));
        assert!(!code.contains("unwrap_or_default"));
    }

    #[test]
    fn param_conversion_option_unbox_uses_throw_not_ok() {
        let arg: FnArg = parse_quote!(value: Option<i32>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("throw_error"));
        assert!(!code.contains(".ok()"));
    }

    #[test]
    fn param_conversion_option_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: Option<HashMap<String, i32>>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(
            code.contains("Option < HashMap < String , i32 > > as ani :: conversions :: FromAni")
        );
        assert!(code.contains("throw_error"));
    }

    #[test]
    fn return_conversion_string_throws_on_conversion_failure() {
        let output: ReturnType = parse_quote!(-> String);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("throw_error"));
        assert!(!code.contains("Err (_) => std :: ptr :: null_mut ()"));
    }

    #[test]
    fn result_return_conversion_throws_on_ok_conversion_failure() {
        let output: ReturnType = parse_quote!(-> Result<HashMap<String, i32>, ani::Error>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("throw_error"));
        assert!(!code.contains("Err (_) => Default :: default ()"));
    }

    #[test]
    fn set_return_conversion_uses_into_raw_object() {
        let output: ReturnType = parse_quote!(-> std::collections::HashSet<String>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("obj . into_raw ()"));
    }

    #[test]
    fn result_i64_return_conversion_uses_default_not_null_mut() {
        let output: ReturnType = parse_quote!(-> Result<i64, ani::Error>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("Default :: default ()"));
        assert!(!code.contains("null_mut"));
        assert!(!code.contains("ToAni :: to_ani"));
    }

    #[test]
    fn result_unit_return_conversion_returns_early_on_error() {
        let output: ReturnType = parse_quote!(-> Result<(), ani::Error>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("return ;"));
        assert!(!code.contains("null_mut"));
    }

    #[test]
    fn explicit_nullish_param_conversion_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: Undefined);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("Undefined as ani :: conversions :: FromAni"));
    }
}
