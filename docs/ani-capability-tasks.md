# ani-rs 能力补齐任务清单（对照 `include/ani.h`）

更新时间：2026-03-01

## 基线
- `__ani_interaction_api` 共 396 项。
- `Env` 层高频 API 已补齐一批（引用、模块、函数/变量、对象字段、Any、FixedArray/Tuple/Enum/UTF16 等）。
- 当前重点从“仅 Env 可用”转到“按 `conversions` 和 napi-rs 风格提供 Rust 层易用封装”。

## 已完成（本轮之前）
1. `Env` 层：引用作用域、WeakRef、Module/Namespace 成员发现、Function/Variable 调用、Array、Type 关系、Object/Class typed 能力、Any、Tuple/Enum/UTF16。
2. 示例：`reference_scope`、`weak_ref`、`module_member`、`function_variable`、`array_generic`、`type_relation`、`class_static`、`object_typed`、`fixed_tuple_enum_utf16`、`any_dynamic`。

## 待补齐能力（按优先级）
1. **P1（已完成）FixedArray Rust 封装（conversions）**
API：
`FixedArray_New_*` `FixedArray_GetRegion_*` `FixedArray_SetRegion_*`
交付：
`FixedBooleanArray/FixedCharArray/FixedByteArray/FixedShortArray/FixedIntArray/FixedLongArray/FixedFloatArray/FixedDoubleArray`
并实现 `ToAni/FromAni/TypeInfo`。
example：
`examples/fixed_array_wrapper`

2. **P2（已完成）TupleValue Rust 封装（conversions）**
API：
`TupleValue_GetItem_*` `TupleValue_SetItem_*` `TupleValue_GetNumberOfItems`
交付：
提供 `TupleValue<'env>` Rust 侧包装结构（基于已有句柄），支持 typed 读写和转换（`ToAni/FromAni/TypeInfo`）。
example：
`examples/tuple_value_wrapper`

3. **P3（已完成）EnumItem Rust 封装（conversions）**
API：
`Enum_GetEnumItemByName` `Enum_GetEnumItemByIndex` `EnumItem_Get*`
交付：
提供 `EnumItem<'env>` 与 `EnumValue`，并补齐 name/index/value 访问与转换辅助。
example：
`examples/enum_item_wrapper`

4. **P4（已完成）Class/Type 反射辅助能力**
API：
`Class_FindGetter` `Class_FindSetter` `Class_FindIndexableGetter` `Class_FindIndexableSetter` `Class_FindIterator`
交付：
补齐 `Env` 缺失接口（`find_getter/find_setter/find_indexable_getter/find_indexable_setter/find_iterator`），并提供示例。
example：
`examples/class_reflect`

5. **P5（已完成）Class 静态字段 by-name 能力**
API：
`Class_GetStaticFieldByName_*` `Class_SetStaticFieldByName_*`
交付：
补齐 `Env` by-name typed 封装（含 `Boolean/Char/Byte/Short/Int/Long/Float/Double/Ref`），和现有 by-handle 接口保持一致。
example：
`examples/class_static_by_name`

6. **P6（已完成）静态 native 方法绑定**
API：
`Class_BindStaticNativeMethods`
交付：
补齐 `bind_class_static_native_methods`，与已有 `bind_class_native_methods` 风格对齐。
example：
`examples/class_bind_static_native`

7. **P7（已完成：以 `_A` 等价方案覆盖）V/可变参族 API 覆盖**
API：
`Function_Call_*_V` `Object_CallMethod*_*_V` `Class_CallStaticMethod*_*_V`
交付：
说明：
Rust 侧无法直接构造 `va_list`，因此 `_V` 族暂不单独暴露；当前通过 `_A` 版本（`&[ani_value]` / `FnArgs+ToAniArgs`）实现同等调用能力。
example：
`examples/call_variadic_v`

8. **P8（已完成）Any 的 Rust 高层封装**
API：
`Any_*`
交付：
基于已有 `FnArgs/ToAniArgs/ToAni/FromAni`，实现 `AnyValue<'env>` 动态对象封装（属性访问、索引访问、`call/call_method/construct`）。
example：
`examples/any_value_wrapper`

## 验收标准
1. 每个任务至少 1 个独立 example。
2. 每个 example 可单独 `cargo check -p <example-package>` 通过。
3. 设计优先复用 `conversions`（`FnArgs/ToAniArgs/ToAni/FromAni`），避免重复 API 层。
