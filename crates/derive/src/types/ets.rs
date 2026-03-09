//! ETS native stub generation from Rust function signatures.
//!
//! Stubs are emitted during macro expansion (compile phase) so no runtime
//! registration/writing is required.

use std::collections::{BTreeMap, btree_map::Entry};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syn::{FnArg, Pat, ReturnType, Signature, Type};

use crate::codegen::should_skip_in_signature;

use super::ani_type::{AniType, PrimitiveType, StringType, WrapperType, resolve_object_type_alias};

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
    signature: String,
    is_static: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EtsObjectDecl {
    target: String,
    fields: Vec<String>,
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
    fields: Vec<String>,
    methods: Vec<(bool, String)>,
}

#[derive(Default)]
struct NamespaceNode {
    functions: Vec<String>,
    classes: BTreeMap<String, ClassNode>,
    children: BTreeMap<String, NamespaceNode>,
}

impl NamespaceNode {
    fn insert_namespace_fn(&mut self, path: &str, signature: &str) {
        let mut node = self;
        for seg in path.split('.').filter(|s| !s.is_empty()) {
            node = namespace_child_mut(&mut node.children, seg);
        }
        node.functions.push(signature.to_string());
    }

    fn insert_class_method(&mut self, path: &str, is_static: bool, signature: &str) {
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
            .methods
            .push((is_static, signature.to_string()));
    }

    fn insert_object_class(&mut self, path: &str, fields: &[String]) {
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
        for field in fields {
            if !class.fields.contains(field) {
                class.fields.push(field.clone());
            }
        }
    }
}

