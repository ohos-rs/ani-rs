---
title: 示例
description: 按使用场景查找可以直接运行和修改的 ani-rs 示例。
---

每个 `examples/*` 目录都是独立 `cdylib` crate，并带有 Rust 测试和 ArkTS smoke 文件。先选择最接近自己 API 的示例，再替换函数和类型。

## 建议阅读顺序

| 目标 | 从这里开始 |
| --- | --- |
| 第一个 native 函数 | `examples/new_basic` |
| Module / Namespace | `examples/module_binding`、`examples/bind_overload` |
| Class 与实例状态 | `examples/impl_block`、`examples/new_class` |
| Object 参数与返回值 | `examples/object_model`、`examples/derive_shapes` |
| 异步 Promise | `examples/async_wrapper` |
| 错误转换 | `examples/error` |
| 引用与 callback | `examples/reference`、`examples/function` |

## 导出与初始化

| Example | 展示内容 |
| --- | --- |
| `new_basic` | 基础参数、返回值和自动注册 |
| `module_binding` | 显式 module descriptor |
| `bind_overload` | 重命名、overload、嵌套 namespace |
| `init_lifecycle` | 绑定前后初始化回调 |
| `ets_declaration` | Module、Namespace、Class 的 ETS 输出 |

## Class 与对象

| Example | 展示内容 |
| --- | --- |
| `impl_block` | 构造器、receiver、getter、setter、静态方法 |
| `new_class` | 独立函数形式的 class 成员 |
| `class_method_overload` | class 方法 overload |
| `class_static` | 静态方法和静态属性 |
| `constructor_overload` | 多构造器签名 |
| `object_model` | 强类型对象和集合嵌套 |
| `object_access` | 动态读取和写入对象字段 |
| `derive_shapes` | named、tuple、unit 与 generic struct |
| `enum_derive` | unit enum 转换 |

## 值与集合

| Example | 展示内容 |
| --- | --- |
| `optional` | `Option<T>` 参数和返回值 |
| `nullish_union` | `Null` 与 `Undefined` |
| `union` | `Either` union |
| `array_generic` | `Vec<T>` 等数组转换 |
| `fixed_array_wrapper` | fixed array |
| `arraybuffer` | 借用和拥有的 ArrayBuffer |
| `record` | `HashMap<String, V>` / Record |
| `map` | `BTreeMap<String, V>` / Map |
| `set` | `HashSet<T>` / Set |
| `bigint` | 大整数转换 |

## 函数与运行时调用

| Example | 展示内容 |
| --- | --- |
| `function` | `Function` 与 `FunctionRef` |
| `call_method` | 从 Rust 调用 ArkTS 方法 |
| `call_variadic_v` | 可变参数调用 |
| `module_member` | 查找和调用 module / namespace 成员 |
| `any_dynamic` | 动态属性、索引和调用 |

## 引用、异步与错误

| Example | 展示内容 |
| --- | --- |
| `reference` | `Ref<T>` 与 `GlobalRef` |
| `reference_scope` | local reference scope |
| `weak_ref` | 弱引用和 upgrade |
| `async_wrapper` | async fn、Promise、Deferred 与引用托管 |
| `error` | `Result`、`Status` 和异常 |
| `wrap_native_ptr` | 原生指针包装与显式释放 |

## 运行一个示例

```bash
cargo test -p ani-example-new-basic
cargo build -p ani-example-new-basic
```

生成声明位于：

```text
examples/new_basic/target/ani-ets/ani_example_new_basic.ets
```

为 OpenHarmony ARM64 构建或在 QEMU 中执行时，继续参考 [构建与加载](/guide/build-and-load/) 和 [测试与调试](/guide/testing/)。
