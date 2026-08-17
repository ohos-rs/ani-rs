//! Bounded, non-blocking pull streams used by ArkTS-facing async iterators.
//!
//! [`StreamSender::send_async`] and [`AsyncStream::recv`] are executor-neutral.
//! Enable `tokio_stream` to pump a `tokio_stream::Stream` into this channel or
//! consume it with `StreamExt`.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};

use crate::env::Env;
use crate::error::{AniErrorPayload, DynAniError, Error, Result, Status};
use crate::scheduler::{RuntimeCancellable, RuntimeRegistration};
use crate::sys;
use crate::types::AniRef;

use super::{PromiseRaw, PromiseValue, ToAni, TypeInfo};

static LIVE_STREAMS: AtomicUsize = AtomicUsize::new(0);
static PENDING_STREAM_WAITERS: AtomicUsize = AtomicUsize::new(0);

enum StreamSettlement<T, E> {
    Item(std::result::Result<T, E>),
    Error(Arc<DynAniError>),
    End,
    Cancelled,
}

struct StreamWaiter<T, E> {
    settle: Box<dyn FnOnce(StreamSettlement<T, E>) -> Result<()> + Send + 'static>,
    _metric: StreamWaiterMetric,
}

enum StreamWriteSettlement {
    Accepted,
    Error(Arc<DynAniError>),
    Closed,
    Cancelled,
}

struct StreamWriteWaiter<T> {
    item: Option<T>,
    settle: Box<dyn FnOnce(StreamWriteSettlement) -> Result<()> + Send + 'static>,
    _metric: StreamWaiterMetric,
}

impl<T> StreamWriteWaiter<T> {
    fn settle(self, settlement: StreamWriteSettlement) -> Result<()> {
        (self.settle)(settlement)
    }
}

struct StreamWaiterMetric;

impl StreamWaiterMetric {
    fn new() -> Self {
        PENDING_STREAM_WAITERS.fetch_add(1, Ordering::AcqRel);
        Self
    }
}

impl Drop for StreamWaiterMetric {
    fn drop(&mut self) {
        PENDING_STREAM_WAITERS.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T, E> StreamWaiter<T, E> {
    fn settle(self, settlement: StreamSettlement<T, E>) -> Result<()> {
        (self.settle)(settlement)
    }
}

struct StreamState<T, E> {
    queue: VecDeque<std::result::Result<T, E>>,
    waiters: VecDeque<StreamWaiter<T, E>>,
    write_waiters: VecDeque<StreamWriteWaiter<T>>,
    senders: usize,
    closed: bool,
    terminal_error: Option<Arc<DynAniError>>,
}

struct StreamInner<T, E> {
    state: Mutex<StreamState<T, E>>,
    space_available: Condvar,
    capacity: usize,
    registration: Mutex<Option<RuntimeRegistration>>,
}

type StreamCloseState<T, E> = (
    Vec<StreamWaiter<T, E>>,
    Vec<StreamWriteWaiter<T>>,
    Option<Arc<DynAniError>>,
);

impl<T, E> StreamInner<T, E> {
    fn close_state(&self, terminal_error: Option<Arc<DynAniError>>) -> StreamCloseState<T, E> {
        let waiters = self
            .state
            .lock()
            .map(|mut state| {
                if state.closed {
                    return (Vec::new(), Vec::new(), state.terminal_error.clone());
                }
                state.closed = true;
                state.queue.clear();
                state.terminal_error = terminal_error;
                (
                    state.waiters.drain(..).collect::<Vec<_>>(),
                    state.write_waiters.drain(..).collect::<Vec<_>>(),
                    state.terminal_error.clone(),
                )
            })
            .unwrap_or_default();
        self.space_available.notify_all();
        waiters
    }
}

impl<T, E> StreamInner<T, E>
where
    T: Send + 'static,
    E: Send + 'static,
{
    fn ensure_runtime_registration(self: &Arc<Self>) -> Result<()> {
        let mut registration = self.registration.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "async stream registration lock poisoned",
            )
        })?;
        if registration.is_none() {
            *registration = Some(crate::scheduler::shared().register_cancellable(self)?);
        }
        Ok(())
    }
}

