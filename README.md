# ANI-RS

A safe, ergonomic Rust library for ArkTS Native Interface (ANI), inspired by [napi-rs](https://napi.rs).

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Introduction

ANI-RS provides Rust bindings for the ArkTS 1.2 Native Interface (ANI), similar to how napi-rs provides bindings for Node.js N-API. It allows you to write native extensions for HarmonyOS/OpenHarmony applications in Rust with minimal boilerplate.

**Key Features:**

- 🚀 **Simple macros** - Just use `#[ani]` to export functions (auto-registered!)
- 🔒 **Type safe** - Automatic conversion between Rust and ArkTS types
- ⚡ **Zero-cost abstractions** - Minimal runtime overhead
- 📦 **Module support** - Bind to modules, namespaces, and classes
- 🧵 **Async bindings** - Export `async fn` as `Promise<T>` with `#[ani(async)]` (via `tokio`)
- 🎯 **Auto-registration** - No need to manually list functions or use `ani_module!`, just like napi-rs!

## Quick Start

### 1. Add dependencies

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs" }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
```

### 2. Write your Rust code

```rust
use ani::prelude::*;
use ani_derive::ani;

// Simple function binding - automatically registered!
#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[ani]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// That's it! No ani_module! needed.
// ANI_Constructor is automatically generated on first #[ani] macro usage.
// Module name is derived from CARGO_PKG_NAME.
```

### 3. Corresponding ArkTS code

```typescript
import { loadLibrary } from 'libmy_module.so';

loadLibrary('my_module');

// Now you can call the native functions
native function add(a: int, b: int): int;
native function greet(name: String): String;

console.log(add(2, 3)); // 5
console.log(greet("World")); // "Hello, World!"
```

## Feature Examples

### Module-level Functions

```rust
#[ani]
pub fn calculate(x: f64) -> f64 {
    x * 2.0 + 1.0
}
```

### Namespace Functions

```rust
#[ani(namespace = "Utils")]
pub fn format_date(year: i32, month: i32, day: i32) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}
```

### Class Methods

```rust
// Static method
#[ani(class = "Calculator", static)]
pub fn create() -> i64 {
    let calc = Box::new(Calculator::default());
    Box::into_raw(calc) as i64
}

// Instance method
#[ani(class = "Calculator")]
pub fn add(this: i64, a: i32, b: i32) -> i32 {
    a + b
}

// Constructor
#[ani(class = "Person", constructor)]
pub fn person_new(name: String, age: i32) -> i64 {
    let person = Box::new(Person { name, age });
    Box::into_raw(person) as i64
}
```

### Async Functions (Promise)

Enable `tokio` support on the `ani` dependency:

```toml
[dependencies]
ani = { git = "https://github.com/ohos-rs/ani-rs", features = ["async"] }
ani-derive = { git = "https://github.com/ohos-rs/ani-rs" }
tokio = { version = "1", default-features = false, features = ["time"] }
```

Note: if you forget to enable `ani` feature `async` (or the lower-level `tokio_rt`),
`#[ani(async)]` bindings can still compile, but the returned `Promise` will reject immediately.

Then export an `async fn` as `Promise<T>`:

```rust
use ani::prelude::*;
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

## Type Mappings

| Rust Type | ANI Signature | ArkTS Type |
|-----------|---------------|------------|
| `bool` | `Z` | `boolean` |
| `i8` | `B` | `byte` |
| `i16` | `S` | `short` |
| `i32` | `I` | `int` |
| `i64` | `J` | `long` |
| `f32` | `F` | `float` |
| `f64` | `D` | `double` |
| `String` | `Lstd/core/String;` | `String` |
| `Vec<T>` | `[T` | `Array<T>` |
| `Option<i32>` | `Lstd/core/Int;` | `Int \| null` |

## Workspace

| Crate | Description |
|-------|-------------|
| `ani-sys` | Raw FFI bindings to the ANI C API |
| `ani` | Safe runtime wrappers, conversions, Promise bridge, refs, and registration |
| `ani-derive` | Procedural macros for `#[ani]`, `#[ani(init)]`, `#[ani(async)]`, and ETS emission |

## Examples

The [`examples`](examples/) workspace currently contains 52 runnable cases covering:

- module / namespace / class bindings
- overload, constructor, getter, setter, and `impl` receiver methods
- object/class derives and ETS public type generation
- async `Promise<T>` export and manual resolver/deferred flows
- refs, `GlobalRef`, `WeakRef`, VM/runtime handles, and ArkVM smoke coverage

Start with:

- [`examples/new_basic`](examples/new_basic)
- [`examples/new_class`](examples/new_class)
- [`examples/impl_block`](examples/impl_block)
- [`examples/async_wrapper`](examples/async_wrapper)
- [`examples/weak_ref`](examples/weak_ref)

## Documentation

Repository docs ship as an Astro + Starlight package in the pnpm workspace at [`website/`](website/):

- [`website/src/content/docs/index.mdx`](website/src/content/docs/index.mdx) - Documentation home
- [`website/src/content/docs/guide/getting-started.md`](website/src/content/docs/guide/getting-started.md) - Quick start
- [`website/src/content/docs/guide/compatibility.md`](website/src/content/docs/guide/compatibility.md) - Usage notes and runtime boundaries
- [`website/src/content/docs/guide/testing.md`](website/src/content/docs/guide/testing.md) - Docker, ArkVM, OpenHarmony QEMU, and HAP verification
- [`website/src/content/docs/reference/capabilities.md`](website/src/content/docs/reference/capabilities.md) - Supported capability matrix
- [`website/src/content/docs/reference/runtime-handles.md`](website/src/content/docs/reference/runtime-handles.md) - Runtime handles and ref lifecycle
- [`website/src/content/docs/design.md`](website/src/content/docs/design.md) - Architecture and design decisions
- [`website/src/content/docs/capability-gap.md`](website/src/content/docs/capability-gap.md) - Capability baseline and scope boundaries
- [`website/src/content/docs/guide/website.md`](website/src/content/docs/guide/website.md) - Documentation development guide

Run the docs locally:

```bash
pnpm install
pnpm docs:dev
pnpm docs:check
pnpm docs:build
```

## Requirements

- Rust 1.70+
- HarmonyOS SDK (for device/emulator testing)
- ArkTS 1.2 compatible runtime

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
