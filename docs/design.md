# ANI-RS 设计文档

## 概述

ANI-RS 是一个为 ArkTS 1.2 设计的 Rust 绑定库，类似于 Node.js 的 napi-rs。ANI (Application Native Interface) 是 ArkTS 的原生接口，类似于 Java 的 JNI。

## 设计理念

1. **像 napi-rs 一样简单** - 使用 `#[ani_bindgen]` 宏即可将 Rust 函数导出为 ArkTS 可调用的原生函数
2. **类型安全** - 自动进行 Rust 类型和 ANI 类型之间的转换
3. **零成本抽象** - 尽可能减少运行时开销
4. **符合人体工程学** - 使用 Rust 的习惯用法，而不是 C 风格的 API

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│                        用户代码                              │
│  #[ani_bindgen]                                             │
│  fn add(a: i32, b: i32) -> i32 { a + b }                   │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────┐
│                     ani-derive                               │
│  - 解析 Rust 函数签名                                        │
│  - 生成 ANI 签名 (mangling)                                  │
│  - 生成 wrapper 函数                                         │
│  - 生成 ANI_Constructor                                      │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────┐
│                        ani-rs                                │
│  - ANIEnv: 环境封装                                          │
│  - 类型转换 traits: ToAni, FromAni                          │
│  - 对象封装: AString, AClass, AObject 等                     │
│  - 错误处理                                                  │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────┐
│                        ani-sys                               │
│  - 原始 FFI 绑定                                             │
│  - ani_env, ani_vm, ani_* 类型                              │
└─────────────────────────────────────────────────────────────┘
```

## 类型映射

### 基本类型

| Rust 类型 | ArkTS 类型 | ANI 类型 | Mangling |
|-----------|-----------|----------|----------|
| `bool` | `boolean` | `ani_boolean` | `Z` |
| `i8` | `byte` | `ani_byte` | `B` |
| `u16` | `char` | `ani_char` | `C` |
| `i16` | `short` | `ani_short` | `S` |
| `i32` | `int` | `ani_int` | `I` |
| `i64` | `long` | `ani_long` | `J` |
| `f32` | `float` | `ani_float` | `F` |
| `f64` | `double/number` | `ani_double` | `D` |
| `()` | `void` | `void` | `V` |

### 引用类型

| Rust 类型 | ArkTS 类型 | ANI 类型 | Mangling |
|-----------|-----------|----------|----------|
| `String` / `&str` | `string` | `ani_string` | `Lstd/core/String;` |
| `Vec<T>` | `T[]` | `ani_array_*` | `[T` |
| `Option<T>` | `T?` | `ani_object` | 根据T |
| `HashMap<K, V>` | `Record<K, V>` | `ani_object` | `Lescompat/Record;` |
| `BigInt` | `bigint` | `ani_object` | `Lescompat/BigInt;` |

### 类和对象

| Rust 类型 | ArkTS 类型 | ANI 类型 | Mangling |
|-----------|-----------|----------|----------|
| `AniClass` | class | `ani_class` | `L<module>/<Class>;` |
| `AniObject` | object | `ani_object` | `L<module>/<Class>;` |
| `AniError` | Error | `ani_error` | `Lstd/core/Error;` |

## 宏 API 设计

### 1. 基本函数绑定

```rust
use ani::prelude::*;

/// 绑定到模块级别的函数
#[ani]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 绑定到特定命名空间
#[ani(namespace = "Math")]
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
```

对应 ArkTS 代码：
```typescript
// 模块级别函数
loadLibrary("mylib")
native function add(a: int, b: int): int;

// 命名空间函数
namespace Math {
    loadLibrary("mylib")
    native function multiply(a: double, b: double): double;
}
```

### 2. 类方法绑定

```rust
use ani::prelude::*;

/// 绑定为类的实例方法
#[ani(class = "Calculator")]
fn calculate(&self, op: String, a: i32, b: i32) -> i32 {
    match op.as_str() {
        "add" => a + b,
        "sub" => a - b,
        _ => 0,
    }
}

/// 绑定为类的静态方法
#[ani(class = "Calculator", static)]
fn create_default() -> i32 {
    0
}
```

对应 ArkTS 代码：
```typescript
class Calculator {
    static { loadLibrary("mylib") }
    native calculate(op: string, a: int, b: int): int;
    native static createDefault(): int;
}
```

### 3. 使用结构体定义类

```rust
use ani::prelude::*;

#[ani]
struct Person {
    #[ani(getter, setter)]
    name: String,
    
    #[ani(getter)]
    age: i32,
}

#[ani]
impl Person {
    #[ani(constructor)]
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
    
    #[ani]
    fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
    }
    
    #[ani(static)]
    fn create_anonymous() -> Person {
        Person { name: "Anonymous".to_string(), age: 0 }
    }
}
```

### 4. 异步函数

```rust
use ani::prelude::*;

#[ani]
async fn fetch_data(url: String) -> Result<String> {
    // 异步获取数据
    let response = reqwest::get(&url).await?;
    Ok(response.text().await?)
}
```

### 5. 模块初始化

```rust
use ani_rs::prelude::*;

#[ani_init]
fn init() -> Result<()> {
    // 自定义初始化逻辑
    println!("Module initialized!");
    Ok(())
}