impl<T, E> Drop for StreamInner<T, E> {
    fn drop(&mut self) {
        LIVE_STREAMS.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<T, E> RuntimeCancellable for StreamInner<T, E>
where
    T: Send + 'static,
    E: Send + 'static,
{
    fn cancel_for_runtime_shutdown(&self) {
        let (read_waiters, write_waiters, _) = self.close_state(None);
        for waiter in read_waiters {
            let _ = waiter.settle(StreamSettlement::Cancelled);
        }
        for waiter in write_waiters {
            let _ = waiter.settle(StreamWriteSettlement::Cancelled);
        }
    }
}

enum ReadOutcome<T, E> {
    Immediate {
        waiter: StreamWaiter<T, E>,
        settlement: StreamSettlement<T, E>,
        released_write: Option<StreamWriteWaiter<T>>,
    },
    Queued,
}

enum WriteOutcome<T, E> {
    Immediate {
        waiter: StreamWriteWaiter<T>,
        settlement: StreamWriteSettlement,
        read_waiter: Option<(StreamWaiter<T, E>, T)>,
    },
    Queued,
}

impl<T, E> StreamInner<T, E> {
    fn lock_state(&self) -> Result<std::sync::MutexGuard<'_, StreamState<T, E>>> {
        self.state
            .lock()
            .map_err(|_| Error::new(Status::GenericFailure, "async stream lock poisoned"))
    }

    fn read_or_wait(&self, waiter: StreamWaiter<T, E>) -> Result<ReadOutcome<T, E>> {
        let mut state = self.lock_state()?;
        if let Some(item) = state.queue.pop_front() {
            let mut released = state.write_waiters.pop_front();
            if let Some(waiter) = released.as_mut() {
                state.queue.push_back(Ok(waiter
                    .item
                    .take()
                    .expect("queued write item is present")));
            } else {
                self.space_available.notify_one();
            }
            Ok(ReadOutcome::Immediate {
                waiter,
                settlement: StreamSettlement::Item(item),
                released_write: released,
            })
        } else if state.closed {
            Ok(ReadOutcome::Immediate {
                waiter,
                settlement: state
                    .terminal_error
                    .as_ref()
                    .map(|error| StreamSettlement::Error(Arc::clone(error)))
                    .unwrap_or(StreamSettlement::End),
                released_write: None,
            })
        } else {
            state.waiters.push_back(waiter);
            Ok(ReadOutcome::Queued)
        }
    }

    fn write_or_wait(&self, mut waiter: StreamWriteWaiter<T>) -> Result<WriteOutcome<T, E>> {
        let mut state = self.lock_state()?;
        if state.closed {
            let settlement = state
                .terminal_error
                .as_ref()
                .map(|error| StreamWriteSettlement::Error(Arc::clone(error)))
                .unwrap_or(StreamWriteSettlement::Closed);
            Ok(WriteOutcome::Immediate {
                waiter,
                settlement,
                read_waiter: None,
            })
        } else if let Some(read_waiter) = state.waiters.pop_front() {
            let item = waiter.item.take().expect("write waiter item is present");
            Ok(WriteOutcome::Immediate {
                waiter,
                settlement: StreamWriteSettlement::Accepted,
                read_waiter: Some((read_waiter, item)),
            })
        } else if state.queue.len() < self.capacity {
            let item = waiter.item.take().expect("write waiter item is present");
            state.queue.push_back(Ok(item));
            Ok(WriteOutcome::Immediate {
                waiter,
                settlement: StreamWriteSettlement::Accepted,
                read_waiter: None,
            })
        } else {
            state.write_waiters.push_back(waiter);
            Ok(WriteOutcome::Queued)
        }
    }

    fn requeue_unread(&self, item: std::result::Result<T, E>) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        if let Some(waiter) = state.waiters.pop_front() {
            drop(state);
            let _ = waiter.settle(StreamSettlement::Item(item));
            return;
        }
        if !state.closed {
            state.queue.push_front(item);
        }
    }
}

/// Sending half of a bounded async-iterator channel.
pub struct StreamSender<T, E = Error> {
    inner: Arc<StreamInner<T, E>>,
}

