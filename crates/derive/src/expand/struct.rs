//! Struct Expansion
//!
//! Expands `#[ani]` macro for structs and derive macros.

use std::collections::BTreeSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, ExprLit, Field, Fields, GenericParam, Generics, Ident, Index,
    ItemStruct, Lit, Member, Token, Variant, punctuated::Punctuated,
};

use crate::parser::{AniAttrs, AttrItem, AttrValue, BindgenAttrs};
use crate::types::ani_type::{
    AniType, ObjectMemberAccessKind, ObjectMemberDescriptor, PrimitiveType,
    register_exact_type_alias, register_object_type_alias, register_object_type_members,
    register_structured_type_alias,
};
use crate::types::{
    EtsDeclKind, EtsObjectMemberDecl, EtsObjectMemberKind, current_module_name,
    emit_compile_ets_object, emit_compile_ets_rendered_decl, ets_public_type_for_syn_type,
    generate_object_field_ets_decl, generate_object_property_ets_decl, qualify_member_descriptor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectAccessKind {
    Field,
    Property,
}

#[derive(Clone)]
struct ObjectFieldSpec {
    member: Member,
    rust_name: String,
    arkts_name: String,
    ty: syn::Type,
    access: ObjectAccessKind,
    emit_private: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObjectStructKind {
    Named,
    Unnamed,
    Unit,
}

#[derive(Default)]
struct ObjectFieldAttrs {
    name: Option<String>,
    property: bool,
    property_name: Option<String>,
}

#[derive(Clone)]
struct EnumVariantSpec {
    rust_ident: Ident,
    arkts_name: String,
    discriminant: i32,
}

#[derive(Clone, Debug)]
struct StructuredEnumFieldSpec {
    rust_name: String,
    arkts_name: String,
    ty: syn::Type,
    input: bool,
    output: bool,
}

#[derive(Clone, Debug)]
enum StructuredEnumShapeSpec {
    Unit,
    Newtype(syn::Type),
    Tuple(Vec<syn::Type>),
    Struct(Vec<StructuredEnumFieldSpec>),
}

#[derive(Clone, Debug)]
struct StructuredEnumVariantSpec {
    serde_name: String,
    arkts_name: String,
    input: bool,
    output: bool,
    shape: StructuredEnumShapeSpec,
}

#[derive(Clone, Debug)]
struct StructuredEnumOptions {
    discriminator: String,
    case: Option<String>,
    nullable: bool,
    input_only: bool,
    output_only: bool,
}

/// Expand `#[ani]` for structs
pub fn expand_struct(
    attrs: BindgenAttrs,
    struct_item: ItemStruct,
    prepare: TokenStream,
) -> TokenStream {
    if attrs.transparent || attrs.array {
        return expand_transparent_struct(struct_item, prepare, attrs.array);
    }
    if let Err(err) = validate_object_item_struct(&struct_item) {
        return err.to_compile_error();
    }

    let struct_name = &struct_item.ident;
    let class_name = attrs
        .class
        .clone()
        .unwrap_or_else(|| struct_name.to_string());
    let object_impls = match expand_object_type_impls(
        struct_name,
        &class_name,
        &struct_item.generics,
        &struct_item.fields,
    ) {
        Ok(tokens) => tokens,
        Err(err) => return err.to_compile_error(),
    };

    quote! {
        #prepare
        #struct_item
        #object_impls
    }
}

fn expand_transparent_struct(
    struct_item: ItemStruct,
    prepare: TokenStream,
    array_mode: bool,
) -> TokenStream {
    if !struct_item.generics.params.is_empty() {
        return syn::Error::new_spanned(
            &struct_item.generics,
            "#[ani(transparent)] and #[ani(array)] do not support generic newtypes",
        )
        .to_compile_error();
    }

    let (field_ty, read, construct) = match &struct_item.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let ty = fields.unnamed.first().expect("length checked").ty.clone();
            (ty, quote! { self.0 }, quote! { Self(value) })
        }
        Fields::Named(fields) if fields.named.len() == 1 => {
            let field = fields.named.first().expect("length checked");
            let Some(ident) = field.ident.clone() else {
                unreachable!("named field has an identifier")
            };
            let ty = field.ty.clone();
            (
                ty,
                quote! { self.#ident },
                quote! { Self { #ident: value } },
            )
        }
        _ => {
            return syn::Error::new_spanned(
                &struct_item.fields,
                "#[ani(transparent)] and #[ani(array)] require exactly one field",
            )
            .to_compile_error();
        }
    };

    let inner = AniType::from_syn_type(&field_ty);
    if array_mode
        && !matches!(
            inner,
            AniType::Wrapper(crate::types::ani_type::WrapperType::Vec(_))
                | AniType::ArrayBuffer
                | AniType::TypedArray(_)
                | AniType::FixedArray(_)
                | AniType::ArrayHandle(_)
        )
    {
        return syn::Error::new_spanned(
            &field_ty,
            "#[ani(array)] inner field must be Vec<T>, ArrayBuffer, TypedArray, or an ANI array wrapper",
        )
        .to_compile_error();
    }
    register_exact_type_alias(&struct_item.ident.to_string(), &field_ty);

    let name = &struct_item.ident;
    quote! {
        #prepare
        #struct_item

        impl ani::conversions::TypeInfo for #name {
            fn type_signature() -> &'static str {
                <#field_ty as ani::conversions::TypeInfo>::type_signature()
            }

            fn ani_c_type() -> &'static str {
                <#field_ty as ani::conversions::TypeInfo>::ani_c_type()
            }

            fn is_primitive() -> bool {
                <#field_ty as ani::conversions::TypeInfo>::is_primitive()
            }
        }

        impl<'env> ani::conversions::ToAni<'env> for #name
        where
            #field_ty: ani::conversions::ToAni<'env>,
        {
            type Output = <#field_ty as ani::conversions::ToAni<'env>>::Output;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                ani::conversions::ToAni::to_ani(#read, env)
            }
        }

        impl<'env> ani::conversions::FromAni<'env> for #name
        where
            #field_ty: ani::conversions::FromAni<'env>,
        {
            type Input = <#field_ty as ani::conversions::FromAni<'env>>::Input;

            unsafe fn from_ani(
                env: &ani::env::Env<'env>,
                input: Self::Input,
            ) -> ani::error::Result<Self> {
                let value = unsafe {
                    <#field_ty as ani::conversions::FromAni<'env>>::from_ani(env, input)
                }?;
                Ok(#construct)
            }
        }
    }
}

