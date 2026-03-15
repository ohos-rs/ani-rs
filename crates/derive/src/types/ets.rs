//! ETS native stub generation from Rust function signatures.
//!
//! Stubs are emitted during macro expansion (compile phase) so no runtime
//! registration/writing is required.

use std::collections::{BTreeMap, btree_map::Entry};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syn::{FnArg, Pat, ReturnType, Signature, Type};

use crate::codegen::{
    ClassCallableDescriptor, ClassDescriptorMember, ClassMemberScope, ClassPropertyDescriptor,
    should_skip_in_signature,
};

use super::ani_type::{
    AniType, FunctionType, PrimitiveType, StringType, WrapperType, resolve_object_type_alias,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EtsDeclKind {
    Global,
    Namespace,
    Class,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsDecl {
    kind: EtsDeclKind,
    target: String,
    rendered: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EtsObjectMemberKind {
    Field,
    Property,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EtsObjectMemberDecl {
    pub name: String,
    pub kind: EtsObjectMemberKind,
    pub is_private: bool,
    pub rendered: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsObjectDecl {
    target: String,
    members: Vec<EtsObjectMemberDecl>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsClassMemberDecl {
    target: String,
    descriptor: Option<ClassDescriptorMember>,
    rendered: String,
}

#[derive(Default)]
struct EtsFileState {
    decls: Vec<EtsDecl>,
    objects: Vec<EtsObjectDecl>,
    class_members: Vec<EtsClassMemberDecl>,
}

static ETS_STATE: OnceLock<Mutex<BTreeMap<PathBuf, EtsFileState>>> = OnceLock::new();

fn state() -> &'static Mutex<BTreeMap<PathBuf, EtsFileState>> {
    ETS_STATE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn output_path() -> Option<PathBuf> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from);
    let pkg = std::env::var("CARGO_PKG_NAME")
        .unwrap_or_else(|_| "ani".to_string())
        .replace('-', "_");

    if let Ok(path) = std::env::var("ANI_ETS_OUTPUT") {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            return Some(path);
        }
        if let Some(manifest_dir) = manifest_dir {
            return Some(manifest_dir.join(path));
        }
        return Some(path);
    }

    if let Some(out_dir) = std::env::var("OUT_DIR").ok().map(PathBuf::from) {
        return Some(out_dir.join(format!("{pkg}.ets")));
    }

    let manifest_dir = manifest_dir?;
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .ok()
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                manifest_dir.join(path)
            }
        })
        .unwrap_or_else(|| manifest_dir.join("target"));

    Some(target_dir.join("ani-ets").join(format!("{pkg}.ets")))
}

fn library_name() -> String {
    std::env::var("ANI_ETS_LIBRARY")
        .or_else(|_| std::env::var("CARGO_PKG_NAME").map(|name| name.replace('-', "_")))
        .unwrap_or_else(|_| "ani".to_string())
}