impl<T, E> Clone for StreamSender<T, E> {
    fn clone(&self) -> Self {
        if let Ok(mut state) = self.inner.state.lock() {
            state.senders = state.senders.saturating_add(1);
        }
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T, E> Drop for StreamSender<T, E> {
    fn drop(&mut self) {
        let should_close = self
            .inner
            .state
            .lock()
            .map(|mut state| {
                state.senders = state.senders.saturating_sub(1);
                state.senders == 0 && !state.closed
            })
            .unwrap_or(false);
        if should_close {
            finish_stream(&self.inner);
        }
    }
}

impl<T, E> StreamSender<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    /// Send an item with bounded backpressure.
    pub fn send(&self, item: T) -> Result<()> {
        self.send_result(Ok(item))
    }

    /// Send an item or stream error with bounded backpressure.
    pub fn send_result(&self, item: std::result::Result<T, E>) -> Result<()> {
        let mut item = Some(item);
        let waiter = {
            let mut state =
                self.inner.state.lock().map_err(|_| {
                    Error::new(Status::GenericFailure, "async stream lock poisoned")
                })?;
            while state.queue.len() >= self.inner.capacity
                && state.waiters.is_empty()
                && !state.closed
            {
                state = self.inner.space_available.wait(state).map_err(|_| {
                    Error::new(Status::GenericFailure, "async stream lock poisoned")
                })?;
            }
            if state.closed {
                return Err(Error::new(
                    Status::Closing,
                    "async stream receiver is closed",
                ));
            }
            if let Some(waiter) = state.waiters.pop_front() {
                Some(waiter)
            } else {
                state
                    .queue
                    .push_back(item.take().expect("stream item is present"));
                None
            }
        };
        if let Some(waiter) = waiter {
            waiter.settle(StreamSettlement::Item(
                item.expect("stream item was not queued"),
            ))?;
        }
        Ok(())
    }

    /// Send without blocking when capacity is available.
    pub fn try_send(&self, item: T) -> Result<()> {
        self.try_send_result(Ok(item))
    }

    /// Return a Promise which resolves once this item has entered the bounded
    /// queue (or was delivered directly to a pending reader).
    ///
    /// Full queues register a non-blocking state-machine waiter. No scheduler
    /// worker is occupied while OpenHarmony `Writable.doWrite` waits to invoke
    /// its completion callback, so callback completion naturally represents
    /// the stream's drain/backpressure boundary.
    pub fn send_promise<'env>(&self, env: &Env<'env>, item: T) -> Result<PromiseRaw<'env, ()>> {
        self.inner.ensure_runtime_registration()?;
        let (deferred, promise) = PromiseRaw::deferred(env)?;
        let vm = env.get_vm()?;
        let write_waiter = StreamWriteWaiter {
            item: Some(item),
            settle: Box::new(move |settlement| {
                vm.with_attached(|env| match settlement {
                    StreamWriteSettlement::Accepted => deferred.resolve_value(env, ()),
                    StreamWriteSettlement::Error(error) => deferred.reject_with_payload(env, error),
                    StreamWriteSettlement::Closed => deferred.reject_with_error(
                        env,
                        Error::new(Status::Closing, "async stream receiver is closed"),
                    ),
                    StreamWriteSettlement::Cancelled => deferred.reject_with_payload(
                        env,
                        crate::async_runtime::runtime_cancellation_error(
                            crate::async_runtime::RuntimeCancelReason::Shutdown,
                        ),
                    ),
                })
            }),
            _metric: StreamWaiterMetric::new(),
        };

        match self.inner.write_or_wait(write_waiter)? {
            WriteOutcome::Immediate {
                waiter,
                settlement,
                read_waiter,
            } => {
                if let Some((read_waiter, item)) = read_waiter {
                    read_waiter.settle(StreamSettlement::Item(Ok(item)))?;
                }
                waiter.settle(settlement)?;
            }
            WriteOutcome::Queued => {}
        }
        Ok(promise)
    }

    /// Send without blocking a worker when the queue is full.
    ///
    /// The future stays pending until a reader consumes an item, the stream
    /// closes, or RuntimeDomain cancellation wins. Dropping the future does
    /// not cancel a send that has already been queued.
    pub fn send_async(&self, item: T) -> SendFuture<T, E> {
        SendFuture {
            inner: Arc::clone(&self.inner),
            item: Some(item),
            shared: None,
        }
    }

    /// Send an error that will reject the next Promise.
    pub fn send_error(&self, error: E) -> Result<()> {
        self.close_with_payload(error);
        Ok(())
    }

    /// Terminate the stream with an arbitrary structured payload.
    pub fn close_with_payload(&self, error: impl AniErrorPayload) {
        let error: Arc<DynAniError> = Arc::new(Box::new(error));
        close_stream_with_error(&self.inner, error);
    }

    /// Close the stream explicitly. Pending `next()` calls resolve as done.
    pub fn close(&self) {
        close_stream(&self.inner, None);
    }

    fn try_send_result(&self, item: std::result::Result<T, E>) -> Result<()> {
        let mut item = Some(item);
        let waiter = {
            let mut state =
                self.inner.state.lock().map_err(|_| {
                    Error::new(Status::GenericFailure, "async stream lock poisoned")
                })?;
            if state.closed {
                return Err(Error::new(
                    Status::Closing,
                    "async stream receiver is closed",
                ));
            }
            if let Some(waiter) = state.waiters.pop_front() {
                Some(waiter)
            } else if state.queue.len() >= self.inner.capacity {
                return Err(Error::new(Status::QueueFull, "async stream queue is full"));
            } else {
                state
                    .queue
                    .push_back(item.take().expect("stream item is present"));
                None
            }
        };
        if let Some(waiter) = waiter {
            waiter.settle(StreamSettlement::Item(
                item.expect("stream item was not queued"),
            ))?;
        }
        Ok(())
    }
}

fn close_stream<T, E>(inner: &Arc<StreamInner<T, E>>, _reason: Option<Error>) {
    let (waiters, write_waiters, terminal_error) = inner.close_state(None);
    for waiter in waiters {
        let settlement = terminal_error
            .as_ref()
            .map(|error| StreamSettlement::Error(Arc::clone(error)))
            .unwrap_or(StreamSettlement::End);
        let _ = waiter.settle(settlement);
    }
    for waiter in write_waiters {
        let _ = waiter.settle(StreamWriteSettlement::Closed);
    }
}

