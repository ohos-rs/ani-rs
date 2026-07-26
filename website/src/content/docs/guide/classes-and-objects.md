---
title: Class、对象与枚举
description: 在 Rust 中定义 ArkTS class、结构化对象、属性与枚举。
---

`ani-rs` 提供两种常用的结构化类型：

- Class：具有实例身份、构造器和方法。
- Object：按字段转换的值对象，适合作为参数或返回值。

## 定义 Class

使用 `AniClass` 派生类型信息，再在 `impl` 上指定 class descriptor：

```rust
use ani::prelude::*;
use ani_derive::{ani, AniClass};

#[derive(AniClass)]
#[ani(class = "Counter")]
pub struct Counter {
    pub value: i32,
}

#[ani(class = "Counter")]
impl Counter {
    #[ani(constructor)]
    pub fn new(
        env: &Env<'_>,
        this: &AniObject<'_>,
        value: i32,
    ) -> Result<()> {
        Counter { value }.write_back_to_ani_object(env, this)
    }

    #[ani(getter)]
    pub fn get_value(&self) -> i32 {
        self.value
    }

    #[ani(setter)]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    #[ani]
    pub fn increment(&mut self, step: i32) -> i32 {
        self.value += step;
        self.value
    }

    #[ani(static)]
    pub fn version() -> i32 {
        1
    }
}
```

`&self`、`&mut self` 和拥有所有权的 `self` 都会按方法语义生成 wrapper。修改字段后，`&mut self` 方法会把状态写回 ArkTS 对象。

## Getter 与 Setter

不带名称的 `getter` / `setter` 会从 Rust 方法名推导属性名。也可以显式指定：

```rust
#[ani(class = "Profile", getter = "displayName")]
pub fn profile_name() -> String {
    "ArkTS".to_string()
}

#[ani(class = "Profile", setter = "displayName")]
pub fn set_profile_name(value: String) {
    let _ = value;
}
```

## 定义值对象

值对象适合配置、请求参数和返回结果：

```rust
use ani_derive::ani;

#[ani(object = "UserProfile")]
pub struct UserProfile {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[ani]
pub fn rename(mut profile: UserProfile, name: String) -> UserProfile {
    profile.name = name;
    profile
}
```

对象在边界处按字段转换。Rust 中对传入值的修改不会自动改变 ArkTS 原对象；需要把修改后的对象作为返回值交回去。

## 派生结构化类型

`#[derive(AniClass)]` 也可以让普通结构体直接参与参数和返回值转换：

```rust
#[derive(AniClass)]
#[ani(class = "Point")]
pub struct Point {
    #[ani(property)]
    pub x: f64,
    #[ani(property)]
    pub y: f64,
}

#[ani]
pub fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
```

`#[ani(property)]` 用于把字段声明为公开 property。命名、tuple 和 unit struct 都可以生成对应的 ETS 类型。

## 枚举

`AniEnum` 用于 unit variants：

```rust
use ani_derive::{ani, AniEnum};

#[derive(Clone, Copy, AniEnum)]
#[ani(name = "Status")]
pub enum Status {
    Idle = 0,
    Running = 2,
    Stopped,
}

#[ani]
pub fn next_status(status: Status) -> Status {
    match status {
        Status::Idle => Status::Running,
        Status::Running => Status::Stopped,
        Status::Stopped => Status::Idle,
    }
}
```

当前 `AniEnum` 只用于不携带数据的 variant。需要携带字段时，请建模为 object 或 class。

## 选择哪一种

| 需要 | 使用 |
| --- | --- |
| 实例方法、状态和构造器 | Class + `AniClass` |
| 结构化参数或返回值 | `#[ani(object)]` |
| 类型需要在多个 API 间保持名义类型 | `#[derive(AniClass)]` |
| 固定的离散值 | `#[derive(AniEnum)]` |

可运行示例位于 `examples/impl_block`、`examples/object_model`、`examples/derive_shapes` 和 `examples/enum_derive`。
