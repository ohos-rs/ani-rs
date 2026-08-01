//! Bounded, non-blocking pull streams used by ArkTS-facing async iterators.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use crate::env::Env;
use crate::error::{AniErrorPayload, DynAniError, Error, Result, Status};
use crate::scheduler::{RuntimeCancellable, RuntimeRegistration};
use crate::sys;
use crate::types::AniRef;

use super::{PromiseRaw, PromiseValue, ToAni, TypeInfo};

static LIVE_STREAMS: AtomicUsize = AtomicUsize::new(0);

enum StreamSettlement<T, E> {
    Item(std::result::Result<T, E>),
    Error(Arc<DynAniError>),
    End,
    Cancelled,
}

struct StreamWaiter<T, E> {
    settle: Box<dyn FnOnce(StreamSettlement<T, E>) -> Result<()> + Send + 'static>,
}

impl<T, E> StreamWaiter<T, E> {
    fn settle(self, settlement: StreamSettlement<T, E>) -> Result<()> {
        (self.settle)(settlement)
    }
}

struct StreamState<T, E> {
    queue: VecDeque<std::result::Result<T, E>>,
    waiters: VecDeque<StreamWaiter<T, E>>,
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

impl<T, E> StreamInner<T, E> {
    fn close_state(
        &self,
        terminal_error: Option<Arc<DynAniError>>,
    ) -> (Vec<StreamWaiter<T, E>>, Option<Arc<DynAniError>>) {
        let waiters = self
            .state
            .lock()
            .map(|mut state| {
                if state.closed {
                    return (Vec::new(), state.terminal_error.clone());
                }
                state.closed = true;
                state.queue.clear();
                state.terminal_error = terminal_error;
                (
                    state.waiters.drain(..).collect::<Vec<_>>(),
                    state.terminal_error.clone(),
                )
            })
            .unwrap_or_default();
        self.space_available.notify_all();
        waiters
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
        for waiter in self.close_state(None).0 {
            let _ = waiter.settle(StreamSettlement::Cancelled);
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

    /// Send an error that will reject the next Promise.
    pub fn send_error(&self, error: E) -> Result<()> {
        let error: Arc<DynAniError> = Arc::new(Box::new(error));
        close_stream_with_error(&self.inner, error);
        Ok(())
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
    let (waiters, terminal_error) = inner.close_state(None);
    for waiter in waiters {
        let settlement = terminal_error
            .as_ref()
            .map(|error| StreamSettlement::Error(Arc::clone(error)))
            .unwrap_or(StreamSettlement::End);
        let _ = waiter.settle(settlement);
    }
}

/// Mark a producer as naturally exhausted without discarding items that were
/// accepted before the last sender went away. There cannot normally be both
/// queued items and waiters, but pairing them here keeps the transition
/// correct under every send/drop interleaving.
fn finish_stream<T, E>(inner: &Arc<StreamInner<T, E>>) {
    let settlements = inner
        .state
        .lock()
        .map(|mut state| {
            if state.closed {
                return Vec::new();
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
            settlements
        })
        .unwrap_or_default();
    inner.space_available.notify_all();
    for (waiter, settlement) in settlements {
        let _ = waiter.settle(settlement);
    }
}

fn return_stream<T, E>(inner: &Arc<StreamInner<T, E>>) {
    let waiters = inner
        .state
        .lock()
        .map(|mut state| {
            state.closed = true;
            state.queue.clear();
            state.terminal_error = None;
            state.waiters.drain(..).collect::<Vec<_>>()
        })
        .unwrap_or_default();
    inner.space_available.notify_all();
    for waiter in waiters {
        let _ = waiter.settle(StreamSettlement::End);
    }
}

fn close_stream_with_error<T, E>(inner: &Arc<StreamInner<T, E>>, error: Arc<DynAniError>) {
    let (waiters, terminal_error) = inner.close_state(Some(error));
    for waiter in waiters {
        let _ = waiter.settle(StreamSettlement::Error(
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
        self.ensure_runtime_registration()?;
        let (deferred, promise) = PromiseRaw::deferred(env)?;
        let vm = env.get_vm()?;
        let mut waiter = Some(StreamWaiter {
            settle: Box::new(move |settlement| {
                vm.with_attached(|env| match settlement {
                    StreamSettlement::Item(Ok(item)) => {
                        deferred.resolve_value(env, AsyncIteratorValue(Some(item)))
                    }
                    StreamSettlement::Item(Err(error)) => deferred.reject_with_payload(env, error),
                    StreamSettlement::Error(error) => deferred.reject_with_payload(env, error),
                    StreamSettlement::End => deferred.resolve_value(env, AsyncIteratorValue(None)),
                    StreamSettlement::Cancelled => deferred.reject_with_error(
                        env,
                        Error::new(
                            Status::Cancelled,
                            "async stream cancelled during runtime shutdown",
                        ),
                    ),
                })
            }),
        });
        let immediate = {
            let mut state =
                self.inner.state.lock().map_err(|_| {
                    Error::new(Status::GenericFailure, "async stream lock poisoned")
                })?;
            if let Some(item) = state.queue.pop_front() {
                self.inner.space_available.notify_one();
                Some(StreamSettlement::Item(item))
            } else if state.closed {
                Some(
                    state
                        .terminal_error
                        .as_ref()
                        .map(|error| StreamSettlement::Error(Arc::clone(error)))
                        .unwrap_or(StreamSettlement::End),
                )
            } else {
                state
                    .waiters
                    .push_back(waiter.take().expect("stream waiter is present"));
                None
            }
        };

        if let Some(settlement) = immediate {
            waiter
                .take()
                .expect("stream waiter was not queued")
                .settle(settlement)?;
        }
        Ok(promise)
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

    fn ensure_runtime_registration(&self) -> Result<()> {
        let mut registration = self.inner.registration.lock().map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "async stream registration lock poisoned",
            )
        })?;
        if registration.is_none() {
            *registration = Some(crate::scheduler::shared().register_cancellable(&self.inner)?);
        }
        Ok(())
    }
}

/// Number of live stream receivers, for leak gates.
pub fn live_async_stream_count() -> usize {
    LIVE_STREAMS.load(Ordering::Acquire)
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
}
