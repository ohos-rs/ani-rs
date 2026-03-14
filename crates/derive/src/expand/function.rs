//! Function Expansion
//!
//! Expands `#[ani]` macro for functions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Signature, Type};

use crate::codegen::{
    ClassCallableDescriptor, ClassDescriptorMember, ClassMemberScope,
    ClassPropertyAccessorDescriptor, ClassPropertyDescriptor, ClassRegisterDescriptor,
    EtsBindingEmission, EtsBindingTarget, ExportPlan, RegisterTarget, WrapperBindingKind,
    emit_export_plan_ets, generate_register_fn, generate_wrapper, has_this_injection,
};
use crate::parser::{BindgenAttrs, InitAttrs};
use crate::types::{
    EtsDeclKind, class_to_descriptor, current_module_name, function_requires_nullish_bridge,
    generate_ctor_signature, generate_fn_ets_binding, generate_fn_signature,
    generate_getter_ets_decl, generate_setter_ets_decl, module_to_descriptor,
    namespace_to_descriptor, qualify_member_descriptor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallableKind {
    Function,
    Constructor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BindingResolveInput {
    pub callable_kind: CallableKind,
    pub wrapper_binding_kind: WrapperBindingKind,
    pub skip_first_arg: bool,
}

impl BindingResolveInput {
    pub(crate) fn is_constructor(self) -> bool {
        matches!(self.callable_kind, CallableKind::Constructor)
    }

    pub(crate) fn is_static(self) -> bool {
        self.wrapper_binding_kind.is_static()
    }
}

pub(crate) fn class_wrapper_binding_kind(is_static: bool) -> WrapperBindingKind {
    if is_static {
        WrapperBindingKind::ClassStatic
    } else {
        WrapperBindingKind::ClassInstance
    }
}

fn resolve_function_binding_input(attrs: &BindgenAttrs, func: &ItemFn) -> BindingResolveInput {
    let callable_kind = if attrs.constructor {
        CallableKind::Constructor
    } else {
        CallableKind::Function
    };
    let is_class_method = attrs.class.is_some();
    let has_self = func
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));
    let has_this = has_this_injection(func);
    let is_static = if matches!(callable_kind, CallableKind::Constructor) {
        false
    } else if is_class_method {
        attrs.is_static
    } else {
        attrs.is_static || (!has_self && !has_this)
    };

    BindingResolveInput {
        callable_kind,
        wrapper_binding_kind: if is_class_method {
            class_wrapper_binding_kind(is_static)
        } else {
            WrapperBindingKind::Global
        },
        skip_first_arg: has_self,
    }
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
    let binding_input = resolve_function_binding_input(&attrs, &func);

    let binding =
        match resolve_binding_plan(&attrs, &func_name.to_string(), &func.sig, binding_input) {
            Ok(binding) => binding,
            Err(err) => return err.to_compile_error(),
        };
    emit_export_plan_ets(&binding);

    // Generate wrapper function name
    let wrapper_name = format_ident!("__ani_native_{}", func_name);
    let register_name = format_ident!("__ani_register_{}", func_name);
    let ctor_register_name = format_ident!("__ani_ctor_register_{}", func_name);

    // Generate wrapper function
    let wrapper = generate_wrapper(&func, &wrapper_name, binding_input.wrapper_binding_kind);

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

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedClassBinding {
    descriptor: ClassDescriptorMember,
    register: ClassRegisterDescriptor,
}

pub(crate) fn resolve_accessor_config(
    attrs: &BindgenAttrs,
    rust_name: &str,
    sig: &Signature,
    binding_input: BindingResolveInput,
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
            "#[ani(getter)] / #[ani(setter)] can only be used on methods bound to a class",
        ));
    }
    if binding_input.is_constructor() {
        return Err(syn::Error::new_spanned(
            sig,
            "#[ani(constructor)] cannot be combined with #[ani(getter)] / #[ani(setter)]",
        ));
    }
    match kind {
        AccessorKind::Getter => validate_getter_signature(sig, binding_input.skip_first_arg)?,
        AccessorKind::Setter => validate_setter_signature(sig, binding_input.skip_first_arg)?,
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
    binding_input: BindingResolveInput,
) -> syn::Result<ExportPlan> {
    let ets_name = attrs.name.clone().unwrap_or_else(|| rust_name.to_string());
    let requires_nullish_bridge =
        function_requires_nullish_bridge(sig, binding_input.skip_first_arg);

    let accessor = resolve_accessor_config(attrs, rust_name, sig, binding_input)?;

    let register_symbol_name = if binding_input.is_constructor() {
        "<ctor>".to_string()
    } else if accessor.is_some() || requires_nullish_bridge {
        format!("__ani_native_{ets_name}")
    } else {
        ets_name.clone()
    };

    let signature = attrs.signature.clone().unwrap_or_else(|| {
        if binding_input.is_constructor() {
            generate_ctor_signature(sig, binding_input.skip_first_arg)
        } else {
            generate_fn_signature(sig, binding_input.skip_first_arg)
        }
    });
    let ets_target = resolve_ets_binding_target(attrs);
    let ets_kind = ets_target.kind;
    let class_binding = resolve_class_binding(
        attrs,
        binding_input,
        &ets_name,
        &register_symbol_name,
        accessor.as_ref(),
    );
    let class_descriptor = class_binding
        .as_ref()
        .map(|binding| binding.descriptor.clone());
    let class_register = class_binding
        .as_ref()
        .map(|binding| binding.register.clone());

    let ets = if let Some(class_descriptor) = &class_descriptor {
        EtsBindingEmission::ClassMember {
            rendered: render_class_member_ets_decl(class_descriptor, sig, binding_input),
        }
    } else {
        EtsBindingEmission::Rendered {
            target: ets_target,
            rendered: generate_fn_ets_binding(
                ets_kind,
                sig,
                &ets_name,
                binding_input.skip_first_arg,
                binding_input.is_static(),
            ),
        }
    };

    Ok(ExportPlan {
        register_symbol_name,
        signature,
        register_target: resolve_register_target(attrs, class_register.as_ref()),
        ets,
        class_descriptor,
        class_register,
    })
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

fn render_class_member_ets_decl(
    class_descriptor: &ClassDescriptorMember,
    sig: &Signature,
    binding_input: BindingResolveInput,
) -> String {
    match class_descriptor {
        ClassDescriptorMember::Constructor(_) => {
            crate::types::generate_ctor_ets_binding(sig, binding_input.skip_first_arg)
        }
        ClassDescriptorMember::Method(descriptor) => generate_fn_ets_binding(
            EtsDeclKind::Class,
            sig,
            &descriptor.public_name,
            binding_input.skip_first_arg,
            binding_input.is_static(),
        ),
        ClassDescriptorMember::Property(descriptor) => {
            render_property_member_ets_decl(&descriptor, sig, binding_input.skip_first_arg)
        }
    }
}

fn render_property_member_ets_decl(
    descriptor: &ClassPropertyDescriptor,
    sig: &Signature,
    skip_first: bool,
) -> String {
    let owner_name = descriptor
        .owner
        .rsplit('.')
        .next()
        .unwrap_or(descriptor.owner.as_str());
    let is_static = matches!(descriptor.scope, ClassMemberScope::Static);

    if let Some(getter) = &descriptor.getter {
        return generate_getter_ets_decl(
            sig,
            &descriptor.public_name,
            &getter.native_symbol_name,
            owner_name,
            skip_first,
            is_static,
        );
    }

    if let Some(setter) = &descriptor.setter {
        return generate_setter_ets_decl(
            sig,
            &descriptor.public_name,
            &setter.native_symbol_name,
            owner_name,
            skip_first,
            is_static,
        );
    }

    panic!("property descriptor must contain a getter or setter accessor")
}

fn resolve_class_binding(
    attrs: &BindgenAttrs,
    binding_input: BindingResolveInput,
    ets_name: &str,
    register_symbol_name: &str,
    accessor: Option<&AccessorConfig>,
) -> Option<ResolvedClassBinding> {
    let owner = attrs.class.clone()?;
    let scope = if binding_input.is_static() {
        ClassMemberScope::Static
    } else {
        ClassMemberScope::Instance
    };

    let descriptor = if binding_input.is_constructor() {
        ClassDescriptorMember::Constructor(ClassCallableDescriptor {
            owner,
            public_name: "constructor".to_string(),
            native_symbol_name: register_symbol_name.to_string(),
            scope,
        })
    } else if let Some(accessor_config) = accessor {
        let mut property = ClassPropertyDescriptor {
            owner,
            public_name: accessor_config.property_name.clone(),
            scope,
            getter: None,
            setter: None,
        };
        let accessor_descriptor = ClassPropertyAccessorDescriptor {
            native_symbol_name: register_symbol_name.to_string(),
        };
        match accessor_config.kind {
            AccessorKind::Getter => property.getter = Some(accessor_descriptor),
            AccessorKind::Setter => property.setter = Some(accessor_descriptor),
        }
        ClassDescriptorMember::Property(property)
    } else {
        ClassDescriptorMember::Method(ClassCallableDescriptor {
            owner,
            public_name: ets_name.to_string(),
            native_symbol_name: register_symbol_name.to_string(),
            scope,
        })
    };
    let register = descriptor.register_descriptor();

    Some(ResolvedClassBinding {
        descriptor,
        register,
    })
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

pub(crate) fn resolve_register_target(
    attrs: &BindgenAttrs,
    class_register: Option<&ClassRegisterDescriptor>,
) -> RegisterTarget {
    let module_name = current_module_name();

    if let Some(class_register) = class_register {
        RegisterTarget::Class {
            descriptor: class_to_descriptor(&qualify_member_descriptor(
                &class_register.owner,
                &module_name,
            )),
            scope: class_register.scope,
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
    fn binding_plan_supports_nullish_constructor_params() {
        let attrs = BindgenAttrs {
            constructor: true,
            class: Some("Person".to_string()),
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn ctor(name: Option<String>)
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Constructor,
            wrapper_binding_kind: WrapperBindingKind::ClassInstance,
            skip_first_arg: false,
        };
        let plan = resolve_binding_plan(&attrs, "ctor", &sig, binding_input)
            .expect("binding plan should resolve");
        match &plan.ets {
            EtsBindingEmission::ClassMember { rendered } => {
                assert_eq!(
                    rendered,
                    "native constructor(name: String | null | undefined);"
                );
            }
            other => panic!("expected class member emission, got {other:?}"),
        }
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
    fn binding_plan_tracks_static_property_descriptor() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            getter: Some("value".to_string()),
            is_static: true,
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn get_value() -> i32
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            wrapper_binding_kind: WrapperBindingKind::ClassStatic,
            skip_first_arg: false,
        };
        let plan = resolve_binding_plan(&attrs, "get_value", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                owner: "Widget".to_string(),
                public_name: "value".to_string(),
                scope: ClassMemberScope::Static,
                getter: Some(ClassPropertyAccessorDescriptor {
                    native_symbol_name: "__ani_native_get_value".to_string(),
                }),
                setter: None,
            }))
        );
    }

    #[test]
    fn binding_plan_tracks_constructor_descriptor() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            constructor: true,
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn ctor(name: String)
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Constructor,
            wrapper_binding_kind: WrapperBindingKind::ClassInstance,
            skip_first_arg: false,
        };
        let plan = resolve_binding_plan(&attrs, "ctor", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Constructor(
                ClassCallableDescriptor {
                    owner: "Widget".to_string(),
                    public_name: "constructor".to_string(),
                    native_symbol_name: "<ctor>".to_string(),
                    scope: ClassMemberScope::Instance,
                }
            ))
        );
        match &plan.ets {
            EtsBindingEmission::ClassMember { rendered } => {
                assert_eq!(rendered, "native constructor(name: string);");
            }
            other => panic!("expected class member emission, got {other:?}"),
        }
    }

    #[test]
    fn binding_plan_register_target_uses_member_scope() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            getter: Some("value".to_string()),
            is_static: true,
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn get_value() -> i32
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            wrapper_binding_kind: WrapperBindingKind::ClassStatic,
            skip_first_arg: false,
        };
        let plan = resolve_binding_plan(&attrs, "get_value", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.register_target,
            RegisterTarget::Class {
                descriptor: "ani_derive.Widget".to_string(),
                scope: ClassMemberScope::Static,
            }
        );
        assert_eq!(
            plan.class_register,
            Some(ClassRegisterDescriptor {
                owner: "Widget".to_string(),
                scope: ClassMemberScope::Static,
            })
        );
    }

    #[test]
    fn binding_plan_tracks_method_descriptor_and_register() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn rename(name: String) -> String
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            wrapper_binding_kind: WrapperBindingKind::ClassInstance,
            skip_first_arg: false,
        };
        let plan = resolve_binding_plan(&attrs, "rename", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Method(
                crate::codegen::ClassCallableDescriptor {
                    owner: "Widget".to_string(),
                    public_name: "rename".to_string(),
                    native_symbol_name: "rename".to_string(),
                    scope: ClassMemberScope::Instance,
                }
            ))
        );
        assert_eq!(
            plan.class_register,
            Some(ClassRegisterDescriptor {
                owner: "Widget".to_string(),
                scope: ClassMemberScope::Instance,
            })
        );
    }

    #[test]
    fn binding_plan_tracks_distinct_property_and_native_names() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            getter: Some("total".to_string()),
            name: Some("native_get_total".to_string()),
            is_static: true,
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn get_total() -> i32
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            wrapper_binding_kind: WrapperBindingKind::ClassStatic,
            skip_first_arg: false,
        };
        let plan = resolve_binding_plan(&attrs, "get_total", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                owner: "Widget".to_string(),
                public_name: "total".to_string(),
                scope: ClassMemberScope::Static,
                getter: Some(ClassPropertyAccessorDescriptor {
                    native_symbol_name: "__ani_native_native_get_total".to_string(),
                }),
                setter: None,
            }))
        );
        match &plan.ets {
            EtsBindingEmission::ClassMember { rendered } => {
                assert!(rendered.contains("static native __ani_native_native_get_total(): int;"));
                assert!(rendered.contains("static get total(): int"));
            }
            other => panic!("expected class member emission, got {other:?}"),
        }
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
