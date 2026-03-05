//! Function Expansion
//!
//! Expands `#[ani]` macro for functions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Type};

use crate::codegen::{
    generate_class_register, generate_module_register, generate_namespace_register,
    generate_wrapper, has_this_injection,
};
use crate::parser::{BindgenAttrs, InitAttrs};
use crate::types::{
    EtsDeclKind, class_to_descriptor, current_module_name, emit_compile_ets_decl,
    generate_ctor_ets_decl, generate_ctor_signature, generate_fn_ets_decl, generate_fn_signature,
    module_to_descriptor, namespace_to_descriptor, qualify_member_descriptor,
};

/// Expand `#[ani]` for functions
pub fn expand_function(attrs: BindgenAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
    if attrs.skip {
        return quote! { #prepare #func };
    }

    if let Err(err) = validate_constructor_usage(&attrs, &func) {
        return err.to_compile_error();
    }

    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    let is_constructor = attrs.constructor;
    let is_class_method = attrs.class.is_some();

    // Determine ArkTS function name.
    // Keep Rust naming by default to avoid unexpected renaming in generated ETS.
    let ets_name = attrs.name.clone().unwrap_or_else(|| func_name.to_string());
    let register_symbol_name = if is_constructor {
        "<ctor>".to_string()
    } else {
        ets_name.clone()
    };

    // Check if function has self parameter (instance method)
    let has_self = func
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));

    // Check if function has This injection (alternative way to indicate instance method)
    let has_this = has_this_injection(&func);

    // Class methods are instance methods by default in ArkTS.
    let is_static = if is_constructor {
        false
    } else if is_class_method {
        attrs.is_static
    } else {
        // Keep old behavior for non-class functions.
        attrs.is_static || (!has_self && !has_this)
    };
    // Only skip explicit receiver (`self`). Injected params are filtered out
    // by `should_skip_in_signature`.
    let skip_first = has_self;

    // Generate signature
    // Note: generate_fn_signature automatically skips injected parameters (Env, This, Class).
    let signature = attrs.signature.clone().unwrap_or_else(|| {
        if is_constructor {
            generate_ctor_signature(&func.sig, skip_first)
        } else {
            generate_fn_signature(&func.sig, skip_first)
        }
    });
    let ets_signature = if is_constructor {
        generate_ctor_ets_decl(&func.sig, skip_first)
    } else {
        generate_fn_ets_decl(&func.sig, &ets_name, skip_first)
    };

    let (ets_kind, ets_target) = if let Some(class) = attrs.class.as_deref() {
        (EtsDeclKind::Class, class.to_string())
    } else if let Some(namespace) = attrs.namespace.as_deref() {
        (EtsDeclKind::Namespace, namespace.to_string())
    } else if let Some(module) = attrs.module.as_deref() {
        if module.is_empty() {
            (EtsDeclKind::Global, String::new())
        } else {
            (EtsDeclKind::Namespace, module.to_string())
        }
    } else {
        (EtsDeclKind::Global, String::new())
    };

    emit_compile_ets_decl(ets_kind, &ets_target, &ets_signature, is_static);

    // Generate wrapper function name
    let wrapper_name = format_ident!("__ani_native_{}", func_name);
    let register_name = format_ident!("__ani_register_{}", func_name);
    let ctor_register_name = format_ident!("__ani_ctor_register_{}", func_name);

    // Generate wrapper function
    let wrapper = generate_wrapper(&func, &wrapper_name, is_class_method, is_static);

    // Generate registration function based on target
    let register_fn = generate_register_fn(
        &attrs,
        is_static,
        &register_name,
        &register_symbol_name,
        &signature,
        &wrapper_name,
    );

    // Generate ctor auto-registration function
    let ctor_fn = quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[::ani::ctor::ctor(crate_path = ::ani::ctor)]
        fn #ctor_register_name() {
            ::ani::module_register::register_module_export(
                #func_name_str,
                #register_name
            );
        }
    };

    quote! {
        #prepare
        #func
        #wrapper
        #register_fn
        #ctor_fn
    }
}

