//! Function Expansion
//!
//! Expands `#[ani]` macro for functions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Signature, Type};

use crate::codegen::{RegisterTarget, generate_register_fn, generate_wrapper, has_this_injection};
use crate::parser::{BindgenAttrs, InitAttrs};
use crate::types::{
    EtsDeclKind, class_to_descriptor, current_module_name, emit_compile_ets_class_member,
    emit_compile_ets_decl, emit_compile_ets_rendered_decl, function_requires_nullish_bridge,
    generate_ctor_ets_decl, generate_ctor_signature, generate_fn_ets_binding,
    generate_fn_signature, generate_getter_ets_decl, generate_setter_ets_decl,
    module_to_descriptor, namespace_to_descriptor, qualify_member_descriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EtsBindingTarget {
    pub kind: EtsDeclKind,
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum EtsBindingEmission {
    Plain {
        target: EtsBindingTarget,
        signature: String,
        is_static: bool,
    },
    Rendered {
        target: EtsBindingTarget,
        rendered: String,
    },
    ClassMember {
        target: String,
        rendered: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BindingPlan {
    pub register_symbol_name: String,
    pub signature: String,
    pub register_target: RegisterTarget,
    pub ets: EtsBindingEmission,
}

/// Expand `#[ani]` for functions
pub fn expand_function(attrs: BindgenAttrs, func: ItemFn, prepare: TokenStream) -> TokenStream {
    if attrs.skip {
        return quote! { #prepare #func };
    }

    if let Err(err) = validate_unsupported_bind_attrs(&attrs, &func) {
        return err.to_compile_error();
    }

    if let Err(err) = validate_constructor_usage(&attrs, &func) {
        return err.to_compile_error();
    }

    let func_name = &func.sig.ident;
    let func_name_str = func_name.to_string();
    let is_constructor = attrs.constructor;
    let is_class_method = attrs.class.is_some();

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

    let binding = match resolve_binding_plan(
        &attrs,
        &func_name.to_string(),
        &func.sig,
        is_static,
        is_constructor,
        skip_first,
    ) {
        Ok(binding) => binding,
        Err(err) => return err.to_compile_error(),
    };
    emit_binding_plan_ets(&binding);

    // Generate wrapper function name
    let wrapper_name = format_ident!("__ani_native_{}", func_name);
    let register_name = format_ident!("__ani_register_{}", func_name);
    let ctor_register_name = format_ident!("__ani_ctor_register_{}", func_name);

    // Generate wrapper function
    let wrapper = generate_wrapper(&func, &wrapper_name, is_class_method, is_static);

    // Generate registration function based on target
    let register_fn = generate_register_fn(
        &register_name,
        &binding.register_target,
        &binding.register_symbol_name,
        &binding.signature,
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

pub(crate) fn validate_constructor_usage(attrs: &BindgenAttrs, func: &ItemFn) -> syn::Result<()> {
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

pub(crate) fn validate_unsupported_bind_attrs(
    attrs: &BindgenAttrs,
    func: &ItemFn,
) -> syn::Result<()> {
    if attrs.is_async {
        return Err(syn::Error::new_spanned(
            &func.sig.ident,
            "#[ani(async)] is not implemented yet; expose async behavior explicitly via Promise APIs",
        ));
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AccessorKind {
    Getter,
    Setter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AccessorConfig {
    pub kind: AccessorKind,
    pub property_name: String,
}

pub(crate) fn resolve_accessor_config(
    attrs: &BindgenAttrs,
    rust_name: &str,
    sig: &Signature,
    is_static: bool,
    is_constructor: bool,
    skip_first: bool,
) -> syn::Result<Option<AccessorConfig>> {
    if attrs.getter.is_some() && attrs.setter.is_some() {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(getter)] and #[ani(setter)] cannot be combined on the same function",
        ));
    }

    let (kind, explicit_name) = if let Some(name) = attrs.getter.as_ref() {
        (AccessorKind::Getter, name.as_str())
    } else if let Some(name) = attrs.setter.as_ref() {
        (AccessorKind::Setter, name.as_str())
    } else {
        return Ok(None);
    };

    if attrs.class.is_none() || attrs.namespace.is_some() || attrs.module.is_some() {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(getter)] / #[ani(setter)] can only be used on instance methods bound to a class",
        ));
    }
    if is_constructor {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(constructor)] cannot be combined with #[ani(getter)] / #[ani(setter)]",
        ));
    }
    if is_static {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(getter)] / #[ani(setter)] do not support static methods",
        ));
    }

    match kind {
        AccessorKind::Getter => validate_getter_signature(sig, skip_first)?,
        AccessorKind::Setter => validate_setter_signature(sig, skip_first)?,
    }

    let property_name = if explicit_name.trim().is_empty() {
        infer_accessor_property_name(rust_name, kind)
    } else {
        explicit_name.trim().to_string()
    };

    Ok(Some(AccessorConfig {
        kind,
        property_name,
    }))
}