fn expand_object_type_impls(
    struct_name: &syn::Ident,
    class_name: &str,
    generics: &Generics,
    fields: &Fields,
) -> syn::Result<TokenStream> {
    let type_params = collect_struct_type_params(generics)?;
    let (struct_kind, field_specs) = collect_object_field_specs(fields)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let env_generics = generics_with_env(generics);
    let (env_impl_generics, _, env_where_clause) = env_generics.split_for_impl();
    let field_bounds = object_field_bounds(&field_specs);
    let method_where_clause = render_where_clause(None, &field_bounds);
    let env_where_with_fields = render_where_clause(env_where_clause, &field_bounds);
    let env_where_with_record_bounds = render_where_clause(
        env_where_clause,
        &[
            quote! { Self: ani::conversions::ToAni<'env, Output = ani::sys::ani_object> },
            quote! { Self: ani::conversions::FromAni<'env, Input = ani::sys::ani_object> },
        ],
    );
    let env_where_with_to_ani = render_where_clause(
        env_where_clause,
        &[quote! { Self: ani::conversions::ToAni<'env, Output = ani::sys::ani_object> }],
    );

    register_object_type_alias(&struct_name.to_string(), class_name);
    register_object_type_members(
        &struct_name.to_string(),
        &field_specs
            .iter()
            .map(|field| ObjectMemberDescriptor {
                rust_name: field.rust_name.clone(),
                arkts_name: field.arkts_name.clone(),
                access: match field.access {
                    ObjectAccessKind::Field => ObjectMemberAccessKind::Field,
                    ObjectAccessKind::Property => ObjectMemberAccessKind::Property,
                },
            })
            .collect::<Vec<_>>(),
    );
    emit_object_decl(class_name, &type_params, &field_specs);

    let module_name = current_module_name();
    let qualified_name = qualify_member_descriptor(class_name, &module_name);
    let class_descriptor = object_descriptor_signature(&qualified_name);
    let qualified_name_lit = syn::LitStr::new(&qualified_name, Span::call_site());
    let class_descriptor_lit = syn::LitStr::new(&class_descriptor, Span::call_site());

    let field_reads = render_field_reads(struct_kind, &field_specs);
    let field_writes_to_new = field_specs.iter().map(generate_field_write_to_new);
    let field_writes_back = field_specs.iter().map(generate_field_write_back);

    Ok(quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// ANI class descriptor usable with `Env::find_class`.
            pub const fn class_descriptor() -> &'static str {
                #class_descriptor_lit
            }

            /// Qualified ArkTS object name.
            pub const fn arkts_name() -> &'static str {
                #qualified_name_lit
            }

            /// Reconstructs this Rust value from an ANI object after its class
            /// has already been validated by the generated wrapper.
            ///
            /// # Safety
            ///
            /// `value` must be a live object owned by the VM associated with
            /// `env`, and it must be an instance of this generated ANI class.
            #[doc(hidden)]
            pub unsafe fn __ani_from_bound_ani_object<'env>(
                env: &ani::env::Env<'env>,
                value: ani::sys::ani_object,
            ) -> ani::error::Result<Self>
            #method_where_clause
            {
                if value.is_null() {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidArgs,
                        format!("Null pointer: {}", stringify!(#struct_name)),
                    ));
                }

                let obj = unsafe { ani::types::AniObject::from_raw(value) };
                Ok(#field_reads)
            }
        }

        impl #impl_generics ani::conversions::TypeInfo for #struct_name #ty_generics #where_clause {
            fn type_signature() -> &'static str {
                Self::class_descriptor()
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }

        impl #env_impl_generics ani::conversions::FromAni<'env> for #struct_name #ty_generics
        #env_where_with_fields
        {
            type Input = ani::sys::ani_object;

            unsafe fn from_ani(env: &ani::env::Env<'env>, value: Self::Input) -> ani::error::Result<Self> {
                if value.is_null() {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidArgs,
                        format!("Null pointer: {}", stringify!(#struct_name)),
                    ));
                }

                let obj = unsafe { ani::types::AniObject::from_raw(value) };
                let class = env.find_class(Self::arkts_name())?;
                if !env.object_instance_of(&obj, &class)? {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidType,
                        format!(
                            "Expected object of type {}, got incompatible instance",
                            Self::arkts_name(),
                        ),
                    ));
                }

                unsafe { Self::__ani_from_bound_ani_object(env, value) }
            }
        }

        impl #env_impl_generics ani::conversions::ToAni<'env> for #struct_name #ty_generics
        #env_where_with_fields
        {
            type Output = ani::sys::ani_object;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                let class = env.find_class(Self::arkts_name())?;
                let ctor = env.find_constructor(&class, ":")?;
                let obj = env.new_object(&class, &ctor, &[])?;
                #(#field_writes_to_new)*
                Ok(obj.into_raw())
            }
        }

        impl #env_impl_generics ani::conversions::WriteBackToAniObject<'env>
            for #struct_name #ty_generics
        #env_where_with_fields
        {
            fn write_back_to_ani_object(
                self,
                env: &ani::env::Env<'env>,
                obj: &ani::types::AniObject<'_>,
            ) -> ani::error::Result<()> {
                #(#field_writes_back)*
                Ok(())
            }
        }

        impl #env_impl_generics ani::conversions::ValidateFromAni<'env>
            for #struct_name #ty_generics
        #env_where_clause
        {
            unsafe fn validate(env: &ani::env::Env<'env>, value: ani::sys::ani_object) -> bool {
                if value.is_null() {
                    return false;
                }

                let obj = unsafe { ani::types::AniObject::from_raw(value) };
                let class = match env.find_class(Self::arkts_name()) {
                    Ok(class) => class,
                    Err(_) => return false,
                };
                env.object_instance_of(&obj, &class).unwrap_or(false)
            }
        }

        impl #env_impl_generics ani::conversions::FromAniObject<'env>
            for #struct_name #ty_generics
        #env_where_with_record_bounds
        {
            unsafe fn from_ani_object(
                env: &ani::env::Env<'env>,
                value: ani::sys::ani_object,
            ) -> ani::error::Result<Self> {
                unsafe {
                    <Self as ani::conversions::FromAni<'env>>::from_ani(env, value)
                }
            }
        }

        impl #env_impl_generics ani::conversions::RecordValue<'env>
            for #struct_name #ty_generics
        #env_where_with_record_bounds
        {
            fn to_record_ref(self, env: &ani::env::Env<'env>) -> ani::error::Result<ani::types::AniRef<'env>> {
                let raw = <Self as ani::conversions::ToAni<'env>>::to_ani(self, env)?;
                Ok(unsafe { ani::types::AniRef::from_raw(raw as ani::sys::ani_ref) })
            }

            fn from_record_ref(
                env: &ani::env::Env<'env>,
                value: &ani::types::AniRef<'env>,
            ) -> ani::error::Result<Self> {
                unsafe {
                    <Self as ani::conversions::FromAni<'env>>::from_ani(
                        env,
                        value.as_raw() as ani::sys::ani_object,
                    )
                }
            }
        }

        impl #env_impl_generics ani::conversions::ToAniObject<'env>
            for #struct_name #ty_generics
        #env_where_with_to_ani
        {
            fn to_ani_object(self, env: &ani::env::Env<'env>) -> ani::error::Result<ani::sys::ani_object> {
                <Self as ani::conversions::ToAni<'env>>::to_ani(self, env)
            }
        }
    })
}

