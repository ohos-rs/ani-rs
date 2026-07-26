---
title: 类型系统与 ETS 面
description: ANI 签名、ArkTS 公共类型与 ToAni/FromAni 转换之间的映射关系。
---

`ani-rs` 的类型系统不是单纯为了做运行时转换，它同时承担三件事：

- 生成 ANI bind signature
- 生成 ArkTS / ETS public type
- 驱动 `ToAni` / `FromAni` 运行时转换

核心抽象是：

- `ToAni`
- `FromAni`
- `AniType`

:::tip
如果你只想快速确认“某种类型现在支不支持”，先跳到 [支持能力总览](/reference/capabilities) 里的“类型系统与转换”表。
:::

## 常见类型映射

| Rust | ANI / bind 语义 | ArkTS / ETS |
| --- | --- | --- |
| `bool` | `Z` | `boolean` |
| `i32` | `I` | `int` |
| `i64` | `J` | `long` |
| `f64` | `D` | `double` |
| `String` | `Lstd/core/String;` | `String` |
| `Vec<T>` | 数组 / fixed-array 兼容签名 | `Array<T>` |
| `Option<T>` | nullish union | `T \| null` |
| `Either<T, Undefined>` | undefined union | `T \| undefined` |
| `Either3<T, Null, Undefined>` | 完整 nullish union | `T \| null \| undefined` |
| `PromiseRaw<T>` | Promise bridge | `Promise<T>` |
| `AniObject` | object-backed handle | `Object` |

## Null 与 Undefined

这部分是仓库里反复强调的点，因为 ArkTS 语言特性和运行时值模型不能混为一谈。

当前规则：

- `Option<T>` 映射成 `T | null`
- `Either<T, Undefined>` 映射成 `T | undefined`
- `Either3<T, Null, Undefined>` 映射成 `T | null | undefined`
- ArkTS 可选参数和可选属性是语言层语法，不会自动从 `Option<T>` 推导出来

可以直接对照：

- `examples/optional`
- `examples/nullish_union`
- `examples/union`

## `Unknown -> Object` 的收敛现状

当前已经进入正式类型分支的内容包括：

- nominal 自定义 object / class
- record / set / map
- array / fixed-array wrapper
- 常见 string-like owned wrapper
- `Deferred<T>` typed resolver handle
- 一批运行时 handle 的 public ETS alias

仍然保留 `Unknown` 兜底的主要是两类：

- genuinely unknown 的自定义 Rust 路径
- 当前 ArkTS / ANI 语义不适合继续 nominal 化的对象面

## 运行时 handle 的 public type model

当前 public ETS surface 已经定型：

| Rust handle | ETS public type |
| --- | --- |
| `AniModule` | `Object` |
| `AniNamespace` | `Object` |
| `AniVariable` | `Object` |
| `AniResolver` | `Object` |
| `GlobalRef` | `Object` |
| `WeakRef` | `WeakRef<Object>` |

这里的关键点不是“把类型弱化成 Object”，而是：

- 这些类型在 runtime 上确实是 object-backed handle
- 文档和生成结果会保留命名语义，便于用户理解它们不是普通业务对象

## 泛型 object/class 的边界

已经支持：

- `#[ani(object)]` / `#[derive(AniClass)]` 的 generic struct
- ETS public declaration 中保留 `class Foo<T>`
- object-backed generic instantiation 的 ArkVM roundtrip

仍然需要注意：

- generic field 的 primitive instantiation 仍受 ArkVM generic slot runtime model 约束
- 这属于运行时边界，不是 derive 漏实现

对照 example：

- `examples/derive_shapes`

## 相关源码

如果你要沿着实现往下看，入口主要在：

- `crates/ani/src/conversions/*`
- `crates/derive/src/types/ani_type.rs`
- `crates/derive/src/types/conversion.rs`
- `crates/derive/src/types/ets.rs`