/// Mark a producer as naturally exhausted without discarding items that were
/// accepted before the last sender went away. There cannot normally be both
/// queued items and waiters, but pairing them here keeps the transition
/// correct under every send/drop interleaving.
fn finish_stream<T, E>(inner: &Arc<StreamInner<T, E>>) {
    let (settlements, write_waiters) = inner
        .state
        .lock()
        .map(|mut state| {
            if state.closed {
                return (Vec::new(), Vec::new());
            }
            state.closed = true;
            let mut settlements = Vec::with_capacity(state.waiters.len());
            while let Some(waiter) = state.waiters.pop_front() {
                let settlement = state
                    .queue
                    .pop_front()
                    .map(StreamSettlement::Item)
                    .unwrap_or(StreamSettlement::End);
                settlements.push((waiter, settlement));
            }
            let write_waiters = state.write_waiters.drain(..).collect::<Vec<_>>();
            (settlements, write_waiters)
        })
        .unwrap_or_default();
    inner.space_available.notify_all();
    for (waiter, settlement) in settlements {
        let _ = waiter.settle(settlement);
    }
    for waiter in write_waiters {
        let _ = waiter.settle(StreamWriteSettlement::Closed);
    }
}

fn return_stream<T, E>(inner: &Arc<StreamInner<T, E>>) {
    let (waiters, write_waiters) = inner
        .state
        .lock()
        .map(|mut state| {
            state.closed = true;
            state.queue.clear();
            state.terminal_error = None;
            (
                state.waiters.drain(..).collect::<Vec<_>>(),
                state.write_waiters.drain(..).collect::<Vec<_>>(),
            )
        })
        .unwrap_or_default();
    inner.space_available.notify_all();
    for waiter in waiters {
        let _ = waiter.settle(StreamSettlement::End);
    }
    for waiter in write_waiters {
        let _ = waiter.settle(StreamWriteSettlement::Closed);
    }
}

fn close_stream_with_error<T, E>(inner: &Arc<StreamInner<T, E>>, error: Arc<DynAniError>) {
    let (waiters, write_waiters, terminal_error) = inner.close_state(Some(error));
    for waiter in waiters {
        let _ = waiter.settle(StreamSettlement::Error(
            terminal_error
                .as_ref()
                .expect("stream terminal error was installed")
                .clone(),
        ));
    }
    for waiter in write_waiters {
        let _ = waiter.settle(StreamWriteSettlement::Error(
            terminal_error
                .as_ref()
                .expect("stream terminal error was installed")
                .clone(),
        ));
    }
}

/// Pull-based bounded stream. Each `next_promise` resolves to an item or
/// `null` when all senders have been dropped.
pub struct AsyncStream<T, E = Error> {
    inner: Arc<StreamInner<T, E>>,
}

impl<T, E> Clone for AsyncStream<T, E> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Semantic alias for APIs exposing an ArkTS-style async iterator.
pub type AsyncIterator<T, E = Error> = AsyncStream<T, E>;
/// Sender alias paired with [`AsyncIterator`].
pub type AsyncIteratorSender<T, E = Error> = StreamSender<T, E>;

/// Native source endpoint used by an API 23+ `@ohos.util.stream.Readable`
/// subclass. The ETS adapter drives [`AsyncStream::next_promise`], so
/// pause/resume remain on the AbortSignal/stream owning ArkTS thread while the
/// Rust side supplies bounded backpressure, close, error, and cancellation.
pub type OhosReadableSource<E = Error> = AsyncStream<super::Uint8Array, E>;

/// Native sink endpoint used by an API 23+ `@ohos.util.stream.Writable`
/// subclass. `doWrite` maps to [`StreamSender::send_promise`]; its callback is
/// invoked when that Promise settles, providing bounded drain/backpressure.
pub type OhosWritableSink<E = Error> = StreamSender<super::Uint8Array, E>;

/// Nullable result used internally to resolve an async-iterator `next()` Promise.
pub struct AsyncIteratorValue<T>(pub Option<T>);

impl<T: TypeInfo> TypeInfo for AsyncIteratorValue<T> {
    fn type_signature() -> &'static str {
        T::type_signature()
    }

    fn ani_c_type() -> &'static str {
        "ani_object"
    }
}