fn generate_field_read_expr(field: &ObjectFieldSpec) -> TokenStream {
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, Span::call_site());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::get_named_field(
                env,
                &obj,
                #field_literal,
            )?
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::get_named_property(
                env,
                &obj,
                #field_literal,
            )?
        },
    }
}

fn generate_field_write_to_new(field: &ObjectFieldSpec) -> TokenStream {
    let member = &field.member;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, Span::call_site());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#member,
                env,
                &obj,
                #field_literal,
            )?;
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::set_named_property(
                self.#member,
                env,
                &obj,
                #field_literal,
            )?;
        },
    }
}

fn generate_field_write_back(field: &ObjectFieldSpec) -> TokenStream {
    let member = &field.member;
    let field_ty = &field.ty;
    let field_literal = syn::LitStr::new(&field.arkts_name, Span::call_site());
    match field.access {
        ObjectAccessKind::Field => quote! {
            <#field_ty as ani::conversions::ObjectField<'env>>::set_named_field(
                self.#member,
                env,
                obj,
                #field_literal,
            )?;
        },
        ObjectAccessKind::Property => quote! {
            <#field_ty as ani::conversions::ObjectProperty<'env>>::set_named_property(
                self.#member,
                env,
                obj,
                #field_literal,
            )?;
        },
    }
}

fn render_field_reads(kind: ObjectStructKind, fields: &[ObjectFieldSpec]) -> TokenStream {
    match kind {
        ObjectStructKind::Named => {
            let initializers = fields.iter().map(|field| {
                let Member::Named(field_name) = &field.member else {
                    unreachable!("named struct field should use named member");
                };
                let value = generate_field_read_expr(field);
                quote! { #field_name: #value }
            });
            quote! {
                Self {
                    #(#initializers,)*
                }
            }
        }
        ObjectStructKind::Unnamed => {
            let values = fields.iter().map(generate_field_read_expr);
            quote! {
                Self(
                    #(#values,)*
                )
            }
        }
        ObjectStructKind::Unit => quote! { Self },
    }
}

fn emit_object_decl(class_name: &str, type_params: &[String], fields: &[ObjectFieldSpec]) {
    let object_field_decls = fields
        .iter()
        .map(|field| {
            let (kind, rendered) = match field.access {
                ObjectAccessKind::Field => (
                    EtsObjectMemberKind::Field,
                    generate_object_field_ets_decl(
                        &field.arkts_name,
                        &field.ty,
                        field.emit_private,
                    ),
                ),
                ObjectAccessKind::Property => (
                    EtsObjectMemberKind::Property,
                    generate_object_property_ets_decl(&field.arkts_name, &field.ty),
                ),
            };
            EtsObjectMemberDecl {
                name: field.arkts_name.clone(),
                kind,
                is_private: field.emit_private,
                rendered,
            }
        })
        .collect::<Vec<_>>();
    emit_compile_ets_object(class_name, type_params, &object_field_decls);
}

fn collect_object_field_specs(
    fields: &Fields,
) -> syn::Result<(ObjectStructKind, Vec<ObjectFieldSpec>)> {
    let mut member_names = BTreeSet::new();
    let mut push_field =
        |member: Member, rust_name: String, field: &Field, default_name: String| {
            let attrs = parse_object_field_attrs(field)?;
            let access = if attrs.property {
                ObjectAccessKind::Property
            } else {
                ObjectAccessKind::Field
            };
            let arkts_name = attrs.property_name.or(attrs.name).unwrap_or(default_name);
            if !member_names.insert(arkts_name.clone()) {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("duplicate ArkTS object member name `{arkts_name}`"),
                ));
            }
            let emit_private = matches!(access, ObjectAccessKind::Field)
                && rust_name.starts_with('_')
                && arkts_name == rust_name;
            Ok(ObjectFieldSpec {
                member,
                rust_name,
                arkts_name,
                ty: field.ty.clone(),
                access,
                emit_private,
            })
        };

    match fields {
        Fields::Named(fields) => {
            let mut out = Vec::with_capacity(fields.named.len());
            for field in &fields.named {
                let Some(rust_ident) = field.ident.clone() else {
                    continue;
                };
                let rust_name = rust_ident.to_string();
                out.push(push_field(
                    Member::Named(rust_ident),
                    rust_name.clone(),
                    field,
                    rust_name,
                )?);
            }
            Ok((ObjectStructKind::Named, out))
        }
        Fields::Unnamed(fields) => {
            let mut out = Vec::with_capacity(fields.unnamed.len());
            for (index, field) in fields.unnamed.iter().enumerate() {
                let rust_name = format!("field{index}");
                out.push(push_field(
                    Member::Unnamed(Index::from(index)),
                    rust_name.clone(),
                    field,
                    rust_name,
                )?);
            }
            Ok((ObjectStructKind::Unnamed, out))
        }
        Fields::Unit => Ok((ObjectStructKind::Unit, Vec::new())),
    }
}

fn parse_object_field_attrs(field: &Field) -> syn::Result<ObjectFieldAttrs> {
    let mut out = ObjectFieldAttrs::default();

    for attr in &field.attrs {
        if !attr.path().is_ident("ani") {
            continue;
        }

        let items = attr.parse_args_with(Punctuated::<AttrItem, Token![,]>::parse_terminated)?;
        for item in items {
            match item.key.to_string().as_str() {
                "name" => match item.value {
                    Some(AttrValue::Str(value)) => out.name = Some(value),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &item.key,
                            "#[ani(name = \"...\")] on object fields requires a string literal",
                        ));
                    }
                },
                "property" => {
                    out.property = true;
                    if let Some(AttrValue::Str(value)) = item.value {
                        out.property_name = Some(value);
                    } else if item.value.is_some() {
                        return Err(syn::Error::new_spanned(
                            &item.key,
                            "#[ani(property = \"...\")] on object fields requires a string literal",
                        ));
                    }
                }
                "getter" | "setter" | "constructor" | "module" | "namespace" | "class"
                | "static" | "async" => {
                    return Err(syn::Error::new_spanned(
                        &item.key,
                        format!("#[ani({})] is not supported on object fields", item.key),
                    ));
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        &item.key,
                        format!("Unknown object field attribute: {other}"),
                    ));
                }
            }
        }
    }

    Ok(out)
}

