# 支持能力总览

> [!IMPORTANT]
> 这页只回答一个问题: “现在 `ani-rs` 到底支持什么，哪些已经过了 example / Docker / ArkVM 回归，哪些还是明确保留的边界？”

## 当前验证基线

当前仓库已经有一条可重复的验证链路：

- `cargo test --workspace -j 1`
- `bash ./scripts/check_example_ets.sh`
- `./scripts/run_arkvm_examples_ubuntu.sh`

已记录的基线结果：

- 52 个 example 全部纳入 smoke
- `.ets` 输出检查通过
- Ubuntu Docker + Linux x64 ArkVM 回归通过

## 1. 导出与注册

| 能力 | 状态 | 示例 | 说明 |
| --- | --- | --- | --- |
| 模块级函数导出 | 已支持 | `new_basic` | 默认 `#[ani]` 路径 |
| `#[ani(module = "...")]` | 已支持 | `module_binding` | 显式模块 descriptor |
| namespace 绑定 | 已支持 | `bind_overload`、`ets_declaration` | 支持嵌套 namespace |
| class 绑定 | 已支持 | `new_class`、`impl_block` | 实例/静态/构造器 |
| getter / setter | 已支持 | `new_class`、`impl_block` | 包含 static property |
| overload | 已支持 | `bind_overload`、`class_method_overload` | 按签名和 target 收敛 |
| `#[ani(init)]` | 已支持 | `init_lifecycle` | 支持 `before_bindings` |
| 重复绑定诊断 | 已支持 | `crates/ani/src/module_register.rs` | 同 target 重复 `name + signature` 会拒绝 |
| 稳定排序注册 | 已支持 | `crates/ani/src/module_register.rs` | 按 `name + signature + pointer` 稳定排序 |

## 2. class / impl / object 能力

| 能力 | 状态 | 示例 | 说明 |
| --- | --- | --- | --- |
| `impl` receiver 方法绑定 | 已支持 | `impl_block` | `self` / `&self` / `&mut self` |
| constructor overload | 已支持 | `constructor_overload` | 多构造器组合 |
| constructor + nullish | 已支持 | `constructor_nullish` | 结合 `Option` / union |
| class static by name | 已支持 | `class_static_by_name` | 运行时按类名查找 |
| object nominal 类型 | 已支持 | `object_model`、`object_typed` | 公共类型与 runtime 值对齐 |
| `#[ani(object)]` | 已支持 | `derive_shapes` | named / tuple / unit / generic |
| `#[derive(AniClass)]` | 已支持 | `derive_shapes` | 生成 class public surface |
| `#[derive(AniEnum)]` unit variant | 已支持 | `enum_derive` | 当前仅 unit variants |
| class reflect | 已支持 | `class_reflect` | 类反射型能力 |

## 3. 异步 / Promise / Tokio

| 能力 | 状态 | 示例 | 说明 |
| --- | --- | --- | --- |
| `#[ani(async)] -> Promise<T>` | 已支持 | `async_wrapper` | 最常用异步入口 |
| async class 方法 | 已支持 | `async_wrapper` | 包含 `self` receiver |
| async constructor / getter / setter | 已支持 | `async_wrapper` | 同步 ArkTS 形态下阻塞等待 |
| `signature = "..."` + async | 已支持 | `async_wrapper` | 已过 smoke |
| `env` / `this` / `class` 注入 | 已支持 | `async_wrapper` | 在 runtime worker 上重建 |
| `Env::promise_new_typed<T>()` | 已支持 | `async_wrapper` | typed promise helper |
| `Env::promise_resolved / rejected` | 已支持 | `async_wrapper` | env-rooted helper |
| `AniResolver` / `Deferred<T>` bridge | 已支持 | `async_wrapper` | resolver 和 deferred 可互转 |
| tokio feature 细拆分 | 已支持 | `crates/ani/Cargo.toml` | `async`、`tokio_rt`、`tokio_time` 等 |
| panic -> rejection | 已支持 | `crates/ani/src/tokio.rs` | Promise 保证最终 settle |

## 4. 类型系统与转换