impl<'env, T> PromiseValue<'env> for AsyncIteratorValue<T>
where
    Option<T>: ToAni<'env, Output = sys::ani_object>,
{
    fn into_promise_ref(self, env: &Env<'env>) -> Result<AniRef<'env>> {
        let value = self.0.to_ani(env)?;
        Ok(unsafe { AniRef::from_raw(value as sys::ani_ref) })
    }
}

/// Create a bounded stream. Capacity must be greater than zero.
pub fn stream_channel<T>(capacity: usize) -> Result<(StreamSender<T>, AsyncStream<T>)> {
    stream_channel_with_error(capacity)
}

/// Create a bounded stream with an application-defined structured error type.
pub fn stream_channel_with_error<T, E>(
    capacity: usize,
) -> Result<(StreamSender<T, E>, AsyncStream<T, E>)> {
    if capacity == 0 {
        return Err(Error::new(
            Status::InvalidArgs,
            "async stream capacity must be greater than zero",
        ));
    }
    LIVE_STREAMS.fetch_add(1, Ordering::AcqRel);
    let inner = Arc::new(StreamInner {
        state: Mutex::new(StreamState {
            queue: VecDeque::with_capacity(capacity),
            waiters: VecDeque::new(),
            write_waiters: VecDeque::new(),
            senders: 1,
            closed: false,
            terminal_error: None,
        }),
        space_available: Condvar::new(),
        capacity,
        registration: Mutex::new(None),
    });
    Ok((
        StreamSender {
            inner: Arc::clone(&inner),
        },
        AsyncStream { inner },
    ))
}

/// Creates paired OpenHarmony byte-stream endpoints with a custom error type.
pub fn ohos_byte_stream_channel_with_error<E>(
    capacity: usize,
) -> Result<(OhosWritableSink<E>, OhosReadableSource<E>)> {
    stream_channel_with_error(capacity)
}

/// Creates paired OpenHarmony byte-stream endpoints using ani-rs errors.
pub fn ohos_byte_stream_channel(capacity: usize) -> Result<(OhosWritableSink, OhosReadableSource)> {
    stream_channel(capacity)
}

impl<T, E> AsyncStream<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    /// Returns true after a successful producer close once every queued item
    /// has been consumed. Terminal errors deliberately remain observable until
    /// the consumer calls [`return_promise`](Self::return_promise).
    pub fn is_exhausted(&self) -> bool {
        self.inner
            .state
            .lock()
            .map(|state| state.closed && state.queue.is_empty() && state.terminal_error.is_none())
            .unwrap_or(false)
    }

    /// Build one Promise for an iterator `next()` call without occupying a
    /// scheduler worker while the stream is idle.
    pub fn next_promise<'env>(
        &self,
        env: &Env<'env>,
    ) -> Result<PromiseRaw<'env, AsyncIteratorValue<T>>> {
        self.inner.ensure_runtime_registration()?;
        let (deferred, promise) = PromiseRaw::deferred(env)?;
        let vm = env.get_vm()?;
        let waiter = StreamWaiter {
            settle: Box::new(move |settlement| {
                vm.with_attached(|env| match settlement {
                    StreamSettlement::Item(Ok(item)) => {
                        deferred.resolve_value(env, AsyncIteratorValue(Some(item)))
                    }
                    StreamSettlement::Item(Err(error)) => deferred.reject_with_payload(env, error),
                    StreamSettlement::Error(error) => deferred.reject_with_payload(env, error),
                    StreamSettlement::End => deferred.resolve_value(env, AsyncIteratorValue(None)),
                    StreamSettlement::Cancelled => deferred.reject_with_payload(
                        env,
                        crate::async_runtime::runtime_cancellation_error(
                            crate::async_runtime::RuntimeCancelReason::Shutdown,
                        ),
                    ),
                })
            }),
            _metric: StreamWaiterMetric::new(),
        };
        match self.inner.read_or_wait(waiter)? {
            ReadOutcome::Immediate {
                waiter,
                settlement,
                released_write,
            } => {
                if let Some(write_waiter) = released_write {
                    write_waiter.settle(StreamWriteSettlement::Accepted)?;
                }
                waiter.settle(settlement)?;
            }
            ReadOutcome::Queued => {}
        }
        Ok(promise)
    }

    /// Wait for the next item without occupying a scheduler worker.
    ///
    /// `Ok(None)` means every producer finished. Stream errors, `throw()`, and
    /// runtime cancellation resolve as `Err`.
    pub fn recv(&self) -> RecvFuture<T, E> {
        RecvFuture {
            inner: Arc::clone(&self.inner),
            shared: None,
        }
    }

    /// Implement AsyncIterator `return()`: stop the producer-facing stream,
    /// resolve every outstanding pull as done, and return a done result.
    pub fn return_promise<'env>(
        &self,
        env: &Env<'env>,
    ) -> Result<PromiseRaw<'env, AsyncIteratorValue<T>>> {
        return_stream(&self.inner);
        PromiseRaw::resolve_value(env, AsyncIteratorValue(None))
    }

    /// Implement AsyncIterator `throw()`: terminate the stream with the exact
    /// custom error payload and reject both outstanding and future pulls.
    pub fn throw_promise<'env, P>(
        &self,
        env: &Env<'env>,
        error: P,
    ) -> Result<PromiseRaw<'env, AsyncIteratorValue<T>>>
    where
        P: AniErrorPayload,
    {
        let error: Arc<DynAniError> = Arc::new(Box::new(error));
        close_stream_with_error(&self.inner, Arc::clone(&error));
        let (deferred, promise) = PromiseRaw::deferred(env)?;
        deferred.reject_with_payload(env, error)?;
        Ok(promise)
    }

    /// Close the receiver and resolve pending pulls as done.
    pub fn close(&self) {
        close_stream(&self.inner, None);
    }
}

/// Number of live stream receivers, for leak gates.
pub fn live_async_stream_count() -> usize {
    LIVE_STREAMS.load(Ordering::Acquire)
}

/// Number of unresolved async-iterator `next()` Promise waiters.
pub fn pending_async_stream_waiter_count() -> usize {
    PENDING_STREAM_WAITERS.load(Ordering::Acquire)
}