fn validate_constructor_usage(attrs: &BindgenAttrs, func: &ItemFn) -> syn::Result<()> {
    if !attrs.constructor {
        return Ok(());
    }

    if attrs.class.is_none() {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[ani(constructor)] can only be used with #[ani(class = \"...\")]",
        ));
    }

    if attrs.is_static {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[ani(constructor)] cannot be combined with #[ani(static)]",
        ));
    }

    if attrs.namespace.is_some() || attrs.module.is_some() {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[ani(constructor)] cannot be used for namespace/module bindings",
        ));
    }

    if attrs.name.is_some() {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[ani(constructor)] does not support custom #[ani(name = ...)]",
        ));
    }

    match &func.sig.output {
        ReturnType::Default => Ok(()),
        ReturnType::Type(_, ty) if is_unit_type(ty) => Ok(()),
        ReturnType::Type(_, ty) if is_ani_result_unit_type(ty) => Ok(()),
        _ => Err(syn::Error::new_spanned(
            &func.sig.output,
            "#[ani(constructor)] return type must be `()` or `ani::error::Result<()>`",
        )),
    }
}

/// Generate appropriate registration function based on attributes
fn generate_register_fn(
    attrs: &BindgenAttrs,
    is_static: bool,
    register_name: &proc_macro2::Ident,
    ets_name: &str,
    signature: &str,
    wrapper_name: &proc_macro2::Ident,
) -> TokenStream {
    let module_name = current_module_name();

    if let Some(ref class) = attrs.class {
        let descriptor = class_to_descriptor(&qualify_member_descriptor(class, &module_name));
        generate_class_register(
            register_name,
            &descriptor,
            is_static,
            ets_name,
            signature,
            wrapper_name,
        )
    } else if let Some(ref ns) = attrs.namespace {
        let descriptor = namespace_to_descriptor(&qualify_member_descriptor(ns, &module_name));
        generate_namespace_register(
            register_name,
            &descriptor,
            ets_name,
            signature,
            wrapper_name,
        )
    } else {
        let descriptor = attrs.module.as_ref().map_or_else(
            || module_to_descriptor(&module_name),
            |m| {
                if m.trim().is_empty() {
                    module_to_descriptor(&module_name)
                } else {
                    module_to_descriptor(m)
                }
            },
        );
        generate_module_register(
            register_name,
            &descriptor,
            ets_name,
            signature,
            wrapper_name,
        )
    }
}

/// Expand `#[ani(init)]` for initialization functions
pub fn expand_init(attrs: InitAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
    let init_signature = match validate_init_signature(&func) {
        Ok(sig) => sig,
        Err(err) => return err.to_compile_error(),
    };

    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    let callback_name = format_ident!("__ani_init_callback_{}", func_name);
    let ctor_register_name = format_ident!("__ani_ctor_register_init_{}", func_name);
    let before_bindings = attrs.before_bindings;

    let call_user_init = if init_signature.accepts_env {
        quote! { #func_name(&__ani_env) }
    } else {
        quote! { #func_name() }
    };

    let env_binding = if init_signature.accepts_env {
        quote! {
            let __ani_env = unsafe { ::ani::env::Env::from_raw_unchecked(env) };
        }
    } else {
        quote! {}
    };

    let callback_body = match init_signature.return_kind {
        InitReturnKind::Unit => quote! {
            #call_user_init;
            ::ani::sys::ani_status_ANI_OK
        },
        InitReturnKind::Result => quote! {
            match #call_user_init {
                Ok(()) => ::ani::sys::ani_status_ANI_OK,
                Err(e) => {
                    let biz_err: ::ani::error::BusinessError = e.into();
                    unsafe { biz_err.throw_into(env) };
                    ::ani::sys::ani_status_ANI_ERROR
                }
            }
        },
    };

    quote! {
        #prepare
        #func

        #[doc(hidden)]
        #[allow(non_snake_case, unused_variables)]
        unsafe extern "C" fn #callback_name(env: *mut ::ani::sys::ani_env) -> ::ani::sys::ani_status {
            #env_binding
            #callback_body
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[::ani::ctor::ctor(crate_path = ::ani::ctor)]
        fn #ctor_register_name() {
            ::ani::module_register::register_init_callback(
                #func_name_str,
                #before_bindings,
                #callback_name,
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InitReturnKind {
    Unit,
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InitSignature {
    accepts_env: bool,
    return_kind: InitReturnKind,
}

fn validate_init_signature(func: &ItemFn) -> syn::Result<InitSignature> {
    if func.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &func.sig.asyncness,
            "#[ani(init)] does not support async functions",
        ));
    }

    let accepts_env = match func.sig.inputs.len() {
        0 => false,
        1 => match func.sig.inputs.first() {
            Some(FnArg::Typed(pat_type)) if is_env_type(&pat_type.ty) => true,
            Some(arg) => {
                return Err(syn::Error::new_spanned(
                    arg,
                    "#[ani(init)] only supports `env: &Env<'_>` as parameter",
                ));
            }
            None => false,
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &func.sig.inputs,
                "#[ani(init)] supports at most one parameter: `env: &Env<'_>`",
            ));
        }
    };

    let return_kind = match &func.sig.output {
        ReturnType::Default => InitReturnKind::Unit,
        ReturnType::Type(_, ty) if is_unit_type(ty) => InitReturnKind::Unit,
        ReturnType::Type(_, ty) if is_ani_result_unit_type(ty) => InitReturnKind::Result,
        _ => {
            return Err(syn::Error::new_spanned(
                &func.sig.output,
                "#[ani(init)] return type must be `()` or `ani::error::Result<()>`",
            ));
        }
    };

    Ok(InitSignature {
        accepts_env,
        return_kind,
    })
}

