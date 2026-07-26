---
title: 引用与生命周期
description: 正确使用 Env、local handle、Ref、GlobalRef、WeakRef 和 FunctionRef。
---

ANI 对象由运行时管理。Rust 代码必须区分当前调用有效的 local handle 与可以长期保存的 global reference。

## Local handle

以下类型通常绑定到当前 `Env` 和 native 调用作用域：

- `AniRef<'env>`
- `AniObject<'env>`
- `AniClass<'env>`
- `AniString<'env>`
- `AniArray<'env>`
- `Function<'env, Args, Return>`

它们适合立即读取、转换或调用，不应保存到全局变量，也不应直接移动到其他线程。

```rust
#[ani]
pub fn object_is_valid(
    env: &Env<'_>,
    object: AniObject<'_>,
) -> Result<bool> {
    let reference = AniRef::from(object);
    Ok(!env.is_null(&reference)?)
}
```

## `Ref<T>`

`Ref<T>` 是带类型的 owning global reference，适合跨 native 调用保存对象：

```rust
use std::sync::Mutex;
use ani::prelude::*;
use ani_derive::ani;

static STORED: Mutex<Option<Ref<AniObject<'static>>>> =
    Mutex::new(None);

#[ani]
pub fn store_object(
    object: Ref<AniObject<'static>>,
) -> Result<()> {
    *STORED.lock().unwrap() = Some(object);
    Ok(())
}

#[ani]
pub fn has_object() -> bool {
    STORED.lock().unwrap().is_some()
}

#[ani]
pub fn use_object(env: &Env<'_>) -> Result<bool> {
    let guard = STORED.lock().unwrap();
    let Some(reference) = guard.as_ref() else {
        return Ok(false);
    };

    let object = reference.borrow(env);
    Ok(!object.is_null())
}
```

从 ArkTS 参数转换得到的 `Ref<T>` 会记录 owning VM，并在释放时清理 global reference。需要确定释放时机时，可以主动从状态中移除或调用 `delete`。

## `GlobalRef`

`GlobalRef` 是低层、无类型的 global reference：

```rust
let global = env.create_global_ref(&local)?;
let local_again = global.to_local(env)?;
global.delete(env)?;
```

只有在类型化 `Ref<T>` 不适用，或需要和底层 ANI API 交互时才直接使用它。

## `WeakRef`

`WeakRef` 不阻止运行时回收对象：

```rust
let weak = env.create_weak_ref(&local)?;

if let Some(value) = weak.upgrade(env)? {
    // value 只在当前 local scope 使用
    env.delete_local_ref(&value)?;
}

weak.delete(env)?;
```

适合缓存、观察者或不应延长对象生命周期的关联。`upgrade` 返回 `None` 是正常状态。

## 保存回调

`Function<Args, Return>` 是 local callback；`FunctionRef<Args, Return>` 可以长期保存：

```rust
static CALLBACK: Mutex<Option<FunctionRef<(i32,), i32>>> =
    Mutex::new(None);

#[ani]
pub fn register_callback(
    callback: FunctionRef<(i32,), i32>,
) {
    *CALLBACK.lock().unwrap() = Some(callback);
}

#[ani]
pub fn call_registered(
    env: &Env<'_>,
    value: i32,
) -> Result<i32> {
    CALLBACK
        .lock()
        .unwrap()
        .as_ref()
        .ok_or_else(|| Error::new(
            Status::NotFound,
            "callback is not registered",
        ))?
        .call(env, (value,))
}
```

`FunctionRef` 可以跨调用保存，但真正执行 callback 仍需要当前有效的 `Env`。

## 选择指南

| 场景 | 类型 |
| --- | --- |
| 当前函数内使用对象 | `AniObject<'_>` / `AniRef<'_>` |
| 跨调用保存对象 | `Ref<T>` |
| 低层无类型长期引用 | `GlobalRef` |
| 不阻止 GC 的缓存 | `WeakRef` |
| 当前函数内调用 callback | `Function<...>` |
| 保存 callback 供后续 ANI 调用 | `FunctionRef<...>` |

详细示例位于 `examples/reference`、`examples/reference_scope`、`examples/weak_ref` 和 `examples/function`。
