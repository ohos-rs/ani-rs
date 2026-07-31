---
title: 错误处理
description: 使用 Result、Error 和 Status 把 Rust 失败转换为 ArkTS 异常或 Promise rejection。
---

可恢复失败应使用 `ani::Result<T>`。同步导出返回 `Err` 时，ArkTS 调用抛出异常；异步导出返回 `Err` 时，Promise reject。

## 返回标准错误

```rust
use ani::prelude::*;
use ani_derive::ani;

#[ani]
pub fn divide(a: f64, b: f64) -> Result<f64> {
    if b == 0.0 {
        return Err(Error::new(
            Status::InvalidArgs,
            "Cannot divide by zero",
        ));
    }

    Ok(a / b)
}
```

ArkTS 侧使用普通异常处理：

```ts
try {
  divide(4.0, 0.0)
} catch (error) {
  console.error(error)
}
```

## 常用 Status

| Status | 适用场景 |
| --- | --- |
| `InvalidArgs` | 参数组合不合法 |
| `InvalidType` | 运行时值类型不匹配 |
| `NotFound` | module、class、method 或业务资源不存在 |
| `OutOfRange` | 索引或数值超出允许范围 |
| `OutOfMemory` | 分配失败 |
| `GenericFailure` | 没有更具体分类的失败 |

错误还可以使用 `Error::with_cause(...)` 保留原因链。

## 可扩展业务错误

`Error<S>` 可通过 `with_code`、`with_metadata` 和 `with_cause` 扩展。业务错误也可以直接实现 `AniErrorPayload`，无需先映射到框架 `Status` 或字符串：

```rust
#[derive(Debug)]
struct AuthError {
    operation: String,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "authentication failed: {}", self.operation)
    }
}

impl AniErrorPayload for AuthError {
    fn ani_status(&self) -> &str { "InvalidCredentials" }
    fn ani_code(&self) -> i32 { 71001 }
    fn ani_message(&self) -> &str { "token is invalid" }
    fn visit_ani_metadata(&self, visit: &mut dyn FnMut(&str, &str)) {
        visit("operation", &self.operation);
    }
}

#[ani(async)]
async fn authenticate(token: String)
    -> std::result::Result<(), AuthError>
{
    Err(AuthError { operation: format!("token:{token}") })
}
```

同步 `Result`、`#[ani(async)]`、`AsyncTask`、`Deferred` 和 async stream 共用这一协议。Promise rejection 使用标准 ArkTS `Error`：`name` 保存 status，`code` 和 `message` 保持业务值，`cause` 保存可扩展的 `Record` 上下文（`status`、`metadata` 和可选嵌套 `cause`）。因此新增业务字段不需要修改调度器或宏生成代码。

```ts
try {
  await authenticate('bad')
} catch (value) {
  const error = value as Error
  console.log(error.name, error.code, error.message)
  const context = error.cause as Record<string, Object>
  const metadata = context['metadata'] as Record<string, string>
  console.log(metadata['operation'])
}
```

## anyhow 集成

启用 `error_anyhow` 后，可以用 `?` 转换 `anyhow::Error`：

```toml
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", features = ["error_anyhow"] }
anyhow = "1"
```

```rust
use anyhow::Context;

fn read_config(path: &str) -> ani::Result<String> {
    std::fs::read_to_string(path)
        .context("failed to read config")
        .map_err(Into::into)
}
```

## 直接抛出运行时错误

拿到 `Env` 时可以使用：

- `env.throw_error_message(...)`
- `env.throw_type_error(...)`
- `env.throw_range_error(...)`
- `env.throw(error)`

一般导出函数仍应直接返回 `Result<T>`，让 wrapper 统一处理返回值和 pending exception。

## Panic

生成的 wrapper 会捕获可 unwind 的 Rust panic 并转换为运行时错误，但 panic 不应作为业务控制流：

- 预期失败使用 `Result`。
- `panic = "abort"` 或进程级 abort 无法恢复。
- 不要让析构、线程或 FFI 回调中的 panic 越过边界。

可运行示例位于 `examples/error`。