fn namespace_child_mut<'a>(
    map: &'a mut BTreeMap<String, NamespaceNode>,
    key: &str,
) -> &'a mut NamespaceNode {
    match map.entry(key.to_string()) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(NamespaceNode::default()),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderedClassMember {
    descriptor: Option<ClassDescriptorMember>,
    rendered: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderedPropertySlot {
    descriptor: ClassPropertyDescriptor,
    getter: Option<String>,
    setter: Option<String>,
}

#[derive(Default)]
struct ClassNode {
    object_members: Vec<EtsObjectMemberDecl>,
    callable_members: Vec<RenderedClassMember>,
    property_slots: BTreeMap<(ClassMemberScope, String), RenderedPropertySlot>,
}

#[derive(Default)]
struct NamespaceNode {
    functions: Vec<String>,
    classes: BTreeMap<String, ClassNode>,
    children: BTreeMap<String, NamespaceNode>,
}

impl NamespaceNode {
    fn insert_namespace_fn(&mut self, path: &str, rendered: &str) {
        let mut node = self;
        for seg in path.split('.').filter(|s| !s.is_empty()) {
            node = namespace_child_mut(&mut node.children, seg);
        }
        node.functions.push(rendered.to_string());
    }

    fn insert_class_member(
        &mut self,
        path: &str,
        descriptor: Option<ClassDescriptorMember>,
        rendered: &str,
    ) {
        let mut parts = path
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let class_name = parts.pop().unwrap_or("NativeClass").to_string();

        let mut node = self;
        for seg in parts {
            node = namespace_child_mut(&mut node.children, seg);
        }

        let class = node.classes.entry(class_name).or_default();
        if let Some(ClassDescriptorMember::Property(property_descriptor)) = descriptor.as_ref() {
            insert_property_slot(class, property_descriptor.clone(), rendered);
            return;
        }

        class.callable_members.push(RenderedClassMember {
            descriptor,
            rendered: rendered.to_string(),
        });
    }

    fn insert_object_class(&mut self, path: &str, members: &[EtsObjectMemberDecl]) {
        let mut parts = path
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let class_name = parts.pop().unwrap_or("ObjectClass").to_string();

        let mut node = self;
        for seg in parts {
            node = namespace_child_mut(&mut node.children, seg);
        }

        let class = node.classes.entry(class_name).or_default();
        for member in members {
            if !class.object_members.contains(member) {
                class.object_members.push(member.clone());
            }
        }
    }
}

fn push_indented_block(out: &mut String, indent: usize, block: &str) {
    let pad = " ".repeat(indent);
    for line in block.lines() {
        out.push_str(&pad);
        out.push_str(line);
        out.push('\n');
    }
}

fn property_slot_key(descriptor: &ClassPropertyDescriptor) -> (ClassMemberScope, String) {
    (descriptor.scope, descriptor.public_name.clone())
}

fn insert_property_slot(
    class: &mut ClassNode,
    descriptor: ClassPropertyDescriptor,
    rendered: &str,
) {
    let key = property_slot_key(&descriptor);
    let slot = class
        .property_slots
        .entry(key)
        .or_insert_with(|| RenderedPropertySlot {
            descriptor: ClassPropertyDescriptor {
                owner: descriptor.owner.clone(),
                public_name: descriptor.public_name.clone(),
                scope: descriptor.scope,
                getter: None,
                setter: None,
            },
            getter: None,
            setter: None,
        });
    let _ = slot.descriptor.merge(&descriptor);

    if descriptor.getter.is_some() {
        slot.getter = Some(rendered.to_string());
    }
    if descriptor.setter.is_some() {
        slot.setter = Some(rendered.to_string());
    }
}

fn callable_member_sort_key(
    descriptor: &ClassCallableDescriptor,
    kind_rank: u8,
) -> (u8, u8, String, String) {
    let scope_rank = match descriptor.scope {
        ClassMemberScope::Static => 0,
        ClassMemberScope::Instance => 1,
    };
    (
        kind_rank,
        scope_rank,
        descriptor.public_name.clone(),
        descriptor.native_symbol_name.clone(),
    )
}

fn class_member_sort_key(member: &RenderedClassMember) -> (u8, u8, String, String) {
    match &member.descriptor {
        Some(ClassDescriptorMember::Constructor(descriptor)) => {
            let mut key = callable_member_sort_key(descriptor, 0);
            key.3 = member.rendered.clone();
            key
        }
        Some(ClassDescriptorMember::Method(descriptor)) => {
            let mut key = callable_member_sort_key(descriptor, 2);
            key.3 = member.rendered.clone();
            key
        }
        Some(ClassDescriptorMember::Property(_)) => (1, 0, member.rendered.clone(), String::new()),
        None => (3, 0, member.rendered.clone(), String::new()),
    }
}

fn object_member_sort_key(member: &EtsObjectMemberDecl) -> (u8, u8, String, String) {
    let visibility_rank = if member.is_private { 1 } else { 0 };
    let kind_rank = match member.kind {
        EtsObjectMemberKind::Field => 0,
        EtsObjectMemberKind::Property => 1,
    };
    (
        visibility_rank,
        kind_rank,
        member.name.clone(),
        member.rendered.clone(),
    )
}

fn property_slot_sort_key(slot: &RenderedPropertySlot) -> (u8, String) {
    let scope_rank = match slot.descriptor.scope {
        ClassMemberScope::Static => 0,
        ClassMemberScope::Instance => 1,
    };
    (scope_rank, slot.descriptor.public_name.clone())
}

fn ensure_exported_block(block: &str) -> String {
    let mut lines = block.lines();
    let Some(first) = lines.next() else {
        return String::new();
    };

    let first = first.trim_start();
    let mut out = if first.starts_with("export ") {
        first.to_string()
    } else {
        format!("export {first}")
    };

    for line in lines {
        out.push('\n');
        out.push_str(line);
    }

    out
}

fn render_class_block(out: &mut String, indent: usize, class_name: &str, class: &ClassNode) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}export class {class_name} {{\n"));

    let mut object_members = class.object_members.clone();
    object_members.sort_by_key(object_member_sort_key);
    for member in object_members {
        push_indented_block(out, indent + 2, &member.rendered);
    }

    let mut constructors = class
        .callable_members
        .iter()
        .filter(|member| {
            matches!(
                member.descriptor.as_ref(),
                Some(ClassDescriptorMember::Constructor(_))
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    constructors.sort_by_key(class_member_sort_key);
    for member in constructors {
        push_indented_block(out, indent + 2, &member.rendered);
    }

    let mut property_slots = class.property_slots.values().cloned().collect::<Vec<_>>();
    property_slots.sort_by_key(property_slot_sort_key);
    for slot in property_slots {
        if let Some(getter) = slot.getter {
            push_indented_block(out, indent + 2, &getter);
        }
        if let Some(setter) = slot.setter {
            push_indented_block(out, indent + 2, &setter);
        }
    }

    let mut other_members = class
        .callable_members
        .iter()
        .filter(|member| {
            !matches!(
                member.descriptor.as_ref(),
                Some(ClassDescriptorMember::Constructor(_))
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    other_members.sort_by_key(class_member_sort_key);
    for member in other_members {
        push_indented_block(out, indent + 2, &member.rendered);
    }

    out.push_str(&format!(
        "{pad}}}
"
    ));
}

fn render_namespace(out: &mut String, indent: usize, name: &str, node: &NamespaceNode) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}export namespace {name} {{\n"));

    for (class_name, class) in &node.classes {
        render_class_block(out, indent + 2, class_name, class);
    }

    let mut fns = node.functions.clone();
    fns.sort();
    for function in fns {
        push_indented_block(out, indent + 2, &ensure_exported_block(&function));
    }

    for (child, child_node) in &node.children {
        render_namespace(out, indent + 2, child, child_node);
    }

    out.push_str(&format!("{pad}}}\n"));
}

fn section_break(out: &mut String, has_content: &mut bool) {
    if *has_content {
        out.push('\n');
    }
    *has_content = true;
}

fn render_decls(
    decls: &[EtsDecl],
    objects: &[EtsObjectDecl],
    class_members: &[EtsClassMemberDecl],
) -> String {
    let mut out = String::from("// Auto-generated by ani-rs at compile time.\n\n");
    let lib_name = library_name();

    let mut globals: Vec<String> = Vec::new();
    let mut root = NamespaceNode::default();

    for object in objects {
        root.insert_object_class(&object.target, &object.members);
    }

    for class_member in class_members {
        root.insert_class_member(
            &class_member.target,
            class_member.descriptor.clone(),
            &class_member.rendered,
        );
    }

    for decl in decls {
        match decl.kind {
            EtsDeclKind::Global => globals.push(decl.rendered.clone()),
            EtsDeclKind::Namespace => root.insert_namespace_fn(&decl.target, &decl.rendered),
            EtsDeclKind::Class => root.insert_class_member(&decl.target, None, &decl.rendered),
        }
    }

    globals.sort();
    let mut has_content = false;

    for (class_name, class) in &root.classes {
        section_break(&mut out, &mut has_content);
        render_class_block(&mut out, 0, class_name, class);
    }

    for (name, node) in &root.children {
        section_break(&mut out, &mut has_content);
        render_namespace(&mut out, 0, name, node);
    }

    if !globals.is_empty() {
        section_break(&mut out, &mut has_content);
        for global in globals {
            push_indented_block(&mut out, 0, &ensure_exported_block(&global));
        }
    }

    if has_content {
        out.push('\n');
        out.push_str(&format!("loadLibrary(\"{lib_name}\");\n"));
    }

    out
}

fn write_ets_file(path: &PathBuf, file_state: &EtsFileState) {
    let content = render_decls(
        &file_state.decls,
        &file_state.objects,
        &file_state.class_members,
    );
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, content);
}

pub fn emit_compile_ets_rendered_decl(kind: EtsDeclKind, target: &str, rendered: &str) {
    let Some(path) = output_path() else {
        return;
    };

    let mut state = state()
        .lock()
        .expect("failed to acquire ets compile-state lock");
    let file_state = state.entry(path.clone()).or_default();
    let item = EtsDecl {
        kind,
        target: target.to_string(),
        rendered: rendered.to_string(),
    };
    if !file_state.decls.contains(&item) {
        file_state.decls.push(item);
    }
    write_ets_file(&path, file_state);
}

pub fn emit_compile_ets_class_member(
    target: &str,
    descriptor: &ClassDescriptorMember,
    rendered: &str,
) {
    let Some(path) = output_path() else {
        return;
    };

    let mut state = state()
        .lock()
        .expect("failed to acquire ets compile-state lock");
    let file_state = state.entry(path.clone()).or_default();
    let item = EtsClassMemberDecl {
        target: target.to_string(),
        descriptor: Some(descriptor.clone()),
        rendered: rendered.to_string(),
    };
    if !file_state.class_members.contains(&item) {
        file_state.class_members.push(item);
    }
    write_ets_file(&path, file_state);
}

pub fn emit_compile_ets_object(target: &str, members: &[EtsObjectMemberDecl]) {
    let Some(path) = output_path() else {
        return;
    };

    let mut state = state()
        .lock()
        .expect("failed to acquire ets compile-state lock");
    let file_state = state.entry(path.clone()).or_default();
    let item = EtsObjectDecl {
        target: target.to_string(),
        members: members.to_vec(),
    };
    if !file_state.objects.contains(&item) {
        file_state.objects.push(item);
    }
    write_ets_file(&path, file_state);
}

pub fn generate_object_field_ets_decl(name: &str, ty: &Type, is_private: bool) -> String {
    let ani_type = AniType::from_syn_type(ty);
    let ets_type = ani_type_to_ets(&ani_type);
    let default_value = default_value_for_object_field(&ani_type, &ets_type);
    let visibility = if is_private { "private " } else { "" };
    format!("{visibility}{name}: {ets_type} = {default_value};")
}

pub fn generate_object_property_ets_decl(name: &str, ty: &Type) -> String {
    let ani_type = AniType::from_syn_type(ty);
    let ets_type = ani_type_to_ets(&ani_type);
    let default_value = default_value_for_object_field(&ani_type, &ets_type);
    let backing_name = format!("__ani_property_{}", sanitize_object_property_name(name));
    format!(
        "private {backing_name}: {ets_type} = {default_value};\nget {name}(): {ets_type} {{\n  return this.{backing_name};\n}}\nset {name}(value: {ets_type}) {{\n  this.{backing_name} = value;\n}}"
    )
}

fn sanitize_object_property_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "value".to_string()
    } else {
        out
    }
}

fn default_value_for_object_field(ty: &AniType, ets_type: &str) -> String {
    match ty {
        AniType::Primitive(PrimitiveType::Bool) => "false".to_string(),
        AniType::Primitive(PrimitiveType::F32 | PrimitiveType::F64) => "0.0".to_string(),
        AniType::Primitive(_) => "0".to_string(),
        AniType::String(StringType::String | StringType::Str) => "\"\"".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(_)) => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Vec(_)) => "[] as ".to_string() + ets_type,
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => {
            default_value_for_object_field(inner, ets_type)
        }
        AniType::Either(_)
        | AniType::Promise(_)
        | AniType::Record(_)
        | AniType::Set(_)
        | AniType::Map(_)
        | AniType::AniObject
        | AniType::ArrayBuffer
        | AniType::Function(_)
        | AniType::FnArgs(_)
        | AniType::Tuple(_)
        | AniType::Unknown(_) => format!("null as {}", ets_type),
        AniType::Unit => "undefined".to_string(),
    }
}

fn function_arg_types_to_ets(args: &Type, option_style: OptionStyle) -> Vec<String> {
    match AniType::from_syn_type(args) {
        AniType::FnArgs(fn_args) => fn_args
            .elements
            .iter()
            .map(|arg| ani_type_to_ets_with_option_style(arg, option_style))
            .collect(),
        AniType::Tuple(items) => items
            .iter()
            .map(|arg| ani_type_to_ets_with_option_style(arg, option_style))
            .collect(),
        AniType::Unit => Vec::new(),
        other => vec![ani_type_to_ets_with_option_style(&other, option_style)],
    }
}

fn function_type_to_ets(func_type: &FunctionType, option_style: OptionStyle) -> String {
    let (args, ret) = match func_type {
        FunctionType::Function { args, ret } | FunctionType::FunctionRef { args, ret } => {
            (args.as_ref(), ret.as_ref())
        }
    };

    let params = function_arg_types_to_ets(args, option_style)
        .into_iter()
        .enumerate()
        .map(|(idx, ty)| format!("arg{idx}: {ty}"))
        .collect::<Vec<_>>()
        .join(", ");
    let ret = ani_type_to_ets_with_option_style(&AniType::from_syn_type(ret), option_style);
    format!("({params}) => {ret}")
}
fn ani_type_to_ets(ty: &AniType) -> String {
    ani_type_to_ets_with_option_style(ty, OptionStyle::Nullish)
}

fn ani_type_to_ets_with_option_style(ty: &AniType, option_style: OptionStyle) -> String {
    match ty {
        AniType::Primitive(p) => primitive_to_ets(p).to_string(),
        AniType::String(StringType::String | StringType::Str) => "string".to_string(),
        AniType::Unit => "void".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(_)) | AniType::Either(_) => {
            collect_union_parts(ty, option_style).render()
        }
        AniType::Wrapper(WrapperType::Vec(inner)) => vec_inner_to_ets(inner),
        AniType::Wrapper(WrapperType::Result(inner)) => {
            ani_type_to_ets_with_option_style(inner, option_style)
        }
        AniType::Wrapper(WrapperType::Ref(inner)) => {
            ani_type_to_ets_with_option_style(inner, option_style)
        }
        AniType::Function(func_type) => function_type_to_ets(func_type, option_style),
        AniType::FnArgs(_) => "Array<Object>".to_string(),
        AniType::Promise(promise) => {
            let inner = promise
                .inner
                .as_ref()
                .map(|t| ani_type_to_ets_with_option_style(t, option_style))
                .unwrap_or_else(|| "void".to_string());
            format!("Promise<{}>", inner)
        }
        AniType::Record(record) => {
            format!(
                "Record<string, {}>",
                ani_type_to_ets_with_option_style(record.value.as_ref(), option_style)
            )
        }
        AniType::Set(set) => {
            format!(
                "Set<{}>",
                ani_type_to_ets_with_option_style(set.element.as_ref(), option_style)
            )
        }
        AniType::Map(map) => {
            format!(
                "Map<{}, {}>",
                ani_type_to_ets_with_option_style(map.key.as_ref(), option_style),
                ani_type_to_ets_with_option_style(map.value.as_ref(), option_style)
            )
        }
        AniType::AniObject => "Object".to_string(),

        AniType::ArrayBuffer => "ArrayBuffer".to_string(),
        AniType::Tuple(items) => format!(
            "[{}]",
            items
                .iter()
                .map(|item| ani_type_to_ets_with_option_style(item, option_style))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        AniType::Unknown(ty) => unknown_type_to_ets(ty).unwrap_or_else(|| "Object".to_string()),
    }
}

fn unknown_type_to_ets(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => path_type_to_ets(type_path),
        Type::Reference(type_ref) => unknown_type_to_ets(type_ref.elem.as_ref()),
        Type::Paren(type_paren) => unknown_type_to_ets(type_paren.elem.as_ref()),
        Type::Group(type_group) => unknown_type_to_ets(type_group.elem.as_ref()),
        _ => None,
    }
}

#[derive(Default)]
struct UnionParts {
    variants: Vec<String>,
    has_null: bool,
    has_undefined: bool,
}

impl UnionParts {
    fn push_variant(&mut self, variant: String) {
        if !self.variants.contains(&variant) {
            self.variants.push(variant);
        }
    }

    fn extend(&mut self, other: UnionParts) {
        for variant in other.variants {
            self.push_variant(variant);
        }
        self.has_null |= other.has_null;
        self.has_undefined |= other.has_undefined;
    }

    fn render(self) -> String {
        let mut parts = self.variants;
        if self.has_null {
            parts.push("null".to_string());
        }
        if self.has_undefined {
            parts.push("undefined".to_string());
        }
        if parts.is_empty() {
            "Object".to_string()
        } else {
            parts.join(" | ")
        }
    }
}

fn collect_union_parts(ty: &AniType, option_style: OptionStyle) -> UnionParts {
    let mut parts = UnionParts::default();
    match ty {
        AniType::Primitive(p) => parts.push_variant(boxed_primitive_to_ets(p).to_string()),
        AniType::String(StringType::String | StringType::Str) => {
            parts.push_variant("String".to_string())
        }
        AniType::Null => parts.has_null = true,
        AniType::Undefined => parts.has_undefined = true,
        AniType::Wrapper(WrapperType::Option(inner)) => {
            parts.extend(collect_union_parts(inner, option_style));
            parts.has_null = true;
            if matches!(option_style, OptionStyle::Nullish) {
                parts.has_undefined = true;
            }
        }
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => {
            parts.extend(collect_union_parts(inner, option_style));
        }
        AniType::Either(either) => {
            for variant in &either.types {
                parts.extend(collect_union_parts(
                    &AniType::from_syn_type(variant),
                    option_style,
                ));
            }
        }
        _ => parts.push_variant(ani_type_to_ets_with_option_style(ty, option_style)),
    }
    parts
}

fn vec_inner_to_ets(inner: &AniType) -> String {
    if let AniType::Primitive(p) = inner {
        return fixed_array_type_name(p).to_string();
    }
    format!("Array<{}>", ani_type_to_ets(inner))
}

fn path_type_to_ets(type_path: &syn::TypePath) -> Option<String> {
    let segment = type_path.path.segments.last()?;
    let ident = segment.ident.to_string();

    if let Some(mapped) = known_ani_runtime_type(&ident) {
        return Some(mapped.to_string());
    }

    let qualified = type_path
        .path
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .filter(|seg| !matches!(seg.as_str(), "crate" | "self" | "super"))
        .collect::<Vec<_>>()
        .join(".");

    if let Some(alias) =
        resolve_object_type_alias(&qualified).or_else(|| resolve_object_type_alias(&ident))
    {
        return Some(alias);
    }

    if is_custom_object_name(&ident) {
        if !qualified.is_empty() {
            return Some(qualified);
        }
        return Some(ident);
    }

    None
}

fn known_ani_runtime_type(ident: &str) -> Option<&'static str> {
    match ident {
        "AniString" => Some("string"),
        "Null" => Some("null"),
        "Undefined" => Some("undefined"),
        "AniArrayBuffer" => Some("ArrayBuffer"),
        "AniFnObject" | "AniFunction" => Some("Function"),
        "AniArray" | "AniArrayRef" | "AniFixedArray" | "AniFixedArrayRef" => Some("Array<Object>"),
        "FixedBooleanArray" | "AniFixedArrayBoolean" => Some("FixedArray<boolean>"),
        "FixedByteArray" | "AniFixedArrayByte" => Some("FixedArray<byte>"),
        "FixedShortArray" | "AniFixedArrayShort" => Some("FixedArray<short>"),
        "FixedCharArray" | "AniFixedArrayChar" => Some("FixedArray<char>"),
        "FixedIntArray" | "AniArrayInt" | "AniFixedArrayInt" => Some("FixedArray<int>"),
        "FixedLongArray" | "AniArrayLong" | "AniFixedArrayLong" => Some("FixedArray<long>"),
        "FixedFloatArray" | "AniFixedArrayFloat" => Some("FixedArray<float>"),
        "FixedDoubleArray" | "AniArrayDouble" | "AniFixedArrayDouble" => Some("FixedArray<double>"),
        "AniRef" | "AniObject" | "AniClass" | "AniType" | "AniModule" | "AniNamespace"
        | "AniEnum" | "AniError" | "AniEnumItem" | "AniTupleValue" | "AniMethod"
        | "AniStaticMethod" | "AniField" | "AniStaticField" | "AniVariable" | "AniResolver"
        | "GlobalRef" | "WeakRef" => Some("Object"),
        _ => None,
    }
}

