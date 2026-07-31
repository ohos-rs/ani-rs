---
title: Cargo Features
description: 按需启用 ani-rs 的错误集成、异步运行时和 Tokio 功能。
---

`ani` 默认启用 `api24`。同步函数、类型转换和引用 API 不需要额外配置。

## 功能列表

| Feature | 作用 |
| --- | --- |
| `api23` | API 23 兼容路径，不使用 API 24 primitive boxing entry point |
| `api24` | API 24+ 原生 primitive boxing；默认 profile |
| `api26` | API 26 发布/QEMU 验证 profile，包含 `api24` |
| `error_anyhow` | 将 `anyhow::Error` 转换为 `ani::Error` |
| `serde-json` | 启用 `Json<T>`、`serde_json::Value` 与结构化 `AniEnum` 的原生 Object bridge |
| `async` | `tokio_rt` 的易用别名 |
| `tokio_rt` | 启用 ani-rs Tokio runtime 与 Promise bridge |
| `tokio_fs` | 启用 Tokio 文件系统 API |
| `tokio_full` | 启用 Tokio `full` feature |
| `tokio_io_std` | 启用 Tokio 标准输入输出 |
| `tokio_io_util` | 启用 Tokio I/O 工具 |
| `tokio_macros` | 启用 Tokio 宏 |
| `tokio_net` | 启用 Tokio 网络 API |
| `tokio_process` | 启用 Tokio 进程 API |
| `tokio_signal` | 启用 Tokio 信号 API |
| `tokio_sync` | 启用 Tokio 同步原语 |
| `tokio_test_util` | 启用 Tokio 测试工具 |
| `tokio_time` | 启用 Tokio 时间 API |

## 同步模块

```toml
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs" }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
```

## 异步模块

只开启实际使用的功能：

```toml
[dependencies]
ani = {
  git = "https://github.com/ohos-rs/ani-rs",
  features = ["async", "tokio_fs", "tokio_time"]
}
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
tokio = {
  version = "1",
  default-features = false,
  features = ["fs", "time"]
}
```

`ani` 的 Tokio feature 会透传给内部 Tokio 依赖。你的 crate 如果直接调用 `tokio::fs` 或 `tokio::time`，仍应在自己的 `tokio` 依赖中开启相同模块。

## anyhow

```toml
[dependencies]
ani = {
  git = "https://github.com/ohos-rs/ani-rs",
  features = ["error_anyhow"]
}
anyhow = "1"
```

## Serde JSON

```toml
[dependencies]
ani = {
  git = "https://github.com/ohos-rs/ani-rs",
  features = ["serde-json"]
}
serde = { version = "1", features = ["derive"] }
```

该 feature 把 serde 数据递归转换为原生 ArkTS `Record`、`Array`、boxed primitive 与 `null`；它不会把任意 serde struct 自动声明为 ArkTS class。

## 选择建议

- 没有 async API：保持默认 features。
- 只有 async Promise：使用 `async`。
- 使用具体 Tokio 模块：在 `async` 之外增加对应 `tokio_*`。
- 库代码希望控制体积与依赖：不要直接使用 `tokio_full`。
- 已有 anyhow 错误链：增加 `error_anyhow`。
- 需要 serde 消息或结构化 enum：增加 `serde-json`。
