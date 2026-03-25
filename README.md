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

## Crates

| Crate | Description |
|-------|-------------|
| `ani-sys` | Raw FFI bindings to ANI C API |
| `ani-core` | Safe wrapper types and traits |
| `ani_derive` | Procedural macros |
| `ani-build` | Build script helpers |
| `ani-ets-gen` | ETS code generation tool |
| `ani-types` | Type definitions for ETS generation |

## Examples

Check out the [examples](examples/) directory:

- **new_basic** - Basic function bindings
- **new_class** - Class method bindings
- **basic** - Original example with build script
- **class** - Complex class examples

## Documentation

- [Design Document](docs/design.md) - Architecture and design decisions
- [ETS Generation](docs/ets-generation.md) - How to generate ETS declaration files
- [API Reference](https://docs.rs/ani-core) - Full API documentation

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