fn collect_struct_type_params(generics: &Generics) -> syn::Result<Vec<String>> {
    let mut out = Vec::new();
    for param in &generics.params {
        match param {
            GenericParam::Type(ty) => out.push(ty.ident.to_string()),
            GenericParam::Lifetime(lifetime) => {
                return Err(syn::Error::new_spanned(
                    lifetime,
                    "ANI object/class derives support type parameters only; lifetime parameters are not supported",
                ));
            }
            GenericParam::Const(const_param) => {
                return Err(syn::Error::new_spanned(
                    const_param,
                    "ANI object/class derives support type parameters only; const generics are not supported",
                ));
            }
        }
    }
    Ok(out)
}

fn validate_object_struct_shape(generics: &Generics) -> syn::Result<()> {
    collect_struct_type_params(generics).map(|_| ())
}

fn validate_object_item_struct(struct_item: &ItemStruct) -> syn::Result<()> {
    validate_object_struct_shape(&struct_item.generics)
}

fn object_descriptor_signature(qualified_name: &str) -> String {
    if qualified_name.starts_with('L') && qualified_name.ends_with(';') {
        qualified_name.to_string()
    } else {
        format!("L{};", qualified_name.replace('.', "/"))
    }
}

fn derive_named_type_name(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("ani") {
            continue;
        }

        let parsed = attr.parse_args::<AniAttrs>()?;
        if let Some(name) = parsed.object_name {
            return Ok(name);
        }
        if let Some(name) = parsed.class {
            return Ok(name);
        }
        if let Some(name) = parsed.name {
            return Ok(name);
        }
    }

    Ok(input.ident.to_string())
}

fn validate_derive_input(input: &DeriveInput) -> syn::Result<()> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(AniClass)] can only be applied to structs",
        ));
    };

    let _ = data;
    validate_object_struct_shape(&input.generics)
}

/// Expand AniClass derive macro
pub fn expand_class_derive(input: DeriveInput) -> TokenStream {
    if let Err(err) = validate_derive_input(&input) {
        return err.to_compile_error();
    }

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(
            &input,
            "#[derive(AniClass)] can only be applied to structs",
        )
        .to_compile_error();
    };

    let class_name = match derive_named_type_name(&input) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error(),
    };

    match expand_object_type_impls(&input.ident, &class_name, &input.generics, &data.fields) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn generics_with_env(generics: &Generics) -> Generics {
    let mut out = generics.clone();
    out.params.insert(0, syn::parse_quote!('env));
    out
}

fn render_where_clause(base: Option<&syn::WhereClause>, extra: &[TokenStream]) -> TokenStream {
    if extra.is_empty() {
        return base.map_or_else(TokenStream::new, |clause| quote! { #clause });
    }

    if let Some(base) = base {
        let predicates = &base.predicates;
        quote! {
            where
                #predicates,
                #(#extra),*
        }
    } else {
        quote! {
            where
                #(#extra),*
        }
    }
}

fn object_field_bounds(fields: &[ObjectFieldSpec]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_ty = &field.ty;
            match field.access {
                ObjectAccessKind::Field => {
                    quote! { #field_ty: ani::conversions::ObjectField<'env> }
                }
                ObjectAccessKind::Property => {
                    quote! { #field_ty: ani::conversions::ObjectProperty<'env> }
                }
            }
        })
        .collect()
}

fn validate_enum_derive_input(input: &DeriveInput) -> syn::Result<&syn::DataEnum> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "#[derive(AniEnum)] can only be applied to enums",
        ));
    };

    if let Some(parameter) = input
        .generics
        .params
        .iter()
        .find(|parameter| !matches!(parameter, GenericParam::Type(_)))
    {
        return Err(syn::Error::new_spanned(
            parameter,
            "#[derive(AniEnum)] supports generic type parameters, but not lifetime or const parameters",
        ));
    }

    Ok(data)
}

fn parse_enum_variant_arkts_name(variant: &Variant) -> syn::Result<Option<String>> {
    for attr in &variant.attrs {
        if !attr.path().is_ident("ani") {
            continue;
        }

        let parsed = attr.parse_args::<AniAttrs>()?;
        if let Some(name) = parsed.name {
            return Ok(Some(name));
        }
    }

    Ok(None)
}

fn parse_enum_discriminant(expr: &Expr) -> syn::Result<i32> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(value),
            ..
        }) => value.base10_parse::<i32>(),
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(value),
                ..
            }) = unary.expr.as_ref()
            {
                Ok(-value.base10_parse::<i32>()?)
            } else {
                Err(syn::Error::new_spanned(
                    expr,
                    "#[derive(AniEnum)] only supports integer literal discriminants",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "#[derive(AniEnum)] only supports integer literal discriminants",
        )),
    }
}

fn collect_enum_variant_specs(data: &syn::DataEnum) -> syn::Result<Vec<EnumVariantSpec>> {
    let mut specs = Vec::new();
    let mut next_discriminant = 0_i32;

    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &variant.fields,
                "#[derive(AniEnum)] currently only supports unit variants",
            ));
        }

        let discriminant = if let Some((_, expr)) = &variant.discriminant {
            parse_enum_discriminant(expr)?
        } else {
            next_discriminant
        };
        next_discriminant = discriminant.checked_add(1).ok_or_else(|| {
            syn::Error::new_spanned(
                variant,
                "#[derive(AniEnum)] discriminant overflow while deriving ANI enum",
            )
        })?;

        let arkts_name =
            parse_enum_variant_arkts_name(variant)?.unwrap_or_else(|| variant.ident.to_string());
        specs.push(EnumVariantSpec {
            rust_ident: variant.ident.clone(),
            arkts_name,
            discriminant,
        });
    }

    Ok(specs)
}

fn render_enum_decl(enum_name: &str, variants: &[EnumVariantSpec]) -> String {
    let members = variants
        .iter()
        .map(|variant| format!("  {} = {}", variant.arkts_name, variant.discriminant))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("enum {enum_name} {{\n{members}\n}}")
}

fn emit_enum_decl(qualified_name: &str, variants: &[EnumVariantSpec]) {
    let mut parts = qualified_name
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let enum_name = parts.pop().unwrap_or(qualified_name);
    let rendered = render_enum_decl(enum_name, variants);

    if parts.is_empty() {
        emit_compile_ets_rendered_decl(EtsDeclKind::Global, "", &rendered);
    } else {
        emit_compile_ets_rendered_decl(EtsDeclKind::Namespace, &parts.join("."), &rendered);
    }
}

