# ANI-RS 设计说明（ArkTS 1.2）

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
  └─ tokio Promise bridge（feature: tokio_rt）
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

约束：

- Promise 形态下，当前仍会把常规参数先转换后再跨线程移交给 runtime worker；因此这部分捕获值仍需满足 `Send + 'static`
- future 的输出值和错误值本身不再因为 runtime bridge 被强制要求 `Send`
- 仍不建议在 async 任务中直接跨线程持有 `AniObject/AniRef` 等 VM handle；应优先依赖注入重建或显式 `GlobalRef` 托管模式
- 当前 Docker/ArkVM 稳定回归已经覆盖全局 env 注入、async constructor/getter/setter 组合、以及 static class 注入；class-instance 的 `this/self` async 路径仍在继续收口，当前主要由 Rust 单测覆盖

`tokio_rt` 行为：

- 开启 `ani` 的 `tokio_rt` feature 时，Promise 形态使用 dedicated local worker 执行 future，blocking 形态使用 current-thread runtime 执行 future
- 未开启时，wrapper 仍可编译，但 Promise 会立即 reject 并提示开启 `tokio_rt`

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

## 引用与生命周期

ANI 引用模型：

- LocalRef：作用域内有效
- GlobalRef：显式创建/显式释放
- WeakRef：可被 GC 失效，使用时需 upgrade 检查

当前策略：

- 句柄能力已提供底层 API 与基础 example
- 在 async 场景中仍限制直接传递线程相关 handle，避免越过 VM/线程边界

## 与 napi-rs 对齐策略

可对齐方向（已在推进）：

- 宏级 async Promise 体验
- panic 安全边界
- 自动注册与低样板导出体验

需要 ANI 语义下重新设计的方向：

- 类似 `ThreadsafeFunction` 的跨线程回调模型
- 引用托管（ref container）与 async handle 参数放开策略

详细差异与路线见：

- `docs/napi-rs-diff.md`
- `docs/capability-gap.md`