fn error_from_payload(payload: &(impl AniErrorPayload + ?Sized)) -> Error {
    let mut error = Error::new(Status::GenericFailure, payload.ani_message())
        .with_status_name(payload.ani_status())
        .with_code(payload.ani_code());
    payload.visit_ani_metadata(&mut |key, value| {
        error.metadata.insert(key.to_string(), value.to_string());
    });
    payload.visit_ani_properties(&mut |key, value| {
        error.insert_property(key.to_string(), value.clone());
    });
    if let Some(stack) = payload.ani_stack() {
        error.set_stack(Some(stack.to_string()));
    }
    error
}

fn decode_read_settlement<T, E: AniErrorPayload>(
    settlement: StreamSettlement<T, E>,
) -> Result<Option<T>> {
    match settlement {
        StreamSettlement::Item(Ok(item)) => Ok(Some(item)),
        StreamSettlement::Item(Err(error)) => Err(error_from_payload(&error)),
        StreamSettlement::Error(error) => Err(error_from_payload(error.as_ref())),
        StreamSettlement::End => Ok(None),
        StreamSettlement::Cancelled => Err(error_from_payload(
            crate::async_runtime::runtime_cancellation_error(
                crate::async_runtime::RuntimeCancelReason::Shutdown,
            )
            .as_ref(),
        )),
    }
}

fn decode_write_settlement(settlement: StreamWriteSettlement) -> Result<()> {
    match settlement {
        StreamWriteSettlement::Accepted => Ok(()),
        StreamWriteSettlement::Error(error) => Err(error_from_payload(error.as_ref())),
        StreamWriteSettlement::Closed => Err(Error::new(
            Status::Closing,
            "async stream receiver is closed",
        )),
        StreamWriteSettlement::Cancelled => Err(error_from_payload(
            crate::async_runtime::runtime_cancellation_error(
                crate::async_runtime::RuntimeCancelReason::Shutdown,
            )
            .as_ref(),
        )),
    }
}

struct AsyncWaiterShared<S> {
    settlement: Mutex<Option<S>>,
    waker: Mutex<Option<Waker>>,
    dropped: AtomicBool,
}

impl<S> AsyncWaiterShared<S> {
    fn new(waker: &Waker) -> Arc<Self> {
        Arc::new(Self {
            settlement: Mutex::new(None),
            waker: Mutex::new(Some(waker.clone())),
            dropped: AtomicBool::new(false),
        })
    }

    fn complete(&self, settlement: S) -> Option<S> {
        if self.dropped.load(Ordering::Acquire) {
            return Some(settlement);
        }
        if let Ok(mut slot) = self.settlement.lock() {
            *slot = Some(settlement);
        }
        if let Ok(mut waker) = self.waker.lock()
            && let Some(waker) = waker.take()
        {
            waker.wake();
        }
        None
    }

    fn take(&self) -> Option<S> {
        self.settlement.lock().ok().and_then(|mut slot| slot.take())
    }

    fn store_waker(&self, waker: &Waker) {
        if let Ok(mut slot) = self.waker.lock()
            && slot
                .as_ref()
                .is_none_or(|current| !current.will_wake(waker))
        {
            *slot = Some(waker.clone());
        }
    }
}

/// Future returned by [`StreamSender::send_async`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct SendFuture<T, E> {
    inner: Arc<StreamInner<T, E>>,
    item: Option<T>,
    shared: Option<Arc<AsyncWaiterShared<StreamWriteSettlement>>>,
}

// The owned item is moved, never polled in place.
impl<T, E> Unpin for SendFuture<T, E> {}

impl<T, E> Future for SendFuture<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(shared) = &this.shared {
            if let Some(settlement) = shared.take() {
                return Poll::Ready(decode_write_settlement(settlement));
            }
            shared.store_waker(cx.waker());
            if let Some(settlement) = shared.take() {
                return Poll::Ready(decode_write_settlement(settlement));
            }
            return Poll::Pending;
        }

        let shared = AsyncWaiterShared::new(cx.waker());
        let shared_cb = Arc::clone(&shared);
        let waiter = StreamWriteWaiter {
            item: this.item.take(),
            settle: Box::new(move |settlement| {
                drop(shared_cb.complete(settlement));
                Ok(())
            }),
            _metric: StreamWaiterMetric::new(),
        };
        match this.inner.write_or_wait(waiter)? {
            WriteOutcome::Immediate {
                waiter,
                settlement,
                read_waiter,
            } => {
                if let Some((read_waiter, item)) = read_waiter {
                    read_waiter.settle(StreamSettlement::Item(Ok(item)))?;
                }
                waiter.settle(settlement)?;
                if let Some(settlement) = shared.take() {
                    Poll::Ready(decode_write_settlement(settlement))
                } else {
                    Poll::Ready(decode_write_settlement(StreamWriteSettlement::Accepted))
                }
            }
            WriteOutcome::Queued => {
                this.shared = Some(shared);
                Poll::Pending
            }
        }
    }
}

