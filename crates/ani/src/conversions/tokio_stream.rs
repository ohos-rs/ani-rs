//! Bridge `tokio-stream` / `futures_core::Stream` with ani-rs pull streams.

use std::future::poll_fn;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio_stream::Stream;

use crate::async_runtime::{RuntimeTask, RuntimeTaskHandle, spawn_runtime_task};
use crate::error::{AniErrorPayload, Error, Result};

use super::{
    AsyncIteratorValue, AsyncStream, OhosReadableSource, PromiseValue, StreamSender, Uint8Array,
    stream_channel_with_error,
};

/// A [`tokio_stream::Stream`] view of an [`AsyncStream`].
///
/// Each `poll_next` waits for the next queued item with the same FIFO waiter
/// protocol as [`AsyncStream::recv`]. End-of-stream is `None`; item, terminal,
/// and cancellation failures are `Some(Err(_))`.
pub struct TokioAsyncStream<T, E = Error> {
    stream: AsyncStream<T, E>,
    pending: Option<super::RecvFuture<T, E>>,
}

impl<T, E> TokioAsyncStream<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    /// Wrap an existing pull stream.
    pub fn new(stream: AsyncStream<T, E>) -> Self {
        Self {
            stream,
            pending: None,
        }
    }

    /// Returns the underlying pull stream.
    pub fn inner(&self) -> &AsyncStream<T, E> {
        &self.stream
    }

    /// Unwraps the pull stream. Any in-flight `poll_next` waiter is dropped.
    pub fn into_inner(self) -> AsyncStream<T, E> {
        self.stream
    }
}

impl<T, E> From<AsyncStream<T, E>> for TokioAsyncStream<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    fn from(stream: AsyncStream<T, E>) -> Self {
        Self::new(stream)
    }
}

impl<T, E> Stream for TokioAsyncStream<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let future = this.pending.get_or_insert_with(|| this.stream.recv());
        match Pin::new(future).poll(cx) {
            Poll::Ready(Ok(Some(item))) => {
                this.pending = None;
                Poll::Ready(Some(Ok(item)))
            }
            Poll::Ready(Ok(None)) => {
                this.pending = None;
                Poll::Ready(None)
            }
            Poll::Ready(Err(error)) => {
                this.pending = None;
                Poll::Ready(Some(Err(error)))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T, E> AsyncStream<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    /// Consume this pull stream as a [`tokio_stream::Stream`].
    pub fn into_tokio_stream(self) -> TokioAsyncStream<T, E> {
        TokioAsyncStream::new(self)
    }

    /// Clone a [`tokio_stream::Stream`] view that shares the same channel.
    pub fn tokio_stream(&self) -> TokioAsyncStream<T, E> {
        TokioAsyncStream::new(self.clone())
    }
}

impl<T, E> StreamSender<T, E>
where
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    /// Forward `source` into this sender until it ends, errors, or the receiver
    /// is closed.
    ///
    /// `Ok(item)` is written with backpressure. `Err(error)` terminates the
    /// stream with that payload. A closed receiver stops the pump.
    pub async fn send_from_stream<S>(&self, source: S) -> Result<()>
    where
        S: Stream<Item = std::result::Result<T, E>>,
    {
        pump_stream(source, self.clone()).await
    }
}

/// Pump a Tokio/futures stream into a new bounded [`AsyncStream`].
pub fn spawn_stream<S, T, E>(source: S, capacity: usize) -> Result<AsyncStream<T, E>>
where
    S: Stream<Item = std::result::Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    spawn_stream_with_handle(source, capacity).map(|(stream, _handle)| stream)
}

/// Pump a Tokio/futures stream and return a cancellation handle.
pub fn spawn_stream_with_handle<S, T, E>(
    source: S,
    capacity: usize,
) -> Result<(AsyncStream<T, E>, RuntimeTaskHandle)>
where
    S: Stream<Item = std::result::Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    spawn_stream_factory_with_handle(move || source, capacity)
}

/// Build a `!Send` stream on the runtime thread and pump it into [`AsyncStream`].
pub fn spawn_stream_factory<Build, S, T, E>(
    build: Build,
    capacity: usize,
) -> Result<AsyncStream<T, E>>
where
    Build: FnOnce() -> S + Send + 'static,
    S: Stream<Item = std::result::Result<T, E>> + 'static,
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    spawn_stream_factory_with_handle(build, capacity).map(|(stream, _handle)| stream)
}

/// Factory form of [`spawn_stream_with_handle`].
pub fn spawn_stream_factory_with_handle<Build, S, T, E>(
    build: Build,
    capacity: usize,
) -> Result<(AsyncStream<T, E>, RuntimeTaskHandle)>
where
    Build: FnOnce() -> S + Send + 'static,
    S: Stream<Item = std::result::Result<T, E>> + 'static,
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    let (sender, stream) = stream_channel_with_error(capacity)?;
    let reject_sender = sender.clone();
    let (task, _handle) = RuntimeTask::new(
        move || async move {
            let _ = pump_stream(build(), sender).await;
        },
        move |error| {
            reject_sender.close_with_payload(error);
        },
    );
    let handle = spawn_runtime_task(task)?;
    Ok((stream, handle))
}

