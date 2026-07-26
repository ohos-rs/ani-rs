---
title: "#[ani] 属性参考"
description: 查询函数、impl、struct、初始化和字段可用的 ani-rs 属性。
---

## 导出目标

| 属性 | 适用位置 | 说明 |
| --- | --- | --- |
| `#[ani]` | 函数、impl、struct | 使用默认 module 或所属 class 导出 |
| `module = "name"` | 函数、impl | 指定 module descriptor |
| `namespace = "A.B"` | 函数、impl | 指定 namespace descriptor |
| `class = "A.B.C"` | 函数、impl、struct | 指定 class descriptor |

`namespace` 也可以写成 `ns`。通常应使用完整名称，避免不同模块中的同名 class 或 namespace 冲突。

## 名称与签名

| 属性 | 说明 |
| --- | --- |
| `name = "publicName"` | 覆盖 ArkTS 导出名 |
| `signature = "..."` | 覆盖自动生成的 ANI 签名 |
| `skip` | 保留 Rust item，但跳过该项生成 |

`signature` 也可以写成 `sig`。优先依赖 Rust 类型推导，只有对接已有 descriptor 时再手写签名。

```rust
#[ani(namespace = "Math", name = "sum")]
pub fn sum_two(a: i32, b: i32) -> i32 {
    a + b
}
```

## Class 成员

| 属性 | 说明 |
| --- | --- |
| `static` | 静态方法或静态属性 |
| `constructor` | 构造器 |
| `getter` | getter，属性名从方法名推导 |
| `getter = "name"` | 指定 getter 属性名 |
| `setter` | setter，属性名从方法名推导 |
| `setter = "name"` | 指定 setter 属性名 |

`static` 也可以写成 `is_static`，`constructor` 也可以写成 `ctor`。

```rust
#[ani(class = "Counter")]
impl Counter {
    #[ani(constructor)]
    pub fn new(
        env: &Env<'_>,
        this: &AniObject<'_>,
    ) -> Result<()> {
        Counter { value: 0 }.write_back_to_ani_object(env, this)
    }

    #[ani(getter)]
    pub fn get_value(&self) -> i32 {
        self.value
    }

    #[ani(static, name = "createDefault")]
    pub fn create_default() -> i32 {
        0
    }
}
```

## 异步

把 `async fn` 标记为 Promise API：

```rust
#[ani(async)]
pub async fn load_value(key: String) -> Result<String> {
    Ok(key)
}
```

`async` 可以与 `class`、`static`、`constructor`、`getter`、`setter`、`name` 和 `signature` 组合。运行时配置见 [异步与 Promise](/guide/async/)。

## 初始化

| 写法 | 执行时机 |
| --- | --- |
| `#[ani(init, before_bindings)]` | native bindings 注册前 |
| `#[ani(init)]` | native bindings 注册后 |

初始化函数可以不返回值，也可以返回 `Result<()>`。`Env<'_>` 参数会自动注入。

## Object

```rust
#[ani(object)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[ani(object = "PublicPoint")]
pub struct NamedPoint {
    pub x: f64,
    pub y: f64,
}
```

字段必须公开才能参与结构化转换。

字段级属性：

| 属性 | 说明 |
| --- | --- |
| `#[ani(property)]` | 使用 property 方式读写字段 |
| `#[ani(property = "name")]` | 同时设置公开 property 名 |
| `#[ani(name = "name")]` | 覆盖字段公开名称 |

## 派生宏

### `AniClass`

```rust
#[derive(AniClass)]
#[ani(class = "Profile")]
pub struct Profile {
    #[ani(property)]
    pub name: String,
}
```

生成结构体的 ANI 转换、类型信息和 ETS class 声明。

### `AniEnum`

```rust
#[derive(AniEnum)]
#[ani(name = "State")]
pub enum State {
    Idle,
    Running,
}
```

只适用于 unit variants。

## 自动注入参数

以下参数不会进入 ArkTS 签名：

| 参数 | 注入值 |
| --- | --- |
| `&Env<'_>` | 当前 ANI 环境 |
| `&AniObject<'_>`，参数名为 `this` | 当前实例 |
| `&AniClass<'_>`，参数名为 `class` | 当前 class |

宏会根据类型和参数位置识别注入项。公开业务参数应避免复用这些特殊形态。
