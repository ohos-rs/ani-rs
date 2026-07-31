---
title: 异步与 Promise
description: 使用 #[ani(async)] 和 Tokio 把 Rust Future 暴露为 ArkTS Promise。
---

## 等待 ArkTS Promise

`PromiseRaw<T>::into_future` 把 ArkTS `Promise<T>` 提升为可跨线程轮询的 Rust `PromiseFuture<T>`：

```rust
#[ani]
pub fn await_arkts(
    env: &Env<'_>,
    promise: PromiseRaw<'_, String>,
) -> Result<PromiseRaw<'static, String>> {
    let future = promise.into_future(env)?;
    ani::tokio::spawn_future(env, future).map(PromiseRaw::into_static)
}
```

Future 持有全局 ANI 引用，并在实际轮询线程自动 attach。`cancel()` 和 Drop 都会释放该引用；ANI 没有 Promise 取消原语，因此取消停止的是 Rust 侧等待，不会强制终止 ArkTS 操作。

异步 I/O 或需要等待的 Rust API，优先写成 `#[ani(async)] async fn`。生成的 ArkTS 返回类型是 `Promise<T>`。

## 开启异步运行时

```toml title="Cargo.toml"
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", features = ["async", "tokio_time"] }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
tokio = { version = "1", default-features = false, features = ["time"] }
```

`async` 是 `tokio_rt` 的易用别名。根据实际使用选择 `tokio_time`、`tokio_fs`、`tokio_net`、`tokio_sync` 等 feature。

:::caution
没有启用 `async` 或 `tokio_rt` 时，`#[ani(async)]` 仍可能通过编译，但返回的 Promise 会立即 reject。
:::

## 导出 async fn

```rust
use ani::prelude::*;
use ani_derive::ani;
use std::time::Duration;

#[ani(async)]
pub async fn delayed_square(
    input: i32,
    delay_ms: i32,
) -> Result<i32> {
    tokio::time::sleep(
        Duration::from_millis(delay_ms.max(0) as u64),
    )
    .await;

    Ok(input * input)
}
```

对应 ArkTS API：

```ts
let result = await delayed_square(7, 20)
console.log(result)
```

返回 `Err` 时 Promise 会 reject。优先使用拥有所有权的参数，例如 `String`、`Vec<T>`、`ArrayBuffer` 或派生对象。

## 异步 Class 方法

`#[ani(async)]` 可以和 class receiver、静态方法以及注入参数组合：

```rust
#[ani(class = "Counter")]
impl Counter {
    #[ani(async)]
    pub async fn increment_later(
        &mut self,
        step: i32,
    ) -> Result<i32> {
        tokio::time::sleep(
            std::time::Duration::from_millis(10),
        )
        .await;

        self.value += step;
        Ok(self.value)
    }

    #[ani(static, async)]
    pub async fn runtime_ready(
        class: &AniClass<'_>,
    ) -> Result<bool> {
        Ok(!class.is_null())
    }
}
```

`env`、`this` 和 `class` 由 wrapper 在合适的运行时线程上恢复。不要把原始 local handle 自行移动进 `tokio::spawn`。

## 手动创建 Promise

需要立即 resolve 或 reject 时，可以使用 `Env` helper：

```rust
#[ani]
pub fn ready_message(
    env: &Env<'_>,
    value: String,
) -> Result<PromiseRaw<'static, String>> {
    env.promise_resolved(format!("ready:{value}"))
        .map(PromiseRaw::into_static)
}
```

需要自行控制完成时机时：

```rust
#[ani]
pub fn create_deferred(
    env: &Env<'_>,
    value: String,
) -> Result<PromiseRaw<'static, String>> {
    let (deferred, promise) = env.promise_new_typed::<String>()?;
    deferred.resolve_value(env, value)?;
    Ok(promise.into_static())
}
```

常用选择：

| 需要 | 使用 |
| --- | --- |
| 普通 Rust Future | `#[ani(async)]` |
| 立即返回成功 Promise | `Env::promise_resolved` |
| 立即返回失败 Promise | `Env::promise_rejected` |
| 手动 resolve / reject | `Env::promise_new_typed` + `Deferred<T>` |
| 已有底层 resolver | `AniResolver` |

## 跨 await 的数据

跨线程或 `await` 边界的数据必须保持有效：

- 普通 Rust 值使用 owned 类型。
- ArkTS 对象长期保存时使用 `Ref<T>` 或 `GlobalRef`。
- 回调长期保存时使用 `FunctionRef<Args, Return>`。
- `Env<'_>`、`AniObject<'_>` 等 local handle 不应放进独立线程或长期任务。

宏会为已支持的引用参数建立托管容器，但自定义 wrapper 仍需要显式设计所有权。更多说明见 [引用与生命周期](/guide/references/)。

## 阻塞工作

`async fn` 适合异步 I/O。长时间 CPU 计算或阻塞系统调用不会因为放进 async 函数就自动变得非阻塞，应使用专用工作线程或 `tokio::task::spawn_blocking`，并限制并发。

完整示例位于 `examples/async_wrapper`。