fn boxed_primitive_to_ets(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Bool => "Boolean",
        PrimitiveType::I8 | PrimitiveType::U8 => "Byte",
        PrimitiveType::I16 => "Short",
        PrimitiveType::U16 | PrimitiveType::Char => "Char",
        PrimitiveType::I32 | PrimitiveType::U32 => "Int",
        PrimitiveType::I64 | PrimitiveType::U64 => "Long",
        PrimitiveType::F32 => "Float",
        PrimitiveType::F64 => "Double",
    }
}

fn fixed_array_type_name(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Bool => "FixedArray<boolean>",
        PrimitiveType::I8 | PrimitiveType::U8 => "FixedArray<byte>",
        PrimitiveType::I16 => "FixedArray<short>",
        PrimitiveType::U16 | PrimitiveType::Char => "FixedArray<char>",
        PrimitiveType::I32 | PrimitiveType::U32 => "FixedArray<int>",
        PrimitiveType::I64 | PrimitiveType::U64 => "FixedArray<long>",
        PrimitiveType::F32 => "FixedArray<float>",
        PrimitiveType::F64 => "FixedArray<double>",
    }
}

fn is_custom_object_name(ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }

    let Some(first) = ident.chars().next() else {
        return false;
    };
    if !first.is_ascii_uppercase() {
        return false;
    }

    !matches!(
        ident,
        "String"
            | "Option"
            | "Result"
            | "Vec"
            | "Box"
            | "Rc"
            | "Arc"
            | "HashMap"
            | "BTreeMap"
            | "HashSet"
            | "BTreeSet"
            | "VecDeque"
            | "LinkedList"
            | "Cow"
            | "PathBuf"
    )
}