fn is_env_type(ty: &Type) -> bool {
    let Type::Reference(type_ref) = ty else {
        return false;
    };

    let Type::Path(type_path) = type_ref.elem.as_ref() else {
        return false;
    };

    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Env")
}

fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}

fn is_ani_result_unit_type(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };
    if segment.ident != "Result" {
        return false;
    }

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return false;
    };
    if args.args.len() != 1 {
        return false;
    }

    let Some(GenericArgument::Type(ok_ty)) = args.args.first() else {
        return false;
    };
    is_unit_type(ok_ty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::BindgenAttrs;
    use syn::parse_quote;

    #[test]
    fn init_supports_no_arg_unit() {
        let func: ItemFn = parse_quote! {
            fn setup() {}
        };
        let parsed = validate_init_signature(&func).expect("should parse init signature");
        assert_eq!(
            parsed,
            InitSignature {
                accepts_env: false,
                return_kind: InitReturnKind::Unit
            }
        );
    }

    #[test]
    fn init_supports_env_and_result() {
        let func: ItemFn = parse_quote! {
            fn setup(env: &Env<'_>) -> ani::error::Result<()> {
                let _ = env;
                Ok(())
            }
        };
        let parsed = validate_init_signature(&func).expect("should parse init signature");
        assert_eq!(
            parsed,
            InitSignature {
                accepts_env: true,
                return_kind: InitReturnKind::Result
            }
        );
    }

    #[test]
    fn init_rejects_multiple_params() {
        let func: ItemFn = parse_quote! {
            fn setup(env: &Env<'_>, a: i32) {}
        };
        assert!(validate_init_signature(&func).is_err());
    }

    #[test]
    fn init_rejects_non_result_return() {
        let func: ItemFn = parse_quote! {
            fn setup() -> i32 {
                1
            }
        };
        assert!(validate_init_signature(&func).is_err());
    }

    #[test]
    fn constructor_rejects_missing_class_attr() {
        let attrs = BindgenAttrs {
            constructor: true,
            ..Default::default()
        };
        let func: ItemFn = parse_quote! {
            fn ctor() {}
        };
        assert!(validate_constructor_usage(&attrs, &func).is_err());
    }

    #[test]
    fn constructor_rejects_non_void_return() {
        let attrs = BindgenAttrs {
            constructor: true,
            class: Some("Person".to_string()),
            ..Default::default()
        };
        let func: ItemFn = parse_quote! {
            fn ctor() -> i64 { 1 }
        };
        assert!(validate_constructor_usage(&attrs, &func).is_err());
    }
}