fn render_class_block(out: &mut String, indent: usize, class_name: &str, class: &ClassNode) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}class {class_name} {{\n"));

    let inner = " ".repeat(indent + 2);
    for field in &class.fields {
        out.push_str(&format!("{inner}{field};\n"));
    }

    let mut methods = class.methods.clone();
    methods.sort_by(|a, b| a.1.cmp(&b.1));
    for (is_static, sig) in methods {
        if is_static {
            out.push_str(&format!("{inner}static native {sig};\n"));
        } else {
            out.push_str(&format!("{inner}native {sig};\n"));
        }
    }

    out.push_str(&format!("{pad}}}\n"));
}

fn render_namespace(out: &mut String, indent: usize, name: &str, node: &NamespaceNode) {
    let pad = " ".repeat(indent);
    out.push_str(&format!("{pad}namespace {name} {{\n"));
    let inner = " ".repeat(indent + 2);

    for (class_name, class) in &node.classes {
        render_class_block(out, indent + 2, class_name, class);
    }

    let mut fns = node.functions.clone();
    fns.sort();
    for sig in fns {
        out.push_str(&format!("{inner}native function {sig};\n"));
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
        root.insert_object_class(&object.target, &object.fields);
    }

    for decl in decls {
        match decl.kind {
            EtsDeclKind::Global => globals.push(decl.signature.clone()),
            EtsDeclKind::Namespace => root.insert_namespace_fn(&decl.target, &decl.signature),
            EtsDeclKind::Class => {
                root.insert_class_method(&decl.target, decl.is_static, &decl.signature)
            }
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
        for sig in globals {
            out.push_str(&format!("native function {sig};\n"));
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

pub fn emit_compile_ets_decl(kind: EtsDeclKind, target: &str, signature: &str, is_static: bool) {
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
        signature: signature.to_string(),
        is_static,
    };
    if !file_state.decls.contains(&item) {
        file_state.decls.push(item);
    }
    write_ets_file(&path, file_state);
}

pub fn emit_compile_ets_object(target: &str, fields: &[String]) {
    let Some(path) = output_path() else {
        return;
    };

    let mut state = state()
        .lock()
        .expect("failed to acquire ets compile-state lock");
    let file_state = state.entry(path.clone()).or_default();
    let item = EtsObjectDecl {
        target: target.to_string(),
        fields: fields.to_vec(),
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
    format!("{name}: {ets_type} = {default_value}")
}

fn default_value_for_object_field(ty: &AniType, ets_type: &str) -> String {
    match ty {
        AniType::Primitive(PrimitiveType::Bool) => "false".to_string(),
        AniType::Primitive(PrimitiveType::F32 | PrimitiveType::F64) => "0.0".to_string(),
        AniType::Primitive(_) => "0".to_string(),
        AniType::String(StringType::String | StringType::Str) => "\"\"".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(_)) => "null".to_string(),
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
fn rust_type_to_ets(ty: &Type) -> String {
    ani_type_to_ets(&AniType::from_syn_type(ty))
}

fn ani_type_to_ets(ty: &AniType) -> String {
    match ty {
        AniType::Primitive(p) => primitive_to_ets(p).to_string(),
        AniType::String(StringType::String | StringType::Str) => "string".to_string(),
        AniType::Unit => "void".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(inner)) => option_inner_to_ets(inner),
        AniType::Wrapper(WrapperType::Vec(inner)) => vec_inner_to_ets(inner),
        AniType::Wrapper(WrapperType::Result(inner)) => ani_type_to_ets(inner),
        AniType::Wrapper(WrapperType::Ref(inner)) => ani_type_to_ets(inner),
        AniType::Function(_) => "Function".to_string(),
        AniType::FnArgs(_) => "Array<Object>".to_string(),
        AniType::Either(either) => {
            let variants = either
                .types
                .iter()
                .map(|ty| ani_type_to_ets_union_variant(&AniType::from_syn_type(ty)))
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
                .map(|t| ani_type_to_ets(t))
                .unwrap_or_else(|| "void".to_string());
            format!("Promise<{}>", inner)
        }
        AniType::Record(record) => {
            format!("Record<string, {}>", ani_type_to_ets(record.value.as_ref()))
        }
        AniType::AniObject => "Object".to_string(),
        AniType::ArrayBuffer => "ArrayBuffer".to_string(),
        AniType::Tuple(items) => format!(
            "[{}]",
            items
                .iter()
                .map(ani_type_to_ets)
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

fn option_inner_to_ets(inner: &AniType) -> String {
    match inner {
        AniType::Primitive(p) => format!("{} | null", boxed_primitive_to_ets(p)),
        AniType::String(StringType::String | StringType::Str) => "String | null".to_string(),
        _ => format!("{} | null", ani_type_to_ets(inner)),
    }
}

fn ani_type_to_ets_union_variant(ty: &AniType) -> String {
    match ty {
        AniType::Primitive(p) => boxed_primitive_to_ets(p).to_string(),
        AniType::String(StringType::String | StringType::Str) => "String".to_string(),
        AniType::Null => "null".to_string(),
        AniType::Undefined => "undefined".to_string(),
        AniType::Wrapper(WrapperType::Option(inner)) => {
            format!("{} | null", ani_type_to_ets_union_variant(inner))
        }
        AniType::Wrapper(WrapperType::Result(inner))
        | AniType::Wrapper(WrapperType::Ref(inner)) => ani_type_to_ets_union_variant(inner),
        _ => ani_type_to_ets(ty),
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

/// Generate ETS declaration signature string for a function.
///
/// Output example: `add(a: int, b: int): int`
pub fn generate_fn_ets_decl(sig: &Signature, ets_name: &str, skip_first: bool) -> String {
    let mut params: Vec<String> = Vec::new();

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
            let ty = rust_type_to_ets(&pat_type.ty);
            params.push(format!("{name}: {ty}"));
        }
    }

    let ret = match &sig.output {
        ReturnType::Default => "void".to_string(),
        ReturnType::Type(_, ty) => rust_type_to_ets(ty),
    };

    format!("{ets_name}({}): {ret}", params.join(", "))
}

/// Generate ETS declaration signature string for a constructor.
///
/// Output example: `constructor(name: string, age: int)`
pub fn generate_ctor_ets_decl(sig: &Signature, skip_first: bool) -> String {
    let mut params: Vec<String> = Vec::new();

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
            let ty = rust_type_to_ets(&pat_type.ty);
            params.push(format!("{name}: {ty}"));
        }
    }

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
                signature: "add(a: int, b: int): int".to_string(),
                is_static: false,
            },
            EtsDecl {
                kind: EtsDeclKind::Namespace,
                target: "Math.Utils".to_string(),
                signature: "sqrt(x: double): double".to_string(),
                is_static: false,
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                signature: "getName(): string".to_string(),
                is_static: false,
            },
            EtsDecl {
                kind: EtsDeclKind::Class,
                target: "example.Person".to_string(),
                signature: "create(name: string): long".to_string(),
                is_static: true,
            },
        ];
        let objects = vec![
            EtsObjectDecl {
                target: "UserProfile".to_string(),
                fields: vec!["id: int".to_string(), "name: string".to_string()],
            },
            EtsObjectDecl {
                target: "example.Person".to_string(),
                fields: vec!["active: boolean".to_string()],
            },
        ];

        let rendered = render_decls(&decls, &objects);

        assert!(!rendered.contains("declare "));
        assert!(rendered.contains("loadLibrary(\""));
        assert!(rendered.contains("native function add(a: int, b: int): int;"));
        assert!(rendered.contains("class UserProfile {"));
        assert!(rendered.contains("id: int;"));
        assert!(rendered.contains("name: string;"));
        assert!(rendered.contains("namespace Math {"));
        assert!(rendered.contains("native function sqrt(x: double): double;"));
        assert!(rendered.contains("namespace example {"));
        assert!(rendered.contains("class Person {"));
        assert!(rendered.contains("active: boolean;"));
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
            "process_user(user: models.UserInfo, maybe: models.UserInfo | null, list: Array<models.UserInfo>): models.UserInfo"
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
            "convert(a: Int | null, b: Boolean | null, c: String | Int): String | Int"
        );
    }

    #[test]
    fn test_generate_fn_ets_decl_option_is_nullable_not_optional() {
        let sig: Signature = syn::parse_quote! {
            fn maybe_value(value: Option<i32>) -> Option<String>
        };
        let decl = generate_fn_ets_decl(&sig, "maybe_value", false);
        assert_eq!(decl, "maybe_value(value: Int | null): String | null");
        assert!(!decl.contains("?:"));
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
