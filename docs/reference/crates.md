# Workspace 结构

当前 workspace 很简单，核心只有 3 个 crate，所有 example 都围绕这 3 层组织。

> [!TIP]
> 如果你要找“支持哪些能力”，不是看 crate 分层，直接去 [支持能力总览](/reference/capabilities)。

## `ani-sys`

职责：

- 原始 ANI C API 绑定
- 为上层安全封装提供最小 FFI surface

定位：

- 只做 raw binding
- 不承载安全抽象、类型系统或 derive 逻辑

文件位置：

- `crates/sys`

## `ani`

职责：

- `Env` / `VM` 安全封装
- `ToAni` / `FromAni` 转换体系
- Promise / Deferred / Resolver bridge
- `GlobalRef` / `WeakRef` 等运行时 handle
- `ANI_Constructor` 执行时的注册和分组 bind
- Tokio runtime bridge

常用入口：

- `ani::prelude::*`

文件位置：

- `crates/ani`

## `ani-derive`

职责：

- 解析 `#[ani]`、`#[ani(init)]`、`#[ani(async)]`
- 生成 wrapper
- 生成 ANI 签名与注册元数据
- 输出 ETS public declaration

文件位置：

- `crates/derive`

## `examples/*`

`examples` 不是附属目录，而是当前仓库验证能力面的主要载体：

- 每个 example 都是 workspace member
- 编译时会各自产出 `.ets`
- ArkVM smoke 按 example 目录逐个构建、编译和运行

当前 example 总数为 52，详细入口见 [示例索引](/guide/examples)。

## Feature 组织

`ani` 当前和异步相关的 feature 已经按 napi-rs 风格细拆：

| Feature | 说明 |
| --- | --- |
| `async` | 易用别名，等价于 `tokio_rt` |
| `tokio_rt` | 打开 ani 自己的 Tokio Promise/runtime bridge |
| `tokio_fs` / `tokio_net` / `tokio_sync` 等 | 透传 Tokio 对应模块能力 |

如果你需要设计层的分工和调用链，而不是 crate 目录说明，继续看 [设计说明](/design)。
