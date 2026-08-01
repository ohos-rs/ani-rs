//! Async Wrapper Example - Wrapping synchronous interfaces for Promise operations.

use ani::conversions::{
    AsyncIteratorValue, AsyncStream, AsyncTask, Deferred, ManagedResource, PromiseRaw,
    RefContainer, StreamSender, ThreadsafeFunction,
};
use ani::prelude::*;
use ani_derive::{ani, AniClass};
use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicI32, AtomicI64, Ordering},
    Mutex, OnceLock,
};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct AsyncDomainError {
    operation: String,
}

impl std::fmt::Display for AsyncDomainError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "async operation {} failed", self.operation)
    }
}

impl std::error::Error for AsyncDomainError {}

impl AniErrorPayload for AsyncDomainError {
    fn ani_status(&self) -> &str {
        "AsyncDomainFailure"
    }
    fn ani_code(&self) -> i32 {
        71001
    }
    fn ani_message(&self) -> &str {
        "domain-specific asynchronous failure"
    }
    fn visit_ani_metadata(&self, visitor: &mut dyn FnMut(&str, &str)) {
        visitor("operation", &self.operation);
    }
    fn visit_ani_properties(&self, visitor: &mut dyn FnMut(&str, &AniErrorValue)) {
        let retryable = AniErrorValue::Bool(false);
        let attempt = AniErrorValue::Integer(2);
        let binary = AniErrorValue::Binary(vec![0x41, 0x4e, 0x49]);
        let mut details = BTreeMap::new();
        details.insert(
            "labels".to_string(),
            AniErrorValue::Array(vec!["async".into(), "qemu".into()]),
        );
        details.insert("active".to_string(), AniErrorValue::Bool(true));
        let details = AniErrorValue::Object(details);
        visitor("retryable", &retryable);
        visitor("attempt", &attempt);
        visitor("binary", &binary);
        visitor("details", &details);
    }
}

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
struct CounterAsyncIteratorState {
    sender: Option<StreamSender<i32, AsyncDomainError>>,
    stream: AsyncStream<i32, AsyncDomainError>,
}

static LATEST_ASYNC_ITERATOR: AtomicI64 = AtomicI64::new(0);

fn async_ctor_note_store() -> &'static Mutex<String> {
    ASYNC_CTOR_NOTE.get_or_init(|| Mutex::new(String::new()))
}

fn async_accessor_note_store() -> &'static Mutex<String> {
    ASYNC_ACCESSOR_NOTE.get_or_init(|| Mutex::new(String::new()))
}

fn iterator_resource(
    env: &Env<'_>,
    this: &AniObject<'_>,
) -> Result<ManagedResource<CounterAsyncIteratorState>> {
    ManagedResource::from_raw(env.get_field_by_name_long(this, "__ani_resource")?)
}

fn latest_iterator_resource() -> Result<ManagedResource<CounterAsyncIteratorState>> {
    let handle = LATEST_ASYNC_ITERATOR.load(Ordering::Acquire);
    if handle <= 0 {
        return Err(Error::new(
            Status::NotFound,
            "CounterAsyncIterator is not initialized",
        ));
    }
    ManagedResource::from_raw(handle)
}

#[ani(class = "CounterAsyncIterator", constructor)]
pub fn counter_async_iterator_new(env: &Env<'_>, this: &AniObject<'_>) -> Result<()> {
    let (sender, stream) = ani::conversions::stream_channel_with_error(16)?;
    let resource = ManagedResource::new(CounterAsyncIteratorState {
        sender: Some(sender),
        stream,
    })?;
    env.set_field_by_name_long(this, "__ani_resource", resource.as_raw())?;
    env.set_field_by_name_boolean(this, "__ani_closed", false)?;
    LATEST_ASYNC_ITERATOR.store(resource.as_raw(), Ordering::Release);
    Ok(())
}