fn primitive_to_ets(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Bool => "boolean",
        PrimitiveType::I8 | PrimitiveType::U8 => "byte",
        PrimitiveType::I16 => "short",
        PrimitiveType::U16 | PrimitiveType::Char => "char",
        PrimitiveType::I32 | PrimitiveType::U32 => "int",
        PrimitiveType::I64 | PrimitiveType::U64 => "long",
        PrimitiveType::F32 => "float",
        PrimitiveType::F64 => "double",
    }
}

fn sanitize_param_name(name: &str, idx: usize) -> String {
    if name.is_empty() {
        return format!("arg{idx}");
    }
    if !name
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return format!("arg{idx}");
    }

    const RESERVED: &[&str] = &[
        "abstract",
        "any",
        "as",
        "boolean",
        "break",
        "byte",
        "case",
        "catch",
        "char",
        "class",
        "const",
        "constructor",
        "continue",
        "declare",
        "default",
        "delete",
        "do",
        "double",
        "else",
        "enum",
        "export",
        "extends",
        "false",
        "finally",
        "float",
        "for",
        "function",
        "if",
        "import",
        "in",
        "instanceof",
        "int",
        "interface",
        "let",
        "long",
        "namespace",
        "native",
        "new",
        "null",
        "object",
        "private",
        "protected",
        "public",
        "record",
        "return",
        "short",
        "static",
        "string",
        "super",
        "switch",
        "this",
        "throw",
        "true",
        "try",
        "type",
        "undefined",
        "var",
        "void",
        "while",
    ];

    if RESERVED.contains(&name) {
        return format!("{name}_");
    }
    name.to_string()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OptionStyle {
    NullOnly,
    Nullish,
}

#[derive(Clone, Debug)]
struct ExposedParamSpec {
    name: String,
    public_ty: String,
    native_ty: String,
    requires_bridge: bool,
}

#[derive(Clone, Debug)]
struct ExposedReturnSpec {
    public_ty: String,
    native_ty: String,
    requires_bridge: bool,
}

fn type_requires_nullish_bridge(ty: &AniType, public_ty: &str, native_ty: &str) -> bool {
    if public_ty == native_ty {
        return false;
    }

    match ty {
        AniType::Wrapper(WrapperType::Option(_)) | AniType::Either(_) => true,
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => {
            type_requires_nullish_bridge(inner, public_ty, native_ty)
        }
        _ => false,
    }
}

fn collect_exposed_param_specs(sig: &Signature, skip_first: bool) -> Vec<ExposedParamSpec> {
    let mut params = Vec::new();

    for (idx, arg) in sig
        .inputs
        .iter()
        .skip(if skip_first { 1 } else { 0 })
        .filter(|arg| !should_skip_in_signature(arg))
        .enumerate()
    {
        if let FnArg::Typed(pat_type) = arg {
            let name = match pat_type.pat.as_ref() {
                Pat::Ident(ident) => ident.ident.to_string(),
                _ => format!("arg{}", idx),
            };
            let name = sanitize_param_name(&name, idx);
            let ani_type = AniType::from_syn_type(&pat_type.ty);
            let public_ty = ani_type_to_ets_with_option_style(&ani_type, OptionStyle::Nullish);
            let native_ty = ani_type_to_ets_with_option_style(&ani_type, OptionStyle::NullOnly);
            params.push(ExposedParamSpec {
                name,
                requires_bridge: type_requires_nullish_bridge(&ani_type, &public_ty, &native_ty),
                public_ty,
                native_ty,
            });
        }
    }

    params
}

fn collect_exposed_params(sig: &Signature, skip_first: bool) -> Vec<(String, String)> {
    collect_exposed_param_specs(sig, skip_first)
        .into_iter()
        .map(|param| (param.name, param.public_ty))
        .collect()
}

fn exposed_return_spec(sig: &Signature) -> ExposedReturnSpec {
    match &sig.output {
        ReturnType::Default => ExposedReturnSpec {
            public_ty: "void".to_string(),
            native_ty: "void".to_string(),
            requires_bridge: false,
        },
        ReturnType::Type(_, ty) => {
            let ani_type = AniType::from_syn_type(ty);
            let public_ty = ani_type_to_ets_with_option_style(&ani_type, OptionStyle::Nullish);
            let native_ty = ani_type_to_ets_with_option_style(&ani_type, OptionStyle::NullOnly);
            ExposedReturnSpec {
                requires_bridge: type_requires_nullish_bridge(&ani_type, &public_ty, &native_ty),
                public_ty,
                native_ty,
            }
        }
    }
}

fn generate_fn_ets_decl_with_style(
    sig: &Signature,
    ets_name: &str,
    skip_first: bool,
    option_style: OptionStyle,
) -> String {
    let params = collect_exposed_param_specs(sig, skip_first)
        .into_iter()
        .map(|param| {
            let ty = match option_style {
                OptionStyle::NullOnly => param.native_ty,
                OptionStyle::Nullish => param.public_ty,
            };
            format!("{}: {}", param.name, ty)
        })
        .collect::<Vec<_>>();
    let ret_spec = exposed_return_spec(sig);
    let ret = match option_style {
        OptionStyle::NullOnly => ret_spec.native_ty,
        OptionStyle::Nullish => ret_spec.public_ty,
    };
    format!("{ets_name}({}): {ret}", params.join(", "))
}

