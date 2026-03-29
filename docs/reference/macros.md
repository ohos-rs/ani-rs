# 宏与派生

这一页只说“用户侧能写什么”，不重复底层 codegen 细节。

> [!IMPORTANT]
> 宏页回答的是“怎么写”，不是“当前能力覆盖到哪”。如果你在确认支持范围，先看 [支持能力总览](/reference/capabilities)。

## `#[ani]`

`#[ani]` 是统一导出入口。按目标不同，可以落到三类绑定面：

- Module
- Namespace
- Class

常见属性：

| 属性 | 作用 |
| --- | --- |
| `module = "..."` | 显式指定运行时模块 descriptor |
| `namespace = "..."` | 绑定到 namespace |
| `class = "..."` | 绑定到 class |
| `name = "..."` | 指定 ArkTS 侧导出名 |
| `static` | class 静态成员 |
| `constructor` | class 构造器 |
| `getter` | class getter |
| `setter` | class setter |
| `signature = "..."` | 显式覆盖生成签名 |

## `#[ani(init)]`

用于模块初始化回调。

支持形态：

- `#[ani(init)]`
- `#[ani(init, before_bindings)]`

适用场景：

- 在绑定执行前准备运行时资源
- 在绑定执行后做额外初始化

完整用法见 `examples/init_lifecycle`。

## `#[ani(async)]`

用于把 Rust `async fn` 导出为 ArkTS `Promise<T>`。

当前已支持组合：

- 普通 async 函数
- class 绑定函数
- Rust `self` receiver
- `env` / `this` / `class` 注入
- `signature = ...`
- `constructor / getter / setter`

更完整的使用与边界说明见 [异步与 Tokio](/guide/async)。

## `#[ani(object)]`

用于把 Rust struct 暴露为 object-backed public type。

常见搭配：

- 命名 struct
- tuple struct
- unit struct
- type-parameter generic struct
- 字段级 `#[ani(property)]`

对照 example：

- `examples/derive_shapes`
- `examples/object_model`

## `#[derive(AniClass)]`

适用于需要 nominal class surface 的 Rust 类型。

当前文档基线：

- named / tuple / unit struct 已支持
- generic struct 已支持 ETS public declaration
- primitive generic instantiation 仍受 ArkVM runtime slot model 约束

## `#[derive(AniEnum)]`

当前仅支持 unit variants。

这是刻意保留的语义边界，而不是暂时漏做：

- ArkTS enum 语义天然更接近 unit variant
- 非 unit variant 若继续支持，通常会演变成另一套 tagged-union / object model

## 从哪里看真实写法

如果你准备直接照着改代码，而不是继续读说明，优先看这些 example：

- `examples/new_basic`
- `examples/new_class`
- `examples/impl_block`
- `examples/derive_shapes`
- `examples/async_wrapper`
