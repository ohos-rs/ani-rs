//! ETS native stub generation from Rust function signatures.
//!
//! Stubs are emitted during macro expansion (compile phase) so no runtime
//! registration/writing is required.

use std::collections::{BTreeMap, HashSet, btree_map::Entry};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use syn::{FnArg, GenericParam, Pat, ReturnType, Signature, Type};

use crate::codegen::{
    ClassDescriptorMember, ClassMemberScope, ClassPropertyDescriptor, should_skip_in_signature,
};

#[cfg(test)]
use crate::codegen::{ClassCallableDescriptor, ClassOpDescriptor, ClassOpKind};

use super::ani_type::{
    AniType, ArrayHandleType, FunctionType, PrimitiveType, RuntimeHandleType, WrapperType,
    is_custom_object_name, resolve_object_type_alias, type_path_qualified_name,
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
    type_params: Vec<String>,
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
static ETS_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

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

impl RenderedClassMember {
    fn sort_key(&self) -> (u8, u8, String, String) {
        self.descriptor
            .as_ref()
            .map(|descriptor| descriptor.class_sort_key(&self.rendered))
            .unwrap_or_else(|| (3, 0, self.rendered.clone(), String::new()))
    }

    fn is_constructor(&self) -> bool {
        self.descriptor
            .as_ref()
            .is_some_and(ClassDescriptorMember::is_constructor)
    }

    fn iterator_factory_target(&self) -> Option<&str> {
        self.descriptor
            .as_ref()
            .and_then(ClassDescriptorMember::iterator_factory_target)
    }

    fn iterator_next_item_type(&self) -> Option<&str> {
        self.descriptor
            .as_ref()
            .and_then(ClassDescriptorMember::iterator_next_item_type)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderedPropertySlot {
    descriptor: ClassPropertyDescriptor,
    getter: Option<String>,
    setter: Option<String>,
}

#[derive(Default)]
struct ClassNode {
    type_params: Vec<String>,
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
        if let Some(property_descriptor) = descriptor
            .as_ref()
            .and_then(ClassDescriptorMember::property)
        {
            insert_property_slot(class, property_descriptor.clone(), rendered);
            return;
        }

        class.callable_members.push(RenderedClassMember {
            descriptor,
            rendered: rendered.to_string(),
        });
    }

    fn insert_object_class(
        &mut self,
        path: &str,
        type_params: &[String],
        members: &[EtsObjectMemberDecl],
    ) {
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
        if class.type_params.is_empty() && !type_params.is_empty() {
            class.type_params = type_params.to_vec();
        }
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

fn insert_property_slot(
    class: &mut ClassNode,
    descriptor: ClassPropertyDescriptor,
    rendered: &str,
) {
    let key = descriptor.slot_key();
    let slot = class
        .property_slots
        .entry(key)
        .or_insert_with(|| RenderedPropertySlot {
            descriptor: descriptor.slot_seed(),
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
    slot.descriptor.sort_key()
}

fn iterator_next_item_type(class: &ClassNode) -> Option<&str> {
    class
        .callable_members
        .iter()
        .find_map(RenderedClassMember::iterator_next_item_type)
}

fn collect_iterator_factory_targets(
    node: &NamespaceNode,
    iterator_targets: &mut std::collections::BTreeSet<String>,
) {
    for class in node.classes.values() {
        for member in &class.callable_members {
            if let Some(target) = member.iterator_factory_target() {
                iterator_targets.insert(target.to_string());
            }
        }
    }

    for child in node.children.values() {
        collect_iterator_factory_targets(child, iterator_targets);
    }
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

fn render_class_block(
    out: &mut String,
    indent: usize,
    class_target: &str,
    class_name: &str,
    class: &ClassNode,
    iterator_targets: &std::collections::BTreeSet<String>,
) {
    let pad = " ".repeat(indent);
    let iterator_suffix = if iterator_targets.contains(class_target) {
        iterator_next_item_type(class)
            .map(|item_ty| format!(" implements Iterator<{item_ty}>"))
            .unwrap_or_default()
    } else {
        String::new()
    };
    let type_params = if class.type_params.is_empty() {
        String::new()
    } else {
        format!("<{}>", class.type_params.join(", "))
    };
    out.push_str(&format!(
        "{pad}export class {class_name}{type_params}{iterator_suffix} {{\n"
    ));

    let mut object_members = class.object_members.clone();
    object_members.sort_by_key(object_member_sort_key);
    for member in object_members {
        push_indented_block(out, indent + 2, &member.rendered);
    }

    let mut constructors = class
        .callable_members
        .iter()
        .filter(|member| member.is_constructor())
        .cloned()
        .collect::<Vec<_>>();
    constructors.sort_by_key(RenderedClassMember::sort_key);
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
        .filter(|member| !member.is_constructor())
        .cloned()
        .collect::<Vec<_>>();
    other_members.sort_by_key(RenderedClassMember::sort_key);
    for member in other_members {
        push_indented_block(out, indent + 2, &member.rendered);
    }

    out.push_str(&format!("{pad}}}\n"));
}

fn render_namespace(
    out: &mut String,
    indent: usize,
    path: &str,
    name: &str,
    node: &NamespaceNode,
    iterator_targets: &std::collections::BTreeSet<String>,
) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}export namespace {name} {{\n"));

    for (class_name, class) in &node.classes {
        let class_target = if path.is_empty() {
            format!("{name}.{class_name}")
        } else {
            format!("{path}.{name}.{class_name}")
        };
        render_class_block(
            out,
            indent + 2,
            &class_target,
            class_name,
            class,
            iterator_targets,
        );
    }

    let mut fns = node.functions.clone();
    fns.sort();
    for function in fns {
        push_indented_block(out, indent + 2, &ensure_exported_block(&function));
    }

    let next_path = if path.is_empty() {
        name.to_string()
    } else {
        format!("{path}.{name}")
    };
    for (child, child_node) in &node.children {
        render_namespace(
            out,
            indent + 2,
            &next_path,
            child,
            child_node,
            iterator_targets,
        );
    }

    out.push_str(&format!("{pad}}}\n"));
}

fn section_break(out: &mut String, has_content: &mut bool) {
    if *has_content {
        out.push('\n');
    }
    *has_content = true;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BuiltinEtsTypeSurface {
    rust_name: &'static str,
    public_ty: &'static str,
    alias_target: Option<&'static str>,
}

impl BuiltinEtsTypeSurface {
    const fn opaque_object(rust_name: &'static str) -> Self {
        Self {
            rust_name,
            public_ty: rust_name,
            alias_target: Some("Object"),
        }
    }

    const fn direct(rust_name: &'static str, public_ty: &'static str) -> Self {
        Self {
            rust_name,
            public_ty,
            alias_target: None,
        }
    }
}

const BUILTIN_ETS_TYPE_SURFACES: &[BuiltinEtsTypeSurface] = &[
    BuiltinEtsTypeSurface::opaque_object("AniRef"),
    BuiltinEtsTypeSurface::opaque_object("GlobalRef"),
    BuiltinEtsTypeSurface::opaque_object("AniType"),
    BuiltinEtsTypeSurface::opaque_object("AniModule"),
    BuiltinEtsTypeSurface::opaque_object("AniNamespace"),
    BuiltinEtsTypeSurface::opaque_object("AniEnum"),
    BuiltinEtsTypeSurface::opaque_object("AniError"),
    BuiltinEtsTypeSurface::opaque_object("AniMethod"),
    BuiltinEtsTypeSurface::opaque_object("AniStaticMethod"),
    BuiltinEtsTypeSurface::opaque_object("AniField"),
    BuiltinEtsTypeSurface::opaque_object("AniStaticField"),
    BuiltinEtsTypeSurface::opaque_object("AniVariable"),
    BuiltinEtsTypeSurface::opaque_object("AniResolver"),
    BuiltinEtsTypeSurface::opaque_object("AnyValue"),
    BuiltinEtsTypeSurface::opaque_object("TupleValue"),
    BuiltinEtsTypeSurface::opaque_object("EnumItem"),
    BuiltinEtsTypeSurface::direct("WeakRef", "WeakRef<Object>"),
];

fn builtin_ets_type_surface(name: &str) -> Option<&'static BuiltinEtsTypeSurface> {
    BUILTIN_ETS_TYPE_SURFACES
        .iter()
        .find(|surface| surface.rust_name == name)
}

fn builtin_ets_public_type(name: &str) -> Option<&'static str> {
    builtin_ets_type_surface(name).map(|surface| surface.public_ty)
}

fn contains_type_token(haystack: &str, token: &str) -> bool {
    let mut search_from = 0usize;
    while let Some(offset) = haystack[search_from..].find(token) {
        let start = search_from + offset;
        let end = start + token.len();
        let prev = haystack[..start].chars().next_back();
        let next = haystack[end..].chars().next();
        let prev_ok = prev.is_none_or(|ch| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '.'));
        let next_ok = next.is_none_or(|ch| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '.'));
        if prev_ok && next_ok {
            return true;
        }
        search_from = end;
    }
    false
}

fn rendered_decl_uses_builtin_alias(
    alias: &str,
    decls: &[EtsDecl],
    objects: &[EtsObjectDecl],
    class_members: &[EtsClassMemberDecl],
) -> bool {
    decls
        .iter()
        .any(|decl| contains_type_token(&decl.rendered, alias))
        || objects.iter().any(|object| {
            object
                .members
                .iter()
                .any(|member| contains_type_token(&member.rendered, alias))
        })
        || class_members
            .iter()
            .any(|member| contains_type_token(&member.rendered, alias))
}

fn append_builtin_opaque_ets_aliases(
    out: &mut String,
    decls: &[EtsDecl],
    objects: &[EtsObjectDecl],
    class_members: &[EtsClassMemberDecl],
    has_content: &mut bool,
) {
    let aliases = BUILTIN_ETS_TYPE_SURFACES
        .iter()
        .filter_map(|surface| {
            surface
                .alias_target
                .map(|target| (surface.public_ty, target))
        })
        .filter(|(alias, _)| rendered_decl_uses_builtin_alias(alias, decls, objects, class_members))
        .collect::<Vec<_>>();
    if aliases.is_empty() {
        return;
    }

    section_break(out, has_content);
    for (alias, target) in aliases {
        push_indented_block(out, 0, &format!("export type {alias} = {target};"));
    }
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
        root.insert_object_class(&object.target, &object.type_params, &object.members);
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

    let mut iterator_targets = std::collections::BTreeSet::new();
    collect_iterator_factory_targets(&root, &mut iterator_targets);

    globals.sort();
    let mut has_content = false;

    append_builtin_opaque_ets_aliases(&mut out, decls, objects, class_members, &mut has_content);

    for (class_name, class) in &root.classes {
        section_break(&mut out, &mut has_content);
        render_class_block(
            &mut out,
            0,
            class_name,
            class_name,
            class,
            &iterator_targets,
        );
    }

    for (name, node) in &root.children {
        section_break(&mut out, &mut has_content);
        render_namespace(&mut out, 0, "", name, node, &iterator_targets);
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

fn write_ets_file(path: &Path, file_state: &EtsFileState) -> io::Result<()> {
    let content = render_decls(
        &file_state.decls,
        &file_state.objects,
        &file_state.class_members,
    );

    if fs::read(path)
        .ok()
        .is_some_and(|existing| existing == content.as_bytes())
    {
        return Ok(());
    }

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("ani.ets");
    let sequence = ETS_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temporary_path = parent.join(format!(
        ".{file_name}.{}.{}.tmp",
        std::process::id(),
        sequence
    ));

    let result = (|| {
        let mut temporary = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temporary_path)?;
        temporary.write_all(content.as_bytes())?;
        temporary.sync_all()?;
        drop(temporary);
        fs::rename(&temporary_path, path)
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    result
}

fn persist_ets_file(path: &Path, file_state: &EtsFileState) {
    write_ets_file(path, file_state).unwrap_or_else(|error| {
        panic!(
            "failed to atomically write generated ETS file `{}`: {error}",
            path.display()
        )
    });
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
    persist_ets_file(&path, file_state);
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
    persist_ets_file(&path, file_state);
}

pub fn emit_compile_ets_object(
    target: &str,
    type_params: &[String],
    members: &[EtsObjectMemberDecl],
) {
    let Some(path) = output_path() else {
        return;
    };

    let mut state = state()
        .lock()
        .expect("failed to acquire ets compile-state lock");
    let file_state = state.entry(path.clone()).or_default();
    let item = EtsObjectDecl {
        target: target.to_string(),
        type_params: type_params.to_vec(),
        members: members.to_vec(),
    };
    if !file_state.objects.contains(&item) {
        file_state.objects.push(item);
    }
    persist_ets_file(&path, file_state);
}

pub fn generate_object_field_ets_decl(name: &str, ty: &Type, is_private: bool) -> String {
    let surface = EtsTypeSurface::from_syn_type(ty);
    let visibility = if is_private { "private " } else { "" };
    format!(
        "{visibility}{name}: {} = {};",
        surface.public_ty, surface.object_default_value
    )
}

pub fn generate_object_property_ets_decl(name: &str, ty: &Type) -> String {
    let surface = EtsTypeSurface::from_syn_type(ty);
    let backing_name = format!("__ani_property_{}", sanitize_object_property_name(name));
    format!(
        "private {backing_name}: {} = {};
get {name}(): {} {{
  return this.{backing_name};
}}
set {name}(value: {}) {{
  this.{backing_name} = value;
}}",
        surface.public_ty, surface.object_default_value, surface.public_ty, surface.public_ty,
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

fn default_object_value_for_ani_type(ty: &AniType, ets_type: &str) -> String {
    match ty {
        AniType::Primitive(PrimitiveType::Bool) => "false".to_string(),
        AniType::Primitive(PrimitiveType::F32 | PrimitiveType::F64) => "0.0".to_string(),
        AniType::Primitive(_) => "0".to_string(),
        AniType::String(_) => "\"\"".to_string(),
        AniType::BigInt => "0n".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(_)) => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Vec(_)) => "[] as ".to_string() + ets_type,
        AniType::FixedArray(_) => "[] as ".to_string() + ets_type,
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => {
            default_object_value_for_ani_type(inner, ets_type)
        }
        AniType::Transparent(inner) => default_object_value_for_ani_type(inner, ets_type),
        AniType::Record(_) => format!("{{}} as {}", ets_type),
        AniType::Set(_) => format!("new {}()", ets_type),
        AniType::Map(_) => format!("new {}()", ets_type),
        AniType::Tuple(_) => format!("[] as {}", ets_type),
        AniType::NativePointer(_) => "0".to_string(),
        AniType::GlobalRef | AniType::WeakRef => format!("null as {}", ets_type),
        AniType::ArrayHandle(handle) => default_array_handle_value(*handle, ets_type),
        AniType::TypeParam(_) => format!("null as {}", ets_type),
        AniType::CustomObject(_) => format!("null as {}", ets_type),
        AniType::RuntimeHandle(handle) => default_runtime_handle_value(*handle, ets_type),
        AniType::Either(_)
        | AniType::Promise(_)
        | AniType::AniObject
        | AniType::AnyValue
        | AniType::TupleValue
        | AniType::EnumItem
        | AniType::ArrayBuffer
        | AniType::Function(_)
        | AniType::FnArgs(_)
        | AniType::Unknown(_) => format!("null as {}", ets_type),
        AniType::Unit => "undefined".to_string(),
    }
}

fn function_arg_types_to_ets(args: &AniType, context: EtsRenderContext) -> Vec<String> {
    match args {
        AniType::FnArgs(fn_args) => fn_args
            .elements
            .iter()
            .map(|arg| ani_type_to_ets_in_context(arg, context))
            .collect(),
        AniType::Tuple(items) => items
            .iter()
            .map(|arg| ani_type_to_ets_in_context(arg, context))
            .collect(),
        AniType::Unit => Vec::new(),
        other => vec![ani_type_to_ets_in_context(other, context)],
    }
}

fn function_type_to_ets(func_type: &FunctionType, context: EtsRenderContext) -> String {
    let (args, ret) = match func_type {
        FunctionType::Function { args, ret, .. } | FunctionType::FunctionRef { args, ret, .. } => {
            (args.as_ref(), ret.as_ref())
        }
    };

    let params = function_arg_types_to_ets(args, context)
        .into_iter()
        .enumerate()
        .map(|(idx, ty)| format!("arg{idx}: {ty}"))
        .collect::<Vec<_>>()
        .join(", ");
    let ret = ani_type_to_ets_in_context(ret, context);
    format!("({params}) => {ret}")
}

fn ani_type_to_ets(ty: &AniType) -> String {
    ani_type_to_ets_in_context(ty, EtsRenderContext::public())
}

pub fn ets_public_type_for_ani_type(ty: &AniType) -> String {
    EtsTypeSurface::from_ani_type(ty).public_ty
}

pub fn ets_public_type_for_syn_type(ty: &Type) -> String {
    ets_public_type_for_ani_type(&AniType::from_syn_type(ty))
}

fn ani_type_to_ets_in_context(ty: &AniType, context: EtsRenderContext) -> String {
    if let Some(parts) = collect_surface_union_parts(ty, context) {
        return parts.render();
    }

    render_non_union_ani_type_to_ets(ty, context)
}

fn render_non_union_ani_type_to_ets(ty: &AniType, context: EtsRenderContext) -> String {
    match ty {
        AniType::Primitive(p) => primitive_to_ets(p).to_string(),
        AniType::String(_) => "string".to_string(),
        AniType::Unit => "void".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Vec(inner)) => vec_inner_to_ets(inner),
        AniType::Wrapper(WrapperType::Result(inner)) => match context.result_style {
            ResultStyle::ThrowingValue => ani_type_to_ets_in_context(inner, context),
        },
        AniType::Wrapper(WrapperType::Ref(inner)) => ani_type_to_ets_in_context(inner, context),
        AniType::Transparent(inner) => ani_type_to_ets_in_context(inner, context),
        AniType::Function(func_type) => function_type_to_ets(func_type, context),
        AniType::FnArgs(_) => "Array<Object>".to_string(),
        AniType::Promise(promise) => {
            let inner = promise
                .inner
                .as_ref()
                .map(|t| ani_type_to_ets_in_context(t, context))
                .unwrap_or_else(|| "void".to_string());
            format!("Promise<{}>", inner)
        }
        AniType::Record(record) => {
            format!(
                "Record<string, {}>",
                ani_type_to_ets_in_context(record.value.as_ref(), context)
            )
        }
        AniType::Set(set) => {
            format!(
                "Set<{}>",
                ani_type_to_ets_in_context(set.element.as_ref(), context)
            )
        }
        AniType::Map(map) => {
            format!(
                "Map<{}, {}>",
                ani_type_to_ets_in_context(map.key.as_ref(), context),
                ani_type_to_ets_in_context(map.value.as_ref(), context)
            )
        }
        AniType::AniObject => "Object".to_string(),
        AniType::BigInt => "bigint".to_string(),
        AniType::GlobalRef => builtin_ets_public_type("GlobalRef").unwrap().to_string(),
        AniType::WeakRef => builtin_ets_public_type("WeakRef").unwrap().to_string(),
        AniType::RuntimeHandle(handle) => runtime_handle_to_ets(*handle).to_string(),
        AniType::ArrayHandle(handle) => array_handle_to_ets(*handle).to_string(),
        AniType::AnyValue => "AnyValue".to_string(),
        AniType::TupleValue => "TupleValue".to_string(),
        AniType::EnumItem => "EnumItem".to_string(),
        AniType::ArrayBuffer => "ArrayBuffer".to_string(),
        AniType::NativePointer(_) => "long".to_string(),
        AniType::FixedArray(p) => value_array_type_name(p).to_string(),
        AniType::TypeParam(name) => name.clone(),
        AniType::CustomObject(type_path) => custom_object_path_to_ets(type_path.as_ref(), context)
            .unwrap_or_else(|| "Object".to_string()),
        AniType::Tuple(items) => format!(
            "[{}]",
            items
                .iter()
                .map(|item| ani_type_to_ets_in_context(item, context))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        AniType::Unknown(ty) => {
            unknown_type_to_ets(ty, context).unwrap_or_else(|| "Object".to_string())
        }

        AniType::Wrapper(WrapperType::Option(_)) | AniType::Either(_) => {
            unreachable!("union-capable types must be handled by collect_surface_union_parts")
        }
    }
}

fn unknown_type_to_ets(ty: &Type, context: EtsRenderContext) -> Option<String> {
    let reparsed = AniType::from_syn_type(ty);
    match reparsed {
        AniType::Unknown(inner) if inner.as_ref() != ty => {
            return unknown_type_to_ets(inner.as_ref(), context);
        }
        AniType::Unknown(_) => {}
        other => return Some(ani_type_to_ets_in_context(&other, context)),
    }

    match ty {
        Type::Path(type_path) => path_type_to_ets(type_path),
        Type::Reference(type_ref) => unknown_type_to_ets(type_ref.elem.as_ref(), context),
        Type::Paren(type_paren) => unknown_type_to_ets(type_paren.elem.as_ref(), context),
        Type::Group(type_group) => unknown_type_to_ets(type_group.elem.as_ref(), context),
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

fn push_surface_variant(parts: &mut UnionParts, ty: &AniType, context: EtsRenderContext) {
    if let Some(inner_parts) = collect_surface_union_parts(ty, context) {
        parts.extend(inner_parts);
    } else {
        parts.push_variant(render_non_union_ani_type_to_ets(ty, context));
    }
}

fn collect_surface_union_parts(ty: &AniType, context: EtsRenderContext) -> Option<UnionParts> {
    let mut parts = UnionParts::default();
    match ty {
        AniType::Primitive(p) => parts.push_variant(primitive_to_ets(p).to_string()),
        AniType::String(_) => parts.push_variant("string".to_string()),
        AniType::Null => parts.has_null = true,
        AniType::Undefined => parts.has_undefined = true,
        AniType::Wrapper(WrapperType::Option(inner)) => {
            push_surface_variant(&mut parts, inner, context);
            parts.has_null = true;
            if matches!(context.option_style, OptionStyle::Nullish) {
                parts.has_undefined = true;
            }
        }
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => {
            return collect_surface_union_parts(inner, context);
        }
        AniType::Either(either) => {
            for variant in &either.types {
                push_surface_variant(&mut parts, variant, context);
            }
        }
        _ => return None,
    }
    Some(parts)
}

fn vec_inner_to_ets(inner: &AniType) -> String {
    if let AniType::Primitive(p) = inner {
        return value_array_type_name(p).to_string();
    }
    format!("Array<{}>", ani_type_to_ets(inner))
}

fn custom_object_path_to_ets(
    type_path: &syn::TypePath,
    context: EtsRenderContext,
) -> Option<String> {
    let base = path_type_to_ets(type_path)?;
    let generics = type_path
        .path
        .segments
        .last()
        .and_then(|segment| match &segment.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                let rendered = args
                    .args
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::GenericArgument::Type(inner) => Some(ani_type_to_ets_in_context(
                            &AniType::from_syn_type(inner),
                            context,
                        )),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                if rendered.is_empty() {
                    None
                } else {
                    Some(rendered)
                }
            }
            _ => None,
        });

    generics
        .map(|args| format!("{base}<{}>", args.join(", ")))
        .or(Some(base))
}

fn path_type_to_ets(type_path: &syn::TypePath) -> Option<String> {
    let segment = type_path.path.segments.last()?;
    let ident = segment.ident.to_string();

    if let Some(mapped) = known_ani_runtime_type(&ident) {
        return Some(mapped.to_string());
    }

    let qualified = type_path_qualified_name(type_path);

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

fn runtime_handle_to_ets(handle: RuntimeHandleType) -> &'static str {
    match handle {
        RuntimeHandleType::Ref => builtin_ets_public_type("AniRef").unwrap(),
        RuntimeHandleType::Class => "Class",
        RuntimeHandleType::Type => builtin_ets_public_type("AniType").unwrap(),
        RuntimeHandleType::Module => builtin_ets_public_type("AniModule").unwrap(),
        RuntimeHandleType::Namespace => builtin_ets_public_type("AniNamespace").unwrap(),
        RuntimeHandleType::String => "string",
        RuntimeHandleType::Enum => builtin_ets_public_type("AniEnum").unwrap(),
        RuntimeHandleType::Error => builtin_ets_public_type("AniError").unwrap(),
        RuntimeHandleType::Method => builtin_ets_public_type("AniMethod").unwrap(),
        RuntimeHandleType::StaticMethod => builtin_ets_public_type("AniStaticMethod").unwrap(),
        RuntimeHandleType::Field => builtin_ets_public_type("AniField").unwrap(),
        RuntimeHandleType::StaticField => builtin_ets_public_type("AniStaticField").unwrap(),
        RuntimeHandleType::Function | RuntimeHandleType::FunctionObject => "Function",
        RuntimeHandleType::Variable => builtin_ets_public_type("AniVariable").unwrap(),
        RuntimeHandleType::Resolver => builtin_ets_public_type("AniResolver").unwrap(),
    }
}

fn default_array_handle_value(handle: ArrayHandleType, ets_type: &str) -> String {
    match handle {
        ArrayHandleType::Array | ArrayHandleType::ArrayRef => format!("[] as {}", ets_type),
        ArrayHandleType::FixedArray | ArrayHandleType::FixedArrayRef => {
            format!("[] as {}", ets_type)
        }
    }
}

fn default_runtime_handle_value(handle: RuntimeHandleType, ets_type: &str) -> String {
    match handle {
        RuntimeHandleType::String => "\"\"".to_string(),
        _ => format!("null as {}", ets_type),
    }
}

fn array_handle_to_ets(handle: ArrayHandleType) -> &'static str {
    match handle {
        ArrayHandleType::Array | ArrayHandleType::ArrayRef => "Array<Object>",
        ArrayHandleType::FixedArray | ArrayHandleType::FixedArrayRef => "FixedArray<Object>",
    }
}

fn known_ani_runtime_type(ident: &str) -> Option<&'static str> {
    if let Some(surface) = builtin_ets_public_type(ident) {
        return Some(surface);
    }

    match ident {
        "AniObject" => Some("Object"),
        "AniClass" => Some("Class"),
        "AniString" => Some("string"),
        "AniEnumItem" | "EnumItem" => Some("EnumItem"),
        "AniTupleValue" | "TupleValue" => Some("TupleValue"),
        "AnyValue" => Some("AnyValue"),
        "Null" => Some("null"),
        "Undefined" => Some("undefined"),
        "AniArrayBuffer" => Some("ArrayBuffer"),
        "AniFnObject" | "AniFunction" => Some("Function"),
        "AniArray" | "AniArrayRef" => Some("Array<Object>"),
        "AniFixedArray" | "AniFixedArrayRef" => Some("FixedArray<Object>"),
        "FixedBooleanArray" | "AniFixedArrayBoolean" => Some("ValueArray<boolean>"),
        "FixedByteArray" | "AniFixedArrayByte" => Some("ValueArray<byte>"),
        "FixedShortArray" | "AniFixedArrayShort" => Some("ValueArray<short>"),
        "FixedCharArray" | "AniFixedArrayChar" => Some("ValueArray<char>"),
        "FixedIntArray" | "AniArrayInt" | "AniFixedArrayInt" => Some("ValueArray<int>"),
        "FixedLongArray" | "AniArrayLong" | "AniFixedArrayLong" => Some("ValueArray<long>"),
        "FixedFloatArray" | "AniFixedArrayFloat" => Some("ValueArray<float>"),
        "FixedDoubleArray" | "AniArrayDouble" | "AniFixedArrayDouble" => Some("ValueArray<double>"),
        _ => None,
    }
}

fn value_array_type_name(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Bool => "ValueArray<boolean>",
        PrimitiveType::I8 => "ValueArray<byte>",
        PrimitiveType::U8 | PrimitiveType::I16 => "ValueArray<short>",
        PrimitiveType::U16 => "ValueArray<char>",
        PrimitiveType::I32 => "ValueArray<int>",
        PrimitiveType::U32 | PrimitiveType::I64 | PrimitiveType::Isize | PrimitiveType::Usize => {
            "ValueArray<long>"
        }
        PrimitiveType::F32 => "ValueArray<float>",
        PrimitiveType::F64 => "ValueArray<double>",
    }
}

fn primitive_to_ets(p: &PrimitiveType) -> &'static str {
    match p {
        PrimitiveType::Bool => "boolean",
        PrimitiveType::I8 => "byte",
        PrimitiveType::U8 | PrimitiveType::I16 => "short",
        PrimitiveType::U16 => "char",
        PrimitiveType::I32 => "int",
        PrimitiveType::U32 | PrimitiveType::I64 | PrimitiveType::Isize | PrimitiveType::Usize => {
            "long"
        }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResultStyle {
    ThrowingValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EtsRenderContext {
    option_style: OptionStyle,
    result_style: ResultStyle,
}

impl EtsRenderContext {
    fn public() -> Self {
        Self {
            option_style: OptionStyle::Nullish,
            result_style: ResultStyle::ThrowingValue,
        }
    }

    fn native() -> Self {
        Self {
            option_style: OptionStyle::NullOnly,
            result_style: ResultStyle::ThrowingValue,
        }
    }

    fn with_option_style(option_style: OptionStyle) -> Self {
        Self {
            option_style,
            ..Self::public()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EtsBridgeStrategy {
    Direct,
    Nullish,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsTypeSurface {
    public_ty: String,
    native_ty: String,
    bridge_strategy: EtsBridgeStrategy,
    object_default_value: String,
}

impl EtsTypeSurface {
    fn from_syn_type(ty: &Type) -> Self {
        Self::from_syn_type_with_type_params(ty, &HashSet::new())
    }

    fn from_syn_type_with_type_params(ty: &Type, type_params: &HashSet<String>) -> Self {
        Self::from_ani_type(&AniType::from_syn_type_with_type_params(ty, type_params))
    }

    fn from_ani_type(ty: &AniType) -> Self {
        let public_ty = ani_type_to_ets_in_context(ty, EtsRenderContext::public());
        let native_ty = ani_type_to_ets_in_context(ty, EtsRenderContext::native());
        Self {
            bridge_strategy: bridge_strategy_for_ani_type(ty, &public_ty, &native_ty),
            object_default_value: default_object_value_for_ani_type(ty, &public_ty),
            public_ty,
            native_ty,
        }
    }

    fn requires_bridge(&self) -> bool {
        matches!(self.bridge_strategy, EtsBridgeStrategy::Nullish)
    }

    fn ty_for_option_style(&self, option_style: OptionStyle) -> &str {
        match option_style {
            OptionStyle::NullOnly => &self.native_ty,
            OptionStyle::Nullish => &self.public_ty,
        }
    }

    fn render_input_expr(&self, value: &str) -> String {
        match self.bridge_strategy {
            EtsBridgeStrategy::Direct => value.to_string(),
            EtsBridgeStrategy::Nullish => format!("{value} == undefined ? null : {value}"),
        }
    }

    fn render_output_body(&self, call_expr: &str) -> String {
        if self.public_ty == "void" {
            return format!("  {call_expr};");
        }

        match self.bridge_strategy {
            EtsBridgeStrategy::Direct => format!("  return {call_expr};"),
            EtsBridgeStrategy::Nullish => format!(
                "  let __ani_result = {call_expr};\n  return __ani_result == null ? undefined : __ani_result;"
            ),
        }
    }

    fn iterator_item_ty(&self) -> &str {
        if matches!(self.bridge_strategy, EtsBridgeStrategy::Nullish) {
            return self
                .native_ty
                .strip_suffix(" | null")
                .unwrap_or(self.native_ty.as_str());
        }

        &self.native_ty
    }
}

#[derive(Clone, Debug)]
struct ExposedParamSpec {
    name: String,
    surface: EtsTypeSurface,
}

#[derive(Clone, Debug)]
struct ExposedReturnSpec {
    surface: EtsTypeSurface,
}

fn bridge_strategy_for_ani_type(
    ty: &AniType,
    public_ty: &str,
    native_ty: &str,
) -> EtsBridgeStrategy {
    if public_ty != native_ty
        && collect_surface_union_parts(ty, EtsRenderContext::public()).is_some()
    {
        EtsBridgeStrategy::Nullish
    } else {
        EtsBridgeStrategy::Direct
    }
}

fn collect_sig_type_params(sig: &Signature) -> HashSet<String> {
    sig.generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => Some(ty.ident.to_string()),
            _ => None,
        })
        .collect()
}

fn render_sig_type_params(sig: &Signature) -> String {
    let params = sig
        .generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(ty) => Some(ty.ident.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if params.is_empty() {
        String::new()
    } else {
        format!("<{}>", params.join(", "))
    }
}

fn collect_exposed_param_specs(sig: &Signature, skip_first: bool) -> Vec<ExposedParamSpec> {
    let mut params = Vec::new();
    let type_params = collect_sig_type_params(sig);

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
            let surface =
                EtsTypeSurface::from_syn_type_with_type_params(&pat_type.ty, &type_params);
            params.push(ExposedParamSpec { name, surface });
        }
    }

    params
}

fn collect_exposed_params(sig: &Signature, skip_first: bool) -> Vec<(String, String)> {
    collect_exposed_param_specs(sig, skip_first)
        .into_iter()
        .map(|param| (param.name, param.surface.public_ty))
        .collect()
}

fn exposed_return_spec(sig: &Signature) -> ExposedReturnSpec {
    match &sig.output {
        ReturnType::Default => ExposedReturnSpec {
            surface: EtsTypeSurface {
                public_ty: "void".to_string(),
                native_ty: "void".to_string(),
                bridge_strategy: EtsBridgeStrategy::Direct,
                object_default_value: "undefined".to_string(),
            },
        },
        ReturnType::Type(_, ty) => {
            let surface =
                EtsTypeSurface::from_syn_type_with_type_params(ty, &collect_sig_type_params(sig));
            ExposedReturnSpec { surface }
        }
    }
}

fn generate_fn_ets_decl_with_style(
    sig: &Signature,
    ets_name: &str,
    skip_first: bool,
    option_style: OptionStyle,
) -> String {
    let context = EtsRenderContext::with_option_style(option_style);
    let params = collect_exposed_param_specs(sig, skip_first)
        .into_iter()
        .map(|param| {
            format!(
                "{}: {}",
                param.name,
                param.surface.ty_for_option_style(context.option_style)
            )
        })
        .collect::<Vec<_>>();
    let ret_spec = exposed_return_spec(sig);
    let ret = ret_spec.surface.ty_for_option_style(context.option_style);
    let generics = render_sig_type_params(sig);
    format!("{ets_name}{generics}({}): {ret}", params.join(", "))
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
    param.surface.render_input_expr(&param.name)
}

fn render_bridge_output_body(call_expr: &str, ret_spec: &ExposedReturnSpec) -> String {
    ret_spec.surface.render_output_body(call_expr)
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
    params.iter().any(|param| param.surface.requires_bridge()) || ret.surface.requires_bridge()
}

pub fn generate_iterator_next_ets_binding(sig: &Signature, skip_first: bool) -> String {
    let params = collect_exposed_params(sig, skip_first);
    debug_assert!(
        params.is_empty(),
        "iterator next should not expose parameters"
    );
    let ret_spec = exposed_return_spec(sig);
    let native_ty = ret_spec.surface.native_ty.clone();
    let item_ty = ret_spec.surface.iterator_item_ty().to_string();

    format!(
        "native __ani_native_next(): {};\nnext(): IteratorResult<{}> {{\n  let __ani_result = this.__ani_native_next();\n  if (__ani_result == null) {{\n    return new IteratorResult<{}>();\n  }}\n  return new IteratorResult<{}>(__ani_result);\n}}",
        native_ty, item_ty, item_ty, item_ty,
    )
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

    let native_ty = if ret_spec.surface.requires_bridge() {
        ret_spec.surface.native_ty.as_str()
    } else {
        ret_spec.surface.public_ty.as_str()
    };
    let body = ret_spec.surface.render_output_body(&call_target);

    format!(
        "{static_prefix}native {backing_name}(): {native_ty};
{static_prefix}get {property_name}(): {} {{
{body}
}}",
        ret_spec.surface.public_ty,
    )
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
            surface: EtsTypeSurface {
                public_ty: "Object".to_string(),
                native_ty: "Object".to_string(),
                bridge_strategy: EtsBridgeStrategy::Direct,
                object_default_value: "null as Object".to_string(),
            },
        });

    let static_prefix = if is_static { "static " } else { "" };
    let call_target = if is_static {
        format!("{owner_name}.{backing_name}")
    } else {
        format!("this.{backing_name}")
    };

    let native_ty = if param.surface.requires_bridge() {
        param.surface.native_ty.as_str()
    } else {
        param.surface.public_ty.as_str()
    };
    let call_arg = param.surface.render_input_expr(&param.name);

    format!(
        "{static_prefix}native {backing_name}({}: {native_ty}): void;
{static_prefix}set {property_name}({}: {}) {{
  {call_target}({call_arg});
}}",
        param.name, param.name, param.surface.public_ty,
    )
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
            format!(
                "{}: {}",
                param.name,
                param.surface.ty_for_option_style(option_style)
            )
        })
        .collect::<Vec<_>>();
    format!("constructor({})", params.join(", "))
}

fn constructor_requires_nullish_bridge(sig: &Signature, skip_first: bool) -> bool {
    collect_exposed_param_specs(sig, skip_first)
        .iter()
        .any(|param| param.surface.requires_bridge())
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
    use crate::codegen::ClassMemberMetadata;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_test_dir(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("ani-rs-ets-{label}-{}-{nonce}", std::process::id()))
    }

    fn class_member_metadata(
        owner: &str,
        public_name: &str,
        scope: ClassMemberScope,
    ) -> ClassMemberMetadata {
        ClassMemberMetadata {
            owner: owner.to_string(),
            public_name: public_name.to_string(),
            scope,
        }
    }

    #[test]
    fn test_render_decls_is_independent_of_declaration_order() {
        let mut decls = vec![
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function zebra(): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function alpha(): int;".to_string(),
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "demo.tools".to_string(),
                rendered: "native function middle(): int;".to_string(),
            },
        ];
        let forward = render_decls(&decls, &[], &[]);
        decls.reverse();
        let reverse = render_decls(&decls, &[], &[]);

        assert_eq!(forward, reverse);
    }

    #[test]
    fn test_write_ets_file_is_atomic_and_skips_identical_content() {
        let root = temporary_test_dir("atomic");
        let output = root.join("nested").join("bindings.ets");
        let file_state = EtsFileState {
            decls: vec![EtsDecl {
                kind: EtsDeclKind::Global,
                target: String::new(),
                rendered: "native function answer(): int;".to_string(),
            }],
            ..EtsFileState::default()
        };

        write_ets_file(&output, &file_state).expect("first atomic ETS write should succeed");
        let first = fs::read(&output).expect("generated ETS file should be readable");
        write_ets_file(&output, &file_state).expect("identical ETS write should succeed");
        let second = fs::read(&output).expect("generated ETS file should remain readable");

        assert_eq!(first, second);
        assert!(
            String::from_utf8(second)
                .expect("generated ETS should be UTF-8")
                .contains("export native function answer(): int;")
        );
        assert!(
            fs::read_dir(output.parent().expect("output should have a parent"))
                .expect("output directory should be readable")
                .all(|entry| !entry
                    .expect("directory entry should be readable")
                    .file_name()
                    .to_string_lossy()
                    .ends_with(".tmp"))
        );

        fs::remove_dir_all(&root).expect("temporary ETS test directory should be removable");
    }

    #[test]
    fn test_write_ets_file_reports_directory_errors() {
        let root = temporary_test_dir("error");
        fs::create_dir_all(&root).expect("temporary root should be created");
        let blocking_file = root.join("not-a-directory");
        fs::write(&blocking_file, b"blocking file").expect("blocking file should be written");
        let output = blocking_file.join("bindings.ets");

        let error = write_ets_file(&output, &EtsFileState::default())
            .expect_err("writing below a regular file should fail");
        assert!(
            matches!(
                error.kind(),
                io::ErrorKind::AlreadyExists | io::ErrorKind::NotADirectory
            ),
            "unexpected error kind: {:?}",
            error.kind()
        );

        fs::remove_dir_all(&root).expect("temporary ETS test directory should be removable");
    }

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
                type_params: Vec::new(),
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
                type_params: Vec::new(),
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
    fn test_render_decls_preserves_object_class_type_params() {
        let objects = vec![EtsObjectDecl {
            target: "models.GenericBox".to_string(),
            type_params: vec!["T".to_string(), "U".to_string()],
            members: vec![EtsObjectMemberDecl {
                name: "value".to_string(),
                kind: EtsObjectMemberKind::Field,
                is_private: false,
                rendered: "value: T = null as T;".to_string(),
            }],
        }];

        let rendered = render_decls(&[], &objects, &[]);

        assert!(rendered.contains("export namespace models {"));
        assert!(rendered.contains("export class GenericBox<T, U> {"));
        assert!(rendered.contains("value: T = null as T;"));
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
                descriptor: Some(ClassDescriptorMember::Constructor(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "example.Person",
                        "constructor",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "<ctor>".to_string(),
                })),
                rendered: "native constructor(name: string, score: int);".to_string(),
            },
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    metadata: class_member_metadata(
                        "example.Person",
                        "name",
                        ClassMemberScope::Instance,
                    ),
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
                    metadata: class_member_metadata(
                        "example.Person",
                        "score",
                        ClassMemberScope::Instance,
                    ),
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
                    metadata: class_member_metadata(
                        "example.Person",
                        "label",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "__ani_native_label".to_string(),
                })),
                rendered: "native label(): string;".to_string(),
            },
            EtsClassMemberDecl {
                target: "example.Person".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "example.Person",
                        "species",
                        ClassMemberScope::Static,
                    ),
                    native_symbol_name: "__ani_native_species".to_string(),
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
    fn test_render_decls_keeps_class_method_overloads() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "demo.MathBox",
                        "mix",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "__ani_native_mix_2".to_string(),
                })),
                rendered: "native mix(left: int, right: int): int;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "demo.MathBox",
                        "mix",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "__ani_native_mix_3".to_string(),
                })),
                rendered: "native mix(left: int, right: int, extra: int): int;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "demo.MathBox",
                        "tag",
                        ClassMemberScope::Static,
                    ),
                    native_symbol_name: "__ani_native_tag_1".to_string(),
                })),
                rendered: "static native tag(value: string): string;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.MathBox".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "demo.MathBox",
                        "tag",
                        ClassMemberScope::Static,
                    ),
                    native_symbol_name: "__ani_native_tag_2".to_string(),
                })),
                rendered: "static native tag(value: string, suffix: string): string;".to_string(),
            },
        ];

        let rendered = render_decls(&[], &[], &class_members);
        assert!(rendered.contains("native mix(left: int, right: int): int;"));
        assert!(rendered.contains("native mix(left: int, right: int, extra: int): int;"));
        assert!(rendered.contains("static native tag(value: string): string;"));
        assert!(rendered.contains("static native tag(value: string, suffix: string): string;"));
    }

    #[test]
    fn test_render_decls_groups_property_accessors_into_slots() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "label",
                        ClassMemberScope::Instance,
                    ),
                    getter: None,
                    setter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_set_label".to_string(),
                    }),
                })),
                rendered: "set label(value: string) {\n  this.__ani_native_set_label(value);\n}"
                    .to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "rename",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "__ani_native_rename".to_string(),
                })),
                rendered: "native __ani_native_rename(name: string): void;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "label",
                        ClassMemberScope::Instance,
                    ),
                    getter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_get_label".to_string(),
                    }),
                    setter: None,
                })),
                rendered: "get label(): string {\n  return this.__ani_native_get_label();\n}"
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
                        metadata: class_member_metadata(
                            "demo.Measure",
                            "constructor",
                            ClassMemberScope::Instance,
                        ),
                        native_symbol_name: "<ctor>".to_string(),
                    },
                )),
                rendered: "native constructor(name: string, total: int);".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Measure".to_string(),
                descriptor: Some(ClassDescriptorMember::Constructor(
                    ClassCallableDescriptor {
                        metadata: class_member_metadata(
                            "demo.Measure",
                            "constructor",
                            ClassMemberScope::Instance,
                        ),
                        native_symbol_name: "<ctor>".to_string(),
                    },
                )),
                rendered: "native constructor(left: int, right: int);".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Measure".to_string(),
                descriptor: Some(ClassDescriptorMember::Method(ClassCallableDescriptor {
                    metadata: class_member_metadata(
                        "demo.Measure",
                        "describe",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "describe".to_string(),
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
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "rename",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "__ani_native_rename".to_string(),
                })),
                rendered: "native __ani_native_rename(name: string): void;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "count",
                        ClassMemberScope::Static,
                    ),
                    getter: None,
                    setter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_set_count".to_string(),
                    }),
                })),
                rendered:
                    "static set count(value: int) {\n  Widget.__ani_native_set_count(value);\n}"
                        .to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Constructor(
                    ClassCallableDescriptor {
                        metadata: class_member_metadata(
                            "demo.Widget",
                            "constructor",
                            ClassMemberScope::Instance,
                        ),
                        native_symbol_name: "<ctor>".to_string(),
                    },
                )),
                rendered: "constructor(name: string)".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Property(ClassPropertyDescriptor {
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "count",
                        ClassMemberScope::Static,
                    ),
                    getter: Some(crate::codegen::ClassPropertyAccessorDescriptor {
                        native_symbol_name: "__ani_native_get_count".to_string(),
                    }),
                    setter: None,
                })),
                rendered: "static get count(): int {\n  return Widget.__ani_native_get_count();\n}"
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
    fn test_render_decls_marks_iterator_class_and_wraps_next_result() {
        let class_members = vec![
            EtsClassMemberDecl {
                target: "demo.Widget".to_string(),
                descriptor: Some(ClassDescriptorMember::Op(ClassOpDescriptor {
                    metadata: class_member_metadata(
                        "demo.Widget",
                        "$_iterator",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "$_iterator".to_string(),
                    kind: ClassOpKind::IteratorFactory {
                        iterator_class: "demo.WidgetIndexIterator".to_string(),
                    },
                })),
                rendered: "native $_iterator(): WidgetIndexIterator;".to_string(),
            },
            EtsClassMemberDecl {
                target: "demo.WidgetIndexIterator".to_string(),
                descriptor: Some(ClassDescriptorMember::Op(ClassOpDescriptor {
                    metadata: class_member_metadata(
                        "demo.WidgetIndexIterator",
                        "next",
                        ClassMemberScope::Instance,
                    ),
                    native_symbol_name: "__ani_native_next".to_string(),
                    kind: ClassOpKind::IteratorNext {
                        item_type: "int".to_string(),
                    },
                })),
                rendered: generate_iterator_next_ets_binding(
                    &syn::parse_quote! { fn next() -> Option<i32> },
                    false,
                ),
            },
        ];

        let rendered = render_decls(&[], &[], &class_members);

        assert!(rendered.contains("export class WidgetIndexIterator implements Iterator<int> {"));
        assert!(rendered.contains("next(): IteratorResult<int> {"));
        assert!(rendered.contains("return new IteratorResult<int>();"));
        assert!(rendered.contains("return new IteratorResult<int>(__ani_result);"));
        assert!(!rendered.contains("next(): int | null | undefined {"));
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
            "native __ani_native_person_get_name(): string | null;
get name(): string | null | undefined {
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
            "native __ani_native_person_set_name(name: string | null): void;
set name(name: string | null | undefined) {
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
            "native function register(cb: (arg0: string | null | undefined) => void): void;"
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
            "pick_user(value: models.UserInfo | string): models.UserInfo"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_known_ani_handle_types() {
        let sig: Signature = syn::parse_quote! {
            fn inspect(s: AniString<'_>, buffer: AniArrayBuffer<'_>, field: AniField, cb: AniFunction) -> AniFunction
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "inspect", false),
            "inspect(s: string, buffer: ArrayBuffer, field: AniField, cb: Function): Function"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_supported_non_object_fallback_types() {
        let sig: Signature = syn::parse_quote! {
            fn normalize(path: std::ffi::OsString, borrowed_os: &std::ffi::OsStr, boxed: Box<std::path::Path>, borrowed_path: &std::path::Path, borrowed_c: &std::ffi::CStr, borrowed: std::borrow::Cow<'static, std::path::Path>, offset: isize, len: usize) -> usize
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "normalize", false),
            "normalize(path: string, borrowed_os: string, boxed: string, borrowed_path: string, borrowed_c: string, borrowed: string, offset: long, len: long): long"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_raw_array_handles_without_unknown_fallback() {
        let sig: Signature = syn::parse_quote! {
            fn inspect(values: AniArray<'_>, refs: AniArrayRef<'_>, fixed: AniFixedArray<'_>, fixed_refs: AniFixedArrayRef<'_>) -> AniFixedArrayRef<'_>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "inspect", false),
            "inspect(values: Array<Object>, refs: Array<Object>, fixed: FixedArray<Object>, fixed_refs: FixedArray<Object>): FixedArray<Object>"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_maps_fixed_array_wrappers_without_unknown_fallback() {
        let sig: Signature = syn::parse_quote! {
            fn roundtrip(values: FixedIntArray, flags: AniFixedArrayBoolean<'_>) -> AniFixedArrayInt<'_>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "roundtrip", false),
            "roundtrip(values: ValueArray<int>, flags: ValueArray<boolean>): ValueArray<int>"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_keeps_global_and_weak_ref_names() {
        let sig: Signature = syn::parse_quote! {
            fn inspect(global: GlobalRef, weak: WeakRef) -> WeakRef
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "inspect", false),
            "inspect(global: GlobalRef, weak: WeakRef<Object>): WeakRef<Object>"
        );
    }

    #[test]
    fn test_runtime_handle_public_type_model_is_explicit() {
        let module_ty: Type = syn::parse_quote!(AniModule<'_>);
        assert_eq!(ets_public_type_for_syn_type(&module_ty), "AniModule");

        let namespace_ty: Type = syn::parse_quote!(AniNamespace<'_>);
        assert_eq!(ets_public_type_for_syn_type(&namespace_ty), "AniNamespace");

        let variable_ty: Type = syn::parse_quote!(AniVariable);
        assert_eq!(ets_public_type_for_syn_type(&variable_ty), "AniVariable");

        let resolver_ty: Type = syn::parse_quote!(AniResolver);
        assert_eq!(ets_public_type_for_syn_type(&resolver_ty), "AniResolver");

        let deferred_ty: Type = syn::parse_quote!(Deferred<String>);
        assert_eq!(ets_public_type_for_syn_type(&deferred_ty), "AniResolver");

        let global_ty: Type = syn::parse_quote!(GlobalRef);
        assert_eq!(ets_public_type_for_syn_type(&global_ty), "GlobalRef");

        let weak_ty: Type = syn::parse_quote!(WeakRef);
        assert_eq!(ets_public_type_for_syn_type(&weak_ty), "WeakRef<Object>");
    }

    #[test]
    fn test_render_decls_emits_handle_aliases_for_object_backed_runtime_handles() {
        let decls = vec![EtsDecl {
            kind: EtsDeclKind::Global,
            target: String::new(),
            rendered: "native function inspect(module: AniModule, namespace: AniNamespace, field: AniField, variable: AniVariable, global: GlobalRef, weak: WeakRef<Object>, error: AniError, value: AnyValue): AniResolver;".to_string(),
        }];
        let rendered = render_decls(&decls, &[], &[]);
        assert!(rendered.contains("export type AniModule = Object;"));
        assert!(rendered.contains("export type AniNamespace = Object;"));
        assert!(rendered.contains("export type AniField = Object;"));
        assert!(rendered.contains("export type AniVariable = Object;"));
        assert!(rendered.contains("export type GlobalRef = Object;"));
        assert!(rendered.contains("export type AniError = Object;"));
        assert!(rendered.contains("export type AnyValue = Object;"));
        assert!(rendered.contains("export type AniResolver = Object;"));
        assert!(!rendered.contains("export type WeakRef ="));
        assert!(rendered.contains(
            "export native function inspect(module: AniModule, namespace: AniNamespace, field: AniField, variable: AniVariable, global: GlobalRef, weak: WeakRef<Object>, error: AniError, value: AnyValue): AniResolver;"
        ));
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
    fn test_generate_fn_ets_decl_maps_deferred_to_resolver_handle() {
        let sig: Signature = syn::parse_quote! {
            fn settle(deferred: Deferred<String>, value: String) -> Deferred<String>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "settle", false),
            "settle(deferred: AniResolver, value: string): AniResolver"
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
    fn test_generate_fn_ets_decl_preserves_transparent_wrappers_around_object_types() {
        register_object_type_alias("crate::models::UserInfo", "models.UserInfo");
        let sig: Signature = syn::parse_quote! {
            fn normalize(
                user: Box<crate::models::UserInfo>,
                maybe: std::sync::Arc<Option<crate::models::UserInfo>>,
                by_name: std::pin::Pin<HashMap<String, crate::models::UserInfo>>,
                label: std::borrow::Cow<'static, str>
            ) -> std::rc::Rc<crate::models::UserInfo>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "normalize", false),
            "normalize(user: models.UserInfo, maybe: models.UserInfo | null | undefined, by_name: Record<string, models.UserInfo>, label: string): models.UserInfo"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_preserves_function_type_params() {
        let sig: Signature = syn::parse_quote! {
            fn identity<T>(value: T, maybe: Option<T>, values: Vec<T>) -> T
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "identity", false),
            "identity<T>(value: T, maybe: T | null | undefined, values: Array<T>): T"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_preserves_callback_function_type_params() {
        let sig: Signature = syn::parse_quote! {
            fn apply<T>(value: T, cb: Function<(T,), T>) -> T
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "apply", false),
            "apply<T>(value: T, cb: (arg0: T) => T): T"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_preserves_nested_container_type_params() {
        let sig: Signature = syn::parse_quote! {
            fn combine<T, U>(
                either: Either<T, U>,
                record_: HashMap<String, T>,
                set: HashSet<U>,
                map: BTreeMap<String, Either<T, U>>
            ) -> BTreeMap<String, Either<T, U>>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "combine", false),
            "combine<T, U>(either: T | U, record_: Record<string, T>, set: Set<U>, map: Map<string, T | U>): Map<string, T | U>"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_preserves_sync_and_cell_wrappers_around_object_types() {
        register_object_type_alias("crate::models::UserInfo", "models.UserInfo");
        let sig: Signature = syn::parse_quote! {
            fn normalize(
                user: std::sync::Mutex<crate::models::UserInfo>,
                maybe: std::sync::RwLock<Option<crate::models::UserInfo>>,
                by_name: std::cell::RefCell<HashMap<String, crate::models::UserInfo>>,
                late: std::sync::OnceLock<crate::models::UserInfo>,
                lazy: std::sync::LazyLock<crate::models::UserInfo>
            ) -> std::mem::MaybeUninit<crate::models::UserInfo>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "normalize", false),
            "normalize(user: models.UserInfo, maybe: models.UserInfo | null | undefined, by_name: Record<string, models.UserInfo>, late: models.UserInfo, lazy: models.UserInfo): models.UserInfo"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_uses_primitive_nullish_variants_for_option_and_either() {
        let sig: Signature = syn::parse_quote! {
            fn convert(a: Option<i32>, b: Option<bool>, c: Either<String, i32>) -> Either<String, i32>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "convert", false),
            "convert(a: int | null | undefined, b: boolean | null | undefined, c: string | int): string | int"
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
            "maybe_value(value: int | null | undefined): string | null | undefined"
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
            "maybe_text(value: string | undefined, fallback: string | null | undefined): undefined"
        );
    }

    #[test]
    fn test_exposed_ets_type_result_strategy_is_explicit() {
        let ty: Type = syn::parse_quote!(Result<Option<i32>, ani::Error>);
        let surface = EtsTypeSurface::from_syn_type(&ty);
        assert_eq!(surface.public_ty, "int | null | undefined");
        assert_eq!(surface.native_ty, "int | null");
        assert!(surface.requires_bridge());
    }

    #[test]
    fn test_exposed_ets_type_surface_tracks_nested_result_either_object() {
        let ty: Type =
            syn::parse_quote!(Result<Either<Option<crate::models::UserInfo>, String>, ani::Error>);
        let surface = EtsTypeSurface::from_syn_type(&ty);

        assert_eq!(
            surface.public_ty,
            "models.UserInfo | string | null | undefined"
        );
        assert_eq!(surface.native_ty, "models.UserInfo | string | null");
        assert!(surface.requires_bridge());
    }

    #[test]
    fn test_exposed_ets_type_surface_centralizes_bridge_helpers() {
        let ty: Type = syn::parse_quote!(Result<Option<crate::models::UserInfo>, ani::Error>);
        let surface = EtsTypeSurface::from_syn_type(&ty);

        assert_eq!(
            surface.ty_for_option_style(OptionStyle::Nullish),
            "models.UserInfo | null | undefined"
        );
        assert_eq!(
            surface.ty_for_option_style(OptionStyle::NullOnly),
            "models.UserInfo | null"
        );
        assert_eq!(
            surface.render_input_expr("user"),
            "user == undefined ? null : user"
        );
        assert_eq!(
            surface.render_output_body("__ani_native_user()"),
            "  let __ani_result = __ani_native_user();\n  return __ani_result == null ? undefined : __ani_result;"
        );
        assert_eq!(surface.iterator_item_ty(), "models.UserInfo");
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
            "normalize(left: string | null | undefined, right: string | null | undefined, user: models.UserInfo | null | undefined): models.UserInfo | null | undefined"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_deduplicates_nested_option_nullish_suffix() {
        let sig: Signature = syn::parse_quote! {
            fn maybe_nested(value: Option<Option<String>>) -> Option<Option<i32>>
        };
        assert_eq!(
            generate_fn_ets_decl(&sig, "maybe_nested", false),
            "maybe_nested(value: string | null | undefined): int | null | undefined"
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
    fn bigint_uses_the_real_ets_surface() {
        let ty: Type = syn::parse_quote!(ani::conversions::BigInt);
        let surface = EtsTypeSurface::from_syn_type(&ty);
        assert_eq!(surface.public_ty, "bigint");
        assert_eq!(surface.object_default_value, "0n");
    }

    #[test]
    fn test_object_field_surface_tracks_container_defaults() {
        let record_ty: Type = syn::parse_quote!(HashMap<String, crate::models::UserInfo>);
        let set_ty: Type = syn::parse_quote!(HashSet<crate::models::UserInfo>);
        let map_ty: Type = syn::parse_quote!(BTreeMap<String, crate::models::UserInfo>);
        let native_ptr_ty: Type =
            syn::parse_quote!(ani::conversions::NativePointer<crate::models::UserInfo>);
        let any_value_ty: Type = syn::parse_quote!(ani::conversions::AnyValue);
        let tuple_value_ty: Type = syn::parse_quote!(ani::conversions::TupleValue);
        let enum_item_ty: Type = syn::parse_quote!(ani::conversions::EnumItem);
        let result_ty: Type = syn::parse_quote!(Result<crate::models::UserInfo, ani::Error>);
        let either_ty: Type = syn::parse_quote!(Either<crate::models::UserInfo, String>);

        assert_eq!(
            EtsTypeSurface::from_syn_type(&record_ty).object_default_value,
            "{} as Record<string, models.UserInfo>"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&set_ty).object_default_value,
            "new Set<models.UserInfo>()"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&map_ty).object_default_value,
            "new Map<string, models.UserInfo>()"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&native_ptr_ty).public_ty,
            "long"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&native_ptr_ty).object_default_value,
            "0"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&any_value_ty).public_ty,
            "AnyValue"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&tuple_value_ty).public_ty,
            "TupleValue"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&enum_item_ty).public_ty,
            "EnumItem"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&result_ty).object_default_value,
            "null as models.UserInfo"
        );
        assert_eq!(
            EtsTypeSurface::from_syn_type(&either_ty).object_default_value,
            "null as models.UserInfo | string"
        );
    }

    #[test]
    fn test_generate_object_field_ets_decl_uses_surface_defaults_for_containers() {
        let record_ty: Type = syn::parse_quote!(HashMap<String, crate::models::UserInfo>);
        let set_ty: Type = syn::parse_quote!(HashSet<crate::models::UserInfo>);
        let map_ty: Type = syn::parse_quote!(BTreeMap<String, crate::models::UserInfo>);

        assert_eq!(
            generate_object_field_ets_decl("record_", &record_ty, false),
            "record_: Record<string, models.UserInfo> = {} as Record<string, models.UserInfo>;"
        );
        assert_eq!(
            generate_object_field_ets_decl("set", &set_ty, false),
            "set: Set<models.UserInfo> = new Set<models.UserInfo>();"
        );
        assert_eq!(
            generate_object_field_ets_decl("map", &map_ty, false),
            "map: Map<string, models.UserInfo> = new Map<string, models.UserInfo>();"
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
            "native constructor(name: string | null | undefined, age: int);"
        );
    }
}
