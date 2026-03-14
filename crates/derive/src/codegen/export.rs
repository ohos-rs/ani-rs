use crate::codegen::RegisterTarget;
use crate::types::{EtsDeclKind, emit_compile_ets_class_member, emit_compile_ets_rendered_decl};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClassMemberScope {
    Instance,
    Static,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassCallableDescriptor {
    pub owner: String,
    pub public_name: String,
    pub native_symbol_name: String,
    pub scope: ClassMemberScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassPropertyAccessorDescriptor {
    pub native_symbol_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassPropertyDescriptor {
    pub owner: String,
    pub public_name: String,
    pub scope: ClassMemberScope,
    pub getter: Option<ClassPropertyAccessorDescriptor>,
    pub setter: Option<ClassPropertyAccessorDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassRegisterDescriptor {
    pub owner: String,
    pub scope: ClassMemberScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassDescriptorMember {
    Constructor(ClassCallableDescriptor),
    Method(ClassCallableDescriptor),
    Property(ClassPropertyDescriptor),
}

impl ClassDescriptorMember {
    pub fn register_descriptor(&self) -> ClassRegisterDescriptor {
        match self {
            ClassDescriptorMember::Constructor(descriptor)
            | ClassDescriptorMember::Method(descriptor) => ClassRegisterDescriptor {
                owner: descriptor.owner.clone(),
                scope: descriptor.scope,
            },
            ClassDescriptorMember::Property(descriptor) => ClassRegisterDescriptor {
                owner: descriptor.owner.clone(),
                scope: descriptor.scope,
            },
        }
    }
}

impl ClassPropertyDescriptor {
    pub fn merge(&mut self, other: &ClassPropertyDescriptor) -> Result<(), String> {
        if self.owner != other.owner
            || self.public_name != other.public_name
            || self.scope != other.scope
        {
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
        let scope_name = match self.scope {
            ClassMemberScope::Instance => "instance",
            ClassMemberScope::Static => "static",
        };
        format!(
            "duplicate {scope_name} property {accessor_name} for `{}` on `{}`",
            self.public_name, self.owner
        )
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

    #[test]
    fn property_descriptor_merges_getter_and_setter() {
        let mut property = ClassPropertyDescriptor {
            owner: "Widget".to_string(),
            public_name: "count".to_string(),
            scope: ClassMemberScope::Instance,
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        };
        let setter = ClassPropertyDescriptor {
            owner: "Widget".to_string(),
            public_name: "count".to_string(),
            scope: ClassMemberScope::Instance,
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
            owner: "Widget".to_string(),
            public_name: "count".to_string(),
            scope: ClassMemberScope::Static,
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        };
        let duplicate_getter = ClassPropertyDescriptor {
            owner: "Widget".to_string(),
            public_name: "count".to_string(),
            scope: ClassMemberScope::Static,
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
    fn class_descriptor_register_descriptor_uses_owner_and_scope() {
        let member = ClassDescriptorMember::Property(ClassPropertyDescriptor {
            owner: "demo.Widget".to_string(),
            public_name: "count".to_string(),
            scope: ClassMemberScope::Static,
            getter: Some(ClassPropertyAccessorDescriptor {
                native_symbol_name: "__ani_native_get_count".to_string(),
            }),
            setter: None,
        });

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
            owner: "Widget".to_string(),
            public_name: "rename".to_string(),
            native_symbol_name: "__ani_native_rename".to_string(),
            scope: ClassMemberScope::Instance,
        });

        assert_eq!(
            member,
            ClassDescriptorMember::Method(ClassCallableDescriptor {
                owner: "Widget".to_string(),
                public_name: "rename".to_string(),
                native_symbol_name: "__ani_native_rename".to_string(),
                scope: ClassMemberScope::Instance,
            })
        );
    }

    #[test]
    fn class_descriptor_preserves_constructor_shape() {
        let member = ClassDescriptorMember::Constructor(ClassCallableDescriptor {
            owner: "Widget".to_string(),
            public_name: "constructor".to_string(),
            native_symbol_name: "<ctor>".to_string(),
            scope: ClassMemberScope::Instance,
        });

        assert_eq!(
            member,
            ClassDescriptorMember::Constructor(ClassCallableDescriptor {
                owner: "Widget".to_string(),
                public_name: "constructor".to_string(),
                native_symbol_name: "<ctor>".to_string(),
                scope: ClassMemberScope::Instance,
            })
        );
    }
}
