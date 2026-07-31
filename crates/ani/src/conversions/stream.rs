//! Bounded pull streams used to implement ArkTS-facing async iterators.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use crate::env::Env;
use crate::error::{AniErrorPayload, DynAniError, Error, Result, Status};
use crate::sys;
use crate::types::AniRef;

use super::{AsyncTask, CancellationToken, PromiseValue, Task, ToAni, TypeInfo};

static LIVE_STREAMS: AtomicUsize = AtomicUsize::new(0);

struct StreamInner<T, E> {
    receiver: Mutex<mpsc::Receiver<std::result::Result<T, E>>>,
}

impl<T, E> Drop for StreamInner<T, E> {
    fn drop(&mut self) {
        LIVE_STREAMS.fetch_sub(1, Ordering::AcqRel);
    }
}

/// Sending half of a bounded async-iterator channel.
pub struct StreamSender<T, E = Error> {
    sender: mpsc::SyncSender<std::result::Result<T, E>>,
}

impl<T, E> Clone for StreamSender<T, E> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<T, E> StreamSender<T, E> {
    /// Send an item with bounded backpressure.
    pub fn send(&self, item: T) -> Result<()> {
        self.send_result(Ok(item))
    }

    /// Send an item or terminal stream error with bounded backpressure.
    pub fn send_result(&self, item: std::result::Result<T, E>) -> Result<()> {
        self.sender
            .send(item)
            .map_err(|_| Error::new(Status::Closing, "async stream receiver is closed"))
    }

    /// Send without blocking when capacity is available.
    pub fn try_send(&self, item: T) -> Result<()> {
        self.sender.try_send(Ok(item)).map_err(|error| match error {
            mpsc::TrySendError::Full(_) => {
                Error::new(Status::QueueFull, "async stream queue is full")
            }
            mpsc::TrySendError::Disconnected(_) => {
                Error::new(Status::Closing, "async stream receiver is closed")
            }
        })
    }

    /// Send an error that will reject the next Promise.
    pub fn send_error(&self, error: E) -> Result<()> {
        self.send_result(Err(error))
    }
}

/// Pull-based bounded stream. Each `next_task` resolves to an item or `null`
/// when all senders have been dropped.
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

/// Nullable result used internally to resolve an async-iterator `next()`
/// Promise. The wrapper keeps the public ArkTS type as `T | null` while
/// avoiding coherence conflicts between primitive Promise boxing and
/// `Option<T>` conversion implementations.
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
    let (sender, receiver) = mpsc::sync_channel(capacity);
    LIVE_STREAMS.fetch_add(1, Ordering::AcqRel);
    Ok((
        StreamSender { sender },
        AsyncStream {
            inner: Arc::new(StreamInner {
                receiver: Mutex::new(receiver),
            }),
        },
    ))
}

impl<T, E> AsyncStream<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    /// Build one scheduler-backed Promise task for an iterator `next()` call.
    pub fn next_task(&self) -> AsyncTask<StreamNextTask<T, E>, AsyncIteratorValue<T>> {
        AsyncTask::new(StreamNextTask {
            inner: Arc::clone(&self.inner),
        })
    }
}

/// Task returned by [`AsyncStream::next_task`].
pub struct StreamNextTask<T, E = Error> {
    inner: Arc<StreamInner<T, E>>,
}

impl<T, E> Task for StreamNextTask<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    type Output = Option<T>;
    type JsValue = AsyncIteratorValue<T>;
    type Error = DynAniError;

    fn compute(
        &mut self,
        cancellation: &CancellationToken,
    ) -> std::result::Result<Self::Output, Self::Error> {
        loop {
            cancellation
                .check()
                .map_err(|error| -> DynAniError { Box::new(error) })?;
            let result = self
                .inner
                .receiver
                .lock()
                .map_err(|_| -> DynAniError {
                    Box::new(Error::new(
                        Status::GenericFailure,
                        "async stream lock poisoned",
                    ))
                })?
                .recv_timeout(Duration::from_millis(10));
            match result {
                Ok(item) => {
                    return item
                        .map(Some)
                        .map_err(|error| Box::new(error) as DynAniError);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(None),
            }
        }
    }

    fn resolve<'env>(
        self,
        _env: &Env<'env>,
        output: Self::Output,
    ) -> std::result::Result<Self::JsValue, Self::Error> {
        Ok(AsyncIteratorValue(output))
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
    fn stream_preserves_order_error_and_end() {
        let (sender, stream) = stream_channel(2).unwrap();
        sender.send(1).unwrap();
        sender.send(2).unwrap();
        drop(sender);
        let mut first = StreamNextTask {
            inner: Arc::clone(&stream.inner),
        };
        let token = CancellationToken::new();
        assert_eq!(first.compute(&token).unwrap(), Some(1));
        assert_eq!(first.compute(&token).unwrap(), Some(2));
        assert_eq!(first.compute(&token).unwrap(), None);
        drop(first);
        assert_eq!(Arc::strong_count(&stream.inner), 1);
        drop(stream);
    }

    #[test]
    fn nonblocking_backpressure_is_reported() {
        let (sender, stream) = stream_channel(1).unwrap();
        sender.try_send(1).unwrap();
        assert_eq!(sender.try_send(2).unwrap_err().status, Status::QueueFull);
        drop(stream);
    }

    #[test]
    fn custom_stream_error_is_not_erased() {
        let (sender, stream) = stream_channel_with_error::<i32, DomainError>(1).unwrap();
        sender.send_error(DomainError).unwrap();
        let mut next = StreamNextTask {
            inner: Arc::clone(&stream.inner),
        };
        let error = next.compute(&CancellationToken::new()).unwrap_err();
        assert_eq!(error.ani_status(), "StreamDomainError");
        assert_eq!(error.ani_code(), 73001);
        assert_eq!(error.ani_message(), "stream domain error");
    }
}
