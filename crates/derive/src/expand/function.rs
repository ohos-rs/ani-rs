//! Function Expansion
//!
//! Expands `#[ani]` macro for functions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Signature, Type};

use crate::codegen::{
    ClassCallableDescriptor, ClassDescriptorMember, ClassMemberMetadata, ClassMemberScope,
    ClassPropertyAccessorDescriptor, ClassPropertyDescriptor, ClassRegisterDescriptor,
    EtsBindingEmission, EtsBindingTarget, ExportPlan, RegisterTarget, WrapperBindingKind,
    emit_export_plan_ets, generate_register_fn, generate_wrapper,
};
use crate::parser::{BindgenAttrs, InitAttrs};
use crate::types::{
    EtsDeclKind, class_to_descriptor, current_module_name, function_requires_nullish_bridge,
    generate_ctor_signature, generate_fn_ets_binding, generate_fn_signature, module_to_descriptor,
    namespace_to_descriptor, qualify_member_descriptor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CallableKind {
    Function,
    Constructor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BindingOwner {
    Global,
    Class { scope: ClassMemberScope },
}

impl BindingOwner {
    pub(crate) fn wrapper_binding_kind(self) -> WrapperBindingKind {
        match self {
            BindingOwner::Global => WrapperBindingKind::Global,
            BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            } => WrapperBindingKind::ClassInstance,
            BindingOwner::Class {
                scope: ClassMemberScope::Static,
            } => WrapperBindingKind::ClassStatic,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SignatureBindingStyle {
    Direct,
    SkipRustReceiver,
}

impl SignatureBindingStyle {
    pub(crate) fn skip_first_arg(self) -> bool {
        matches!(self, SignatureBindingStyle::SkipRustReceiver)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BindingResolveInput {
    pub callable_kind: CallableKind,
    pub owner: BindingOwner,
    pub signature_style: SignatureBindingStyle,
}

impl BindingResolveInput {
    pub(crate) fn is_constructor(self) -> bool {
        matches!(self.callable_kind, CallableKind::Constructor)
    }

    pub(crate) fn is_static(self) -> bool {
        matches!(
            self.owner,
            BindingOwner::Class {
                scope: ClassMemberScope::Static,
            }
        )
    }

    pub(crate) fn wrapper_binding_kind(self) -> WrapperBindingKind {
        self.owner.wrapper_binding_kind()
    }

    pub(crate) fn skip_first_arg(self) -> bool {
        self.signature_style.skip_first_arg()
    }

    pub(crate) fn class_scope(self) -> Option<ClassMemberScope> {
        match self.owner {
            BindingOwner::Global => None,
            BindingOwner::Class { scope } => Some(scope),
        }
    }

    pub(crate) fn requires_nullish_bridge(self, sig: &Signature) -> bool {
        function_requires_nullish_bridge(sig, self.skip_first_arg())
    }

    pub(crate) fn native_signature(self, sig: &Signature) -> String {
        if self.is_constructor() {
            generate_ctor_signature(sig, self.skip_first_arg())
        } else {
            generate_fn_signature(sig, self.skip_first_arg())
        }
    }

    pub(crate) fn render_ets_binding(
        self,
        kind: EtsDeclKind,
        sig: &Signature,
        ets_name: &str,
    ) -> String {
        generate_fn_ets_binding(kind, sig, ets_name, self.skip_first_arg(), self.is_static())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedCallableBinding {
    input: BindingResolveInput,
    signature: String,
    requires_nullish_bridge: bool,
}

impl ResolvedCallableBinding {
    fn new(attrs: &BindgenAttrs, sig: &Signature, input: BindingResolveInput) -> Self {
        Self {
            input,
            signature: attrs
                .signature
                .clone()
                .unwrap_or_else(|| input.native_signature(sig)),
            requires_nullish_bridge: input.requires_nullish_bridge(sig),
        }
    }

    fn register_symbol_name(
        &self,
        ets_name: &str,
        class_member_plan: Option<&ResolvedClassMemberPlan>,
    ) -> String {
        class_member_plan
            .map(|plan| plan.register_symbol_name(ets_name, self.requires_nullish_bridge))
            .unwrap_or_else(|| {
                if self.requires_nullish_bridge {
                    format!("__ani_native_{ets_name}")
                } else {
                    ets_name.to_string()
                }
            })
    }

    fn render_ets_emission(
        &self,
        ets_target: EtsBindingTarget,
        sig: &Signature,
        ets_name: &str,
        class_descriptor: Option<&ClassDescriptorMember>,
    ) -> EtsBindingEmission {
        if let Some(class_descriptor) = class_descriptor {
            EtsBindingEmission::ClassMember {
                rendered: class_descriptor.render_ets_binding(sig, self.input.skip_first_arg()),
            }
        } else {
            EtsBindingEmission::Rendered {
                target: ets_target.clone(),
                rendered: self
                    .input
                    .render_ets_binding(ets_target.kind, sig, ets_name),
            }
        }
    }
}

fn resolve_function_binding_input(attrs: &BindgenAttrs, func: &ItemFn) -> BindingResolveInput {
    let callable_kind = if attrs.constructor {
        CallableKind::Constructor
    } else {
        CallableKind::Function
    };
    let has_self = func
        .sig
        .inputs
        .first()
        .is_some_and(|arg| matches!(arg, FnArg::Receiver(_)));
    let owner = if attrs.class.is_some() {
        BindingOwner::Class {
            scope: if attrs.constructor || !attrs.is_static {
                ClassMemberScope::Instance
            } else {
                ClassMemberScope::Static
            },
        }
    } else {
        BindingOwner::Global
    };

    BindingResolveInput {
        callable_kind,
        owner,
        signature_style: if has_self {
            SignatureBindingStyle::SkipRustReceiver
        } else {
            SignatureBindingStyle::Direct
        },
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
    let wrapper = generate_wrapper(&func, &wrapper_name, binding_input.wrapper_binding_kind());

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
enum ClassMemberPlanKind {
    Constructor,
    Method { public_name: String },
    Property(AccessorConfig),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedClassMemberPlan {
    owner: String,
    scope: ClassMemberScope,
    kind: ClassMemberPlanKind,
}

impl ResolvedClassMemberPlan {
    pub(crate) fn property_name(&self) -> Option<&str> {
        match &self.kind {
            ClassMemberPlanKind::Property(accessor) => Some(accessor.property_name.as_str()),
            _ => None,
        }
    }

    fn register_symbol_name(&self, ets_name: &str, requires_nullish_bridge: bool) -> String {
        match self.kind {
            ClassMemberPlanKind::Constructor => "<ctor>".to_string(),
            ClassMemberPlanKind::Property(_) => format!("__ani_native_{ets_name}"),
            ClassMemberPlanKind::Method { .. } if requires_nullish_bridge => {
                format!("__ani_native_{ets_name}")
            }
            ClassMemberPlanKind::Method { .. } => ets_name.to_string(),
        }
    }

    fn descriptor(&self, native_symbol_name: &str) -> ClassDescriptorMember {
        match &self.kind {
            ClassMemberPlanKind::Constructor => {
                ClassDescriptorMember::Constructor(ClassCallableDescriptor {
                    metadata: ClassMemberMetadata {
                        owner: self.owner.clone(),
                        public_name: "constructor".to_string(),
                        scope: self.scope,
                    },
                    native_symbol_name: native_symbol_name.to_string(),
                })
            }
            ClassMemberPlanKind::Method { public_name } => {
                ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: ClassMemberMetadata {
                        owner: self.owner.clone(),
                        public_name: public_name.clone(),
                        scope: self.scope,
                    },
                    native_symbol_name: native_symbol_name.to_string(),
                })
            }
            ClassMemberPlanKind::Property(accessor) => {
                let mut property = ClassPropertyDescriptor {
                    metadata: ClassMemberMetadata {
                        owner: self.owner.clone(),
                        public_name: accessor.property_name.clone(),
                        scope: self.scope,
                    },
                    getter: None,
                    setter: None,
                };
                let accessor_descriptor = ClassPropertyAccessorDescriptor {
                    native_symbol_name: native_symbol_name.to_string(),
                };
                match accessor.kind {
                    AccessorKind::Getter => property.getter = Some(accessor_descriptor),
                    AccessorKind::Setter => property.setter = Some(accessor_descriptor),
                }
                ClassDescriptorMember::Property(property)
            }
        }
    }
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
        AccessorKind::Getter => validate_getter_signature(sig, binding_input.skip_first_arg())?,
        AccessorKind::Setter => validate_setter_signature(sig, binding_input.skip_first_arg())?,
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
    let class_member_plan =
        resolve_class_member_plan(attrs, rust_name, sig, binding_input, &ets_name)?;
    resolve_binding_plan_with_class_plan(
        attrs,
        &ets_name,
        sig,
        binding_input,
        class_member_plan.as_ref(),
    )
}

pub(crate) fn resolve_binding_plan_with_class_plan(
    attrs: &BindgenAttrs,
    ets_name: &str,
    sig: &Signature,
    binding_input: BindingResolveInput,
    class_member_plan: Option<&ResolvedClassMemberPlan>,
) -> syn::Result<ExportPlan> {
    let callable = ResolvedCallableBinding::new(attrs, sig, binding_input);
    let register_symbol_name = callable.register_symbol_name(ets_name, class_member_plan);
    let ets_target = resolve_ets_binding_target(attrs);
    let class_descriptor = class_member_plan.map(|plan| plan.descriptor(&register_symbol_name));
    let class_register = class_descriptor
        .as_ref()
        .map(ClassDescriptorMember::register_descriptor);
    let ets = callable.render_ets_emission(ets_target, sig, ets_name, class_descriptor.as_ref());

    Ok(ExportPlan {
        register_symbol_name,
        signature: callable.signature,
        register_target: resolve_register_target(attrs, class_register.as_ref()),
        ets,
        class_descriptor,
        class_register,
    })
}

pub(crate) fn resolve_class_member_plan(
    attrs: &BindgenAttrs,
    rust_name: &str,
    sig: &Signature,
    binding_input: BindingResolveInput,
    ets_name: &str,
) -> syn::Result<Option<ResolvedClassMemberPlan>> {
    let owner = match attrs.class.clone() {
        Some(owner) => owner,
        None => return Ok(None),
    };
    let scope = binding_input
        .class_scope()
        .expect("class bindings must include class scope");

    if binding_input.is_constructor() {
        return Ok(Some(ResolvedClassMemberPlan {
            owner,
            scope,
            kind: ClassMemberPlanKind::Constructor,
        }));
    }

    if let Some(accessor) = resolve_accessor_config(attrs, rust_name, sig, binding_input)? {
        return Ok(Some(ResolvedClassMemberPlan {
            owner,
            scope,
            kind: ClassMemberPlanKind::Property(accessor),
        }));
    }

    Ok(Some(ResolvedClassMemberPlan {
        owner,
        scope,
        kind: ClassMemberPlanKind::Method {
            public_name: ets_name.to_string(),
        },
    }))
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
    fn binding_input_maps_owner_and_signature_style_into_runtime_flags() {
        let input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Static,
            },
            signature_style: SignatureBindingStyle::SkipRustReceiver,
        };

        assert!(input.is_static());
        assert_eq!(input.class_scope(), Some(ClassMemberScope::Static));
        assert_eq!(
            input.wrapper_binding_kind(),
            WrapperBindingKind::ClassStatic
        );
        assert!(input.skip_first_arg());
    }

    #[test]
    fn binding_input_computes_native_signatures_and_bridge_policy() {
        let method_sig: Signature = parse_quote! {
            fn rename(&self, value: Option<String>) -> Option<String>
        };
        let method_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::SkipRustReceiver,
        };
        assert!(method_input.requires_nullish_bridge(&method_sig));
        assert_eq!(
            method_input.native_signature(&method_sig),
            "X{C{std.core.String}C{std.core.Null}}:X{C{std.core.String}C{std.core.Null}}"
        );

        let ctor_sig: Signature = parse_quote! {
            fn new(name: String)
        };
        let ctor_input = BindingResolveInput {
            callable_kind: CallableKind::Constructor,
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::Direct,
        };
        assert_eq!(
            ctor_input.native_signature(&ctor_sig),
            "C{std.core.String}:"
        );
    }

    #[test]
    fn resolved_callable_binding_centralizes_symbol_and_ets_rendering() {
        let attrs = BindgenAttrs::default();
        let sig: Signature = parse_quote! {
            fn maybe_name(name: Option<String>) -> Option<String>
        };
        let input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            owner: BindingOwner::Global,
            signature_style: SignatureBindingStyle::Direct,
        };
        let callable = ResolvedCallableBinding::new(&attrs, &sig, input);

        assert!(callable.requires_nullish_bridge);
        assert_eq!(
            callable.signature,
            "X{C{std.core.String}C{std.core.Null}}:X{C{std.core.String}C{std.core.Null}}"
        );
        assert_eq!(
            callable.register_symbol_name("maybe_name", None),
            "__ani_native_maybe_name"
        );

        let ets = callable.render_ets_emission(
            EtsBindingTarget {
                kind: EtsDeclKind::Global,
                target: String::new(),
            },
            &sig,
            "maybe_name",
            None,
        );
        assert_eq!(
            ets,
            EtsBindingEmission::Rendered {
                target: EtsBindingTarget {
                    kind: EtsDeclKind::Global,
                    target: String::new(),
                },
                rendered: "native function __ani_native_maybe_name(name: string | null): string | null;\nfunction maybe_name(name: string | null | undefined): string | null | undefined {\n  let __ani_result = __ani_native_maybe_name(name == undefined ? null : name);\n  return __ani_result == null ? undefined : __ani_result;\n}".to_string(),
            }
        );
    }

    #[test]
    fn class_member_plan_tracks_property_and_register_symbol_policy() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            getter: Some("value".to_string()),
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn get_value() -> i32
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::Direct,
        };

        let plan = resolve_class_member_plan(&attrs, "get_value", &sig, binding_input, "get_value")
            .expect("class member plan should resolve")
            .expect("class member plan should exist");

        assert_eq!(plan.property_name(), Some("value"));
        assert_eq!(
            plan.register_symbol_name("get_value", false),
            "__ani_native_get_value"
        );
    }

    #[test]
    fn class_member_plan_uses_ets_name_for_method_descriptor() {
        let attrs = BindgenAttrs {
            class: Some("Widget".to_string()),
            name: Some("renamePublic".to_string()),
            ..Default::default()
        };
        let sig: Signature = parse_quote! {
            fn rename(name: String) -> String
        };
        let binding_input = BindingResolveInput {
            callable_kind: CallableKind::Function,
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::Direct,
        };

        let plan = resolve_class_member_plan(&attrs, "rename", &sig, binding_input, "renamePublic")
            .expect("class member plan should resolve")
            .expect("class member plan should exist");

        assert_eq!(plan.property_name(), None);
        assert_eq!(
            plan.register_symbol_name("renamePublic", false),
            "renamePublic"
        );
        assert_eq!(
            plan.descriptor("renamePublic"),
            ClassDescriptorMember::Method(ClassCallableDescriptor {
                metadata: ClassMemberMetadata {
                    owner: "Widget".to_string(),
                    public_name: "renamePublic".to_string(),
                    scope: ClassMemberScope::Instance,
                },
                native_symbol_name: "renamePublic".to_string(),
            })
        );
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
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::Direct,
        };
        let plan = resolve_binding_plan(&attrs, "ctor", &sig, binding_input)
            .expect("binding plan should resolve");
        match &plan.ets {
            EtsBindingEmission::ClassMember { rendered } => {
                assert_eq!(
                    rendered,
                    "native constructor(name: string | null | undefined);"
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
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Static,
            },
            signature_style: SignatureBindingStyle::Direct,
        };
        let plan = resolve_binding_plan(&attrs, "get_value", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                metadata: ClassMemberMetadata {
                    owner: "Widget".to_string(),
                    public_name: "value".to_string(),
                    scope: ClassMemberScope::Static,
                },
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
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::Direct,
        };
        let plan = resolve_binding_plan(&attrs, "ctor", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Constructor(
                ClassCallableDescriptor {
                    metadata: ClassMemberMetadata {
                        owner: "Widget".to_string(),
                        public_name: "constructor".to_string(),
                        scope: ClassMemberScope::Instance,
                    },
                    native_symbol_name: "<ctor>".to_string(),
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
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Static,
            },
            signature_style: SignatureBindingStyle::Direct,
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
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Instance,
            },
            signature_style: SignatureBindingStyle::Direct,
        };
        let plan = resolve_binding_plan(&attrs, "rename", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Method(
                crate::codegen::ClassCallableDescriptor {
                    metadata: ClassMemberMetadata {
                        owner: "Widget".to_string(),
                        public_name: "rename".to_string(),
                        scope: ClassMemberScope::Instance,
                    },
                    native_symbol_name: "rename".to_string(),
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
            owner: BindingOwner::Class {
                scope: ClassMemberScope::Static,
            },
            signature_style: SignatureBindingStyle::Direct,
        };
        let plan = resolve_binding_plan(&attrs, "get_total", &sig, binding_input)
            .expect("binding plan should resolve");
        assert_eq!(
            plan.class_descriptor,
            Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                metadata: ClassMemberMetadata {
                    owner: "Widget".to_string(),
                    public_name: "total".to_string(),
                    scope: ClassMemberScope::Static,
                },
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