pub(crate) fn resolve_binding_plan(
    attrs: &BindgenAttrs,
    rust_name: &str,
    sig: &Signature,
    is_static: bool,
    is_constructor: bool,
    skip_first: bool,
) -> syn::Result<BindingPlan> {
    let ets_name = attrs.name.clone().unwrap_or_else(|| rust_name.to_string());
    let requires_nullish_bridge = function_requires_nullish_bridge(sig, skip_first);
    if is_constructor && requires_nullish_bridge {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(constructor)] does not support nullish-exposed parameters yet; use explicit ArkTS wrappers instead",
        ));
    }

    let register_symbol_name = if is_constructor {
        "<ctor>".to_string()
    } else if requires_nullish_bridge {
        format!("__ani_native_{ets_name}")
    } else {
        ets_name.clone()
    };

    let accessor =
        resolve_accessor_config(attrs, rust_name, sig, is_static, is_constructor, skip_first)?;

    let signature = attrs.signature.clone().unwrap_or_else(|| {
        if is_constructor {
            generate_ctor_signature(sig, skip_first)
        } else {
            generate_fn_signature(sig, skip_first)
        }
    });
    let ets_target = resolve_ets_binding_target(attrs);

    let ets = if let Some(accessor) = &accessor {
        EtsBindingEmission::ClassMember {
            target: ets_target.target.clone(),
            rendered: render_accessor_ets_decl(accessor, sig, &register_symbol_name, skip_first),
        }
    } else if is_constructor {
        EtsBindingEmission::Plain {
            target: ets_target,
            signature: generate_ctor_ets_decl(sig, skip_first),
            is_static,
        }
    } else {
        EtsBindingEmission::Rendered {
            target: ets_target.clone(),
            rendered: generate_fn_ets_binding(
                ets_target.kind,
                sig,
                &ets_name,
                skip_first,
                is_static,
            ),
        }
    };

    Ok(BindingPlan {
        register_symbol_name,
        signature,
        register_target: resolve_register_target(attrs, is_static),
        ets,
    })
}

pub(crate) fn emit_binding_plan_ets(binding: &BindingPlan) {
    match &binding.ets {
        EtsBindingEmission::Plain {
            target,
            signature,
            is_static,
        } => emit_compile_ets_decl(target.kind, &target.target, signature, *is_static),
        EtsBindingEmission::Rendered { target, rendered } => {
            emit_compile_ets_rendered_decl(target.kind, &target.target, rendered)
        }
        EtsBindingEmission::ClassMember { target, rendered } => {
            emit_compile_ets_class_member(target, rendered)
        }
    }
}

fn resolve_ets_binding_target(attrs: &BindgenAttrs) -> EtsBindingTarget {
    if let Some(class) = attrs.class.as_deref() {
        EtsBindingTarget {
            kind: EtsDeclKind::Class,
            target: class.to_string(),
        }
    } else if let Some(namespace) = attrs.namespace.as_deref() {
        EtsBindingTarget {
            kind: EtsDeclKind::Namespace,
            target: namespace.to_string(),
        }
    } else if let Some(module) = attrs.module.as_deref() {
        if module.is_empty() {
            EtsBindingTarget {
                kind: EtsDeclKind::Global,
                target: String::new(),
            }
        } else {
            EtsBindingTarget {
                kind: EtsDeclKind::Namespace,
                target: module.to_string(),
            }
        }
    } else {
        EtsBindingTarget {
            kind: EtsDeclKind::Global,
            target: String::new(),
        }
    }
}

fn render_accessor_ets_decl(
    accessor: &AccessorConfig,
    sig: &Signature,
    backing_name: &str,
    skip_first: bool,
) -> String {
    match accessor.kind {
        AccessorKind::Getter => {
            generate_getter_ets_decl(sig, &accessor.property_name, backing_name, skip_first)
        }
        AccessorKind::Setter => {
            generate_setter_ets_decl(sig, &accessor.property_name, backing_name, skip_first)
        }
    }
}

