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

Future 持有全局 ANI 引用。生成的 ETS continuation bridge 在原 Promise 上注册 `then`/reject 回调，settle 时直接唤醒 Rust waiter；等待过程不读取 Promise 的私有字段，也不占用 worker 或 timer。`cancel()` 和 Drop 都会注销等待并释放引用；ANI 没有 Promise 取消原语，因此取消不会强制终止 ArkTS 自身的操作。

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

`async fn` 适合异步 I/O。长时间 CPU 计算或阻塞系统调用不会因为放进 async 函数就自动变得非阻塞。对于可取消的专用工作，使用 `Task`、`AsyncTask<T>` 和 `CancellationToken`：

```rust
use ani::prelude::*;

struct Square { input: i32 }

impl Task for Square {
    type Output = i32;
    type JsValue = i32;
    type Error = Error;

    fn compute(&mut self, cancel: &CancellationToken) -> Result<i32> {
        cancel.check()?;
        Ok(self.input * self.input)
    }

    fn resolve<'env>(self, _env: &Env<'env>, value: i32) -> Result<i32> {
        Ok(value)
    }
}

#[ani]
pub fn square_in_background(input: i32) -> AsyncTask<Square> {
    AsyncTask::new(Square { input })
}
```

`Task::Error` 不固定为 ani-rs 的 `Error`；任何实现 `AniErrorPayload` 的业务错误都可以直接使用，调度边界不会先转换为字符串。

## RuntimeKernel 生命周期

Promise、`AsyncTask`、`ThreadsafeFunction` 和 async stream 共用一个惰性初始化的 `RuntimeKernel`。模块析构会先进入 closing 状态，拒绝新任务，统一取消仍在等待的操作，drain 已接收的工作并 join worker/timer；完成后内核回到 dormant，后续调用会建立新的 generation。

应用通常不需要手动管理它。测试或需要显式停机的宿主可以使用：

```rust
use ani::prelude::{runtime_kernel, shutdown_runtime};

let before = runtime_kernel().metrics();
shutdown_runtime()?;

// 下一次提交会自动初始化新的 generation。
let after = runtime_kernel().metrics();
```

不要从 RuntimeKernel 自己的 worker 内调用同步 shutdown；该路径会返回错误，以免等待当前线程造成死锁。

## 跨线程回调

`ThreadsafeFunction<Args, Return>` 是带容量的可克隆队列。dispatcher 在调用 ArkTS 前 attach 到所属 VM；`Blocking` 提供背压，`NonBlocking` 在队列满时返回 `Status::QueueFull`。`call` 返回的 `ThreadsafeFunctionCall` 既可以 `.wait()`，也可以作为 Future `await`；`close()` 后拒绝新任务并排空已入队任务。

```rust
let pending = callback.call(
    ("ready".to_string(),),
    ThreadsafeFunctionCallMode::NonBlocking,
)?;
let result = pending.wait()?;
```

完整示例位于 `examples/async_wrapper`。

## Async Iterator 与 Stream

`stream_channel(capacity)` 创建默认错误类型的有界 pull stream；需要自定义业务错误时使用 `stream_channel_with_error::<T, E>(capacity)`。每次 `next_promise()` 注册一个 FIFO waiter；没有数据时 waiter 保持休眠，数据、关闭、错误或取消发生时才被唤醒，不占用共享 worker。channel 自然断开时会先交付已经接收的队列项，再解析为 done：

```rust
#[derive(Debug)]
struct FeedError { message: String }

impl std::fmt::Display for FeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl AniErrorPayload for FeedError {
    fn ani_status(&self) -> &str { "FeedUnavailable" }
    fn ani_code(&self) -> i32 { 72001 }
    fn ani_message(&self) -> &str { &self.message }
}

let (sender, stream) =
    stream_channel_with_error::<i32, FeedError>(32)?;
sender.send(1)?;
sender.send_error(FeedError { message: "closed".into() })?;
let promise = stream.next_promise(env)?;
```

队列有界并提供背压；多个并发 `next()` 按注册顺序完成。生成的 iterator 还提供 `returnIterator()` 和 `throwIterator(reason)`：前者取消生产者并完成所有 waiter，后者保留原始 ArkTS rejection 对象并拒绝 waiter。RuntimeKernel shutdown 会走同一取消路径。

:::note
20260728 OpenHarmony ArkTS 1.2 编译器不接受 class 中的 `[Symbol.asyncIterator]`、`return()` 和 `throw()` 源码声明，因此生成接口使用可编译的 `asyncIterator()`、`returnIterator()`、`throwIterator()` 名称。`next`、背压、return/throw、错误和析构生命周期语义完整；如果平台后续开放标准方法名，生成层可以在不修改 Rust stream API 的情况下映射到标准协议名。
:::
