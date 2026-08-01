---
title: 异步与 Promise
description: 使用 #[ani(async)]、RuntimeDomain 和可替换执行器导出 Promise。
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

默认错误类型是 `ArktsRejection`，会保留原 rejection、`stack`、带环 `cause` 图和 typed metadata。需要领域错误时，使用 `into_future_with_decoder` 传入对象安全的 `RejectionDecoder<E>`；decoder 同时接收 ArkTS rejection 与运行时取消，错误类型不被固定成 ani-rs `Error`。

异步 I/O 或需要等待的 Rust API，优先写成 `#[ani(async)] async fn`。生成的 ArkTS 返回类型是 `Promise<T>`。

## 开启异步运行时

```toml title="Cargo.toml"
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", features = ["async", "tokio_time"] }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
tokio = { version = "1", default-features = false, features = ["time"] }
```

`async-runtime` 只提供执行器无关 SPI；`async` 额外选择内置 Tokio backend。根据实际使用选择 `tokio_time`、`tokio_fs`、`tokio_net`、`tokio_sync` 等 feature。

:::caution
仅启用 `async-runtime` 时，应用必须在第一次异步调用前注册 `AsyncRuntime`；否则 Promise 以结构化错误 reject。自定义 runtime 不需要链接 Tokio。
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

## RuntimeDomain 与自定义执行器

`#[ani(async)]`、Promise、`AsyncTask`、`ThreadsafeFunction` 和 async stream 全部进入同一个 RuntimeDomain。生成宏提交的是 `RuntimeTask`，不再调用 Tokio API。carrier 本身可跨线程；backend 必须在选定线程调用 `into_local_future()`，并在同一线程 poll/drop 可能为 `!Send` 的 ANI Future。

应用可以实现并注册完整 backend：

```rust
unsafe impl AsyncRuntime for AppRuntime {
    fn spawn(&self, task: RuntimeTask)
        -> std::result::Result<(), AsyncRuntimeRejection<RuntimeTask>>
    { /* 提交 carrier */ }

    fn spawn_blocking(&self, task: RuntimeBlockingTask)
        -> std::result::Result<(), AsyncRuntimeRejection<RuntimeBlockingTask>>
    { /* 提交后调用 task.run() */ }

    fn block_on(&self, future: Pin<&mut dyn Future<Output = ()>>) -> Result<()> { /* ... */ }
    fn start(&self) -> Result<()> { Ok(()) }
    fn shutdown(&self) -> Result<()> { /* cancel、drain、join */ Ok(()) }
}

register_async_runtime(AppRuntime::new())?;
```

`shutdown` 是 unsafe contract：返回前所有 backend thread、task、closure 和 waker 必须停止执行 addon 代码。析构默认有 30 秒 watchdog，可用 `ANI_RUNTIME_SHUTDOWN_TIMEOUT_MS` 调整；非协作任务超时会 fail-fast，不会在 SO 卸载后继续运行。

```rust
let before = runtime_kernel().metrics();
shutdown_runtime_domain()?;
let after = runtime_kernel().metrics(); // 下一次提交会启动新 generation
```

## ArkTS 主动取消

`spawn_future_result_factory_with_handle` 返回 Promise 与 `RuntimeTaskHandle`。调用 `cancel_with(Box<dyn AniErrorPayload>)` 可使用任意业务错误；`bridge_token()` 把 handle 注册到生成的 `AniCancelHandle` native bridge。ETS 必须在 `AbortSignal` 创建线程读取 `signal.reason` 并调用 `handle.cancel(reason)`，worker 从不访问线程亲和的 AbortSignal。

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

队列有界并提供背压；多个并发 `next()` 按注册顺序完成。生成的 iterator 还提供 `returnIterator()` 和 `throwIterator(reason)`：前者取消生产者并完成所有 waiter，后者保留原始 ArkTS rejection 对象并拒绝 waiter。RuntimeDomain shutdown 会走同一取消路径。

二进制流可使用 `ohos_byte_stream_channel[_with_error]`，得到 `OhosReadableSource` 与 `OhosWritableSink`。API 23+ 的 `@ohos.util.stream.Readable/Writable` 子类在 ETS 线程调用 pull/write；Rust 端保留 bounded queue、drain、close、error、背压和取消语义。

:::note
上游 QEMU `v20260731` 使用的 OpenHarmony ArkTS 1.2 编译器不接受 class 中的 `[Symbol.asyncIterator]`、`return()` 和 `throw()` 源码声明，因此生成接口使用可编译的 `asyncIterator()`、`returnIterator()`、`throwIterator()` 名称。`next`、背压、return/throw、错误和析构生命周期语义完整；如果平台后续开放标准方法名，生成层可以在不修改 Rust stream API 的情况下映射到标准协议名。
:::