fn expand_enum_type_impls(
    enum_name: &Ident,
    qualified_name: &str,
    variants: &[EnumVariantSpec],
) -> TokenStream {
    register_object_type_alias(&enum_name.to_string(), qualified_name);
    emit_enum_decl(qualified_name, variants);

    let descriptor = object_descriptor_signature(&qualify_member_descriptor(
        qualified_name,
        &current_module_name(),
    ));
    let descriptor_lit = syn::LitStr::new(&descriptor, Span::call_site());
    let qualified_name_lit = syn::LitStr::new(qualified_name, Span::call_site());

    let to_ani_arms = variants.iter().map(|variant| {
        let rust_ident = &variant.rust_ident;
        let arkts_name = syn::LitStr::new(&variant.arkts_name, Span::call_site());
        quote! { Self::#rust_ident => #arkts_name }
    });
    let from_ani_arms = variants.iter().map(|variant| {
        let rust_ident = &variant.rust_ident;
        let arkts_name = syn::LitStr::new(&variant.arkts_name, Span::call_site());
        quote! { #arkts_name => Ok(Self::#rust_ident) }
    });

    quote! {
        impl #enum_name {
            /// ANI enum descriptor usable with `Env::find_enum`.
            pub const fn enum_descriptor() -> &'static str {
                #descriptor_lit
            }

            /// Qualified ArkTS enum name.
            pub const fn arkts_name() -> &'static str {
                #qualified_name_lit
            }
        }

        impl ani::conversions::TypeInfo for #enum_name {
            fn type_signature() -> &'static str {
                Self::enum_descriptor()
            }

            fn ani_c_type() -> &'static str {
                "ani_enum_item"
            }
        }

        impl<'env> ani::conversions::ToAni<'env> for #enum_name {
            type Output = ani::sys::ani_enum_item;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                let enm = env.find_enum(Self::arkts_name())?;
                let item_name = match self {
                    #(#to_ani_arms,)*
                };
                let item = env.get_enum_item_by_name(&enm, item_name)?;
                Ok(item.into_raw())
            }
        }

        impl<'env> ani::conversions::FromAni<'env> for #enum_name {
            type Input = ani::sys::ani_enum_item;

            unsafe fn from_ani(env: &ani::env::Env<'env>, value: Self::Input) -> ani::error::Result<Self> {
                if value.is_null() {
                    return Err(ani::error::Error::new(
                        ani::error::Status::InvalidArgs,
                        format!("Null pointer: {}", stringify!(#enum_name)),
                    ));
                }

                let item = ani::conversions::EnumItem::from_handle(unsafe {
                    ani::types::AniEnumItem::from_raw(value)
                });
                let name = item.name(env)?;
                match name.as_str() {
                    #(#from_ani_arms,)*
                    _ => Err(ani::error::Error::new(
                        ani::error::Status::InvalidType,
                        format!("Unknown enum item `{}` for {}", name, stringify!(#enum_name)),
                    )),
                }
            }
        }
    }
}

fn rename_case(value: &str, case: Option<&str>) -> String {
    let Some(case) = case else {
        return value.to_string();
    };
    let mut words = Vec::new();
    let mut current = String::new();
    for (index, ch) in value.chars().enumerate() {
        if ch == '_' || ch == '-' || ch == ' ' {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        if ch.is_uppercase() && index != 0 && !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
        current.extend(ch.to_lowercase());
    }
    if !current.is_empty() {
        words.push(current);
    }
    match case {
        "snake_case" | "snake" => words.join("_"),
        "kebab-case" | "kebab" => words.join("-"),
        "lowercase" | "lower" => words.join(""),
        "UPPERCASE" | "upper" => words.join("").to_uppercase(),
        "camelCase" | "camel" => words
            .iter()
            .enumerate()
            .map(|(index, word)| {
                if index == 0 {
                    word.clone()
                } else {
                    let mut chars = word.chars();
                    chars
                        .next()
                        .map(|head| head.to_uppercase().collect::<String>() + chars.as_str())
                        .unwrap_or_default()
                }
            })
            .collect(),
        "PascalCase" | "pascal" => words
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                chars
                    .next()
                    .map(|head| head.to_uppercase().collect::<String>() + chars.as_str())
                    .unwrap_or_default()
            })
            .collect(),
        _ => value.to_string(),
    }
}

fn serde_rename(attrs: &[syn::Attribute]) -> syn::Result<Option<String>> {
    let mut rename = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("serde")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
            }
            Ok(())
        })?;
    }
    Ok(rename)
}

fn item_ani_attrs(attrs: &[syn::Attribute]) -> syn::Result<AniAttrs> {
    let mut merged = AniAttrs::default();
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("ani")) {
        let parsed = attr.parse_args::<AniAttrs>()?;
        if parsed.name.is_some() {
            merged.name = parsed.name;
        }
        if parsed.discriminant.is_some() {
            merged.discriminant = parsed.discriminant;
        }
        if parsed.case.is_some() {
            merged.case = parsed.case;
        }
        merged.skip |= parsed.skip;
        merged.input_only |= parsed.input_only;
        merged.output_only |= parsed.output_only;
        merged.nullable |= parsed.nullable;
    }
    if merged.input_only && merged.output_only {
        return Err(syn::Error::new_spanned(
            attrs.first().unwrap_or_else(|| unreachable!()),
            "an ANI enum item cannot be both input_only and output_only",
        ));
    }
    Ok(merged)
}

fn collect_structured_enum_specs(
    input: &DeriveInput,
    data: &syn::DataEnum,
) -> syn::Result<(StructuredEnumOptions, Vec<StructuredEnumVariantSpec>)> {
    let attrs = item_ani_attrs(&input.attrs)?;
    let options = StructuredEnumOptions {
        discriminator: attrs.discriminant.unwrap_or_else(|| "type".to_string()),
        case: attrs.case,
        nullable: attrs.nullable,
        input_only: attrs.input_only,
        output_only: attrs.output_only,
    };
    let mut variants = Vec::new();
    for variant in &data.variants {
        let attrs = item_ani_attrs(&variant.attrs)?;
        let rust_name = variant.ident.to_string();
        let serde_name = serde_rename(&variant.attrs)?.unwrap_or_else(|| rust_name.clone());
        let arkts_name = attrs
            .name
            .unwrap_or_else(|| rename_case(&rust_name, options.case.as_deref()));
        let input = !options.output_only && !attrs.output_only && !attrs.skip;
        let output = !options.input_only && !attrs.input_only && !attrs.skip;
        let shape = match &variant.fields {
            Fields::Unit => StructuredEnumShapeSpec::Unit,
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                StructuredEnumShapeSpec::Newtype(fields.unnamed[0].ty.clone())
            }
            Fields::Unnamed(fields) => StructuredEnumShapeSpec::Tuple(
                fields
                    .unnamed
                    .iter()
                    .map(|field| field.ty.clone())
                    .collect(),
            ),
            Fields::Named(fields) => {
                let mut specs = Vec::new();
                for field in &fields.named {
                    let field_attrs = item_ani_attrs(&field.attrs)?;
                    let rust_name = field.ident.as_ref().expect("named field").to_string();
                    let serde_name =
                        serde_rename(&field.attrs)?.unwrap_or_else(|| rust_name.clone());
                    let arkts_name = field_attrs
                        .name
                        .unwrap_or_else(|| rename_case(&rust_name, options.case.as_deref()));
                    specs.push(StructuredEnumFieldSpec {
                        rust_name: serde_name,
                        arkts_name,
                        ty: field.ty.clone(),
                        input: input && !field_attrs.output_only && !field_attrs.skip,
                        output: output && !field_attrs.input_only && !field_attrs.skip,
                    });
                }
                StructuredEnumShapeSpec::Struct(specs)
            }
        };
        variants.push(StructuredEnumVariantSpec {
            serde_name,
            arkts_name,
            input,
            output,
            shape,
        });
    }
    Ok((options, variants))
}

