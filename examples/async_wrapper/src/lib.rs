//! Async Wrapper Example - Wrapping synchronous interfaces for Promise operations.

use ani::conversions::{PromiseRaw, RefContainer};
use ani::prelude::*;
use ani_derive::{ani, AniClass};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Mutex, OnceLock,
};
use std::thread;
use std::time::Duration;

fn expensive_computation(input: i32) -> i32 {
    thread::sleep(Duration::from_millis(100));
    input * input
}

fn fetch_data(url: &str) -> String {
    thread::sleep(Duration::from_millis(50));
    format!("Response from: {}", url)
}

static ASYNC_CTOR_TOTAL: AtomicI32 = AtomicI32::new(0);
static ASYNC_CTOR_NOTE: OnceLock<Mutex<String>> = OnceLock::new();
static ASYNC_ACCESSOR_NOTE: OnceLock<Mutex<String>> = OnceLock::new();

fn async_ctor_note_store() -> &'static Mutex<String> {
    ASYNC_CTOR_NOTE.get_or_init(|| Mutex::new(String::new()))
}

fn async_accessor_note_store() -> &'static Mutex<String> {
    ASYNC_ACCESSOR_NOTE.get_or_init(|| Mutex::new(String::new()))
}

#[derive(Debug, Default, PartialEq, Eq, AniClass)]
#[ani(class = "AsyncWidget")]
pub struct AsyncWidget {
    pub _tag: i32,
}

#[ani(class = "AsyncWidget")]
impl AsyncWidget {
    #[ani(constructor)]
    pub fn new(env: &Env<'_>, this: &AniObject<'_>) -> Result<()> {
        AsyncWidget { _tag: 0 }.write_back_to_ani_object(env, this)
    }

    #[ani(async)]
    pub async fn bump(&mut self, delta: i32) -> Result<i32> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        self._tag += delta;
        Ok(self._tag)
    }

    #[ani(async)]
    pub async fn describe(&self, env: &Env<'_>) -> Result<String> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        let text = env.create_string(&format!("widget:{}", self._tag))?;
        env.get_string(&text)
    }

    #[ani(static, async)]
    pub async fn class_handle_ready(class: &AniClass<'_>) -> Result<bool> {
        tokio::time::sleep(Duration::from_millis(5)).await;
        Ok(!class.as_raw().is_null())
    }
}

#[ani(class = "AsyncCtorBox", async, constructor)]
pub async fn async_ctor_box_new(label: String, total: i32) -> Result<()> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    ASYNC_CTOR_TOTAL.store(total, Ordering::SeqCst);
    if let Ok(mut slot) = async_ctor_note_store().lock() {
        *slot = format!("ctor:{label}");
    }
    Ok(())
}

#[ani(class = "AsyncCtorBox", getter = "total")]
pub fn async_ctor_box_total() -> i32 {
    ASYNC_CTOR_TOTAL.load(Ordering::SeqCst)
}

#[ani(class = "AsyncCtorBox", getter = "note")]
pub fn async_ctor_box_note() -> String {
    async_ctor_note_store()
        .lock()
        .map(|slot| slot.clone())
        .unwrap_or_default()
}

#[ani(class = "AsyncAccessorBox", constructor)]
pub fn async_accessor_box_new(initial_note: String) {
    if let Ok(mut slot) = async_accessor_note_store().lock() {
        *slot = initial_note;
    }
}

#[ani(class = "AsyncAccessorBox", getter = "note")]
pub fn async_accessor_box_note() -> String {
    async_accessor_note_store()
        .lock()
        .map(|slot| slot.clone())
        .unwrap_or_default()
}

#[ani(class = "AsyncAccessorBox", async, getter = "summary")]
pub async fn async_accessor_box_summary() -> Result<String> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    Ok(format!("note:{}", async_accessor_box_note()))
}

#[ani(class = "AsyncAccessorBox", async, setter = "note")]
pub async fn async_accessor_box_set_note(note: String) -> Result<()> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    if let Ok(mut slot) = async_accessor_note_store().lock() {
        *slot = note;
    }
    Ok(())
}

