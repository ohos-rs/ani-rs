use syn::Signature;

use crate::codegen::RegisterTarget;
use crate::types::{
    EtsDeclKind, emit_compile_ets_class_member, emit_compile_ets_rendered_decl,
    generate_ctor_ets_binding, generate_fn_ets_binding, generate_getter_ets_decl,
    generate_iterator_next_ets_binding, generate_setter_ets_decl,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClassMemberScope {
    Instance,
    Static,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClassMemberMetadata {
    pub owner: String,
    pub public_name: String,
    pub scope: ClassMemberScope,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClassOpKind {
    IndexGetter,
    IndexSetter,
    IteratorFactory { iterator_class: String },
    IteratorNext { item_type: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassCallableDescriptor {
    pub metadata: ClassMemberMetadata,
    pub native_symbol_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassOpDescriptor {
    pub metadata: ClassMemberMetadata,
    pub native_symbol_name: String,
    pub kind: ClassOpKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassPropertyAccessorDescriptor {
    pub native_symbol_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassPropertyDescriptor {
    pub metadata: ClassMemberMetadata,
    pub getter: Option<ClassPropertyAccessorDescriptor>,
    pub setter: Option<ClassPropertyAccessorDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassRegisterDescriptor {
    pub owner: String,
    pub scope: ClassMemberScope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassMemberRenderGroup {
    Constructor,
    Property,
    Callable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassDescriptorMember {
    Constructor(ClassCallableDescriptor),
    Method(ClassCallableDescriptor),
    Property(ClassPropertyDescriptor),
    Op(ClassOpDescriptor),
}

impl ClassMemberScope {
    pub fn is_static(self) -> bool {
        matches!(self, ClassMemberScope::Static)
    }

    pub fn sort_rank(self) -> u8 {
        match self {
            ClassMemberScope::Static => 0,
            ClassMemberScope::Instance => 1,
        }
    }
}

impl ClassMemberMetadata {
    pub fn register_descriptor(&self) -> ClassRegisterDescriptor {
        ClassRegisterDescriptor {
            owner: self.owner.clone(),
            scope: self.scope,
        }
    }

    pub fn owner_name(&self) -> &str {
        self.owner.rsplit('.').next().unwrap_or(self.owner.as_str())
    }
}

impl ClassDescriptorMember {
    pub fn metadata(&self) -> &ClassMemberMetadata {
        match self {
            ClassDescriptorMember::Constructor(descriptor)
            | ClassDescriptorMember::Method(descriptor) => &descriptor.metadata,
            ClassDescriptorMember::Property(descriptor) => &descriptor.metadata,
            ClassDescriptorMember::Op(descriptor) => &descriptor.metadata,
        }
    }

    pub fn render_group(&self) -> ClassMemberRenderGroup {
        match self {
            ClassDescriptorMember::Constructor(_) => ClassMemberRenderGroup::Constructor,
            ClassDescriptorMember::Property(_) => ClassMemberRenderGroup::Property,
            ClassDescriptorMember::Method(_) | ClassDescriptorMember::Op(_) => {
                ClassMemberRenderGroup::Callable
            }
        }
    }

    pub fn property(&self) -> Option<&ClassPropertyDescriptor> {
        match self {
            ClassDescriptorMember::Property(descriptor) => Some(descriptor),
            _ => None,
        }
    }

    pub fn op_kind(&self) -> Option<&ClassOpKind> {
        match self {
            ClassDescriptorMember::Op(descriptor) => Some(&descriptor.kind),
            _ => None,
        }
    }

    pub fn iterator_factory_target(&self) -> Option<&str> {
        self.op_kind()
            .and_then(ClassOpKind::iterator_factory_target)
    }

    pub fn iterator_next_item_type(&self) -> Option<&str> {
        self.op_kind()
            .and_then(ClassOpKind::iterator_next_item_type)
    }

    pub fn is_constructor(&self) -> bool {
        matches!(self.render_group(), ClassMemberRenderGroup::Constructor)
    }

    pub fn class_sort_key(&self, rendered: &str) -> (u8, u8, String, String) {
        let group_rank = match self.render_group() {
            ClassMemberRenderGroup::Constructor => 0,
            ClassMemberRenderGroup::Property => 1,
            ClassMemberRenderGroup::Callable => 2,
        };
        (
            group_rank,
            self.metadata().scope.sort_rank(),
            self.metadata().public_name.clone(),
            rendered.to_string(),
        )
    }

    pub fn register_descriptor(&self) -> ClassRegisterDescriptor {
        self.metadata().register_descriptor()
    }

    pub fn render_ets_binding(&self, sig: &Signature, skip_first: bool) -> String {
        match self {
            ClassDescriptorMember::Constructor(_) => generate_ctor_ets_binding(sig, skip_first),
            ClassDescriptorMember::Method(descriptor) => {
                descriptor.render_ets_binding(sig, skip_first)
            }
            ClassDescriptorMember::Property(descriptor) => {
                descriptor.render_ets_binding(sig, skip_first)
            }
            ClassDescriptorMember::Op(descriptor) => descriptor.render_ets_binding(sig, skip_first),
        }
    }
}

impl ClassCallableDescriptor {
    pub fn render_ets_binding(&self, sig: &Signature, skip_first: bool) -> String {
        generate_fn_ets_binding(
            EtsDeclKind::Class,
            sig,
            &self.metadata.public_name,
            skip_first,
            self.metadata.scope.is_static(),
        )
    }
}

impl ClassOpDescriptor {
    pub fn render_ets_binding(&self, sig: &Signature, skip_first: bool) -> String {
        match &self.kind {
            ClassOpKind::IteratorNext { .. } => generate_iterator_next_ets_binding(sig, skip_first),
            _ => generate_fn_ets_binding(
                EtsDeclKind::Class,
                sig,
                &self.metadata.public_name,
                skip_first,
                self.metadata.scope.is_static(),
            ),
        }
    }
}

impl ClassOpKind {
    pub fn public_name(&self) -> &str {
        match self {
            ClassOpKind::IndexGetter => "$_get",
            ClassOpKind::IndexSetter => "$_set",
            ClassOpKind::IteratorFactory { .. } => "$_iterator",
            ClassOpKind::IteratorNext { .. } => "next",
        }
    }

    pub fn iterator_factory_target(&self) -> Option<&str> {
        match self {
            ClassOpKind::IteratorFactory { iterator_class } => Some(iterator_class.as_str()),
            _ => None,
        }
    }

    pub fn iterator_next_item_type(&self) -> Option<&str> {
        match self {
            ClassOpKind::IteratorNext { item_type } => Some(item_type.as_str()),
            _ => None,
        }
    }
}

impl ClassPropertyDescriptor {
    pub fn slot_key(&self) -> (ClassMemberScope, String) {
        (self.metadata.scope, self.metadata.public_name.clone())
    }

    pub fn sort_key(&self) -> (u8, String) {
        (
            self.metadata.scope.sort_rank(),
            self.metadata.public_name.clone(),
        )
    }

    pub fn slot_seed(&self) -> ClassPropertyDescriptor {
        ClassPropertyDescriptor {
            metadata: self.metadata.clone(),
            getter: None,
            setter: None,
        }
    }

    pub fn merge(&mut self, other: &ClassPropertyDescriptor) -> Result<(), String> {
        if self.metadata != other.metadata {
            return Err("property descriptor targets do not match".to_string());
        }

        if let Some(getter) = &other.getter {
            self.try_insert_getter(getter.clone())?;
        }
        if let Some(setter) = &other.setter {
            self.try_insert_setter(setter.clone())?;
        }

        Ok(())
    }

    fn try_insert_getter(
        &mut self,
        descriptor: ClassPropertyAccessorDescriptor,
    ) -> Result<(), String> {
        if self.getter.is_some() {
            return Err(self.duplicate_accessor_error("getter"));
        }
        self.getter = Some(descriptor);
        Ok(())
    }

    fn try_insert_setter(
        &mut self,
        descriptor: ClassPropertyAccessorDescriptor,
    ) -> Result<(), String> {
        if self.setter.is_some() {
            return Err(self.duplicate_accessor_error("setter"));
        }
        self.setter = Some(descriptor);
        Ok(())
    }

    fn duplicate_accessor_error(&self, accessor_name: &str) -> String {
        let scope_name = match self.metadata.scope {
            ClassMemberScope::Instance => "instance",
            ClassMemberScope::Static => "static",
        };
        format!(
            "duplicate {scope_name} property {accessor_name} for `{}` on `{}`",
            self.metadata.public_name, self.metadata.owner
        )
    }

    pub fn render_ets_binding(&self, sig: &Signature, skip_first: bool) -> String {
        let owner_name = self.metadata.owner_name();

        if let Some(getter) = &self.getter {
            return generate_getter_ets_decl(
                sig,
                &self.metadata.public_name,
                &getter.native_symbol_name,
                owner_name,
                skip_first,
                self.metadata.scope.is_static(),
            );
        }

        if let Some(setter) = &self.setter {
            return generate_setter_ets_decl(
                sig,
                &self.metadata.public_name,
                &setter.native_symbol_name,
                owner_name,
                skip_first,
                self.metadata.scope.is_static(),
            );
        }

        panic!("property descriptor must contain a getter or setter accessor")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EtsBindingTarget {
    pub kind: EtsDeclKind,
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EtsBindingEmission {
    Rendered {
        target: EtsBindingTarget,
        rendered: String,
    },
    ClassMember {
        rendered: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportPlan {
    pub register_symbol_name: String,
    pub signature: String,
    pub register_target: RegisterTarget,
    pub ets: EtsBindingEmission,
    pub class_descriptor: Option<ClassDescriptorMember>,
    pub class_register: Option<ClassRegisterDescriptor>,
}

pub fn emit_export_plan_ets(binding: &ExportPlan) {
    match &binding.ets {
        EtsBindingEmission::Rendered { target, rendered } => {
            emit_compile_ets_rendered_decl(target.kind, &target.target, rendered)
        }
        EtsBindingEmission::ClassMember { rendered } => {
            let class_target = binding
                .class_register
                .as_ref()
                .map(|member| member.owner.as_str())
                .expect("class member ETS emission requires class register descriptor");
            let class_descriptor = binding
                .class_descriptor
                .as_ref()
                .expect("class member ETS emission requires class descriptor");
            emit_compile_ets_class_member(class_target, class_descriptor, rendered)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(owner: &str, public_name: &str, scope: ClassMemberScope) -> ClassMemberMetadata {
        ClassMemberMetadata {
            owner: owner.to_string(),
            public_name: public_name.to_string(),
            scope,
        }
    }

    #[test]
    fn property_descriptor_merges_getter_and_setter() {
        let mut property = ClassPropertyDescriptor {
            metadata: metadata("Widget", "count", ClassMemberScope::Instance),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        };
        let setter = ClassPropertyDescriptor {
            metadata: metadata("Widget", "count", ClassMemberScope::Instance),
            getter: None,
            setter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_set_count".to_string(),
            }),
        };

        property
            .merge(&setter)
            .expect("getter/setter pair should merge");

        assert_eq!(
            property.getter,
            Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            })
        );
        assert_eq!(
            property.setter,
            Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_set_count".to_string(),
            })
        );
    }

    #[test]
    fn property_descriptor_rejects_duplicate_getter() {
        let mut property = ClassPropertyDescriptor {
            metadata: metadata("Widget", "count", ClassMemberScope::Static),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        };
        let duplicate_getter = ClassPropertyDescriptor {
            metadata: metadata("Widget", "count", ClassMemberScope::Static),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_read_count".to_string(),
            }),
            setter: None,
        };

        let err = property
            .merge(&duplicate_getter)
            .expect_err("duplicate getter should be rejected");

        assert!(err.contains("duplicate static property getter"));
        assert!(err.contains("count"));
        assert!(err.contains("Widget"));
    }

    #[test]
    fn property_descriptor_reports_slot_key_and_sort_key() {
        let property = ClassPropertyDescriptor {
            metadata: metadata("demo.Widget", "count", ClassMemberScope::Static),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        };

        assert_eq!(
            property.slot_key(),
            (ClassMemberScope::Static, "count".to_string())
        );
        assert_eq!(property.sort_key(), (0, "count".to_string()));
    }

    #[test]
    fn property_descriptor_slot_seed_clears_accessors() {
        let property = ClassPropertyDescriptor {
            metadata: metadata("demo.Widget", "count", ClassMemberScope::Instance),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_set_count".to_string(),
            }),
        };

        assert_eq!(
            property.slot_seed(),
            ClassPropertyDescriptor {
                metadata: metadata("demo.Widget", "count", ClassMemberScope::Instance),
                getter: None,
                setter: None,
            }
        );
    }

    #[test]
    fn class_member_metadata_computes_register_descriptor() {
        let metadata = metadata("demo.Widget", "count", ClassMemberScope::Static);

        assert_eq!(
            metadata.register_descriptor(),
            ClassRegisterDescriptor {
                owner: "demo.Widget".to_string(),
                scope: ClassMemberScope::Static,
            }
        );
    }

    #[test]
    fn class_descriptor_register_descriptor_uses_owner_and_scope() {
        let member = ClassDescriptorMember::Property(ClassPropertyDescriptor {
            metadata: metadata("demo.Widget", "count", ClassMemberScope::Static),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        });

        assert_eq!(
            member.metadata(),
            &metadata("demo.Widget", "count", ClassMemberScope::Static)
        );
        assert_eq!(
            member
                .property()
                .map(|property| property.metadata.public_name.as_str()),
            Some("count")
        );
        assert_eq!(member.op_kind(), None);
        assert!(!member.is_constructor());
        assert_eq!(member.class_sort_key("static get count(): int").0, 1);
        assert_eq!(
            member.register_descriptor(),
            ClassRegisterDescriptor {
                owner: "demo.Widget".to_string(),
                scope: ClassMemberScope::Static,
            }
        );
    }

    #[test]
    fn class_descriptor_preserves_method_names() {
        let member = ClassDescriptorMember::Method(ClassCallableDescriptor {
            metadata: metadata("Widget", "rename", ClassMemberScope::Instance),
            native_symbol_name: "__ani_native_rename".to_string(),
        });

        assert_eq!(
            member,
            ClassDescriptorMember::Method(ClassCallableDescriptor {
                metadata: metadata("Widget", "rename", ClassMemberScope::Instance),
                native_symbol_name: "__ani_native_rename".to_string(),
            })
        );
    }

    #[test]
    fn class_descriptor_preserves_constructor_shape() {
        let member = ClassDescriptorMember::Constructor(ClassCallableDescriptor {
            metadata: metadata("Widget", "constructor", ClassMemberScope::Instance),
            native_symbol_name: "<ctor>".to_string(),
        });

        assert!(member.is_constructor());
        assert_eq!(member.property(), None);
        assert_eq!(
            member,
            ClassDescriptorMember::Constructor(ClassCallableDescriptor {
                metadata: metadata("Widget", "constructor", ClassMemberScope::Instance),
                native_symbol_name: "<ctor>".to_string(),
            })
        );
    }

    #[test]
    fn class_op_kind_helpers_report_public_names_and_iterator_metadata() {
        let iterator = ClassOpKind::IteratorFactory {
            iterator_class: "demo.WidgetIndexIterator".to_string(),
        };
        let next = ClassOpKind::IteratorNext {
            item_type: "int".to_string(),
        };

        assert_eq!(ClassOpKind::IndexGetter.public_name(), "$_get");
        assert_eq!(ClassOpKind::IndexSetter.public_name(), "$_set");
        assert_eq!(iterator.public_name(), "$_iterator");
        assert_eq!(
            iterator.iterator_factory_target(),
            Some("demo.WidgetIndexIterator")
        );
        assert_eq!(iterator.iterator_next_item_type(), None);
        assert_eq!(next.public_name(), "next");
        assert_eq!(next.iterator_factory_target(), None);
        assert_eq!(next.iterator_next_item_type(), Some("int"));
    }

    #[test]
    fn class_descriptor_member_reports_iterator_metadata() {
        let factory = ClassDescriptorMember::Op(ClassOpDescriptor {
            metadata: metadata("demo.Widget", "$_iterator", ClassMemberScope::Instance),
            native_symbol_name: "$_iterator".to_string(),
            kind: ClassOpKind::IteratorFactory {
                iterator_class: "demo.WidgetIndexIterator".to_string(),
            },
        });
        let next = ClassDescriptorMember::Op(ClassOpDescriptor {
            metadata: metadata(
                "demo.WidgetIndexIterator",
                "next",
                ClassMemberScope::Instance,
            ),
            native_symbol_name: "__ani_native_next".to_string(),
            kind: ClassOpKind::IteratorNext {
                item_type: "int".to_string(),
            },
        });
        let method = ClassDescriptorMember::Method(ClassCallableDescriptor {
            metadata: metadata("demo.Widget", "rename", ClassMemberScope::Instance),
            native_symbol_name: "rename".to_string(),
        });

        assert_eq!(
            factory.iterator_factory_target(),
            Some("demo.WidgetIndexIterator")
        );
        assert_eq!(factory.iterator_next_item_type(), None);
        assert_eq!(next.iterator_factory_target(), None);
        assert_eq!(next.iterator_next_item_type(), Some("int"));
        assert_eq!(method.iterator_factory_target(), None);
        assert_eq!(method.iterator_next_item_type(), None);
    }

    #[test]
    fn class_method_descriptor_renders_static_binding() {
        let member = ClassDescriptorMember::Method(ClassCallableDescriptor {
            metadata: metadata("Widget", "sum", ClassMemberScope::Static),
            native_symbol_name: "sum".to_string(),
        });
        let sig: Signature = syn::parse_quote! {
            fn sum(a: i32, b: i32) -> i32
        };

        assert_eq!(member.render_group(), ClassMemberRenderGroup::Callable);
        assert_eq!(
            member
                .class_sort_key("static native sum(a: int, b: int): int;")
                .0,
            2
        );
        assert_eq!(
            member.render_ets_binding(&sig, false),
            "static native sum(a: int, b: int): int;"
        );
    }

    #[test]
    fn class_op_descriptor_renders_iterator_next_binding() {
        let member = ClassDescriptorMember::Op(ClassOpDescriptor {
            metadata: metadata("WidgetIndexIterator", "next", ClassMemberScope::Instance),
            native_symbol_name: "__ani_native_next".to_string(),
            kind: ClassOpKind::IteratorNext {
                item_type: "int".to_string(),
            },
        });
        let sig: Signature = syn::parse_quote! {
            fn next() -> Option<i32>
        };

        assert_eq!(member.render_group(), ClassMemberRenderGroup::Callable);
        assert_eq!(
            member
                .op_kind()
                .and_then(ClassOpKind::iterator_next_item_type),
            Some("int")
        );
        assert!(!member.is_constructor());
        assert_eq!(
            member.render_ets_binding(&sig, false),
            "native __ani_native_next(): int | null;\nnext(): IteratorResult<int> {\n  let __ani_result = this.__ani_native_next();\n  return {\n    done: __ani_result == null,\n    value: __ani_result == null ? undefined : __ani_result\n  };\n}"
        );
    }

    #[test]
    fn class_descriptor_reports_class_sort_key() {
        let ctor = ClassDescriptorMember::Constructor(ClassCallableDescriptor {
            metadata: metadata("Widget", "constructor", ClassMemberScope::Instance),
            native_symbol_name: "<ctor>".to_string(),
        });
        let property = ClassDescriptorMember::Property(ClassPropertyDescriptor {
            metadata: metadata("Widget", "count", ClassMemberScope::Static),
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        });
        let method = ClassDescriptorMember::Method(ClassCallableDescriptor {
            metadata: metadata("Widget", "rename", ClassMemberScope::Instance),
            native_symbol_name: "__ani_native_rename".to_string(),
        });

        assert_eq!(ctor.render_group(), ClassMemberRenderGroup::Constructor);
        assert_eq!(property.render_group(), ClassMemberRenderGroup::Property);
        assert_eq!(method.render_group(), ClassMemberRenderGroup::Callable);
        assert_eq!(
            ctor.class_sort_key("constructor(name: string)"),
            (
                0,
                1,
                "constructor".to_string(),
                "constructor(name: string)".to_string()
            )
        );
        assert_eq!(
            property.class_sort_key("static get count(): int"),
            (
                1,
                0,
                "count".to_string(),
                "static get count(): int".to_string()
            )
        );
        assert_eq!(
            method.class_sort_key("native rename(name: string): void;"),
            (
                2,
                1,
                "rename".to_string(),
                "native rename(name: string): void;".to_string()
            )
        );
    }

    #[test]
    fn class_property_descriptor_renders_nullish_setter_binding() {
        let member = ClassDescriptorMember::Property(ClassPropertyDescriptor {
            metadata: metadata("Widget", "name", ClassMemberScope::Instance),
            getter: None,
            setter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_set_name".to_string(),
            }),
        });
        let sig: Signature = syn::parse_quote! {
            fn set_name(value: Option<String>)
        };

        assert_eq!(
            member.render_ets_binding(&sig, false),
            "native __ani_native_set_name(value: string | null): void;\nset name(value: string | null | undefined) {\n  this.__ani_native_set_name(value == undefined ? null : value);\n}"
        );
    }
}