fn render_native_function_decl(kind: EtsDeclKind, signature: &str, is_static: bool) -> String {
    match kind {
        EtsDeclKind::Global | EtsDeclKind::Namespace => {
            format!("native function {signature};")
        }
        EtsDeclKind::Class => {
            if is_static {
                format!("static native {signature};")
            } else {
                format!("native {signature};")
            }
        }
    }
}

fn render_bridge_input_expr(param: &ExposedParamSpec) -> String {
    if param.requires_bridge {
        format!("{} == undefined ? null : {}", param.name, param.name)
    } else {
        param.name.clone()
    }
}

fn render_bridge_output_body(call_expr: &str, ret_spec: &ExposedReturnSpec) -> String {
    if ret_spec.public_ty == "void" {
        format!("  {call_expr};")
    } else if ret_spec.requires_bridge {
        format!(
            "  let __ani_result = {call_expr};
  return __ani_result == null ? undefined : __ani_result;"
        )
    } else {
        format!("  return {call_expr};")
    }
}

fn render_nullish_bridge_binding(
    kind: EtsDeclKind,
    sig: &Signature,
    ets_name: &str,
    skip_first: bool,
    is_static: bool,
) -> String {
    let native_name = format!("__ani_native_{ets_name}");
    let native_signature =
        generate_fn_ets_decl_with_style(sig, &native_name, skip_first, OptionStyle::NullOnly);
    let public_signature = generate_fn_ets_decl(sig, ets_name, skip_first);
    let params = collect_exposed_param_specs(sig, skip_first);
    let ret_spec = exposed_return_spec(sig);

    let call_args = params
        .iter()
        .map(render_bridge_input_expr)
        .collect::<Vec<_>>()
        .join(", ");

    let call_expr = match kind {
        EtsDeclKind::Global | EtsDeclKind::Namespace => format!("{native_name}({call_args})"),
        EtsDeclKind::Class => format!("this.{native_name}({call_args})"),
    };

    let body = render_bridge_output_body(&call_expr, &ret_spec);

    let wrapper_decl = match kind {
        EtsDeclKind::Global | EtsDeclKind::Namespace => {
            format!(
                "function {public_signature} {{
{body}
}}"
            )
        }
        EtsDeclKind::Class => {
            if is_static {
                format!(
                    "static {public_signature} {{
{body}
}}"
                )
            } else {
                format!(
                    "{public_signature} {{
{body}
}}"
                )
            }
        }
    };

    format!(
        "{}
{}",
        render_native_function_decl(kind, &native_signature, is_static),
        wrapper_decl
    )
}

pub fn function_requires_nullish_bridge(sig: &Signature, skip_first: bool) -> bool {
    let params = collect_exposed_param_specs(sig, skip_first);
    let ret = exposed_return_spec(sig);
    params.iter().any(|param| param.requires_bridge) || ret.requires_bridge
}

pub fn generate_fn_ets_binding(
    kind: EtsDeclKind,
    sig: &Signature,
    ets_name: &str,
    skip_first: bool,
    is_static: bool,
) -> String {
    if function_requires_nullish_bridge(sig, skip_first) {
        render_nullish_bridge_binding(kind, sig, ets_name, skip_first, is_static)
    } else {
        let signature = generate_fn_ets_decl(sig, ets_name, skip_first);
        render_native_function_decl(kind, &signature, is_static)
    }
}

/// Generate ETS declaration signature string for a function.
///
/// Output example: `add(a: int, b: int): int`
pub fn generate_fn_ets_decl(sig: &Signature, ets_name: &str, skip_first: bool) -> String {
    generate_fn_ets_decl_with_style(sig, ets_name, skip_first, OptionStyle::Nullish)
}

pub fn generate_getter_ets_decl(
    sig: &Signature,
    property_name: &str,
    backing_name: &str,
    owner_name: &str,
    skip_first: bool,
    is_static: bool,
) -> String {
    let params = collect_exposed_params(sig, skip_first);
    debug_assert!(params.is_empty(), "getter should not expose parameters");
    let ret_spec = exposed_return_spec(sig);

    let static_prefix = if is_static { "static " } else { "" };
    let call_target = if is_static {
        format!("{owner_name}.{backing_name}()")
    } else {
        format!("this.{backing_name}()")
    };

    if ret_spec.requires_bridge {
        format!(
            "{static_prefix}native {backing_name}(): {};
{static_prefix}get {property_name}(): {} {{
  let __ani_result = {call_target};
  return __ani_result == null ? undefined : __ani_result;
}}",
            ret_spec.native_ty, ret_spec.public_ty,
        )
    } else {
        format!(
            "{static_prefix}native {backing_name}(): {};
{static_prefix}get {property_name}(): {} {{
  return {call_target};
}}",
            ret_spec.public_ty, ret_spec.public_ty,
        )
    }
}

pub fn generate_setter_ets_decl(
    sig: &Signature,
    property_name: &str,
    backing_name: &str,
    owner_name: &str,
    skip_first: bool,
    is_static: bool,
) -> String {
    let params = collect_exposed_param_specs(sig, skip_first);
    debug_assert_eq!(
        params.len(),
        1,
        "setter should expose exactly one parameter"
    );
    let param = params
        .into_iter()
        .next()
        .unwrap_or_else(|| ExposedParamSpec {
            name: "value".to_string(),
            public_ty: "Object".to_string(),
            native_ty: "Object".to_string(),
            requires_bridge: false,
        });

    let static_prefix = if is_static { "static " } else { "" };
    let call_target = if is_static {
        format!("{owner_name}.{backing_name}")
    } else {
        format!("this.{backing_name}")
    };

    if param.requires_bridge {
        format!(
            "{static_prefix}native {backing_name}({}: {}): void;
{static_prefix}set {property_name}({}: {}) {{
  {call_target}({} == undefined ? null : {});
}}",
            param.name, param.native_ty, param.name, param.public_ty, param.name, param.name,
        )
    } else {
        format!(
            "{static_prefix}native {backing_name}({}: {}): void;
{static_prefix}set {property_name}({}: {}) {{
  {call_target}({});
}}",
            param.name, param.public_ty, param.name, param.public_ty, param.name,
        )
    }
}

/// Generate ETS declaration signature string for a constructor.
///
/// Output example: `constructor(name: string, age: int)`
pub fn generate_ctor_ets_decl(sig: &Signature, skip_first: bool) -> String {
    let params = collect_exposed_params(sig, skip_first)
        .into_iter()
        .map(|(name, ty)| format!("{name}: {ty}"))
        .collect::<Vec<_>>();
    format!("constructor({})", params.join(", "))
}

fn generate_ctor_ets_decl_with_style(
    sig: &Signature,
    skip_first: bool,
    option_style: OptionStyle,
) -> String {
    let params = collect_exposed_param_specs(sig, skip_first)
        .into_iter()
        .map(|param| {
            let ty = match option_style {
                OptionStyle::NullOnly => param.native_ty,
                OptionStyle::Nullish => param.public_ty,
            };
            format!("{}: {}", param.name, ty)
        })
        .collect::<Vec<_>>();
    format!("constructor({})", params.join(", "))
}

fn constructor_requires_nullish_bridge(sig: &Signature, skip_first: bool) -> bool {
    collect_exposed_param_specs(sig, skip_first)
        .iter()
        .any(|param| param.requires_bridge)
}

pub fn generate_ctor_ets_binding(sig: &Signature, skip_first: bool) -> String {
    if constructor_requires_nullish_bridge(sig, skip_first) {
        return format!(
            "native {};",
            generate_ctor_ets_decl_with_style(sig, skip_first, OptionStyle::Nullish)
        );
    }

    format!("native {};", generate_ctor_ets_decl(sig, skip_first))
}