#[ani]
pub fn async_square(n: i32) -> i32 {
    expensive_computation(n)
}

#[ani]
pub fn async_fetch(url: String) -> String {
    fetch_data(&url)
}

#[ani]
pub fn async_compute_start(input: i32) -> i32 {
    input.abs() % 1000
}

#[ani]
pub fn async_check_status(task_id: i32) -> bool {
    task_id > 0
}

#[ani]
pub fn async_get_result(task_id: i32) -> i32 {
    task_id * task_id
}

#[ani]
pub fn batch_compute(count: i32) -> i64 {
    (0..count).map(|i| expensive_computation(i) as i64).sum()
}

#[ani]
pub fn promise_resolve_int(env: &Env<'_>, value: i32) -> Result<i64> {
    let promise = PromiseRaw::<i32>::resolve_int(env, value)?.into_object();
    Ok(promise.as_raw() as i64)
}

#[ani]
pub fn promise_resolve_string(env: &Env<'_>, value: String) -> Result<i64> {
    let promise = PromiseRaw::<String>::resolve_string(env, &value)?.into_object();
    Ok(promise.as_raw() as i64)
}

#[ani]
pub fn promise_reject(env: &Env<'_>, message: String) -> Result<i64> {
    let promise = PromiseRaw::<()>::reject(env, &message)?.into_object();
    Ok(promise.as_raw() as i64)
}

#[ani]
pub fn promise_delayed(env: &Env<'_>, delay_ms: i32, value: String) -> Result<i64> {
    let (deferred, promise) = PromiseRaw::<String>::deferred(env)?;

    if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms as u64));
    }

    deferred.resolve_string(env, &value)?;
    Ok(promise.into_object().as_raw() as i64)
}

#[ani]
pub fn promise_maybe_succeed(env: &Env<'_>, should_succeed: bool, value: i32) -> Result<i64> {
    let (deferred, promise) = PromiseRaw::<i32>::deferred(env)?;

    if should_succeed {
        deferred.resolve_int(env, value)?;
    } else {
        deferred.reject(env, "Operation failed as requested")?;
    }

    Ok(promise.into_object().as_raw() as i64)
}

#[ani(class = "example.MyClass")]
pub fn get_info(_env: &Env<'_>, this: &AniObject<'_>) -> String {
    let _ = this;
    "Instance info from Rust".to_string()
}

#[ani(class = "example.MyClass", static)]
pub fn create(env: &Env<'_>, _name: String) -> Result<i64> {
    let raw = env.get_undefined_object()?;
    Ok(raw as i64)
}

#[ani(async)]
pub async fn env_roundtrip(env: &Env<'_>, value: String) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    let text = env.create_string(&format!("env:{value}"))?;
    env.get_string(&text)
}

#[ani(async)]
pub async fn async_object_strict_equals(
    env: &Env<'_>,
    lhs: AniObject<'_>,
    rhs: AniObject<'_>,
) -> Result<bool> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    let lhs_ref: AniRef<'_> = lhs.into();
    let rhs_ref: AniRef<'_> = rhs.into();
    env.reference_strict_equals(&lhs_ref, &rhs_ref)
}

#[ani(async)]
pub async fn async_ref_roundtrip(env: &Env<'_>, value: AniRef<'_>) -> Result<bool> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    let global = env.create_global_ref(&value)?;
    let restored = env.local_ref_from_global_ref(&global)?;
    let same = env.reference_strict_equals(&value, &restored)?;
    env.delete_global_ref(global)?;
    Ok(same)
}

#[ani(class = "AsyncWidget", async)]
pub async fn this_handle_ready(this: &AniObject<'_>) -> Result<bool> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    Ok(!this.as_raw().is_null())
}

#[ani(async, signature = "C{std.core.String}:C{std.core.Promise}")]
pub async fn signature_override_echo(value: String) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    Ok(format!("sig:{value}"))
}

