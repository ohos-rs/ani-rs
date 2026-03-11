//! ETS native stub generation from Rust function signatures.
//!
//! Stubs are emitted during macro expansion (compile phase) so no runtime
//! registration/writing is required.

use std::collections::{btree_map::Entry, BTreeMap};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syn::{FnArg, Pat, ReturnType, Signature, Type};

use crate::codegen::should_skip_in_signature;

use super::ani_type::{resolve_object_type_alias, AniType, PrimitiveType, StringType, WrapperType};

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

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsObjectDecl {
    target: String,
    members: Vec<String>,
}

#[derive(Default)]
struct EtsFileState {
    decls: Vec<EtsDecl>,
    objects: Vec<EtsObjectDecl>,
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

#[derive(Default)]
struct ClassNode {
    members: Vec<String>,
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

    fn insert_class_member(&mut self, path: &str, rendered: &str) {
        let mut parts = path
            .split('.')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let class_name = parts.pop().unwrap_or("NativeClass").to_string();

        let mut node = self;
        for seg in parts {
            node = namespace_child_mut(&mut node.children, seg);
        }

        node.classes
            .entry(class_name)
            .or_default()
            .members
            .push(rendered.to_string());
    }

    fn insert_object_class(&mut self, path: &str, members: &[String]) {
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
            if !class.members.contains(member) {
                class.members.push(member.clone());
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

fn render_class_block(out: &mut String, indent: usize, class_name: &str, class: &ClassNode) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}class {class_name} {{\n"));

    let mut members = class.members.clone();
    members.sort();
    for member in members {
        push_indented_block(out, indent + 2, &member);
    }

    out.push_str(&format!("{pad}}}\n"));
}

fn render_namespace(out: &mut String, indent: usize, name: &str, node: &NamespaceNode) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}namespace {name} {{\n"));

    for (class_name, class) in &node.classes {
        render_class_block(out, indent + 2, class_name, class);
    }

    let mut fns = node.functions.clone();
    fns.sort();
    for function in fns {
        push_indented_block(out, indent + 2, &function);
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

fn render_decls(decls: &[EtsDecl], objects: &[EtsObjectDecl]) -> String {
    let mut out = String::from("// Auto-generated by ani-rs at compile time.\n\n");
    let lib_name = library_name();

    let mut globals: Vec<String> = Vec::new();
    let mut root = NamespaceNode::default();

    for object in objects {
        root.insert_object_class(&object.target, &object.members);
    }

    for decl in decls {
        match decl.kind {
            EtsDeclKind::Global => globals.push(decl.rendered.clone()),
            EtsDeclKind::Namespace => root.insert_namespace_fn(&decl.target, &decl.rendered),
            EtsDeclKind::Class => root.insert_class_member(&decl.target, &decl.rendered),
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
            push_indented_block(&mut out, 0, &global);
        }
    }

    if has_content {
        out.push('\n');
        out.push_str(&format!("loadLibrary(\"{lib_name}\");\n"));
    }

    out
}

fn write_ets_file(path: &PathBuf, file_state: &EtsFileState) {
    let content = render_decls(&file_state.decls, &file_state.objects);
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

pub fn emit_compile_ets_decl(kind: EtsDeclKind, target: &str, signature: &str, is_static: bool) {
    let rendered = match kind {
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
    };
    emit_compile_ets_rendered_decl(kind, target, &rendered);
}

pub fn emit_compile_ets_class_member(target: &str, rendered: &str) {
    emit_compile_ets_rendered_decl(EtsDeclKind::Class, target, rendered);
}

pub fn emit_compile_ets_object(target: &str, members: &[String]) {
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

pub fn generate_object_field_ets_decl(name: &str, ty: &Type) -> String {
    let ani_type = AniType::from_syn_type(ty);
    let ets_type = ani_type_to_ets(&ani_type);
    let default_value = default_value_for_object_field(&ani_type, &ets_type);
    format!("{name}: {ets_type} = {default_value};")
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
        | AniType::AniObject
        | AniType::ArrayBuffer
        | AniType::Function(_)
        | AniType::FnArgs(_)
        | AniType::Tuple(_)
        | AniType::Unknown(_) => format!("null as {}", ets_type),
        AniType::Unit => "undefined".to_string(),
    }
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
        AniType::Wrapper(WrapperType::Option(inner)) => option_inner_to_ets(inner, option_style),
        AniType::Wrapper(WrapperType::Vec(inner)) => vec_inner_to_ets(inner),
        AniType::Wrapper(WrapperType::Result(inner)) => {
            ani_type_to_ets_with_option_style(inner, option_style)
        }
        AniType::Wrapper(WrapperType::Ref(inner)) => {
            ani_type_to_ets_with_option_style(inner, option_style)
        }
        AniType::Function(_) => "Function".to_string(),
        AniType::FnArgs(_) => "Array<Object>".to_string(),
        AniType::Either(either) => {
            let variants = either
                .types
                .iter()
                .map(|ty| ani_type_to_ets_union_variant(&AniType::from_syn_type(ty), option_style))
                .collect::<Vec<_>>();
            if variants.is_empty() {
                "Object".to_string()
            } else {
                variants.join(" | ")
            }
        }
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

fn option_inner_to_ets(inner: &AniType, option_style: OptionStyle) -> String {
    let inner = match inner {
        AniType::Primitive(p) => boxed_primitive_to_ets(p).to_string(),
        AniType::String(StringType::String | StringType::Str) => "String".to_string(),
        _ => ani_type_to_ets_with_option_style(inner, option_style),
    };
    match option_style {
        OptionStyle::NullOnly => format!("{} | null", inner),
        OptionStyle::Nullish => format!("{} | null | undefined", inner),
    }
}

fn ani_type_to_ets_union_variant(ty: &AniType, option_style: OptionStyle) -> String {
    match ty {
        AniType::Primitive(p) => boxed_primitive_to_ets(p).to_string(),
        AniType::String(StringType::String | StringType::Str) => "String".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(inner)) => match option_style {
            OptionStyle::NullOnly => format!(
                "{} | null",
                ani_type_to_ets_union_variant(inner, option_style)
            ),
            OptionStyle::Nullish => {
                format!(
                    "{} | null | undefined",
                    ani_type_to_ets_union_variant(inner, option_style)
                )
            }
        },
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => {
            ani_type_to_ets_union_variant(inner, option_style)
        }
        _ => ani_type_to_ets_with_option_style(ty, option_style),
    }
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

fn type_requires_nullish_bridge(public_ty: &str, native_ty: &str) -> bool {
    public_ty != native_ty
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
                requires_bridge: type_requires_nullish_bridge(&public_ty, &native_ty),
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
                requires_bridge: type_requires_nullish_bridge(&public_ty, &native_ty),
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
                members: vec!["id: int = 0;".to_string(), "name: string = "";".to_string()],
            },
            EtsObjectDecl {
                target: "example.Person".to_string(),
                members: vec!["active: boolean = false;".to_string()],
            },
        ];

        let rendered = render_decls(&decls, &objects);

        assert!(!rendered.contains("declare "));
        assert!(rendered.contains("loadLibrary(\""));
        assert!(rendered.contains("native function add(a: int, b: int): int;"));
        assert!(rendered.contains("class UserProfile {"));
        assert!(rendered.contains("id: int = 0;"));
        assert!(rendered.contains("name: string = "";"));
        assert!(rendered.contains("namespace Math {"));
        assert!(rendered.contains("native function sqrt(x: double): double;"));
        assert!(rendered.contains("namespace example {"));
        assert!(rendered.contains("class Person {"));
        assert!(rendered.contains("active: boolean = false;"));
        assert!(rendered.contains("native getName(): string;"));
        assert!(rendered.contains("static native create(name: string): long;"));
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
    fn test_generate_ctor_ets_decl() {
        let sig: Signature = syn::parse_quote! {
            fn person_new(this: i64, name: String, age: i32)
        };
        assert_eq!(
            generate_ctor_ets_decl(&sig, true),
            "constructor(name: string, age: int)"
        );
    }
}