| 能力 | 状态 | 示例 | 说明 |
| --- | --- | --- | --- |
| primitive 类型映射 | 已支持 | `new_basic` | `bool/i32/i64/f64/...` |
| 字符串与 string-like owned wrapper | 已支持 | `string_like_owned` | `String` 及相关 owned wrapper |
| `Option<T>` -> `T | null` | 已支持 | `optional` | `null` 与 `undefined` 分离 |
| `Either` / `Either3` union | 已支持 | `union`、`nullish_union` | 精确 ETS public type |
| record | 已支持 | `record` | record public surface |
| map / set | 已支持 | `map`、`set` | 容器转换 |
| `Vec<T>` / `VecDeque<T>` / `LinkedList<T>` | 已支持 | `array_generic` | public type 与 bind signature 已收敛 |
| fixed array / tuple / enum item | 已支持 | `fixed_array_wrapper`、`tuple_value_wrapper`、`enum_item_wrapper` | 多种值包装 |
| ArrayBuffer | 已支持 | `arraybuffer` | buffer 读写 |
| bigint | 已支持 | `bigint` | 大整数映射 |
| interface / type relation | 已支持 | `interface`、`type_relation` | 类型关系和接口面 |
| `Unknown -> Object` 收缩 | 已支持 | `derive_shapes`、`record`、`array_generic` 等 | 只剩 genuinely unknown 兜底路径 |

## 5. 运行时句柄与生命周期

| 能力 | 状态 | 示例 | 说明 |
| --- | --- | --- | --- |
| `Env` / local scope | 已支持 | `reference_scope` | local ref 生命周期 |
| `AniObject` / `AniClass` / `AniRef` | 已支持 | `reference`、`object_runtime` | 基础 runtime handle |
| `AniModule` / `AniNamespace` / `AniVariable` | 已支持 | `module_member` | 运行时查找与访问 |
| `AniResolver` | 已支持 | `async_wrapper` | Promise resolve/reject handle |
| `GlobalRef` | 已支持 | `reference`、`weak_ref` | create/use/delete/async bridge |
| `WeakRef` | 已支持 | `weak_ref` | upgrade、GC invalidation |
| `Ref<T>` / `FunctionRef<...>` | 已支持 | `async_wrapper` | managed owning-handle 语义 |
| `VM` / `VmOptions` | 已支持 | `vm` | 版本与基础 VM 能力 |
| error handle | 已支持 | `error` | throw / reject / error object |

## 6. ETS 生成与发布面

| 能力 | 状态 | 示例 | 说明 |
| --- | --- | --- | --- |
| 自动生成 `.ets` | 已支持 | 全量 examples | `target/ani-ets/*.ets` |
| `loadLibrary(...)` 注入 | 已支持 | 全量 examples | ETS smoke 已检查 |
| public signature 精确化 | 已支持 | `type-system` 相关 examples | 减少 `Unknown -> Object` |
| `.d.ets` | 当前不提供 | `scripts/check_example_ets.sh` | 属于当前 scope 决策 |
| `declare` 风格输出 | 当前不提供 | `scripts/check_example_ets.sh` | 属于当前 scope 决策 |

## 7. 已明确的边界

这些不是“忘了做”，而是当前版本明确保留的边界：

| 条目 | 当前结论 |
| --- | --- |
| `ThreadsafeFunction` 类跨线程回调公开能力 | 当前没有稳定公开 API |
| `#[derive(AniEnum)]` 非 unit variant | 当前不支持 |
| generic object/class 的 primitive instantiation | 受 ArkVM runtime slot model 限制 |
| 任意自定义 wrapper 的 async 自动托管 | 未覆盖部分仍建议显式 `GlobalRef` / `RefContainer` |

## 8. 推荐阅读路径

如果你是按任务找能力，建议直接跳：

- 想写导出: [绑定模型](/guide/binding-model)
- 想写 Promise: [异步与 Tokio](/guide/async)
- 想查运行时句柄: [运行时句柄](/reference/runtime-handles)
- 想看类型收敛和 nullish 语义: [类型系统与 ETS 面](/reference/type-system)
- 想看底层差异和 scope 决策: [能力缺口清单](/capability-gap)