#[cfg(test)]
mod tests {
    use super::super::ani_type::register_object_type_alias;
    use super::*;

    #[test]
    fn test_render_decls_uses_ani_native_style() {
        let decls = vec![
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function add(a: int, b: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "Math.Utils".to_string(),
                rendered: "native function sqrt(x: double): double;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                rendered: "native getName(): string;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                rendered: "static native create(name: string): long;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                rendered: "native person_get_age(): int;
get age(): int {
  return this.person_get_age();
}"
                .to_string(),
            },
        ];
        let objects = vec![
            EtsObjectDecl {
                target: "UserProfile".to_string(),
                members: vec![
                    EtsObjectMemberDecl {
                        name: "id".to_string(),
                        kind: EtsObjectMemberKind::Field,
                        is_private: false,
                        rendered: "id: int = 0;".to_string(),
                    },
                    EtsObjectMemberDecl {
                        name: "name".to_string(),
                        kind: EtsObjectMemberKind::Field,
                        is_private: false,
                        rendered: "name: string = \"\";".to_string(),
                    },
                ],
            },
            EtsObjectDecl {
                target: "example.Person".to_string(),
                members: vec![EtsObjectMemberDecl {
                    name: "active".to_string(),
                    kind: EtsObjectMemberKind::Field,
                    is_private: false,
                    rendered: "active: boolean = false;".to_string(),
                }],
            },
        ];

        let rendered = render_decls(&decls, &objects, &[]);

        assert!(!rendered.contains("declare "));
        assert!(rendered.contains("loadLibrary(\""));
        assert!(rendered.contains("native function add(a: int, b: int): int;"));
        assert!(rendered.contains("class UserProfile {"));
        assert!(rendered.contains("id: int = 0;"));
        assert!(rendered.contains("name: string = \"\";"));
        assert!(rendered.contains("namespace Math {"));
        assert!(rendered.contains("native function sqrt(x: double): double;"));
        assert!(rendered.contains("namespace example {"));
        assert!(rendered.contains("class Person {"));
        assert!(rendered.contains("active: boolean = false;"));
        assert!(rendered.contains("native getName(): string;"));
        assert!(rendered.contains("static native create(name: string): long;"));
    }