/// Pump a byte stream into an OpenHarmony readable source.
pub fn spawn_ohos_readable_from_stream<S, B, E>(
    source: S,
    capacity: usize,
) -> Result<OhosReadableSource<E>>
where
    S: Stream<Item = std::result::Result<B, E>> + Send + 'static,
    B: Into<Uint8Array> + Send + 'static,
    E: AniErrorPayload,
{
    spawn_stream(
        MappedByteStream {
            inner: Box::pin(source),
        },
        capacity,
    )
}

struct MappedByteStream<S> {
    inner: Pin<Box<S>>,
}

impl<S, B, E> Stream for MappedByteStream<S>
where
    S: Stream<Item = std::result::Result<B, E>>,
    B: Into<Uint8Array>,
{
    type Item = std::result::Result<Uint8Array, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut().inner.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(bytes))) => Poll::Ready(Some(Ok(bytes.into()))),
            Poll::Ready(Some(Err(error))) => Poll::Ready(Some(Err(error))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn pump_stream<S, T, E>(source: S, sender: StreamSender<T, E>) -> Result<()>
where
    S: Stream<Item = std::result::Result<T, E>>,
    T: Send + 'static,
    E: AniErrorPayload,
    AsyncIteratorValue<T>: for<'env> PromiseValue<'env>,
{
    let mut source = std::pin::pin!(source);
    loop {
        let next = poll_fn(|cx| Stream::poll_next(source.as_mut(), cx)).await;
        match next {
            Some(Ok(item)) => sender.send_async(item).await?,
            Some(Err(error)) => {
                sender.send_error(error)?;
                return Ok(());
            }
            None => return Ok(()),
        }
    }
}

#[cfg(all(test, feature = "tokio_rt"))]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;

    #[test]
    fn spawn_stream_delivers_items_in_order() {
        let source = tokio_stream::iter([Ok(1), Ok(2), Ok(3)]);
        let stream = spawn_stream::<_, i32, Error>(source, 2).unwrap();
        let items = crate::async_runtime::block_on_future_result(async move {
            let mut values = Vec::new();
            let mut stream = stream.into_tokio_stream();
            while let Some(item) = stream.next().await {
                values.push(item?);
            }
            Ok::<_, Error>(values)
        })
        .unwrap()
        .unwrap();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn spawn_stream_cancel_rejects_pending_recv() {
        let (stream, handle) =
            spawn_stream_with_handle::<_, i32, Error>(tokio_stream::pending::<Result<i32>>(), 1)
                .unwrap();
        handle.cancel(crate::async_runtime::RuntimeCancelReason::Explicit(
            "stop".into(),
        ));
        let result =
            crate::async_runtime::block_on_future_result(async move { stream.recv().await })
                .unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn spawn_stream_error_rejects_recv() {
        let source = tokio_stream::iter([
            Ok(1),
            Err(Error::new(crate::error::Status::GenericFailure, "boom")),
        ]);
        let stream = spawn_stream::<_, i32, Error>(source, 1).unwrap();
        let result = crate::async_runtime::block_on_future_result(async move {
            assert_eq!(stream.recv().await?, Some(1));
            stream.recv().await
        })
        .unwrap();
        assert!(result.is_err());
    }

    #[test]
    fn send_from_stream_respects_backpressure() {
        let (sender, stream) = stream_channel_with_error::<i32, Error>(1).unwrap();
        let source = tokio_stream::iter([Ok(1), Ok(2), Ok(3)]);
        let items = crate::async_runtime::block_on_future_result(async move {
            let pump = async move { sender.send_from_stream(source).await };
            let collect = async move {
                let mut values = Vec::new();
                while let Some(item) = stream.recv().await? {
                    values.push(item);
                }
                Ok::<_, Error>(values)
            };
            let (pump_result, values) = join2(pump, collect).await;
            pump_result?;
            values
        })
        .unwrap()
        .unwrap();
        assert_eq!(items, vec![1, 2, 3]);
    }

    async fn join2<A, B, RA, RB>(left: A, right: B) -> (RA, RB)
    where
        A: std::future::Future<Output = RA>,
        B: std::future::Future<Output = RB>,
    {
        let mut left = std::pin::pin!(left);
        let mut right = std::pin::pin!(right);
        let mut left_out = None;
        let mut right_out = None;
        poll_fn(|cx| {
            if left_out.is_none()
                && let Poll::Ready(value) = left.as_mut().poll(cx)
            {
                left_out = Some(value);
            }
            if right_out.is_none()
                && let Poll::Ready(value) = right.as_mut().poll(cx)
            {
                right_out = Some(value);
            }
            if left_out.is_some() && right_out.is_some() {
                Poll::Ready((left_out.take().unwrap(), right_out.take().unwrap()))
            } else {
                Poll::Pending
            }
        })
        .await
    }
}