impl<T, E> Drop for SendFuture<T, E> {
    fn drop(&mut self) {
        if let Some(shared) = &self.shared {
            shared.dropped.store(true, Ordering::Release);
        }
    }
}

/// Future returned by [`AsyncStream::recv`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct RecvFuture<T, E> {
    inner: Arc<StreamInner<T, E>>,
    shared: Option<Arc<AsyncWaiterShared<StreamSettlement<T, E>>>>,
}

impl<T, E> Future for RecvFuture<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    type Output = Result<Option<T>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(shared) = &this.shared {
            if let Some(settlement) = shared.take() {
                return Poll::Ready(decode_read_settlement(settlement));
            }
            shared.store_waker(cx.waker());
            if let Some(settlement) = shared.take() {
                return Poll::Ready(decode_read_settlement(settlement));
            }
            return Poll::Pending;
        }

        let shared = AsyncWaiterShared::new(cx.waker());
        let shared_cb = Arc::clone(&shared);
        let inner = Arc::clone(&this.inner);
        let waiter = StreamWaiter {
            settle: Box::new(move |settlement| {
                if let Some(settlement) = shared_cb.complete(settlement) {
                    inner.requeue_unread(match settlement {
                        StreamSettlement::Item(item) => item,
                        other => {
                            drop(other);
                            return Ok(());
                        }
                    });
                }
                Ok(())
            }),
            _metric: StreamWaiterMetric::new(),
        };
        match this.inner.read_or_wait(waiter)? {
            ReadOutcome::Immediate {
                waiter,
                settlement,
                released_write,
            } => {
                if let Some(write_waiter) = released_write {
                    write_waiter.settle(StreamWriteSettlement::Accepted)?;
                }
                waiter.settle(settlement)?;
                if let Some(settlement) = shared.take() {
                    Poll::Ready(decode_read_settlement(settlement))
                } else {
                    Poll::Ready(Ok(None))
                }
            }
            ReadOutcome::Queued => {
                this.shared = Some(shared);
                Poll::Pending
            }
        }
    }
}

