---
title: 值与类型转换
description: Rust 类型、ANI 签名与生成 ETS 类型之间的映射。
---

函数参数和返回值通过三个核心 trait 连接 Rust 与 ANI：

- `FromAni`：把 ArkTS 传入值转换为 Rust。
- `ToAni`：把 Rust 返回值转换为 ANI。
- `AniType`：提供绑定签名和 ETS 公共类型。

使用 `#[ani]` 时通常不需要手动调用这些 trait。

## 基础类型

| Rust | ANI 签名 | ArkTS / ETS |
| --- | --- | --- |
| `()` | `V` | `void` |
| `bool` | `Z` | `boolean` |
| `i8` / `u8` | `B` | `byte` |
| `i16` / `u16` | `S` / `C` | `short` / `char` |
| `i32` / `u32` | `I` | `int` |
| `i64` / `u64` | `J` | `long` |
| `f32` | `F` | `float` |
| `f64` | `D` | `double` |
| `String` | string reference | `string` |
| `BigInt` | `Lstd/core/BigInt;` | `bigint` |

选择整数类型时要考虑 ArkTS 的数值范围。不要仅为了方便把指针或句柄暴露成普通 `long`；优先使用 class 或引用 wrapper。

`BigInt` 使用规范化十进制字符串在 Rust 侧无损持有任意精度值：

```rust
use ani::conversions::BigInt;
use ani::error::Result;
use ani_derive::ani;

#[ani]
pub fn bigint_roundtrip(value: BigInt) -> BigInt {
    value
}

#[ani]
pub fn bigint_from_text(value: String) -> Result<BigInt> {
    BigInt::from_decimal(value)
}
```

只有显式调用 `BigInt::to_i64()` 时才会缩窄；超出范围会返回 `OutOfRange`，不会截断。

## Null 与 Undefined

`null` 和 `undefined` 是两个不同的值：

| Rust | ArkTS |
| --- | --- |
| `Option<T>` | `T \| null` |
| `Null` | `null` |
| `Undefined` | `undefined` |
| `Either<T, Undefined>` | `T \| undefined` |
| `Either3<T, Null, Undefined>` | `T \| null \| undefined` |

```rust
use ani::conversions::{Either3, Null, Undefined};
use ani_derive::ani;

#[ani]
pub fn describe(
    value: Either3<String, Null, Undefined>,
) -> String {
    match value {
        Either3::A(text) => text,
        Either3::B(_) => "null".to_string(),
        Either3::C(_) => "undefined".to_string(),
    }
}
```

ArkTS 可选参数和可选属性是语言语法，不会仅因为 Rust 使用 `Option<T>` 自动生成。

## 数组和集合

| Rust | ArkTS / ETS |
| --- | --- |
| `Vec<T>` | `Array<T>` |
| `[T; N]` | fixed array / tuple-compatible value |
| `BTreeMap<String, V>` | `Map<string, V>` |
| `HashMap<String, V>` | `Record<string, V>` |
| `HashSet<T>` | `Set<T>` |

集合转换会遍历全部元素，复杂度随元素数量增长。如果只读取二进制数据，优先使用借用的 `ArrayBufferSlice<'_>`。

## ArrayBuffer

```rust
use ani::prelude::{ArrayBuffer, ArrayBufferSlice};
use ani_derive::ani;

#[ani]
pub fn checksum(bytes: ArrayBufferSlice<'_>) -> i64 {
    bytes.iter().map(|byte| *byte as i64).sum()
}

#[ani]
pub fn zeros(size: i32) -> ArrayBuffer {
    ArrayBuffer::zeroed(size.max(0) as usize)
}
```

- `ArrayBufferSlice<'_>` 是当前调用作用域内的只读借用。
- `ArrayBuffer` 拥有数据，适合返回或跨出当前转换步骤。

## Union

使用 `Either<A, B>`、`Either3<A, B, C>` 等类型表达有限 union：

```rust
use ani::conversions::Either;

#[ani]
pub fn normalize(value: Either<String, i32>) -> String {
    match value {
        Either::A(text) => text,
        Either::B(number) => number.to_string(),
    }
}
```

## 回调

`Function<Args, Return>` 只能在当前调用作用域使用：

```rust
use ani::prelude::*;

#[ani]
pub fn apply(
    env: &Env<'_>,
    callback: Function<(i32,), i32>,
    value: i32,
) -> Result<i32> {
    callback.call(env, (value,))
}
```

需要保存回调供后续 ANI 调用时，使用 `FunctionRef<Args, Return>`。工作线程优先使用语义别名 `ThreadsafeFunction` 和 `call_attached`，它会为当前线程自动 attach/detach：

```rust
use ani::prelude::*;

fn run_on_worker(callback: ThreadsafeFunction<(String,), String>) -> Result<String> {
    callback.call_attached(("ready".to_string(),))
}
```

需要把 Rust 对象交给 ArkTS 保存时，优先使用 `ManagedResource<T>`，不要传裸指针。它使用不复用的整数句柄、运行时类型检查和串行可变访问；ArkTS 的显式 `close` 或 `FinalizationRegistry` 回调应调用 Rust 导出的释放函数。

## 自定义转换

只有在现有类型和 `#[ani(object)]` 无法表达时，才实现自定义 `ToAni`、`FromAni` 与 `AniType`。三个实现必须保持一致：

1. 绑定签名描述的类型能被 `FromAni` 接收。
2. ETS 公共类型和运行时值一致。
3. `ToAni` 返回的值满足同一签名。

优先从 `examples/optional`、`examples/nullish_union`、`examples/array_generic`、`examples/arraybuffer`、`examples/map` 和 `examples/function` 选择接近的现成类型。