fn validate_getter_signature(sig: &Signature, skip_first: bool) -> syn::Result<()> {
    if exposed_arg_count(sig, skip_first) != 0 {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(getter)] must not expose any ArkTS parameters",
        ));
    }

    match &sig.output {
        ReturnType::Default => Err(syn::Error::new_spanned(
            &sig.output,
            "#[ani(getter)] must return a value or Result<T>",
        )),
        ReturnType::Type(_, ty) if is_unit_type(ty) || is_ani_result_unit_type(ty) => {
            Err(syn::Error::new_spanned(
                &sig.output,
                "#[ani(getter)] must return a value or Result<T>",
            ))
        }
        ReturnType::Type(_, _) => Ok(()),
    }
}

fn validate_setter_signature(sig: &Signature, skip_first: bool) -> syn::Result<()> {
    if exposed_arg_count(sig, skip_first) != 1 {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(setter)] must expose exactly one ArkTS parameter",
        ));
    }

    match &sig.output {
        ReturnType::Default => Ok(()),
        ReturnType::Type(_, ty) if is_unit_type(ty) || is_ani_result_unit_type(ty) => Ok(()),
        _ => Err(syn::Error::new_spanned(
            &sig.output,
            "#[ani(setter)] return type must be `()` or `ani::error::Result<()>`",
        )),
    }
}

fn exposed_arg_count(sig: &Signature, skip_first: bool) -> usize {
    sig.inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !crate::codegen::should_skip_in_signature(arg))
        .count()
}

fn infer_accessor_property_name(rust_name: &str, kind: AccessorKind) -> String {
    let stripped = match kind {
        AccessorKind::Getter => strip_accessor_prefix(rust_name, &["get_", "get"]),
        AccessorKind::Setter => strip_accessor_prefix(rust_name, &["set_", "set"]),
    };
    stripped.unwrap_or_else(|| rust_name.to_string())
}

fn strip_accessor_prefix(name: &str, prefixes: &[&str]) -> Option<String> {
    for prefix in prefixes {
        if let Some(rest) = name.strip_prefix(prefix) {
            if rest.is_empty() {
                continue;
            }
            if *prefix == "get" || *prefix == "set" {
                let mut chars = rest.chars();
                let first = chars.next()?;
                if !first.is_ascii_uppercase() {
                    continue;
                }
                let mut property = first.to_ascii_lowercase().to_string();
                property.push_str(chars.as_str());
                return Some(property);
            }
            return Some(rest.to_string());
        }
    }
    None
}

pub(crate) fn resolve_register_target(attrs: &BindgenAttrs, is_static: bool) -> RegisterTarget {
    let module_name = current_module_name();

    if let Some(class) = attrs.class.as_deref() {
        RegisterTarget::Class {
            descriptor: class_to_descriptor(&qualify_member_descriptor(class, &module_name)),
            is_static,
        }
    } else if let Some(namespace) = attrs.namespace.as_deref() {
        RegisterTarget::Namespace(namespace_to_descriptor(&qualify_member_descriptor(
            namespace,
            &module_name,
        )))
    } else {
        let descriptor = attrs.module.as_ref().map_or_else(
            || module_to_descriptor(&module_name),
            |module_name_override| {
                if module_name_override.trim().is_empty() {
                    module_to_descriptor(&module_name)
                } else {
                    module_to_descriptor(module_name_override)
                }
            },
        );
        RegisterTarget::Module(descriptor)
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

    #[test]
    fn binding_plan_rejects_nullish_constructor_params() {
        let attrs = BindgenAttrs {
            constructor: true,
            class: Some("Person".to_string()),
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn ctor(name: Option<String>)
        };
        assert!(resolve_binding_plan(&attrs, "ctor", &sig, false, true, false).is_err());
    }

    #[test]
    fn getter_setter_attrs_are_not_rejected_by_unsupported_attr_validation() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            getter: Some("value".to_string()),
            ..Default::default()
        };
        let func: ItemFn = parse_quote! { fn get_value() -> i32 { 1 } };
        assert!(validate_unsupported_bind_attrs(&attrs, &func).is_ok());
    }

    #[test]
    fn rejects_async_attr_for_now() {
        let attrs = BindgenAttrs {
            is_async: true,
            ..Default::default()
        };
        let func: ItemFn = parse_quote! { fn compute() -> i32 { 1 } };
        assert!(validate_unsupported_bind_attrs(&attrs, &func).is_err());
    }
}
