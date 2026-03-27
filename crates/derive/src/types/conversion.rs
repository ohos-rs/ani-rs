//! Type Conversion Code Generation
//!
//! Generates code for converting between Rust and ANI types.
//! Uses the structured AniType system instead of string-based pattern matching.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{FnArg, Pat, ReturnType, Type, TypePath};

use super::ani_type::{
    AniType, PrimitiveType, StringType, WrapperType, extract_transparent_wrapper_inner_type,
    is_custom_object_type_path,
};

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

/// Generate parameter conversion code for function arguments, using a custom error handler.
///
/// The `on_error` tokens are injected into each `Err(e) => { ... }` arm and
/// may reference `env` and `e`. This is useful for async bindings that must
/// return a rejected Promise instead of throwing synchronously.
pub fn generate_param_conversions_with_custom_error(
    params: &[&FnArg],
    on_error: &TokenStream,
) -> TokenStream {
    let conversions: Vec<TokenStream> = params
        .iter()
        .enumerate()
        .map(|(i, param)| generate_single_param_conversion_with_custom_error(i, param, on_error))
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

fn generate_single_param_conversion_with_custom_error(
    index: usize,
    param: &FnArg,
    on_error: &TokenStream,
) -> TokenStream {
    let FnArg::Typed(pat_type) = param else {
        return quote! {};
    };

    let param_name = extract_param_name(&pat_type.pat, index);
    let converted_name = format_ident!("{}_converted", param_name);
    let ty = &pat_type.ty;
    let ani_type = AniType::from_syn_type(ty);

    generate_type_conversion_with_custom_error(
        &param_name,
        &converted_name,
        ty,
        &ani_type,
        on_error,
    )
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
        AniType::Primitive(p) => generate_primitive_conversion(param_name, converted_name, p),
        AniType::String(s @ (StringType::String | StringType::Str)) => {
            generate_string_type_conversion(param_name, converted_name, s, on_error_return)
        }
        AniType::String(_) => {
            generate_generic_from_ani_conversion(param_name, converted_name, ty, on_error_return)
        }
        _ if uses_typed_from_ani_param_conversion(ani_type) => {
            generate_generic_from_ani_conversion(param_name, converted_name, ty, on_error_return)
        }
        _ => quote! { let #converted_name = #param_name; },
    }
}

fn generate_type_conversion_with_custom_error(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    ani_type: &AniType,
    on_error: &TokenStream,
) -> TokenStream {
    match ani_type {
        AniType::Primitive(p) => generate_primitive_conversion(param_name, converted_name, p),
        AniType::String(s @ (StringType::String | StringType::Str)) => {
            generate_string_type_conversion_with_custom_error(
                param_name,
                converted_name,
                s,
                on_error,
            )
        }
        AniType::String(_) => {
            generate_generic_from_ani_conversion_with_custom_error(
                param_name,
                converted_name,
                ty,
                on_error,
            )
        }
        _ if uses_typed_from_ani_param_conversion(ani_type) => {
            generate_generic_from_ani_conversion_with_custom_error(
                param_name,
                converted_name,
                ty,
                on_error,
            )
        }
        _ => quote! { let #converted_name = #param_name; },
    }
}

fn uses_typed_from_ani_param_conversion(ani_type: &AniType) -> bool {
    matches!(
        ani_type,
        AniType::AniObject
            | AniType::GlobalRef
            | AniType::WeakRef
            | AniType::RuntimeHandle(_)
            | AniType::ArrayHandle(_)
            | AniType::Null
            | AniType::Undefined
            | AniType::Function(_)
            | AniType::FnArgs(_)
            | AniType::Either(_)
            | AniType::Promise(_)
            | AniType::Record(_)
            | AniType::Set(_)
            | AniType::Map(_)
            | AniType::ArrayBuffer
            | AniType::Tuple(_)
            | AniType::AnyValue
            | AniType::TupleValue
            | AniType::EnumItem
            | AniType::NativePointer(_)
            | AniType::FixedArray(_)
            | AniType::CustomObject(_)
            | AniType::Unknown(_)
            | AniType::Wrapper(WrapperType::Option(_))
            | AniType::Wrapper(WrapperType::Vec(_))
            | AniType::Wrapper(WrapperType::Ref(_))
            | AniType::String(StringType::CString)
            | AniType::String(StringType::PathBuf)
            | AniType::String(StringType::BoxStr)
            | AniType::String(StringType::CowStr)
    )
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
        PrimitiveType::Isize => quote! { let #converted_name = #param_name as isize; },
        PrimitiveType::Usize => quote! { let #converted_name = #param_name as usize; },
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
        _ => unreachable!("generate_string_type_conversion only supports String and &str"),
    }
}

fn generate_string_type_conversion_with_custom_error(
    param_name: &Ident,
    converted_name: &Ident,
    string_type: &StringType,
    on_error: &TokenStream,
) -> TokenStream {
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
        _ => {
            unreachable!("generate_string_type_conversion_with_custom_error only supports String and &str")
        }
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

fn generate_generic_from_ani_conversion_with_custom_error(
    param_name: &Ident,
    converted_name: &Ident,
    ty: &Type,
    on_error: &TokenStream,
) -> TokenStream {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UnknownTypeReturnStrategy {
    DirectPassthrough,
    ToAniNullFallback,
}

fn unknown_type_return_strategy(ty: &Type) -> UnknownTypeReturnStrategy {
    match ty {
        Type::Ptr(_) => UnknownTypeReturnStrategy::DirectPassthrough,
        Type::Reference(type_ref) => unknown_type_return_strategy(type_ref.elem.as_ref()),
        Type::Paren(type_paren) => unknown_type_return_strategy(type_paren.elem.as_ref()),
        Type::Group(type_group) => unknown_type_return_strategy(type_group.elem.as_ref()),
        Type::Path(type_path) => classify_type_path_return_strategy(type_path),
        _ => UnknownTypeReturnStrategy::DirectPassthrough,
    }
}

fn classify_type_path_return_strategy(type_path: &TypePath) -> UnknownTypeReturnStrategy {
    if is_raw_ani_type_path(type_path) {
        return UnknownTypeReturnStrategy::DirectPassthrough;
    }

    let Some(last) = type_path.path.segments.last() else {
        return UnknownTypeReturnStrategy::DirectPassthrough;
    };
    let ident = last.ident.to_string();

    if let Some(inner) = extract_transparent_wrapper_inner_type(&ident, &last.arguments) {
        return unknown_type_return_strategy(&inner);
    }

    if is_known_runtime_wrapper_type(&ident) || is_custom_object_type_path(type_path) {
        UnknownTypeReturnStrategy::ToAniNullFallback
    } else {
        UnknownTypeReturnStrategy::DirectPassthrough
    }
}

fn is_raw_ani_type_path(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
        .map(|ident| ident.starts_with("ani_"))
        .unwrap_or(false)
}

fn is_known_runtime_wrapper_type(ident: &str) -> bool {
    matches!(
        ident,
        "AniRef"
            | "AniObject"
            | "AniClass"
            | "AniType"
            | "AniModule"
            | "AniNamespace"
            | "AniString"
            | "AniEnum"
            | "AniEnumItem"
            | "AniTupleValue"
            | "AniArray"
            | "AniArrayRef"
            | "AniFixedArray"
            | "AniFixedArrayRef"
            | "FixedBooleanArray"
            | "AniFixedArrayBoolean"
            | "FixedByteArray"
            | "AniFixedArrayByte"
            | "FixedShortArray"
            | "AniFixedArrayShort"
            | "FixedCharArray"
            | "AniFixedArrayChar"
            | "FixedIntArray"
            | "AniArrayInt"
            | "AniFixedArrayInt"
            | "FixedLongArray"
            | "AniArrayLong"
            | "AniFixedArrayLong"
            | "FixedFloatArray"
            | "AniFixedArrayFloat"
            | "FixedDoubleArray"
            | "AniArrayDouble"
            | "AniFixedArrayDouble"
            | "AniMethod"
            | "AniStaticMethod"
            | "AniField"
            | "AniStaticField"
            | "AniVariable"
            | "AniResolver"
            | "AnyValue"
            | "TupleValue"
            | "EnumItem"
            | "GlobalRef"
            | "WeakRef"
    )
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

fn is_to_ani_object_raw_type(ani_type: &AniType) -> bool {
    matches!(
        ani_type,
        AniType::Record(_) | AniType::Set(_) | AniType::Map(_)
    )
}

fn is_to_ani_value_type(ani_type: &AniType) -> bool {
    matches!(
        ani_type,
        AniType::String(StringType::Str)
            | AniType::AniObject
            | AniType::RuntimeHandle(_)
            | AniType::AnyValue
            | AniType::TupleValue
            | AniType::EnumItem
            | AniType::ArrayBuffer
            | AniType::Either(_)
            | AniType::Null
            | AniType::Undefined
            | AniType::Function(_)
            | AniType::Tuple(_)
            | AniType::FixedArray(_)
            | AniType::CustomObject(_)
            | AniType::Wrapper(WrapperType::Option(_))
            | AniType::Wrapper(WrapperType::Vec(_))
            | AniType::Wrapper(WrapperType::Ref(_))
    )
}

fn is_to_ani_raw_string_type(ani_type: &AniType) -> bool {
    matches!(
        ani_type,
        AniType::String(StringType::CString)
            | AniType::String(StringType::PathBuf)
            | AniType::String(StringType::BoxStr)
            | AniType::String(StringType::CowStr)
    )
}

fn is_to_ani_default_value_type(ani_type: &AniType) -> bool {
    matches!(ani_type, AniType::NativePointer(_))
}

fn uses_null_pointer_error_fallback(ani_type: &AniType) -> bool {
    matches!(
        ani_type,
        AniType::String(StringType::String) | AniType::Promise(_)
    ) || is_to_ani_raw_string_type(ani_type)
        || is_to_ani_object_raw_type(ani_type)
        || is_to_ani_value_type(ani_type)
}

fn generate_value_return_conversion(
    ani_type: &AniType,
    value_expr: TokenStream,
    original_ty: Option<&Type>,
) -> TokenStream {
    let on_null_error = generate_return_conversion_error(quote! { std::ptr::null_mut() });
    let on_default_error = generate_return_conversion_error(quote! { Default::default() });

    match ani_type {
        AniType::Primitive(p) => match p {
            PrimitiveType::Bool => quote! { if #value_expr { 1 } else { 0 } },
            PrimitiveType::U8 => quote! { #value_expr as ani::sys::ani_byte },
            PrimitiveType::U32 => quote! { #value_expr as ani::sys::ani_int },
            PrimitiveType::U64 => quote! { #value_expr as ani::sys::ani_long },
            PrimitiveType::Isize | PrimitiveType::Usize => {
                quote! { #value_expr as ani::sys::ani_long }
            }
            PrimitiveType::Char => quote! { (#value_expr as u32) as ani::sys::ani_char },
            _ => quote! { #value_expr },
        },
        AniType::String(StringType::String | StringType::Str) => quote! {
            {
                let env_wrapper = ani::env::Env::from_raw_unchecked(env);
                match env_wrapper.create_string(&#value_expr) {
                    Ok(s) => s.into_raw(),
                    Err(e) => { #on_null_error }
                }
            }
        },
        _ if is_to_ani_raw_string_type(ani_type) => {
            generate_to_ani_conversion(value_expr, on_null_error, true)
        }
        AniType::Unit => quote! {},
        AniType::Promise(_) => quote! { #value_expr.into_raw() },
        _ if is_to_ani_object_raw_type(ani_type) => {
            generate_to_ani_conversion(value_expr, on_null_error, true)
        }
        _ if is_to_ani_default_value_type(ani_type) => {
            generate_to_ani_conversion(value_expr, on_default_error, false)
        }
        _ if is_to_ani_value_type(ani_type) => {
            generate_to_ani_conversion(value_expr, on_null_error, false)
        }
        _ => {
            if let Some(original_ty) = original_ty {
                match unknown_type_return_strategy(original_ty) {
                    UnknownTypeReturnStrategy::DirectPassthrough => quote! { #value_expr },
                    UnknownTypeReturnStrategy::ToAniNullFallback => {
                        generate_to_ani_conversion(value_expr, on_null_error, false)
                    }
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
        _ if is_to_ani_default_value_type(ok_type) => quote! { Default::default() },
        _ if uses_null_pointer_error_fallback(ok_type) => quote! { std::ptr::null_mut() },
        _ => {
            if let Some(original_ok_ty) = original_ok_ty {
                match unknown_type_return_strategy(original_ok_ty) {
                    UnknownTypeReturnStrategy::DirectPassthrough => quote! { Default::default() },
                    UnknownTypeReturnStrategy::ToAniNullFallback => quote! { std::ptr::null_mut() },
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
    fn param_conversion_cstring_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: std::ffi::CString);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("CString as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn param_conversion_pathbuf_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: std::path::PathBuf);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("PathBuf as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn param_conversion_box_str_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: Box<str>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("Box < str > as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn param_conversion_cow_str_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: std::borrow::Cow<'static, str>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("Cow < 'static , str > as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
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
    fn param_conversion_vec_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(items: Vec<String>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("Vec < String > as ani :: conversions :: FromAni"));
        assert!(code.contains("items as < Vec < String > as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn param_conversion_function_ref_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(cb: FunctionRef<(i32,), String>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("FunctionRef < (i32 ,) , String > as ani :: conversions :: FromAni"));
        assert!(
            code.contains(
                "cb as < FunctionRef < (i32 ,) , String > as ani :: conversions :: FromAni"
            )
        );
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn param_conversion_arraybuffer_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(buffer: ArrayBuffer);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("ArrayBuffer as ani :: conversions :: FromAni"));
        assert!(code.contains("buffer as < ArrayBuffer as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn param_conversion_usize_and_isize_use_long_casts() {
        let args: [FnArg; 2] = [parse_quote!(len: usize), parse_quote!(offset: isize)];
        let refs = args.iter().collect::<Vec<_>>();
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&refs, &on_error_return).to_string();
        assert!(code.contains("let len_converted = len as usize"));
        assert!(code.contains("let offset_converted = offset as isize"));
    }

    #[test]
    fn param_conversion_fixed_array_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(values: FixedIntArray);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("FixedIntArray as ani :: conversions :: FromAni"));
        assert!(code.contains("values as < FixedIntArray as ani :: conversions :: FromAni"));
    }

    #[test]
    fn transparent_wrapper_object_return_strategy_uses_to_ani_fallback() {
        let ty: Type = parse_quote!(Box<crate::models::UserInfo>);
        assert_eq!(
            unknown_type_return_strategy(&ty),
            UnknownTypeReturnStrategy::ToAniNullFallback
        );

        let ty: Type = parse_quote!(std::sync::Arc<crate::models::UserInfo>);
        assert_eq!(
            unknown_type_return_strategy(&ty),
            UnknownTypeReturnStrategy::ToAniNullFallback
        );
    }

    #[test]
    fn sync_and_cell_wrapped_object_return_strategy_uses_to_ani_fallback() {
        let ty: Type = parse_quote!(std::sync::Mutex<crate::models::UserInfo>);
        assert_eq!(
            unknown_type_return_strategy(&ty),
            UnknownTypeReturnStrategy::ToAniNullFallback
        );

        let ty: Type = parse_quote!(std::sync::RwLock<crate::models::UserInfo>);
        assert_eq!(
            unknown_type_return_strategy(&ty),
            UnknownTypeReturnStrategy::ToAniNullFallback
        );

        let ty: Type = parse_quote!(std::cell::RefCell<crate::models::UserInfo>);
        assert_eq!(
            unknown_type_return_strategy(&ty),
            UnknownTypeReturnStrategy::ToAniNullFallback
        );

        let ty: Type = parse_quote!(std::sync::OnceLock<crate::models::UserInfo>);
        assert_eq!(
            unknown_type_return_strategy(&ty),
            UnknownTypeReturnStrategy::ToAniNullFallback
        );
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
    fn return_conversion_option_uses_to_ani() {
        let output: ReturnType = parse_quote!(-> Option<i32>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("Ok (v) => v"));
    }

    #[test]
    fn result_option_return_conversion_uses_null_mut_fallback() {
        let output: ReturnType = parse_quote!(-> Result<Option<i32>, ani::Error>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("null_mut"));
        assert!(code.contains("ToAni :: to_ani"));
    }

    #[test]
    fn return_conversion_function_uses_to_ani() {
        let output: ReturnType = parse_quote!(-> Function<(), String>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("Ok (v) => v"));
    }

    #[test]
    fn return_conversion_any_value_uses_to_ani() {
        let output: ReturnType = parse_quote!(-> ani::conversions::AnyValue);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("null_mut"));
    }

    #[test]
    fn param_conversion_any_value_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: ani::conversions::AnyValue);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("AnyValue as ani :: conversions :: FromAni"));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn return_conversion_native_pointer_uses_typed_to_ani_with_default_fallback() {
        let output: ReturnType =
            parse_quote!(-> ani::conversions::NativePointer<crate::NativeResource>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("Default :: default ()"));
        assert!(!code.contains("null_mut"));
    }

    #[test]
    fn result_native_pointer_return_conversion_uses_default_fallback() {
        let output: ReturnType = parse_quote!(-> Result<ani::conversions::NativePointer<crate::NativeResource>, ani::Error>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("Default :: default ()"));
        assert!(!code.contains("null_mut"));
    }

    #[test]
    fn param_conversion_native_pointer_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(ptr: ani::conversions::NativePointer<crate::NativeResource>);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains(
            "NativePointer < crate :: NativeResource > as ani :: conversions :: FromAni"
        ));
        assert!(code.contains(":: Input"));
    }

    #[test]
    fn return_conversion_str_creates_string() {
        let output: ReturnType = parse_quote!(-> &str);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("create_string"));
        assert!(code.contains("into_raw"));
    }

    #[test]
    fn return_conversion_cstring_uses_to_ani_into_raw() {
        let output: ReturnType = parse_quote!(-> std::ffi::CString);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("into_raw"));
        assert!(!code.contains("create_string"));
    }

    #[test]
    fn return_conversion_pathbuf_uses_to_ani_into_raw() {
        let output: ReturnType = parse_quote!(-> std::path::PathBuf);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("into_raw"));
        assert!(!code.contains("create_string"));
    }

    #[test]
    fn return_conversion_box_str_uses_to_ani_into_raw() {
        let output: ReturnType = parse_quote!(-> Box<str>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("into_raw"));
        assert!(!code.contains("create_string"));
    }

    #[test]
    fn return_conversion_cow_str_uses_to_ani_into_raw() {
        let output: ReturnType = parse_quote!(-> std::borrow::Cow<'static, str>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("into_raw"));
        assert!(!code.contains("create_string"));
    }

    #[test]
    fn explicit_nullish_param_conversion_uses_typed_from_ani() {
        let arg: FnArg = parse_quote!(value: Undefined);
        let on_error_return = quote! { return Default::default(); };
        let code = generate_param_conversions(&[&arg], &on_error_return).to_string();
        assert!(code.contains("Undefined as ani :: conversions :: FromAni"));
    }

    #[test]
    fn return_conversion_raw_ani_object_is_passthrough() {
        let output: ReturnType = parse_quote!(-> ani::sys::ani_object);
        let code = generate_return_conversion(&output).to_string();
        assert!(!code.contains("ToAni :: to_ani"));
        assert_eq!(code.trim(), "result");
    }

    #[test]
    fn return_conversion_raw_pointer_is_passthrough() {
        let output: ReturnType = parse_quote!(-> *mut core::ffi::c_void);
        let code = generate_return_conversion(&output).to_string();
        assert!(!code.contains("ToAni :: to_ani"));
        assert_eq!(code.trim(), "result");
    }

    #[test]
    fn return_conversion_vecdeque_uses_to_ani() {
        let output: ReturnType = parse_quote!(-> std::collections::VecDeque<String>);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("Ok (v) => v"));
    }

    #[test]
    fn return_conversion_custom_object_uses_to_ani() {
        let output: ReturnType = parse_quote!(-> crate::models::UserInfo);
        let code = generate_return_conversion(&output).to_string();
        assert!(code.contains("ToAni :: to_ani"));
        assert!(code.contains("null_mut"));
    }
}
