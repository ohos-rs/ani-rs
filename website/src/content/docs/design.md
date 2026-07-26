---
title: ANI-RS 设计说明
description: ani-rs 面向 ArkTS 1.2 ANI 的架构分层、类型系统与安全边界。
---

## 概述

`ani-rs` 是面向 ArkTS 1.2 ANI 的 Rust 绑定库，目标是提供类似 `napi-rs` 的使用体验，但遵循 ANI 的运行时与 ABI 约束。

核心原则：

- 使用 `#[ani]` / `#[ani(init)]` / `#[ani(async)]` 提供低样板绑定体验
- 以类型安全为核心，统一 Rust/ANI/ETS 三侧的类型表达
- 默认安全边界优先（panic 不穿越 ABI，错误统一转换）

## 架构分层

```text
用户代码 (examples/*, 业务 crate)
  └─ #[ani] / #[ani(init)] / #[ani(async)]
     ↓
ani-derive
  ├─ 属性解析（module/namespace/class/getter/setter/async/...）
  ├─ wrapper 生成（参数转换、返回值转换、panic 边界）
  ├─ ANI 签名与注册目标决策
  └─ ETS 声明生成
     ↓
ani
  ├─ Env/VM 安全封装
  ├─ ToAni/FromAni 转换体系
  ├─ Promise/Deferred/Resolver
  ├─ 自动注册与绑定执行（ANI_Constructor）
  └─ tokio Promise bridge（feature: async / tokio_rt）
     ↓
ani-sys
  └─ ANI C API 原始 FFI 绑定
```

## 绑定模型

ANI 绑定目标分为三类：

- Module：模块级函数绑定
- Namespace：命名空间绑定
- Class：实例/静态方法与访问器绑定

`ani-rs` 注册路径：

1. 宏生成 `ctor` 注册回调
2. `ANI_Constructor` 触发执行回调，先收集待绑定项
3. 运行时按目标分组执行 `Module/Namespace/Class` 的 BindNative API

`#[ani(module = "...")]` 语义：

- 影响运行时 Module descriptor（`FindModule` / `Module_BindNativeFunctions`）
- ETS 侧仍按模块级函数生成（不强制生成为 namespace）

## 宏能力

### `#[ani]`

用于函数/方法导出，支持：

- module/namespace/class 目标
- constructor/getter/setter/static
- overload、类操作符（迭代器、索引等）
- 错误返回 `Result<T, E>` 自动转 ArkTS throw

### `#[ani(init)]`

用于模块初始化回调，支持：

- `#[ani(init, before_bindings)]`：在 native 绑定前执行
- `#[ani(init)]`：在 native 绑定后执行
- 可选 `env: &Env<'_>` 注入

### `#[ani(async)]`

当前支持形态：

- `#[ani(async)] async fn foo(...) -> Result<T, E>`
- 普通 async 函数、class 绑定函数、以及带 Rust `self` receiver 的 async 方法，导出为 ArkTS `Promise<T>`
- `#[ani(async)]` 可与 `signature = ...` 组合
- `#[ani(async, constructor/getter/setter)]` 可用，保持 constructor/getter/setter 的同步 ArkTS 形态，并在 wrapper 内阻塞等待 Rust future 完成
- 支持注入 `env` / `this` / `class`；Promise 形态下会在运行时线程重新构建这些线程关联句柄
- 对 `AniObject / AniRef / AnyValue / AniArray* / AniFixedArray* / AniString / AniClass / AniModule / AniNamespace / AniError / AniFnObject` 等 ref-backed 常规参数，Promise 形态下会自动通过 `RefContainer` 托管并在运行时线程恢复

约束：

- Promise 形态下，当前仍会把常规参数先转换后再跨线程移交给 runtime worker；因此这部分捕获值仍需满足 `Send + 'static`
- future 的输出值和错误值本身不再因为 runtime bridge 被强制要求 `Send`
- 仍不建议用户手写跨线程持有任意 VM handle；优先依赖宏自动 `RefContainer` 托管、注入重建，或显式 `GlobalRef/RefContainer` 模式
- 当前 Docker/ArkVM 稳定回归已经覆盖全局 env 注入、async constructor/getter/setter 组合、static class 注入，以及 class-instance 的 `this/self` 与直接 `AniObject/AniRef` 常规参数路径

Tokio feature 设计：

- `async = ["tokio_rt"]`：对齐 napi-rs 的易用别名，建议优先启用
- `tokio_rt`：打开 ani 自身的 tokio runtime / Promise bridge
- 额外暴露 `tokio_fs` / `tokio_full` / `tokio_io_std` / `tokio_io_util` / `tokio_macros` / `tokio_net` / `tokio_process` / `tokio_signal` / `tokio_sync` / `tokio_test_util` / `tokio_time`，命名与 napi-rs 对齐
- 开启 `async` 或 `tokio_rt` 时，Promise 形态使用 dedicated local worker 执行 future，blocking 形态使用 current-thread runtime 执行 future
- 未开启时，wrapper 仍可编译，但 Promise 会立即 reject 并提示开启 `async` / `tokio_rt`

## 错误与 panic 边界

- 同步 wrapper 与 receiver wrapper 都在 `catch_unwind` 内执行
- panic 统一转 ArkTS 异常（或 Promise rejection），不允许跨 ABI 传播
- async task panic/cancel 会转换为 Promise rejection，保证 Promise 最终 settle

## 类型系统与 ETS 生成

核心类型层：

- `ToAni` / `FromAni`：运行时值转换
- `AniType`：签名与 ETS public type 的统一中间表示

当前重点：

- 区分 `null` 与 `undefined` 语义
- 优先生成精确 public ETS type，持续减少 `Unknown -> Object` fallback
- 运行时 handle 的 public ETS model 明确分两类：
  - `AniModule / AniNamespace / AniVariable / AniResolver / GlobalRef` 这类 object-backed handle 保留命名，但导出为 `export type X = Object`
  - `WeakRef` 保持为 `WeakRef<Object>`，直接对齐 ArkTS `std.core.WeakRef` 容器语义

## 引用与生命周期

ANI 引用模型：

- LocalRef：作用域内有效
- GlobalRef：显式创建/显式释放
- WeakRef：可被 GC 失效，使用时需 upgrade 检查

当前策略：

- 句柄能力已提供底层 API 与基础 example
- `GlobalRef / WeakRef` 现已补齐基础 helper：`delete`、`to_local` / `to_object` / `to_class`、`upgrade`、`is_alive` / `is_released`
- `WeakRef` example 已覆盖 create/use/delete/upgrade、仅弱引用下的 GC invalidation，以及 `GlobalRef` 存活/释放两个生命周期阶段
- 在 async 场景中，优先依赖 `RefContainer` 或注入重建，避免手写越过 VM/线程边界的 handle 生命周期

## 与 napi-rs 对齐策略

可对齐方向（已在推进）：

- 宏级 async Promise 体验
- panic 安全边界
- 自动注册与低样板导出体验

需要 ANI 语义下重新设计的方向：

- 类似 `ThreadsafeFunction` 的跨线程回调模型
- 引用托管（ref container）与 async handle 参数放开策略

详细差异与路线见：

- [ani-rs 与 napi-rs 的设计差异](/napi-rs-diff)
- [能力缺口清单](/capability-gap)