#[ani(class = "CounterAsyncIterator", name = "next")]
pub fn counter_async_iterator_next<'env>(
    env: &Env<'env>,
    this: &AniObject<'_>,
) -> Result<PromiseRaw<'env, AsyncIteratorValue<i32>>> {
    if env.get_field_by_name_boolean(this, "__ani_closed")? {
        return PromiseRaw::resolve_value(env, AsyncIteratorValue::<i32>(None));
    }
    let resource = iterator_resource(env, this)?;
    let promise = resource.with(|state| state.stream.next_promise(env))??;
    let exhausted = resource.with(|state| state.stream.is_exhausted())?;
    if exhausted {
        let _ = resource.close()?;
        env.set_field_by_name_boolean(this, "__ani_closed", true)?;
    }
    Ok(promise)
}

#[ani(class = "CounterAsyncIterator", name = "return")]
pub fn counter_async_iterator_return<'env>(
    env: &Env<'env>,
    this: &AniObject<'_>,
) -> Result<PromiseRaw<'env, AsyncIteratorValue<i32>>> {
    if env.get_field_by_name_boolean(this, "__ani_closed")? {
        return PromiseRaw::resolve_value(env, AsyncIteratorValue::<i32>(None));
    }
    let resource = iterator_resource(env, this)?;
    let promise = resource.with(|state| state.stream.return_promise(env))??;
    let _ = resource.close()?;
    env.set_field_by_name_boolean(this, "__ani_closed", true)?;
    Ok(promise)
}

#[ani(class = "CounterAsyncIterator", name = "throw")]
pub fn counter_async_iterator_throw<'env>(
    env: &Env<'env>,
    this: &AniObject<'_>,
    reason: AniRef<'_>,
) -> Result<PromiseRaw<'env, AsyncIteratorValue<i32>>> {
    let error = PreservedArktsError::new(env, &reason)?;
    if env.get_field_by_name_boolean(this, "__ani_closed")? {
        return PromiseRaw::reject_with_payload(env, error);
    }
    let resource = iterator_resource(env, this)?;
    let promise = resource.with(|state| state.stream.throw_promise(env, error))??;
    let _ = resource.close()?;
    env.set_field_by_name_boolean(this, "__ani_closed", true)?;
    Ok(promise)
}

#[ani]
pub fn push_async_iterator_value(value: i32) -> Result<()> {
    latest_iterator_resource()?.with(|state| {
        state
            .sender
            .as_ref()
            .ok_or_else(|| Error::new(Status::Closing, "CounterAsyncIterator is finished"))?
            .send(value)
    })?
}

#[ani]
pub fn finish_async_iterator() -> Result<()> {
    latest_iterator_resource()?.with_mut(|state| {
        state.sender.take();
    })
}

#[ani]
pub fn fail_async_iterator(operation: String) -> Result<()> {
    latest_iterator_resource()?.with(|state| {
        state
            .sender
            .as_ref()
            .ok_or_else(|| Error::new(Status::Closing, "CounterAsyncIterator is finished"))?
            .send_error(AsyncDomainError { operation })
    })?
}

#[derive(Debug, Default, PartialEq, Eq, AniClass)]
#[ani(class = "AsyncWidget")]
pub struct AsyncWidget {
    pub _tag: i32,
}

#[ani(class = "AsyncWidget")]
impl AsyncWidget {
    #[ani(constructor)]
    #[allow(clippy::new_ret_no_self)]
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

#[ani]
pub fn promise_new_typed_resolve(
    env: &Env<'_>,
    value: String,
) -> Result<PromiseRaw<'static, String>> {
    let (deferred, promise) = env.promise_new_typed::<String>()?;
    deferred.resolve_value(env, format!("typed:{value}"))?;
    Ok(promise.into_static())
}