impl<T, E> Drop for RecvFuture<T, E> {
    fn drop(&mut self) {
        if let Some(shared) = &self.shared {
            shared.dropped.store(true, Ordering::Release);
            if let Some(StreamSettlement::Item(item)) = shared.take() {
                self.inner.requeue_unread(item);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct DomainError;

    impl std::fmt::Display for DomainError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("domain error")
        }
    }

    impl AniErrorPayload for DomainError {
        fn ani_status(&self) -> &str {
            "StreamDomainError"
        }

        fn ani_code(&self) -> i32 {
            73001
        }

        fn ani_message(&self) -> &str {
            "stream domain error"
        }
    }

    #[test]
    fn queue_preserves_order_backpressure_and_close() {
        let (sender, stream) = stream_channel::<i32>(2).unwrap();
        sender.send(1).unwrap();
        sender.send(2).unwrap();
        assert_eq!(sender.try_send(3).unwrap_err().status, Status::QueueFull);
        let queued = stream.inner.state.lock().unwrap();
        assert_eq!(queued.queue.len(), 2);
        drop(queued);
        sender.close();
        assert!(stream.inner.state.lock().unwrap().closed);
        assert_eq!(sender.send(4).unwrap_err().status, Status::Closing);
    }

    #[test]
    fn last_sender_closes_stream_without_a_worker_job() {
        let (sender, stream) = stream_channel::<i32>(1).unwrap();
        let completed = crate::scheduler::shared().metrics().completed;
        drop(sender);
        assert!(stream.inner.state.lock().unwrap().closed);
        assert_eq!(crate::scheduler::shared().metrics().completed, completed);
    }

    #[test]
    fn last_sender_preserves_items_accepted_before_natural_end() {
        let (sender, stream) = stream_channel::<i32>(2).unwrap();
        sender.send(10).unwrap();
        sender.send(20).unwrap();
        drop(sender);
        let state = stream.inner.state.lock().unwrap();
        assert!(state.closed);
        assert_eq!(state.queue.len(), 2);
        assert!(matches!(state.queue[0], Ok(10)));
        assert!(matches!(state.queue[1], Ok(20)));
    }

    #[test]
    fn custom_error_type_is_accepted() {
        let (sender, stream) = stream_channel_with_error::<i32, DomainError>(1).unwrap();
        sender.send_error(DomainError).unwrap();
        let state = stream.inner.state.lock().unwrap();
        assert!(state.closed);
        assert!(state.terminal_error.is_some());
    }

    #[test]
    fn concurrent_waiters_are_fifo_and_do_not_schedule_worker_jobs() {
        let (sender, stream) = stream_channel::<i32>(2).unwrap();
        let completed = crate::scheduler::shared().metrics().completed;
        let (settled_sender, settled_receiver) = std::sync::mpsc::channel();
        {
            let mut state = stream.inner.state.lock().unwrap();
            for index in 0..32 {
                let settled_sender = settled_sender.clone();
                state.waiters.push_back(StreamWaiter {
                    settle: Box::new(move |settlement| {
                        let value = match settlement {
                            StreamSettlement::Item(Ok(value)) => value,
                            _ => -1,
                        };
                        settled_sender.send((index, value)).unwrap();
                        Ok(())
                    }),
                    _metric: StreamWaiterMetric::new(),
                });
            }
        }
        for value in 0..32 {
            sender.send(value).unwrap();
        }
        for expected in 0..32 {
            assert_eq!(settled_receiver.recv().unwrap(), (expected, expected));
        }
        assert_eq!(crate::scheduler::shared().metrics().completed, completed);
    }

    #[test]
    fn return_overrides_terminal_error_for_future_pulls() {
        let (sender, stream) = stream_channel_with_error::<i32, DomainError>(1).unwrap();
        sender.send_error(DomainError).unwrap();
        return_stream(&stream.inner);
        let state = stream.inner.state.lock().unwrap();
        assert!(state.closed);
        assert!(state.terminal_error.is_none());
    }

    #[test]
    fn shutdown_cancellation_settles_all_waiters() {
        let (_sender, stream) = stream_channel::<i32>(2).unwrap();
        let cancelled = Arc::new(AtomicUsize::new(0));
        {
            let mut state = stream.inner.state.lock().unwrap();
            for _ in 0..16 {
                let cancelled = Arc::clone(&cancelled);
                state.waiters.push_back(StreamWaiter {
                    settle: Box::new(move |settlement| {
                        if matches!(settlement, StreamSettlement::Cancelled) {
                            cancelled.fetch_add(1, Ordering::AcqRel);
                        }
                        Ok(())
                    }),
                    _metric: StreamWaiterMetric::new(),
                });
            }
        }
        stream.inner.cancel_for_runtime_shutdown();
        assert_eq!(cancelled.load(Ordering::Acquire), 16);
        assert!(stream.inner.state.lock().unwrap().closed);
    }

    #[test]
    fn blocked_producer_is_released_when_receiver_closes() {
        let (sender, stream) = stream_channel::<i32>(1).unwrap();
        sender.send(1).unwrap();
        let blocked = sender.clone();
        let producer = std::thread::spawn(move || blocked.send(2));
        std::thread::sleep(std::time::Duration::from_millis(5));
        stream.close();
        assert_eq!(
            producer.join().unwrap().unwrap_err().status,
            Status::Closing
        );
    }

    #[test]
    fn recv_completes_immediately_when_queued() {
        let (sender, stream) = stream_channel::<i32>(1).unwrap();
        sender.send(3).unwrap();
        let mut recv = std::pin::pin!(stream.recv());
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        assert!(matches!(
            recv.as_mut().poll(&mut context),
            Poll::Ready(Ok(Some(3)))
        ));
    }

    #[test]
    fn send_async_is_released_by_recv() {
        let (sender, stream) = stream_channel::<i32>(1).unwrap();
        sender.send(1).unwrap();
        let mut send = std::pin::pin!(sender.send_async(2));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        assert!(send.as_mut().poll(&mut context).is_pending());

        let mut recv = std::pin::pin!(stream.recv());
        assert!(matches!(
            recv.as_mut().poll(&mut context),
            Poll::Ready(Ok(Some(1)))
        ));
        assert!(matches!(
            send.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));

        let mut recv = std::pin::pin!(stream.recv());
        assert!(matches!(
            recv.as_mut().poll(&mut context),
            Poll::Ready(Ok(Some(2)))
        ));
    }

    #[test]
    fn recv_sees_natural_end_after_last_sender_drops() {
        let (sender, stream) = stream_channel::<i32>(1).unwrap();
        drop(sender);
        let mut recv = std::pin::pin!(stream.recv());
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        assert!(matches!(
            recv.as_mut().poll(&mut context),
            Poll::Ready(Ok(None))
        ));
    }

    #[test]
    fn pending_writable_waiters_close_without_scheduler_work() {
        let (_sender, stream) = stream_channel::<i32>(1).unwrap();
        let closing_thread = std::thread::current().id();
        let baseline = pending_async_stream_waiter_count();
        let (settled_tx, settled_rx) = std::sync::mpsc::channel();
        stream
            .inner
            .state
            .lock()
            .unwrap()
            .write_waiters
            .push_back(StreamWriteWaiter {
                item: Some(7),
                settle: Box::new(move |settlement| {
                    settled_tx
                        .send((
                            matches!(settlement, StreamWriteSettlement::Closed),
                            std::thread::current().id(),
                        ))
                        .unwrap();
                    Ok(())
                }),
                _metric: StreamWaiterMetric::new(),
            });
        assert_eq!(pending_async_stream_waiter_count(), baseline + 1);
        stream.close();
        let (closed, settlement_thread) = settled_rx.recv().unwrap();
        assert!(closed);
        assert_eq!(settlement_thread, closing_thread);
        assert_eq!(pending_async_stream_waiter_count(), baseline);
    }
}
