---
title: 互操作能力边界
description: ani-rs 与 napi-rs / jni-rs 的能力对照，以及 ANI C API 的固有限制。
---

ani-rs 的 API 设计对齐 napi-rs 与 jni-rs 的成熟约定，但 ANI C API 本身与 Node-API / JNI 的能力面不同。本页说明哪些能力已对齐、哪些以不同形式提供、哪些受 ANI 限制暂不可用，帮助你在迁移或选型时正确设定预期。

## 已对齐的能力

| 能力 | napi-rs / jni-rs 对应 | ani-rs 提供 |
| --- | --- | --- |
| 线程安全函数 | napi `ThreadsafeFunction` | `ThreadsafeFunction` |
| 异步任务 | napi `AsyncTask` / `Task` | `AsyncTask` + `Task` trait |
| Promise 双向桥接 | napi `Promise<T>` | `PromiseRaw` / `Promise` / `Deferred` |
| 自定义异步运行时 | napi `tokio_rt` | `AsyncRuntime` trait + `register_async_runtime` |
| 全局/弱引用 RAII | jni `GlobalRef` / napi `Ref` | `GlobalRef` / `WeakRef`（`Drop` 自动释放） |
| 单个局部引用 RAII | jni `AutoLocal` | `AutoLocal`（`env.auto_local(...)`） |
| 局部引用作用域 | jni `with_local_frame` | `env.create_local_scope`（RAII guard） |
| 原生资源句柄 | napi `External` / `wrap` | `NativePointer` / `ManagedResource` |
| serde 集成 | napi `serde-json` | `Json` / `serde_json::Value` 转换 |
| Date | napi `Date` | `SystemTime` / `chrono::DateTime<Utc>` ↔ `escompat.Date` |
| 同步迭代器 | JS 迭代协议 | `AniIterator<T>`（消费）+ `*Iterator` class 绑定（生产） |
| 异步迭代器 | napi 异步迭代 | `#[ani]` AsyncIterator class 绑定 |
| VM 线程挂载 | jni `attach_current_thread`（scoped/permanent） | `attach_current_thread` / `attach_current_thread_permanently` |

## 以不同形式提供的能力

- **字符串访问**：JNI 的 `JavaStr`（`GetStringUTFChars`）可零拷贝借用 VM 内部字符串；ANI 只提供拷贝式 `String_GetUTF8`。ani-rs 的 `env.get_string` 总是产生一次拷贝。
- **数组访问**：JNI 的 `GetPrimitiveArrayCritical` 支持临界区零拷贝；ANI 定长数组通过 `FixedArray_GetRegion` / `SetRegion` 拷贝访问。`OwnedTypedArray<T>` 对 TypedArray/ArrayBuffer 的读写同样基于拷贝。
- **类工厂 / 属性描述符**：napi-rs 可在运行时动态定义 class 与属性。ArkTS 1.2 是静态类型系统，class 形状必须在编译期由 ETS 声明确定；ani-rs 通过 `#[ani(class = ...)]` 宏在编译期生成 ETS 声明与绑定。

## ANI 限制暂不可用的能力

- **零拷贝外部缓冲区**：Node-API 的 `napi_create_external_arraybuffer` 可将原生内存直接暴露为 ArrayBuffer 且带 finalizer；ANI 没有等价接口，Rust ↔ ArkTS 的二进制数据必须经过一次拷贝。
- **对象监视器**：JNI 的 `MonitorEnter` / `MonitorExit` 可对任意对象加锁；ANI 无对应接口。跨语言同步请使用 Rust 侧原语（`Mutex` 等）配合 `GlobalRef`。
- **实例数据与 env 级清理钩子**：Node-API 的 `napi_set_instance_data` / `napi_add_env_cleanup_hook` 无 ANI 等价物；进程级状态请使用 Rust `static` + `OnceLock`，卸载清理依赖 `ani_destroy` 生命周期回调。
- **DirectByteBuffer**：JNI 可将原生内存包装成 `java.nio.DirectByteBuffer`；ArkTS 无对应机制。

这些限制来自 ANI C API 的能力面。若后续 OpenHarmony 版本补充相关接口，ani-rs 会跟进对齐。