#[ani]
pub fn promise_env_resolved(env: &Env<'_>, value: String) -> Result<PromiseRaw<'static, String>> {
    env.promise_resolved(format!("env:{value}"))
        .map(PromiseRaw::into_static)
}

#[ani]
pub fn promise_env_rejected(env: &Env<'_>, message: String) -> Result<PromiseRaw<'static, String>> {
    env.promise_rejected::<String>(&message)
        .map(PromiseRaw::into_static)
}

#[ani]
pub fn promise_resolver_resolve_value(
    env: &Env<'_>,
    value: String,
) -> Result<PromiseRaw<'static, String>> {
    let (resolver, promise) = env.promise_new()?;
    resolver.resolve_value(env, format!("resolver:{value}"))?;
    Ok(unsafe { PromiseRaw::<String>::from_raw(promise.into_raw()) }.into_static())
}

#[ani]
pub fn promise_resolver_into_deferred(
    env: &Env<'_>,
    value: String,
) -> Result<PromiseRaw<'static, String>> {
    let (resolver, promise) = env.promise_new()?;
    let deferred: Deferred<String> = resolver.into_deferred();
    deferred.resolve_value(env, format!("bridge:{value}"))?;
    Ok(unsafe { PromiseRaw::<String>::from_raw(promise.into_raw()) }.into_static())
}

#[ani]
pub fn promise_resolver_reject_message(
    env: &Env<'_>,
    message: String,
) -> Result<PromiseRaw<'static, ()>> {
    let (resolver, promise) = env.promise_new()?;
    resolver.reject_message(env, &message)?;
    Ok(unsafe { PromiseRaw::<()>::from_raw(promise.into_raw()) }.into_static())
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

#[ani(async)]
pub async fn async_call_scoped_callback(
    env: &Env<'_>,
    callback: Function<'_, (String,), String>,
    value: String,
) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    callback.call(env, (value,))
}

#[ani(async)]
pub async fn async_echo_function_ref(
    callback: FunctionRef<(String,), String>,
) -> Result<FunctionRef<(String,), String>> {
    tokio::time::sleep(Duration::from_millis(5)).await;
    Ok(callback)
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
pub fn tokio_await_arkts_promise(
    env: &Env<'_>,
    promise: PromiseRaw<'_, String>,
) -> Result<PromiseRaw<'static, String>> {
    let future = promise.into_future(env)?;
    ani::tokio::spawn_future(env, future).map(PromiseRaw::into_static)
}

#[ani]
pub fn pending_string_promise<'env>(env: &Env<'env>) -> Result<PromiseRaw<'env, String>> {
    let (_resolver, promise) = PromiseRaw::deferred(env)?;
    Ok(promise)
}

#[ani]
pub fn cancel_arkts_promise_wait(env: &Env<'_>, promise: PromiseRaw<'_, String>) -> Result<bool> {
    let mut future = promise.into_future(env)?;
    let first = future.cancel()?;
    let second = future.cancel()?;
    Ok(first && !second && future.is_cancelled())
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
        let local: AniObject<'_> = container.to_local(env)?;
        let ty = env.get_object_type(&local)?;
        Ok(!ty.as_raw().is_null())
    })
    .map(PromiseRaw::into_static)
}

#[ani]
pub fn tokio_manual_global_ref_container_ready(
    env: &Env<'_>,
    value: AniObject<'_>,
) -> Result<PromiseRaw<'static, bool>> {
    let vm = env.get_vm()?;
    let value_ref: AniRef<'_> = value.into();
    let global = env.create_global_ref(&value_ref)?;
    let container = RefContainer::new(env, &global)?;
    global.delete(env)?;

    ani::tokio::spawn_future_factory(env, move || async move {
        tokio::time::sleep(Duration::from_millis(5)).await;
        let attach = vm.attach_current_thread_scoped()?;
        let env = attach.env();
        let local: AniObject<'_> = container.to_local(env)?;
        let ty = env.get_object_type(&local)?;
        Ok(!ty.as_raw().is_null())
    })
    .map(PromiseRaw::into_static)
}

#[ani]
pub fn tokio_manual_typed_ref_container_ready(
    env: &Env<'_>,
    value: Ref<AniObject<'static>>,
) -> Result<PromiseRaw<'static, bool>> {
    let vm = env.get_vm()?;
    let container = RefContainer::new(env, &value)?;

    ani::tokio::spawn_future_factory(env, move || async move {
        tokio::time::sleep(Duration::from_millis(5)).await;
        let attach = vm.attach_current_thread_scoped()?;
        let env = attach.env();
        let local: AniObject<'_> = container.to_local(env)?;
        let ty = env.get_object_type(&local)?;
        Ok(!ty.as_raw().is_null())
    })
    .map(PromiseRaw::into_static)
}

