use crate::codegen::RegisterTarget;
use crate::types::{
    emit_compile_ets_class_member, emit_compile_ets_decl, emit_compile_ets_rendered_decl,
    EtsDeclKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EtsBindingTarget {
    pub kind: EtsDeclKind,
    pub target: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EtsBindingEmission {
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
pub struct ExportPlan {
    pub register_symbol_name: String,
    pub signature: String,
    pub register_target: RegisterTarget,
    pub ets: EtsBindingEmission,
}

pub fn emit_export_plan_ets(binding: &ExportPlan) {
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
