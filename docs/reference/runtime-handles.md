# 运行时句柄

这页聚焦 `ani-rs` 里的 runtime handle，而不是宏语法。

如果你在问下面这些问题，这页就是入口：

- 哪些类型是 local ref，哪些是可跨线程托管的 owning handle？
- `GlobalRef` 和 `WeakRef` 分别适合什么场景？
- async 场景下哪些句柄能自动托管，哪些还需要手动处理？

## 总体模型

`ani-rs` 里的 runtime handle 可以先粗分成三类：

1. 线程绑定的 local handle
2. runtime 查找句柄
3. 显式生命周期句柄

## 1. 线程绑定的 local handle

| 类型 | 语义 | 典型场景 |
| --- | --- | --- |
| `Env<'_>` | 当前线程上的 ANI 交互入口 | 创建值、抛错、查找 module/class |
| `AniRef<'_>` | 通用 local ref | 底层引用兜底面 |
| `AniObject<'_>` | ArkTS object | object 参数和返回值 |
| `AniClass<'_>` | ArkTS class | class 查找、构造、静态成员 |
| `AniString<'_>` | ArkTS string | 字符串转换 |
| `AniModule<'_>` / `AniNamespace<'_>` | 运行时 module / namespace | 运行时查找 |
| `AniVariable<'_>` | 变量句柄 | get/set 变量 |
| `AniResolver` | Promise resolver | resolve / reject |

这些句柄的共同点：

- 绑定到当前线程 / 当前 env
- 不应该直接跨线程长期持有
- 在 async 场景里需要依赖注入重建或 `RefContainer` 托管

## 2. 显式生命周期句柄

### `GlobalRef`

适合：

- 需要跨作用域或跨线程持有对象
- 需要延长 object 生命周期
- async 桥接中的句柄保活

当前已经覆盖的能力：

- create
- use
- delete
- 恢复为 local object / class
- async bridge source

对应示例：

- `examples/reference`
- `examples/weak_ref`
- `examples/async_wrapper`

### `WeakRef`

适合：

- 不想阻止 GC 回收对象
- 只在真正需要时尝试 upgrade

当前已经覆盖的能力：

- create
- `is_alive`
- `upgrade`
- delete
- GC invalidation 回归

对应示例：

- `examples/weak_ref`

## 3. 托管 owning handle

### `Ref<T>`

`Ref<T>` 现在更接近 napi-rs 风格的 managed owning handle：

- `FromAni` 时记录 VM
- `Drop` 时自动释放 global ref
- 返回 ArkTS 时重新 materialize local handle

这适合在 Rust 侧保存一个“拥有生命周期责任”的句柄，而不是单纯拿一个 local ref 临时用一下。

### `FunctionRef<...>`

用于托管函数对象引用，特别是在 async bridge 中保留 callback。

当前已覆盖：

- sync callback return
- async callback return
- manual tokio helper

## async 场景下怎么选

::: tip 经验法则
如果你只是把对象传进 `#[ani(async)]`，优先依赖宏自动托管；如果你要自己把句柄塞进线程、任务队列或长期状态里，优先显式转成 `GlobalRef`、`Ref<T>` 或 `FunctionRef<...>`。
:::

### 推荐路径

| 场景 | 推荐做法 |
| --- | --- |
| 普通 `#[ani(async)]` 参数 | 依赖宏和 `RefContainer` |
| 需要长期持有 object | `GlobalRef` 或 `Ref<T>` |
| 需要长期持有 callback | `FunctionRef<...>` |
| 只想弱持有对象 | `WeakRef` |

### 仍需注意

- `Env` 本身不是 `Send` / `Sync`
- 自定义 wrapper 如果没有进入 `RefContainer` 覆盖面，仍要手动托管
- Promise 路径里未自动托管的捕获值，必要时仍需满足 `Send + 'static`

## public ETS type model

当前文档和生成结果对 runtime handle 的 public ETS surface 已收敛到下面的模型：

| Rust handle | ETS public type |
| --- | --- |
| `AniModule` | `Object` |
| `AniNamespace` | `Object` |
| `AniVariable` | `Object` |
| `AniResolver` | `Object` |
| `GlobalRef` | `Object` |
| `WeakRef` | `WeakRef<Object>` |

这表示：

- 这些类型在运行时本质上是 object-backed handle
- 文档保留它们的命名语义，避免和普通业务 object 混淆

## 从哪里看代码

优先看这些示例：

- `examples/reference`
- `examples/reference_scope`
- `examples/weak_ref`
- `examples/module_member`
- `examples/async_wrapper`
- `examples/vm`

实现入口主要在：

- `crates/ani/src/types.rs`
- `crates/ani/src/env.rs`
- `crates/ani/src/conversions/reference.rs`
- `crates/ani/src/conversions/promise.rs`