// 或者使用 ani_module! 宏自动生成
ani_module! {
    name: "my_module",
    init: init,
    functions: [add, subtract, multiply],
    classes: [Calculator, Person],
    namespaces: [Math],
}
```

## 核心 Traits

### ToAni - Rust 到 ANI 的转换

```rust
pub trait ToAni<'env> {
    type Output;
    
    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output>;
}

// 实现示例
impl<'env> ToAni<'env> for i32 {
    type Output = ani_int;
    
    fn to_ani(self, _env: &Env<'env>) -> Result<Self::Output> {
        Ok(self)
    }
}

impl<'env> ToAni<'env> for String {
    type Output = AniString<'env>;
    
    fn to_ani(self, env: &Env<'env>) -> Result<Self::Output> {
        env.create_string(&self)
    }
}
```

### FromAni - ANI 到 Rust 的转换

```rust
pub trait FromAni<'env>: Sized {
    type Input;
    
    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self>;
}

// 实现示例
impl<'env> FromAni<'env> for i32 {
    type Input = ani_int;
    
    fn from_ani(_env: &Env<'env>, value: Self::Input) -> Result<Self> {
        Ok(value)
    }
}

impl<'env> FromAni<'env> for String {
    type Input = AniString<'env>;
    
    fn from_ani(env: &Env<'env>, value: Self::Input) -> Result<Self> {
        env.get_string(&value)
    }
}
```

## 错误处理

```rust
use ani::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AniError {
    #[error("ANI error: {0}")]
    Ani(AniStatus),
    
    #[error("Null pointer: {0}")]
    NullPtr(&'static str),
    
    #[error("Type conversion error")]
    TypeConversion,
    
    #[error("Exception thrown")]
    Exception,
    
    #[error("Custom error: {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, AniError>;
```

当函数返回 `Result<T, E>` 时，如果是 `Err`，会自动抛出 ANI 异常。

## 绑定目标

ANI 支持三种绑定目标：

1. **Module** - 模块级别的函数
2. **Namespace** - 命名空间中的函数
3. **Class** - 类的方法（实例方法或静态方法）

### 绑定注册流程

```
ANI_Constructor (入口点)
    ↓
获取 ani_env
    ↓
┌───────────────────────────────────────────────┐
│ FindModule/FindNamespace/FindClass            │
│     ↓                                         │
│ Module_BindNativeFunctions /                  │
│ Namespace_BindNativeFunctions /               │
│ Class_BindNativeMethods                       │
└───────────────────────────────────────────────┘
    ↓
返回 ANI_VERSION_1
```

## Mangling 规则

ANI 使用类似 JNI 的签名格式：

```
参数类型[参数类型...]:返回类型
```

示例：
- `fn add(a: i32, b: i32) -> i32` → `II:I`
- `fn greet(name: String) -> String` → `Lstd/core/String;:Lstd/core/String;`
- `fn process(data: Vec<i32>)` → `[I:V`
- `fn create() -> Person` → `:Lmodule_name/Person;`

## 生命周期管理

ANI 中的引用有三种类型：

1. **局部引用 (Local Reference)** - 函数返回后自动释放
2. **全局引用 (Global Reference)** - 需要手动释放
3. **弱引用 (Weak Reference)** - 可能被 GC 回收

```rust
// 局部引用（默认）
let local_string = env.create_string("hello")?;

// 创建全局引用
let global_string = env.create_global_ref(&local_string)?;

// 弱引用
let weak_ref = env.create_weak_ref(&local_string)?;
```

## 装箱/拆箱

当基本类型用于可选参数或泛型时，需要进行装箱：

```rust
// 可选参数会自动装箱
#[ani]
fn process(value: Option<i32>) -> i32 {
    value.unwrap_or(0)
}
```

对应的 ANI 签名：`Lstd/core/Int;:I`（int 被装箱为 Int 类）

## 最佳实践

1. **始终使用 `#[ani]`** 而不是手动编写 FFI 代码
2. **使用 `Result<T>` 进行错误处理** - 错误会自动转换为 ANI 异常
3. **避免直接操作原始指针** - 使用封装好的类型
4. **注意生命周期** - 不要在函数返回后持有局部引用
5. **使用 `ani_module!`** 宏简化模块注册

## 与 napi-rs 的对比

| 特性 | napi-rs | ani-rs |
|------|---------|--------|
| 目标运行时 | Node.js V8 | ArkTS VM |
| 签名格式 | 无 | JNI 风格 mangling |
| 类绑定 | `#[napi]` | `#[ani_bindgen(class = "...")]` |
| 异步 | Promise | Future/Promise |
| 错误处理 | Result → throw | Result → throw |
| 初始化 | `#[module_exports]` | `#[ani_init]` + `ANI_Constructor` |

## 示例项目结构

```
my-ani-lib/
├── Cargo.toml
├── src/
│   └── lib.rs
├── ets/
│   └── index.ets    # ArkTS 声明文件
└── build.rs         # 可选：生成 .ets 文件
```

### Cargo.toml

```toml
[package]
name = "my-ani-lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
ani = { version = "0.1", features = ["derive"], package = "ani-core" }

[build-dependencies]
ani-build = "0.1"
```

### src/lib.rs

```rust
use ani::prelude::*;

#[ani]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

ani_module! {
    name: "hello_module",
    functions: [hello],
}
```

### ets/index.ets

```typescript
loadLibrary("my_ani_lib")

native function hello(name: string): string;

function main() {
    console.log(hello("World")); // 输出: Hello, World!
}
```