fn ets_identifier_fragment(value: &str) -> String {
    let mut output = String::new();
    let mut capitalize = true;
    for character in value.chars() {
        if !character.is_ascii_alphanumeric() {
            capitalize = true;
            continue;
        }
        if output.is_empty() && character.is_ascii_digit() {
            output.push('V');
        }
        if capitalize {
            output.extend(character.to_uppercase());
            capitalize = false;
        } else {
            output.push(character);
        }
    }
    if output.is_empty() {
        "Variant".to_string()
    } else {
        output
    }
}

fn render_structured_variant_interface(
    enum_name: &str,
    variant: &StructuredEnumVariantSpec,
    discriminator: &str,
    input: bool,
    alias_parameters: &str,
) -> Option<(String, String)> {
    if (input && !variant.input) || (!input && !variant.output) {
        return None;
    }
    let direction = if input { "Input" } else { "Output" };
    let interface_name = format!(
        "{enum_name}{direction}{}",
        ets_identifier_fragment(&variant.arkts_name)
    );
    let mut fields = vec![format!(
        "  readonly {}: {:?};",
        ets_property_key(discriminator),
        variant.arkts_name
    )];
    match &variant.shape {
        StructuredEnumShapeSpec::Unit => {}
        StructuredEnumShapeSpec::Newtype(ty) => {
            fields.push(format!("  value: {};", ets_public_type_for_syn_type(ty)));
        }
        StructuredEnumShapeSpec::Tuple(types) => {
            fields.push(format!(
                "  value: [{}];",
                types
                    .iter()
                    .map(ets_public_type_for_syn_type)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        StructuredEnumShapeSpec::Struct(specs) => {
            fields.extend(specs.iter().filter_map(|field| {
                let enabled = if input { field.input } else { field.output };
                enabled.then(|| {
                    format!(
                        "  {}: {};",
                        ets_property_key(&field.arkts_name),
                        ets_public_type_for_syn_type(&field.ty)
                    )
                })
            }));
        }
    }
    Some((
        format!("{interface_name}{alias_parameters}"),
        format!(
            "interface {interface_name}{alias_parameters} {{\n{}\n}}",
            fields.join("\n")
        ),
    ))
}

fn ets_property_key(value: &str) -> String {
    let mut characters = value.chars();
    let valid_start = characters
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic());
    let valid_rest =
        characters.all(|character| character == '_' || character.is_ascii_alphanumeric());
    if valid_start && valid_rest {
        value.to_string()
    } else {
        format!("{value:?}")
    }
}

fn structured_materializer_value_expr(ty: &syn::Type, value: &str) -> String {
    match AniType::from_syn_type(ty) {
        AniType::Primitive(PrimitiveType::Bool) => {
            format!("({value} as Boolean).valueOf()")
        }
        AniType::Primitive(PrimitiveType::I8) => format!("({value} as Numeric).toByte()"),
        AniType::Primitive(PrimitiveType::U8 | PrimitiveType::I16) => {
            format!("({value} as Numeric).toShort()")
        }
        AniType::Primitive(PrimitiveType::U16) => {
            format!("({value} as Numeric).toInt().toChar()")
        }
        AniType::Primitive(PrimitiveType::I32) => format!("({value} as Numeric).toInt()"),
        AniType::Primitive(
            PrimitiveType::U32 | PrimitiveType::I64 | PrimitiveType::Isize | PrimitiveType::Usize,
        ) => format!("({value} as Numeric).toLong()"),
        AniType::Primitive(PrimitiveType::F32) => format!("({value} as Numeric).toFloat()"),
        AniType::Primitive(PrimitiveType::F64) => format!("({value} as Numeric).toDouble()"),
        _ => format!("{value} as {}", ets_public_type_for_syn_type(ty)),
    }
}

fn render_structured_output_materializer(
    qualified_name: &str,
    variants: &[StructuredEnumVariantSpec],
    discriminator: &str,
    alias_parameters: &str,
) -> String {
    let directional = format!("{qualified_name}Output");
    let helper = directional
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    let namespace = qualified_name
        .rsplit_once('.')
        .map(|(namespace, _)| format!("{namespace}."))
        .unwrap_or_default();
    let enum_name = qualified_name.rsplit('.').next().unwrap_or(qualified_name);
    let discriminator_key = ets_property_key(discriminator);
    let mut branches = Vec::new();
    for variant in variants.iter().filter(|variant| variant.output) {
        let variant_type = format!(
            "{namespace}{enum_name}Output{}{}",
            ets_identifier_fragment(&variant.arkts_name),
            alias_parameters
        );
        let mut fields = vec![format!("{}: {:?}", discriminator_key, variant.arkts_name)];
        let mut prefix = String::new();
        match &variant.shape {
            StructuredEnumShapeSpec::Unit => {}
            StructuredEnumShapeSpec::Newtype(ty) => fields.push(format!(
                "value: {}",
                structured_materializer_value_expr(ty, "__ani_record[\"value\"]")
            )),
            StructuredEnumShapeSpec::Tuple(types) => {
                prefix.push_str("    let __ani_values = __ani_record[\"value\"] as Object[];\n");
                let values = types
                    .iter()
                    .enumerate()
                    .map(|(index, ty)| {
                        structured_materializer_value_expr(ty, &format!("__ani_values[{index}]"))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                fields.push(format!("value: [{values}]"));
            }
            StructuredEnumShapeSpec::Struct(specs) => {
                for field in specs.iter().filter(|field| field.output) {
                    let key = ets_property_key(&field.arkts_name);
                    let access = format!("__ani_record[{:?}]", field.arkts_name);
                    fields.push(format!(
                        "{key}: {}",
                        structured_materializer_value_expr(&field.ty, &access)
                    ));
                }
            }
        }
        branches.push(format!(
            "  if (__ani_tag == {:?}) {{\n{prefix}    let __ani_value: {variant_type} = {{ {} }};\n    return __ani_value;\n  }}",
            variant.arkts_name,
            fields.join(", ")
        ));
    }
    format!(
        "function __ani_materialize_{helper}{alias_parameters}(value: Object): {directional}{alias_parameters} {{\n  let __ani_record = value as Record<string, Object>;\n  let __ani_tag = __ani_record[{discriminator:?}] as string;\n{}\n  throw new TypeError(\"unknown structured enum discriminator: \" + __ani_tag);\n}}",
        branches.join("\n")
    )
}

fn structured_value_kind_tokens(ty: &syn::Type) -> TokenStream {
    let kind = match AniType::from_syn_type(ty) {
        AniType::Primitive(PrimitiveType::Bool) => quote! { Boolean },
        AniType::Primitive(PrimitiveType::I8) => quote! { Byte },
        AniType::Primitive(PrimitiveType::U8 | PrimitiveType::I16) => quote! { Short },
        AniType::Primitive(PrimitiveType::U16) => quote! { Char },
        AniType::Primitive(PrimitiveType::I32) => quote! { Int },
        AniType::Primitive(
            PrimitiveType::U32 | PrimitiveType::I64 | PrimitiveType::Isize | PrimitiveType::Usize,
        ) => quote! { Long },
        AniType::Primitive(PrimitiveType::F32) => quote! { Float },
        AniType::Primitive(PrimitiveType::F64) => quote! { Double },
        _ => quote! { Ref },
    };
    quote! { ani::conversions::StructuredEnumValueKind::#kind }
}

fn expand_structured_enum(
    input: &DeriveInput,
    data: &syn::DataEnum,
    qualified_name: &str,
) -> TokenStream {
    let enum_name = &input.ident;
    let (options, variants) = match collect_structured_enum_specs(input, data) {
        Ok(specs) => specs,
        Err(error) => return error.to_compile_error(),
    };
    register_structured_type_alias(&enum_name.to_string());

    let mut parts = qualified_name
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let ets_name = parts.pop().unwrap_or(qualified_name);
    let type_parameters = input
        .generics
        .type_params()
        .map(|parameter| parameter.ident.to_string())
        .collect::<Vec<_>>();
    let alias_parameters = if type_parameters.is_empty() {
        String::new()
    } else {
        format!("<{}>", type_parameters.join(", "))
    };
    let render_direction = |input| {
        let interfaces = variants
            .iter()
            .filter_map(|variant| {
                render_structured_variant_interface(
                    ets_name,
                    variant,
                    &options.discriminator,
                    input,
                    &alias_parameters,
                )
            })
            .collect::<Vec<_>>();
        let union = if interfaces.is_empty() {
            "never".to_string()
        } else {
            interfaces
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        };
        (interfaces, union)
    };
    let (input_interfaces, input_union) = render_direction(true);
    let (output_interfaces, output_union) = render_direction(false);
    let nullable = if options.nullable { " | null" } else { "" };
    let mut rendered = input_interfaces
        .into_iter()
        .chain(output_interfaces)
        .map(|(_, declaration)| declaration)
        .collect::<Vec<_>>();
    rendered.extend([
        format!("type {ets_name}Input{alias_parameters} = {input_union}{nullable};"),
        format!("type {ets_name}Output{alias_parameters} = {output_union}{nullable};"),
        format!(
            "type {ets_name}{alias_parameters} = {ets_name}Input{alias_parameters} | {ets_name}Output{alias_parameters};"
        ),
    ]);
    if parts.is_empty() {
        for declaration in rendered {
            emit_compile_ets_rendered_decl(EtsDeclKind::Global, "", &declaration);
        }
    } else {
        for declaration in rendered {
            emit_compile_ets_rendered_decl(EtsDeclKind::Namespace, &parts.join("."), &declaration);
        }
    }
    emit_compile_ets_rendered_decl(
        EtsDeclKind::Global,
        "",
        &render_structured_output_materializer(
            qualified_name,
            &variants,
            &options.discriminator,
            &alias_parameters,
        ),
    );

    let discriminator = syn::LitStr::new(&options.discriminator, Span::call_site());
    let schema_variants = variants
        .iter()
        .map(|variant| {
            let rust_name = syn::LitStr::new(&variant.serde_name, Span::call_site());
            let arkts_name = syn::LitStr::new(&variant.arkts_name, Span::call_site());
            let input = variant.input;
            let output = variant.output;
            let shape = match &variant.shape {
                StructuredEnumShapeSpec::Unit => {
                    quote! { ani::conversions::StructuredEnumShape::Unit }
                }
                StructuredEnumShapeSpec::Newtype(ty) => {
                    let kind = structured_value_kind_tokens(ty);
                    quote! { ani::conversions::StructuredEnumShape::Newtype(#kind) }
                }
                StructuredEnumShapeSpec::Tuple(types) => {
                    let kinds = types.iter().map(structured_value_kind_tokens);
                    quote! { ani::conversions::StructuredEnumShape::Tuple(&[#(#kinds),*]) }
                }
                StructuredEnumShapeSpec::Struct(fields) => {
                    let fields = fields.iter().map(|field| {
                        let rust_name = syn::LitStr::new(&field.rust_name, Span::call_site());
                        let arkts_name = syn::LitStr::new(&field.arkts_name, Span::call_site());
                        let input = field.input;
                        let output = field.output;
                        let kind = structured_value_kind_tokens(&field.ty);
                        quote! {
                            ani::conversions::StructuredEnumField {
                                rust_name: #rust_name,
                                arkts_name: #arkts_name,
                                kind: #kind,
                                input: #input,
                                output: #output,
                            }
                        }
                    });
                    quote! { ani::conversions::StructuredEnumShape::Struct(&[#(#fields),*]) }
                }
            };
            quote! {
                ani::conversions::StructuredEnumVariant {
                    rust_name: #rust_name,
                    arkts_name: #arkts_name,
                    shape: #shape,
                    input: #input,
                    output: #output,
                }
            }
        })
        .collect::<Vec<_>>();
    let schema_variants_to = schema_variants.clone();
    let schema_variants_from = schema_variants;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut env_generics = input.generics.clone();
    env_generics.params.insert(0, syn::parse_quote!('env));
    let (env_impl_generics, _, _) = env_generics.split_for_impl();
    let serialize_where = render_where_clause(
        input.generics.where_clause.as_ref(),
        &[quote! { Self: ani::serde::Serialize }],
    );
    let deserialize_where = render_where_clause(
        input.generics.where_clause.as_ref(),
        &[quote! { Self: ani::serde::de::DeserializeOwned }],
    );

    quote! {
        impl #impl_generics ani::conversions::TypeInfo for #enum_name #ty_generics #where_clause {
            fn type_signature() -> &'static str {
                "Lstd/core/Object;"
            }

            fn ani_c_type() -> &'static str {
                "ani_object"
            }
        }

        impl #env_impl_generics ani::conversions::ToAni<'env> for #enum_name #ty_generics
        #serialize_where
        {
            type Output = ani::sys::ani_object;

            fn to_ani(self, env: &ani::env::Env<'env>) -> ani::error::Result<Self::Output> {
                let value = ani::serde_json::to_value(self).map_err(|error| {
                    ani::error::Error::new(ani::error::Status::InvalidType, error.to_string())
                })?;
                let schema = &[#(#schema_variants_to),*];
                let value = ani::conversions::encode_structured_enum(
                    value,
                    #discriminator,
                    schema,
                )?;
                ani::conversions::ToAni::to_ani(ani::conversions::Json::new(value), env)
            }
        }

        impl #env_impl_generics ani::conversions::FromAni<'env> for #enum_name #ty_generics
        #deserialize_where
        {
            type Input = ani::sys::ani_object;

            unsafe fn from_ani(
                env: &ani::env::Env<'env>,
                value: Self::Input,
            ) -> ani::error::Result<Self> {
                let schema = &[#(#schema_variants_from),*];
                let value = unsafe {
                    ani::conversions::decode_structured_enum_from_ani(
                        env,
                        value,
                        #discriminator,
                        schema,
                    )?
                };
                ani::serde_json::from_value(value).map_err(|error| {
                    ani::error::Error::new(ani::error::Status::InvalidType, error.to_string())
                })
            }
        }
    }
}

/// Expand AniEnum derive macro
pub fn expand_enum_derive(input: DeriveInput) -> TokenStream {
    let data = match validate_enum_derive_input(&input) {
        Ok(data) => data,
        Err(err) => return err.to_compile_error(),
    };

    let enum_name = match derive_named_type_name(&input) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error(),
    };
    if data
        .variants
        .iter()
        .any(|variant| !matches!(variant.fields, Fields::Unit))
    {
        return expand_structured_enum(&input, data, &enum_name);
    }
    let variants = match collect_enum_variant_specs(data) {
        Ok(variants) => variants,
        Err(err) => return err.to_compile_error(),
    };

    expand_enum_type_impls(&input.ident, &enum_name, &variants)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn derive_ani_class_uses_explicit_class_name() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            #[ani(class = "models.ExplicitDerivedProfile")]
            struct ExplicitDerivedProfile {
                id: i32,
                name: String,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(
            expanded.contains("impl ani :: conversions :: TypeInfo for ExplicitDerivedProfile")
        );
        assert!(expanded.contains("pub const fn arkts_name () -> & 'static str"));
        assert!(expanded.contains("models.ExplicitDerivedProfile"));
        assert!(expanded.contains(
            "impl < 'env > ani :: conversions :: FromAni < 'env > for ExplicitDerivedProfile"
        ));
        assert!(expanded.contains(
            "impl < 'env > ani :: conversions :: ToAni < 'env > for ExplicitDerivedProfile"
        ));
    }

    #[test]
    fn derive_ani_class_supports_object_name_override() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            #[ani(object = "models.ObjectAlias")]
            struct ObjectAlias {
                id: i32,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("models.ObjectAlias"));
        assert!(expanded.contains("impl ani :: conversions :: TypeInfo for ObjectAlias"));
    }

    #[test]
    fn derive_ani_class_supports_field_level_property_metadata() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct PropertyObject {
                #[ani(property)]
                name: String,
                value: i32,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("ObjectProperty"));
        assert!(expanded.contains("get_named_property"));
        assert!(expanded.contains("set_named_property"));
    }

    #[test]
    fn derive_ani_class_keeps_underscore_backing_fields_for_runtime() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct BackingObject {
                _name: String,
                value: i32,
            }
        };

        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("get_named_field"));
        assert!(expanded.contains("set_named_field"));
        assert!(expanded.contains("_name"));
    }

    #[test]
    fn derive_ani_class_supports_generic_tuple_and_unit_structs() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct TupleUser<T>(T, #[ani(property)] bool);
        };
        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("impl < T > ani :: conversions :: TypeInfo for TupleUser < T >"));
        assert!(expanded.contains(
            "impl < 'env , T > ani :: conversions :: ToAni < 'env > for TupleUser < T >"
        ));
        assert!(expanded.contains("T : ani :: conversions :: ObjectField < 'env >"));
        assert!(expanded.contains("bool : ani :: conversions :: ObjectProperty < 'env >"));
        assert!(expanded.contains("self . 0"));
        assert!(expanded.contains("self . 1"));
        assert!(expanded.contains("field0"));
        assert!(expanded.contains("field1"));

        let unit_input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct Marker;
        };
        let unit_expanded = expand_class_derive(unit_input).to_string();
        assert!(unit_expanded.contains("impl ani :: conversions :: TypeInfo for Marker"));
        assert!(unit_expanded.contains("Ok (Self)"));
    }

    #[test]
    fn derive_ani_class_rejects_non_type_generics() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniClass)]
            struct Borrowed<'a> {
                name: &'a str,
            }
        };
        let expanded = expand_class_derive(input).to_string();
        assert!(expanded.contains("lifetime parameters are not supported"));
    }

    #[test]
    fn derive_ani_enum_emits_type_info_and_conversion_impls() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniEnum)]
            #[ani(name = "models.Status")]
            enum Status {
                Idle,
                Running = 4,
                Stopped,
            }
        };

        let expanded = expand_enum_derive(input).to_string();
        assert!(expanded.contains("impl ani :: conversions :: TypeInfo for Status"));
        assert!(expanded.contains("pub const fn enum_descriptor () -> & 'static str"));
        assert!(expanded.contains("pub const fn arkts_name () -> & 'static str"));
        assert!(expanded.contains("ani_enum_item"));
        assert!(expanded.contains("env . find_enum (Self :: arkts_name ())"));
        assert!(expanded.contains("env . get_enum_item_by_name (& enm , item_name)"));
        assert!(expanded.contains("models.Status"));
    }

    #[test]
    fn derive_ani_enum_supports_variant_name_override() {
        let input: DeriveInput = parse_quote! {
            #[derive(AniEnum)]
            enum Outcome {
                #[ani(name = "Ok")]
                Success,
                Failure,
            }
        };

        let expanded = expand_enum_derive(input).to_string();
        assert!(expanded.contains("\"Ok\" => Ok (Self :: Success)"));
        assert!(expanded.contains("Self :: Success => \"Ok\""));
    }

    #[test]
    fn derive_ani_enum_supports_structured_variants_through_native_objects() {
        let input: DeriveInput = parse_quote! {
            #[ani(discriminator = "kind", case = "camelCase", nullable)]
            enum Payload<T> {
                #[ani(input_only)]
                Legacy(T),
                RenamedField {
                    #[ani(rename = "payloadText")]
                    payload: T,
                    #[ani(skip)]
                    local_only: bool,
                },
                #[ani(output_only)]
                Generated(T),
            }
        };
        let expanded = expand_enum_derive(input).to_string();
        assert!(expanded.contains("conversions :: Json :: new"));
        assert!(expanded.contains("ani_object"));
        assert!(expanded.contains("encode_structured_enum"));
        assert!(expanded.contains("decode_structured_enum"));
        assert!(expanded.contains("payloadText"));
        assert!(expanded.contains("StructuredEnumVariant"));
        assert!(!expanded.contains("AniString"));
    }
}