#[ani]
pub fn tokio_delayed_square(
    env: &Env<'_>,
    input: i32,
    delay_ms: i32,
) -> Result<PromiseRaw<'static, i32>> {
    ani::tokio::spawn_future(env, async move {
        if delay_ms > 0 {
            tokio::time::sleep(Duration::from_millis(delay_ms as u64)).await;
        }
        Ok(input * input)
    })
    .map(PromiseRaw::into_static)
}

#[ani]
pub fn tokio_fetch_text(env: &Env<'_>, url: String) -> Result<PromiseRaw<'static, String>> {
    ani::tokio::spawn_future(env, async move {
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(fetch_data(&url))
    })
    .map(PromiseRaw::into_static)
}

#[ani]
pub fn tokio_fail(env: &Env<'_>, message: String) -> Result<PromiseRaw<'static, String>> {
    ani::tokio::spawn_future(env, async move {
        tokio::time::sleep(Duration::from_millis(5)).await;
        Err(Error::new(Status::InvalidArgs, message))
    })
    .map(PromiseRaw::into_static)
}

#[ani]
pub fn tokio_manual_ref_container_ready(
    env: &Env<'_>,
    value: AniObject<'_>,
) -> Result<PromiseRaw<'static, bool>> {
    let vm = env.get_vm()?;
    let container = RefContainer::new(env, &value)?;

    ani::tokio::spawn_future_factory(env, move || async move {
        tokio::time::sleep(Duration::from_millis(5)).await;
        let attach = vm.attach_current_thread_scoped()?;
        let env = attach.env();
        let local: AniObject<'_> = container.to_local(&env)?;
        let ty = env.get_object_type(&local)?;
        Ok(!ty.as_raw().is_null())
    })
    .map(PromiseRaw::into_static)
}

// ============================================================================
// #[ani(async)] macro-based async bindings.
// ============================================================================

#[ani(async)]
pub async fn tokio_delayed_square_async(input: i32, delay_ms: i32) -> Result<i32> {
    if delay_ms > 0 {
        tokio::time::sleep(Duration::from_millis(delay_ms as u64)).await;
    }
    Ok(input * input)
}

#[ani(async)]
pub async fn tokio_fetch_text_async(url: String) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(10)).await;
    Ok(fetch_data(&url))
}

#[ani(async)]
pub async fn tokio_fail_async(message: String) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    Err(Error::new(Status::InvalidArgs, message))
}

#[ani(async)]
pub async fn tokio_void_async(delay_ms: i32) -> Result<()> {
    if delay_ms > 0 {
        tokio::time::sleep(Duration::from_millis(delay_ms as u64)).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_widget_logic_works() {
        let mut widget = AsyncWidget { _tag: 0 };

        let bumped = ani::tokio::block_on_future_result(widget.bump(2))
            .expect("runtime should execute")
            .expect("future should succeed");
        assert_eq!(bumped, 2);
        assert_eq!(widget._tag, 2);
    }

    #[test]
    fn async_signature_override_logic_works() {
        let echoed =
            ani::tokio::block_on_future_result(signature_override_echo("value".to_string()))
                .expect("runtime should execute")
                .expect("future should succeed");
        assert_eq!(echoed, "sig:value");
    }

    #[test]
    fn async_ref_container_examples_compile() {
        let _ = async_object_strict_equals;
        let _ = async_ref_roundtrip;
        let _ = tokio_manual_ref_container_ready;
    }

    #[test]
    fn async_constructor_combo_logic_works() {
        ani::tokio::block_on_future_result(async_ctor_box_new("demo".to_string(), 9))
            .expect("runtime should execute")
            .expect("constructor should succeed");
        assert_eq!(async_ctor_box_total(), 9);
        assert_eq!(async_ctor_box_note(), "ctor:demo");
    }

    #[test]
    fn async_accessor_combo_logic_works() {
        async_accessor_box_new("start".to_string());

        let summary = ani::tokio::block_on_future_result(async_accessor_box_summary())
            .expect("runtime should execute")
            .expect("getter should succeed");
        assert_eq!(summary, "note:start");

        ani::tokio::block_on_future_result(async_accessor_box_set_note("done".to_string()))
            .expect("runtime should execute")
            .expect("setter should succeed");
        assert_eq!(async_accessor_box_note(), "done");
    }
}
