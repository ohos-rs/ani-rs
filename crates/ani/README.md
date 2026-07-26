# ANI-RS

Safe, ergonomic Rust bindings for the ArkTS 1.2 Native Interface.

## Quick start

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs" }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
```

```rust
use ani_derive::ani;

#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani]
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}
```

The first `#[ani]` generates `ANI_Constructor`; exported items are registered automatically when ArkTS loads the native library.

## Async functions

```toml
[dependencies]
ani = {
  git = "https://github.com/ohos-rs/ani-rs",
  features = ["async", "tokio_time"]
}
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
tokio = { version = "1", default-features = false, features = ["time"] }
```

```rust
use ani::prelude::*;
use ani_derive::ani;

#[ani(async)]
pub async fn load_value(key: String) -> Result<String> {
    Ok(format!("value:{key}"))
}
```

## Crates

| Crate | Purpose |
| --- | --- |
| `ani` | Runtime wrappers, conversions, errors, Promise bridge and references |
| `ani-derive` | `#[ani]`, `AniClass` and `AniEnum` code generation |
| `ani-sys` | Raw ANI C bindings |

For classes, type conversions, generated ETS, OpenHarmony builds and troubleshooting, see the [user documentation](../../website/src/content/docs/guide/getting-started.md).

Licensed under MIT OR Apache-2.0.
