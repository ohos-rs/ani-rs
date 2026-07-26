---
title: 异步与 Tokio
description: 使用 #[ani(async)]、Deferred 与 AniResolver 桥接 ArkTS Promise。
---

`ani-rs` 里所有异步能力都围绕一条主线收敛：把 Rust `async fn`、手动 `Deferred<T>` 和运行时 `AniResolver` 统一桥接到 ArkTS `Promise<T>`。

:::note
当前最推荐的路径就是 `#[ani(async)]`。只有在你需要手动控制 Promise 生命周期或自己托管句柄时，再下探到 `Deferred<T>`、`AniResolver` 或 manual tokio helper。
:::

## Feature 设计

当前 `ani` crate 的相关 feature 如下：

| Feature | 作用 |
| --- | --- |
| `async` | 易用别名，等价于开启 `tokio_rt` |
| `tokio_rt` | 打开 ani 自身的 tokio runtime / Promise bridge |
| `tokio_time` | 透传 Tokio `time` 能力 |
| `tokio_fs` / `tokio_net` / `tokio_sync` 等 | 按 napi-rs 命名方式细拆的 Tokio passthrough features |

最常见的配置：

```toml title="推荐配置"
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", package = "ani", features = ["async", "tokio_time"] }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs", package = "ani-derive" }
tokio = { version = "1", default-features = false, features = ["time"] }
```

```toml title="最小开启"
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", package = "ani", features = ["tokio_rt"] }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs", package = "ani-derive" }
tokio = { version = "1", default-features = false, features = ["rt", "sync"] }
```

## `#[ani(async)]`

最直接的用法是导出一个返回 `Result<T>` 的 async 函数：

```rust
use ani::Result;
use ani_derive::ani;
use std::time::Duration;

#[ani(async)]
pub async fn delayed_square(input: i32, delay_ms: i32) -> Result<i32> {
    if delay_ms > 0 {
        tokio::time::sleep(Duration::from_millis(delay_ms as u64)).await;
    }
    Ok(input * input)
}
```

当前已补齐的组合能力：

- `env` / `this` / `class` 注入
- Rust `self` receiver
- `signature = "..."`
- `constructor / getter / setter`
- 常见 ref-backed 参数的 `RefContainer` 托管

对应的完整示例集中在 `examples/async_wrapper`。

:::tip
如果你只是要一个 async native API，先写 `#[ani(async)] async fn ... -> Result<T>` 即可。不要一开始就自己管理 resolver 和跨线程句柄。
:::

## Promise helper

如果你不想只依赖宏，也可以直接使用 env-rooted Promise helper：

```rust
use ani::prelude::*;

pub fn promise_ready(env: &Env<'_>, value: String) -> ani::Result<PromiseRaw<String>> {
    env.promise_resolved(value)
}
```

当前已形成统一能力面的接口包括：

- `Env::promise_new_typed<T>()`
- `Env::promise_resolved(...)`
- `Env::promise_rejected(...)`
- `Env::promise_rejected_with_error(...)`
- `AniResolver::resolve_value(...)`
- `AniResolver::reject_error(...)`
- `AniResolver::reject_message(...)`
- `AniResolver::into_deferred(...)`
- `Deferred<T>::new(...)`
- `Deferred<T>::from_resolver(...)`

## 常见用法怎么选

| 场景 | 推荐入口 |
| --- | --- |
| 导出一个 Promise 方法 | `#[ani(async)]` |
| 立即返回已 resolve / reject 的 Promise | `Env::promise_resolved / promise_rejected` |
| 想手动控制 resolve / reject 时机 | `Env::promise_new_typed<T>()` + `Deferred<T>` |
| 已经拿到了 resolver handle | `AniResolver` |

## `RefContainer` 与句柄托管

异步桥接时最容易出问题的是“把线程关联的 runtime handle 直接跨线程搬运”。当前策略是：

- 对常见 ref-backed 参数，由宏自动进入 `RefContainer`
- 对 `this` / `class` 注入，在运行时线程上重建线程关联句柄
- 对 `GlobalRef`、`Ref<T>`、`FunctionRef<...>`，manual tokio helper 也已经能统一托管

已经覆盖的实测路径包括：

- local object handle
- typed `Ref<AniObject<'static>>`
- `GlobalRef`
- `FunctionRef<...>`

## 仍然存在的运行时边界

这些点需要明确，不然很容易把“已支持”误解成“完全无条件放开”：

- Promise 形态下，调用线程会先完成常规参数转换，再把可捕获值移交给 runtime worker
- 因此尚未自动托管覆盖的那部分捕获值，仍需满足 `Send + 'static`
- 对自定义 wrapper 或手写 handle，如果不在 `RefContainer` 覆盖面内，仍建议显式使用 `GlobalRef` / `RefContainer`

## 验证入口

优先看这几个例子：

- `examples/async_wrapper`
- `examples/reference`
- `examples/weak_ref`

实际回归命令见 [测试与 ArkVM 回归](/guide/testing)。如果想先从全局视角确认能力范围，转到 [支持能力总览](/reference/capabilities)。