    #[test]
    fn test_render_decls_supports_mixed_module_namespace_and_namespaced_class_exports() {
        let decls = vec![
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function add(a: int, b: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "AniMath.Utils".to_string(),
                rendered: "native function sqrt(x: double): double;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "AniMath.Utils".to_string(),
                rendered: "native function sum3(a: int, b: int, c: int): int;".to_string(),
            },
        ];
        let class_members = vec![
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Constructor(
                    ClassCallableDescriptor {
                        owner: "example.Person".to_string(),
                        public_name: "constructor".to_string(),
                        native_symbol_name: "<ctor>".to_string(),
                        scope: ClassMemberScope::Instance,
                    },
                )),
                rendered: "native constructor(name: string, score: int);".to_string(),
            },
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    owner: "example.Person".to_string(),
                    public_name: "name".to_string(),
                    scope: ClassMemberScope::Instance,
                    getter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_get_name".to_string(),
                    }),
                    setter: None,
                })),
                rendered: "get name(): string {\n  return this.__ani_native_get_name();\n}"
                    .to_string(),
            },
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    owner: "example.Person".to_string(),
                    public_name: "score".to_string(),
                    scope: ClassMemberScope::Instance,
                    getter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_get_score".to_string(),
                    }),
                    setter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_set_score".to_string(),
                    }),
                })),
                rendered: "get score(): int {\n  return this.__ani_native_get_score();\n}\nset score(value: int) {\n  this.__ani_native_set_score(value);\n}"
                    .to_string(),
            },
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "example.Person".to_string(),
                    public_name: "label".to_string(),
                    native_symbol_name: "__ani_native_label".to_string(),
                    scope: ClassMemberScope::Instance,
                })),
                rendered: "native label(): string;".to_string(),
            },
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "example.Person".to_string(),
                    public_name: "species".to_string(),
                    native_symbol_name: "__ani_native_species".to_string(),
                    scope: ClassMemberScope::Static,
                })),
                rendered: "static native species(): string;".to_string(),
            },
        ];

        let rendered = render_decls(&decls, &[], &class_members);

        assert!(rendered.contains("native function add(a: int, b: int): int;"));
        assert!(rendered.contains("namespace AniMath {"));
        assert!(rendered.contains("namespace Utils {"));
        assert!(rendered.contains("native function sqrt(x: double): double;"));
        assert!(rendered.contains("native function sum3(a: int, b: int, c: int): int;"));
        assert!(rendered.contains("namespace example {"));
        assert!(rendered.contains("class Person {"));
        assert!(rendered.contains("native constructor(name: string, score: int);"));
        assert!(rendered.contains("get name(): string"));
        assert!(rendered.contains("get score(): int"));
        assert!(rendered.contains("set score(value: int)"));
        assert!(rendered.contains("native label(): string;"));
        assert!(rendered.contains("static native species(): string;"));
    }

    #[test]
    fn test_render_decls_keeps_global_and_namespace_overloads() {
        let decls = vec![
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function sum(a: int, b: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function sum(a: int, b: int, c: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function concat(left: string, right: string): string;"
                    .to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "ops".to_string(),
                rendered: "native function sum(a: int, b: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "ops".to_string(),
                rendered: "native function sum(a: int, b: int, c: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "ops".to_string(),
                rendered: "native function concat(left: string, right: string): string;"
                    .to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "A".to_string(),
                rendered: "native function recursiveFunction(value: int): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "A.B".to_string(),
                rendered: "native function sumB(a: int, b: int): int;".to_string(),
            },
        ];

        let rendered = render_decls(&decls, &[], &[]);

        assert!(rendered.contains("export native function sum(a: int, b: int): int;"));
        assert!(rendered.contains("export native function sum(a: int, b: int, c: int): int;"));
        assert!(
            rendered
                .contains("export native function concat(left: string, right: string): string;")
        );
        assert!(rendered.contains("export namespace ops {"));
        assert!(rendered.contains("export native function sum(a: int, b: int): int;"));
        assert!(rendered.contains("export native function sum(a: int, b: int, c: int): int;"));
        assert!(
            rendered
                .contains("export native function concat(left: string, right: string): string;")
        );
        assert!(rendered.contains("export namespace A {"));
        assert!(rendered.contains("export native function recursiveFunction(value: int): int;"));
        assert!(rendered.contains("export namespace B {"));
        assert!(rendered.contains("export native function sumB(a: int, b: int): int;"));
    }

    #[test]
    fn test_render_decls_keeps_class_method_overloads() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.MathBox".to_string(),
                    public_name: "mix".to_string(),
                    native_symbol_name: "__ani_native_mix_2".to_string(),
                    scope: ClassMemberScope::Instance,
                })),
                rendered: "native mix(left: int, right: int): int;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.MathBox".to_string(),
                    public_name: "mix".to_string(),
                    native_symbol_name: "__ani_native_mix_3".to_string(),
                    scope: ClassMemberScope::Instance,
                })),
                rendered: "native mix(left: int, right: int, extra: int): int;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.MathBox".to_string(),
                    public_name: "tag".to_string(),
                    native_symbol_name: "__ani_native_tag_1".to_string(),
                    scope: ClassMemberScope::Static,
                })),
                rendered: "static native tag(value: string): string;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.MathBox".to_string(),
                    public_name: "tag".to_string(),
                    native_symbol_name: "__ani_native_tag_2".to_string(),
                    scope: ClassMemberScope::Static,
                })),
                rendered: "static native tag(value: string, suffix: string): string;".to_string(),
            },
        ];

        let rendered = render_decls(&[], &[], &class_members);

        assert!(rendered.contains("export class MathBox {"));
        assert!(rendered.contains("native mix(left: int, right: int): int;"));
        assert!(rendered.contains("native mix(left: int, right: int, extra: int): int;"));
        assert!(rendered.contains("static native tag(value: string): string;"));
        assert!(rendered.contains("static native tag(value: string, suffix: string): string;"));
    }

    #[test]
    fn test_render_decls_sorts_object_members_by_metadata() {
        let objects = vec![EtsObjectDecl {
            target: "demo.Measure".to_string(),
            members: vec![
                EtsObjectMemberDecl {
                    name: "_kind".to_string(),
                    kind: EtsObjectMemberKind::Field,
                    is_private: true,
                    rendered: "private _kind: string = \"\";".to_string(),
                },
                EtsObjectMemberDecl {
                    name: "label".to_string(),
                    kind: EtsObjectMemberKind::Field,
                    is_private: false,
                    rendered: "label: string = \"\";".to_string(),
                },
                EtsObjectMemberDecl {
                    name: "count".to_string(),
                    kind: EtsObjectMemberKind::Property,
                    is_private: false,
                    rendered: "private __ani_property_count: int = 0;
get count(): int {
  return this.__ani_property_count;
}
set count(value: int) {
  this.__ani_property_count = value;
}"
                    .to_string(),
                },
            ],
        }];

        let rendered = render_decls(&[], &objects, &[]);
        let label_idx = rendered
            .find("label: string = \"\";")
            .expect("public field should exist");
        let property_idx = rendered
            .find("get count(): int")
            .expect("property should exist");
        let private_idx = rendered
            .find("private _kind: string = \"\";")
            .expect("private field should exist");

        assert!(label_idx < property_idx);
        assert!(property_idx < private_idx);
    }
    #[test]
    fn test_render_decls_groups_property_accessors_into_slots() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    owner: "demo.Widget".to_string(),
                    public_name: "label".to_string(),
                    scope: ClassMemberScope::Instance,
                    getter: None,
                    setter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_set_label".to_string(),
                    }),
                })),
                rendered: "set label(value: string) {
  this.__ani_native_set_label(value);
}"
                .to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.Widget".to_string(),
                    public_name: "rename".to_string(),
                    native_symbol_name: "__ani_native_rename".to_string(),
                    scope: ClassMemberScope::Instance,
                })),
                rendered: "native __ani_native_rename(name: string): void;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    owner: "demo.Widget".to_string(),
                    public_name: "label".to_string(),
                    scope: ClassMemberScope::Instance,
                    getter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_get_label".to_string(),
                    }),
                    setter: None,
                })),
                rendered: "get label(): string {
  return this.__ani_native_get_label();
}"
                .to_string(),
            },
        ];

        let rendered = render_decls(&[], &[], &class_members);
        let getter_idx = rendered
            .find("get label(): string")
            .expect("getter should exist");
        let setter_idx = rendered
            .find("set label(value: string)")
            .expect("setter should exist");
        let method_idx = rendered
            .find("native __ani_native_rename(name: string): void;")
            .expect("method should exist");

        assert!(getter_idx < setter_idx);
        assert!(setter_idx < method_idx);
    }

    #[test]
    fn test_render_decls_sorts_constructor_overloads_deterministically() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.Measure".to_string(),
                descriptor: Some(ClassDescriptorMember::Constructor(
                    ClassCallableDescriptor {
                        owner: "demo.Measure".to_string(),
                        public_name: "constructor".to_string(),
                        native_symbol_name: "<ctor>".to_string(),
                        scope: ClassMemberScope::Instance,
                    },
                )),
                rendered: "native constructor(name: string, total: int);".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Measure".to_string(),
                descriptor: Some(ClassDescriptorMember::Constructor(
                    ClassCallableDescriptor {
                        owner: "demo.Measure".to_string(),
                        public_name: "constructor".to_string(),
                        native_symbol_name: "<ctor>".to_string(),
                        scope: ClassMemberScope::Instance,
                    },
                )),
                rendered: "native constructor(left: int, right: int);".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Measure".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.Measure".to_string(),
                    public_name: "describe".to_string(),
                    native_symbol_name: "describe".to_string(),
                    scope: ClassMemberScope::Instance,
                })),
                rendered: "native describe(): string;".to_string(),
            },
        ];

        let rendered = render_decls(&[], &[], &class_members);
        let pair_ctor_idx = rendered
            .find("native constructor(left: int, right: int);")
            .expect("int ctor should exist");
        let named_ctor_idx = rendered
            .find("native constructor(name: string, total: int);")
            .expect("string ctor should exist");
        let describe_idx = rendered
            .find("native describe(): string;")
            .expect("method should exist");

        assert!(pair_ctor_idx < named_ctor_idx);
        assert!(named_ctor_idx < describe_idx);
    }

    #[test]
    fn test_render_decls_sorts_class_members_by_metadata() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    owner: "demo.Widget".to_string(),
                    public_name: "rename".to_string(),
                    native_symbol_name: "__ani_native_rename".to_string(),
                    scope: ClassMemberScope::Instance,
                })),
                rendered: "native __ani_native_rename(name: string): void;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    owner: "demo.Widget".to_string(),
                    public_name: "count".to_string(),
                    scope: ClassMemberScope::Static,
                    getter: None,
                    setter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_set_count".to_string(),
                    }),
                })),
                rendered: "static set count(value: int) {
  Widget.__ani_native_set_count(value);
}"
                .to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Constructor(
                    ClassCallableDescriptor {
                        owner: "demo.Widget".to_string(),
                        public_name: "constructor".to_string(),
                        native_symbol_name: "<ctor>".to_string(),
                        scope: ClassMemberScope::Instance,
                    },
                )),
                rendered: "constructor(name: string)".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    owner: "demo.Widget".to_string(),
                    public_name: "count".to_string(),
                    scope: ClassMemberScope::Static,
                    getter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_get_count".to_string(),
                    }),
                    setter: None,
                })),
                rendered: "static get count(): int {
  return Widget.__ani_native_get_count();
}"
                .to_string(),
            },
        ];

        let rendered = render_decls(&[], &[], &class_members);
        let ctor_idx = rendered
            .find("constructor(name: string)")
            .expect("ctor should exist");
        let getter_idx = rendered
            .find("static get count(): int")
            .expect("getter should exist");
        let setter_idx = rendered
            .find("static set count(value: int)")
            .expect("setter should exist");
        let method_idx = rendered
            .find("native __ani_native_rename(name: string): void;")
            .expect("method should exist");

        assert!(ctor_idx < getter_idx);
        assert!(getter_idx < setter_idx);
        assert!(setter_idx < method_idx);
    }

    #[test]
    fn test_generate_fn_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn add(a: i32, b: i32) -> i32
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "add", false),
            "add(a: int, b: int): int"
        );
    }

    #[test]
    fn test_generate_getter_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn person_get_age() -> i32
        };
        assert_eq!(
            generate_getter_ets_decl(&sig, "age", "person_get_age", "Person", false, false),
            "native person_get_age(): int;
get age(): int {
  return this.person_get_age();
}"
        );
    }

    #[test]
    fn test_generate_setter_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn person_set_age(age: i32)
        };
        assert_eq!(
            generate_setter_ets_decl(&sig, "age", "person_set_age", "Person", false, false),
            "native person_set_age(age: int): void;
set age(age: int) {
  this.person_set_age(age);
}"
        );
    }

    #[test]
    fn test_generate_static_getter_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn widget_revision() -> i32
        };
        assert_eq!(
            generate_getter_ets_decl(
                &sig,
                "revision",
                "__ani_native_revision",
                "Widget",
                false,
                true
            ),
            "static native __ani_native_revision(): int;
static get revision(): int {
  return Widget.__ani_native_revision();
}"
        );
    }

    #[test]
    fn test_generate_static_setter_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn widget_set_revision(value: i32)
        };
        assert_eq!(
            generate_setter_ets_decl(
                &sig,
                "revision",
                "__ani_native_set_revision",
                "Widget",
                false,
                true
            ),
            "static native __ani_native_set_revision(value: int): void;
static set revision(value: int) {
  Widget.__ani_native_set_revision(value);
}"
        );
    }

    #[test]
    fn test_generate_getter_ets_decl_bridges_nullish_return() {
        let sig: Signature = syn::parse_quote! {
            fn person_get_name() -> Option<String>
        };
        assert_eq!(
            generate_getter_ets_decl(
                &sig,
                "name",
                "__ani_native_person_get_name",
                "Person",
                false,
                false
            ),
            "native __ani_native_person_get_name(): String | null;
get name(): String | null | undefined {
  let __ani_result = this.__ani_native_person_get_name();
  return __ani_result == null ? undefined : __ani_result;
}"
        );
    }

    #[test]
    fn test_generate_setter_ets_decl_bridges_nullish_param() {
        let sig: Signature = syn::parse_quote! {
            fn person_set_name(name: Option<String>)
        };
        assert_eq!(
            generate_setter_ets_decl(
                &sig,
                "name",
                "__ani_native_person_set_name",
                "Person",
                false,
                false
            ),
            "native __ani_native_person_set_name(name: String | null): void;
set name(name: String | null | undefined) {
  this.__ani_native_person_set_name(name == undefined ? null : name);
}"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_renders_precise_function_types() {
        let sig: Signature = syn::parse_quote! {
            fn install(
                cb: Function<(i32, String), bool>,
                cb_ref: FunctionRef<FnArgs<(bool, i32)>, Result<String>>
            ) -> Function<(), String>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "install", false),
            "install(cb: (arg0: int, arg1: string) => boolean, cb_ref: (arg0: boolean, arg1: int) => string): () => string"
        );
    }

    #[test]
    fn test_generate_fn_ets_binding_does_not_bridge_nested_callback_nullish_types() {
        let sig: Signature = syn::parse_quote! {
            fn register(cb: Function<(Option<String>,), ()>)
        };
        assert_eq!(
            generate_fn_ets_binding(EtsDeclKind::Global, &sig, "register", false, false),
            "native function register(cb: (arg0: String | null | undefined) => void): void;"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_keeps_custom_object_types() {
        let sig: Signature = syn::parse_quote! {
            fn process_user(
                user: crate::models::UserInfo,
                maybe: Option<crate::models::UserInfo>,
                list: Vec<crate::models::UserInfo>
            ) -> crate::models::UserInfo
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "process_user", false),
            "process_user(user: models.UserInfo, maybe: models.UserInfo | null | undefined, list: Array<models.UserInfo>): models.UserInfo"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_uses_registered_object_alias() {
        register_object_type_alias("AliasedProfile", "models.AliasedProfile");
        let sig: Signature = syn::parse_quote! {
            fn process_user(user: AliasedProfile) -> AliasedProfile
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "process_user", false),
            "process_user(user: models.AliasedProfile): models.AliasedProfile"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_keeps_custom_object_types_inside_result_and_either() {
        let sig: Signature = syn::parse_quote! {
            fn pick_user(value: Either<crate::models::UserInfo, String>) -> Result<crate::models::UserInfo>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "pick_user", false),
            "pick_user(value: models.UserInfo | String): models.UserInfo"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_known_ani_handle_types() {
        let sig: Signature = syn::parse_quote! {
            fn inspect(s: AniString<'_>, buffer: AniArrayBuffer<'_>, field: AniField, cb: AniFunction) -> AniFunction
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "inspect", false),
            "inspect(s: string, buffer: ArrayBuffer, field: Object, cb: Function): Function"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_hashmap_to_record() {
        let sig: Signature = syn::parse_quote! {
            fn summarize(input: HashMap<String, i32>) -> HashMap<String, String>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "summarize", false),
            "summarize(input: Record<string, int>): Record<string, string>"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_keeps_object_container_types_precise() {
        register_object_type_alias("crate::models::UserInfo", "models.UserInfo");
        let sig: Signature = syn::parse_quote! {
            fn collect(
                record: HashMap<String, crate::models::UserInfo>,
                set: HashSet<crate::models::UserInfo>,
                map: BTreeMap<String, crate::models::UserInfo>
            ) -> BTreeMap<String, crate::models::UserInfo>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "collect", false),
            "collect(record_: Record<string, models.UserInfo>, set: Set<models.UserInfo>, map: Map<string, models.UserInfo>): Map<string, models.UserInfo>"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_uses_boxed_variants_for_option_and_either() {
        let sig: Signature = syn::parse_quote! {
            fn convert(a: Option<i32>, b: Option<bool>, c: Either<String, i32>) -> Either<String, i32>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "convert", false),
            "convert(a: Int | null | undefined, b: Boolean | null | undefined, c: String | Int): String | Int"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_option_is_nullable_not_optional() {
        let sig: Signature = syn::parse_quote! {
            fn maybe_value(value: Option<i32>) -> Option<String>
        };
        let decl = generate_fn_ets_decl(&sig, "maybe_value", false);
        assert_eq!(
            decl,
            "maybe_value(value: Int | null | undefined): String | null | undefined"
        );
        assert!(!decl.contains("?:"));
    }

    #[test]
    fn test_generate_fn_ets_binding_bridges_nested_result_option_object() {
        let sig: Signature = syn::parse_quote! {
            fn maybe_user(flag: bool) -> Result<Option<crate::models::UserInfo>>
        };
        assert_eq!(
            generate_fn_ets_binding(EtsDeclKind::Global, &sig, "maybe_user", false, true),
            "native function __ani_native_maybe_user(flag: boolean): models.UserInfo | null;
function maybe_user(flag: boolean): models.UserInfo | null | undefined {
  let __ani_result = __ani_native_maybe_user(flag);
  return __ani_result == null ? undefined : __ani_result;
}"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_explicit_nullish_types() {
        let sig: Signature = syn::parse_quote! {
            fn maybe_text(value: Either<String, Undefined>, fallback: Either3<String, Null, Undefined>) -> Undefined
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "maybe_text", false),
            "maybe_text(value: String | undefined, fallback: String | null | undefined): undefined"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_deduplicates_nested_nullish_unions() {
        let sig: Signature = syn::parse_quote! {
            fn normalize(
                left: Either<Option<String>, Undefined>,
                right: Option<Either<String, Null>>,
                user: Either<Result<crate::models::UserInfo>, Option<crate::models::UserInfo>>
            ) -> Option<Either<crate::models::UserInfo, Null>>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "normalize", false),
            "normalize(left: String | null | undefined, right: String | null | undefined, user: models.UserInfo | null | undefined): models.UserInfo | null | undefined"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_deduplicates_nested_option_nullish_suffix() {
        let sig: Signature = syn::parse_quote! {
            fn maybe_nested(value: Option<Option<String>>) -> Option<Option<i32>>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "maybe_nested", false),
            "maybe_nested(value: String | null | undefined): Int | null | undefined"
        );
    }

    #[test]
    fn test_generate_object_field_ets_decl_can_emit_private_backing_field() {
        let ty: Type = syn::parse_quote!(String);
        assert_eq!(
            generate_object_field_ets_decl("_name", &ty, true),
            "private _name: string = \"\";"
        );
        assert_eq!(
            generate_object_field_ets_decl("name", &ty, false),
            "name: string = \"\";"
        );
    }

    #[test]
    fn test_generate_ctor_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn person_new(this: i64, name: String, age: i32)
        };
        assert_eq!(
            generate_ctor_ets_decl(&sig, true),
            "constructor(name: string, age: int)"
        );
        assert_eq!(
            generate_ctor_ets_binding(&sig, true),
            "native constructor(name: string, age: int);"
        );
    }

    #[test]
    fn test_generate_ctor_ets_binding_bridges_nullish_param() {
        let sig: Signature = syn::parse_quote! {
            fn person_new(this: i64, name: Option<String>, age: i32)
        };
        assert_eq!(
            generate_ctor_ets_binding(&sig, true),
            "native constructor(name: String | null | undefined, age: int);"
        );
    }
}