#[ani]
pub fn tokio_manual_function_ref_container_call(
    env: &Env<'_>,
    callback: ThreadsafeFunction<(String,), String>,
    value: String,
) -> Result<PromiseRaw<'static, String>> {
    ani::tokio::spawn_future_factory(env, move || async move {
        tokio::time::sleep(Duration::from_millis(5)).await;
        callback.call_attached((value,))
    })
    .map(PromiseRaw::into_static)
}

pub struct SquareTask {
    input: i32,
}

pub struct SlowCancellableTask {
    delay_ms: i32,
}

impl Task for SlowCancellableTask {
    type Output = i32;
    type JsValue = i32;
    type Error = Error;

    fn compute(&mut self, cancellation: &CancellationToken) -> Result<Self::Output> {
        let mut remaining = self.delay_ms.max(0) as u64;
        while remaining > 0 {
            cancellation.check()?;
            let slice = remaining.min(5);
            thread::sleep(Duration::from_millis(slice));
            remaining -= slice;
        }
        cancellation.check()?;
        Ok(self.delay_ms)
    }

    fn resolve<'env>(self, _env: &Env<'env>, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

impl Task for SquareTask {
    type Output = i32;
    type JsValue = i32;
    type Error = Error;

    fn compute(&mut self, cancellation: &CancellationToken) -> Result<Self::Output> {
        cancellation.check()?;
        Ok(self.input * self.input)
    }

    fn resolve<'env>(self, _env: &Env<'env>, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[ani]
pub fn background_square(input: i32) -> AsyncTask<SquareTask, i32> {
    AsyncTask::new(SquareTask { input })
}

#[ani]
pub fn background_cancellable(delay_ms: i32) -> AsyncTask<SlowCancellableTask, i32> {
    AsyncTask::new(SlowCancellableTask { delay_ms })
}

/// Cancel the current shared generation, join every worker/timer, then prove
/// that a new submission lazily starts a clean generation.
#[ani]
pub fn runtime_kernel_shutdown_and_restart() -> Result<bool> {
    let kernel = runtime_kernel();
    let before = kernel.metrics().generation;
    shutdown_runtime()?;
    let drained = kernel.metrics().workers == 0;
    let marker = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let scheduled_marker = std::sync::Arc::clone(&marker);
    kernel.schedule(move || scheduled_marker.store(true, Ordering::Release))?;
    if !kernel.wait_idle(Duration::from_secs(2)) {
        return Err(Error::new(
            Status::GenericFailure,
            "restarted RuntimeKernel did not drain",
        ));
    }
    let after = kernel.metrics().generation;
    Ok(drained && after > before && marker.load(Ordering::Acquire))
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
pub async fn custom_async_error(
    operation: String,
) -> std::result::Result<String, AsyncDomainError> {
    Err(AsyncDomainError { operation })
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
        let _ = async_call_scoped_callback;
        let _ = async_echo_function_ref;
        let _ = promise_new_typed_resolve;
        let _ = promise_env_resolved;
        let _ = promise_env_rejected;
        let _ = promise_resolver_resolve_value;
        let _ = promise_resolver_into_deferred;
        let _ = promise_resolver_reject_message;
        let _ = tokio_manual_ref_container_ready;
        let _ = tokio_manual_global_ref_container_ready;
        let _ = tokio_manual_typed_ref_container_ready;
        let _ = tokio_manual_function_ref_container_call;
        let _ = background_square;
    }

    #[test]
    fn background_task_supports_cooperative_cancellation() {
        let task = AsyncTask::new(SquareTask { input: 8 });
        let token = task.cancellation_token();
        token.cancel();
        assert!(token.is_cancelled());
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
