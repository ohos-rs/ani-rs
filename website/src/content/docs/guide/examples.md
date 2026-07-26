---
title: 示例索引
description: 按能力分类浏览仓库内 52 个可运行的 ANI 示例。
---

仓库当前有 52 个 example。下面按“你要验证什么能力”来分组，而不是简单按目录字母序罗列。

:::tip
如果你不是在找代码，而是在问“某个能力现在到底支不支持”，先看 [支持能力总览](/reference/capabilities)。这页更适合按 example 找入口。
:::

## 基础绑定与注册

| Example | 能力点 |
| --- | --- |
| `new_basic` | 最小 `#[ani]` 导出、基础参数和返回值转换 |
| `module_binding` | `#[ani(module = "...")]` 的显式模块绑定 |
| `bind_overload` | 函数名重写与 overload |
| `init_lifecycle` | `#[ani(init)]` 与 `before_bindings` 生命周期 |
| `ets_declaration` | `.ets` 输出面、namespace/class 混合导出 |
| `template` | 常见导出模板的集合页 |

## Class / impl / 属性访问

| Example | 能力点 |
| --- | --- |
| `new_class` | 构造器、实例方法、静态方法 |
| `impl_block` | `impl` receiver、property merge、index/iterator operator |
| `class_static` | 静态方法与静态属性 |
| `class_static_by_name` | 运行时按类名查找静态成员 |
| `class_method_overload` | class method overload |
| `constructor_overload` | 多个 constructor 组合 |
| `constructor_nullish` | constructor + nullish 参数 |
| `class_bind_static_native` | 绑定已存在的静态 native 能力 |
| `class_reflect` | class 反射能力 |

## 对象模型与类型系统

| Example | 能力点 |
| --- | --- |
| `derive_shapes` | `#[derive(AniClass)]` 与 `#[ani(object)]` 的 named/tuple/unit/generic 形态 |
| `object_model` | nominal object/class 类型与属性暴露 |
| `object_typed` | typed object 参数和返回值 |
| `object_access` | object property get/set |
| `object_runtime` | 运行时 object 操作 |
| `type_relation` | 类型关系判断 |
| `interface` | interface/public type surface |
| `enum_derive` | `#[derive(AniEnum)]` unit variant |
| `enum_item_wrapper` | enum item wrapper |

## 容器、集合与复合值

| Example | 能力点 |
| --- | --- |
| `array_generic` | `Vec<T>` / `VecDeque<T>` / `LinkedList<T>` 与 object array |
| `fixed_array_wrapper` | fixed array wrapper |
| `fixed_tuple_enum_utf16` | fixed tuple / enum item / UTF-16 string 场景 |
| `record` | record 类型 |
| `map` | `Map` 转换 |
| `set` | `Set` 转换 |
| `tuple_value_wrapper` | tuple value wrapper |
| `any_value_wrapper` | `AnyValue` wrapper |
| `arraybuffer` | ArrayBuffer 读写 |
| `bigint` | bigint 映射 |

## Nullish、Union 与字符串类 wrapper

| Example | 能力点 |
| --- | --- |
| `optional` | `Option<T>` 参数与返回值 |
| `union` | `Either` / union 类型 |
| `nullish_union` | `null` / `undefined` 语义分离 |
| `string_like_owned` | `String`、路径、字符串类 owned wrapper |
| `setfield` | 字段写入与 nullish 组合场景 |

## 函数、调用与动态值

| Example | 能力点 |
| --- | --- |
| `function` | function object、callback、returning callback |
| `function_variable` | function variable 与运行时查找 |
| `call_method` | native 调 ArkTS 方法 |
| `call_variadic_v` | variadic call |
| `any_dynamic` | dynamic object / dynamic function 调用 |
| `module_member` | module member / namespace member 查找 |

## 引用、生命周期与运行时句柄

| Example | 能力点 |
| --- | --- |
| `reference` | `Ref` / `GlobalRef` 基础使用 |
| `reference_scope` | local ref scope 行为 |
| `weak_ref` | `WeakRef` 生命周期、upgrade 与 invalidation |
| `wrap_native_ptr` | native pointer 包装与显式释放 |
| `vm` | `VM`、版本和 options |
| `error` | `AniError`、异常抛出和错误转换 |

## 异步与 Promise

| Example | 能力点 |
| --- | --- |
| `async_wrapper` | `#[ani(async)]`、Promise helper、Tokio bridge、注入与 ref-container |

## 如何用这页

建议按下面的顺序找示例：

- 想从零开始，先看 `new_basic`
- 想写 class，先看 `new_class` 和 `impl_block`
- 想确认类型映射，先看 `derive_shapes`、`record`、`array_generic`
- 想确认异步和 Promise，直接看 `async_wrapper`
- 想确认运行时句柄生命周期，先看 `reference` 和 `weak_ref`

如果你更关心底层设计而不是 example 入口，直接去看 [设计说明](/design) 和 [能力缺口清单](/capability-gap)。
