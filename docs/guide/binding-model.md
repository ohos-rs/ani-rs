# 绑定模型

`ani-rs` 的核心不是“把 Rust 函数暴露出去”这么简单，而是把导出目标明确映射到 ANI 的三类绑定面：

- Module
- Namespace
- Class

这也是它和 `napi-rs` 在底层注册模型上最根本的差异。

## 一个对照示例

::: code-group

```rust [Module]
#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust [Namespace]
#[ani(namespace = "Math.Utils")]
pub fn clamp_to_zero(input: i32) -> i32 {
    input.max(0)
}
```

```rust [Class]
#[ani(class = "Counter")]
pub fn inc(this: i64, step: i32) -> i32 {
    step
}
```

:::

## Module 绑定

不带额外目标属性时，`#[ani]` 默认生成模块级导出：

```rust
use ani_derive::ani;

#[ani]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

如果运行时模块名需要和 crate 名脱钩，可以显式指定：

```rust
#[ani(module = "explicit_module_binding")]
pub fn answer() -> i32 {
    42
}
```

这类场景可以直接参考 `examples/module_binding`。

## Namespace 绑定

`namespace = "A.B"` 会把函数绑定到嵌套 namespace descriptor：

```rust
#[ani(namespace = "Math.Utils")]
pub fn clamp_to_zero(input: i32) -> i32 {
    input.max(0)
}
```

适用场景：

- 你想把导出结果收敛到一组逻辑域下
- 你不希望把所有 native symbol 都挂在模块根上

对应 example：

- `examples/bind_overload`
- `examples/ets_declaration`
- `examples/module_member`

## Class 绑定

Class 目标支持实例方法、静态方法、构造器、getter、setter 和 operator 风格成员。

```rust
use ani::prelude::*;
use ani_derive::ani;

#[ani(class = "Counter")]
pub struct Counter {
    value: i32,
}

#[ani(class = "Counter")]
impl Counter {
    #[ani(constructor)]
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    #[ani(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }

    #[ani(setter)]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    #[ani]
    pub fn inc(&mut self, step: i32) -> i32 {
        self.value += step;
        self.value
    }

    #[ani(static)]
    pub fn version() -> i32 {
        1
    }
}
```

真实覆盖更完整的例子见：

- `examples/new_class`
- `examples/impl_block`
- `examples/class_method_overload`
- `examples/class_static`
- `examples/class_static_by_name`

## Overload 与名称重写

`ani-rs` 支持通过 `name = "..."` 把多个 Rust 函数收敛到同一个 ArkTS 导出名，并依赖签名完成 overload：

```rust
#[ani(name = "sum")]
pub fn sum_i32(a: i32, b: i32) -> i32 {
    a + b
}

#[ani(name = "sum")]
pub fn sum_f64(a: f64, b: f64) -> f64 {
    a + b
}
```

当前注册阶段已经具备两条约束：

- 同一个绑定 target 下，会按 `name + signature + pointer` 稳定排序
- 同一个绑定 target 下，如果 `name + signature` 重复，会在 bind 前直接报错

这部分实现和测试位于：

- `crates/ani/src/module_register.rs`

## 自动注册流程

运行时注册并不是“导出一个函数指针”这么直接，而是分成两段：

1. 宏展开阶段生成 `ctor` 回调，把绑定信息 enqueue 到全局 pending 列表
2. `ANI_Constructor` 触发后，按 target 分组，再调用对应的 `BindNativeFunctions / BindNativeMethods`

这种两段式模型是为了适配 ANI 对 descriptor 和签名的要求。更完整的底层说明见 [设计说明](/design)。

## 何时该用哪种目标

优先级可以这样判断：

- 纯工具函数，直接用 Module
- 需要逻辑分组但不需要对象语义，用 Namespace
- 需要实例状态、静态成员、访问器或运算符语义，用 Class

如果你只是想找现成可运行的代码，而不是继续读抽象设计，直接去 [示例索引](/guide/examples)。
