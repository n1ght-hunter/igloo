//! Host capabilities used by guest-owned jobs.

use crate::{
    bindings::iced::app::capabilities::{self, CapabilityError, Duration, KeyEvent},
    plugin_manager::MyState,
};
use futures::StreamExt;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::{SystemTime, UNIX_EPOCH},
};
use wasmtime::{
    StoreContextMut,
    component::{Accessor, Destination, StreamProducer, StreamReader, StreamResult},
};

fn duration(value: Duration, nonzero: bool) -> Result<std::time::Duration, CapabilityError> {
    if value.nanoseconds >= 1_000_000_000 {
        return Err(CapabilityError::InvalidArgument(
            "nanoseconds must be below 1e9".into(),
        ));
    }
    let value = std::time::Duration::new(value.seconds, value.nanoseconds);
    if tokio::time::Instant::now().checked_add(value).is_none() {
        return Err(CapabilityError::InvalidArgument(
            "duration exceeds clock range".into(),
        ));
    }
    if nonzero && value.is_zero() {
        return Err(CapabilityError::InvalidArgument(
            "interval must be positive".into(),
        ));
    }
    Ok(value)
}

pub(crate) struct Producer<T> {
    pub stream: futures::stream::BoxStream<'static, T>,
    pub cleanup: Option<Box<dyn FnOnce() + Send>>,
}
impl<T> Drop for Producer<T> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}
impl<T: wasmtime::component::ComponentType + Send + Unpin + 'static> StreamProducer<MyState>
    for Producer<T>
{
    type Item = T;
    type Buffer = Option<T>;
    fn poll_produce<'a>(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _store: StoreContextMut<'a, MyState>,
        mut destination: Destination<'a, T, Option<T>>,
        finish: bool,
    ) -> Poll<wasmtime::Result<StreamResult>> {
        match self.stream.poll_next_unpin(cx) {
            Poll::Ready(Some(value)) => {
                destination.set_buffer(Some(value));
                Poll::Ready(Ok(StreamResult::Completed))
            }
            Poll::Ready(None) => Poll::Ready(Ok(StreamResult::Dropped)),
            Poll::Pending if finish => Poll::Ready(Ok(StreamResult::Cancelled)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl capabilities::Host for MyState {}
impl capabilities::HostWithStore<MyState> for wasmtime::component::HasSelf<MyState> {
    async fn sleep(
        _access: &Accessor<MyState, Self>,
        delay: Duration,
    ) -> wasmtime::Result<Result<(), CapabilityError>> {
        match duration(delay, false) {
            Ok(delay) => {
                tokio::time::sleep(delay).await;
                Ok(Ok(()))
            }
            Err(error) => Ok(Err(error)),
        }
    }
    async fn read_clipboard(
        access: &Accessor<MyState, Self>,
    ) -> wasmtime::Result<Result<Option<String>, CapabilityError>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        access.with(|mut store| {
            store
                .data_mut()
                .effects
                .unbounded_send(crate::plugin_manager::Effect::Read(tx))
        })?;
        Ok(rx
            .await
            .unwrap_or(Err(CapabilityError::Failed("host closed".into()))))
    }
    async fn write_clipboard(
        access: &Accessor<MyState, Self>,
        contents: String,
    ) -> wasmtime::Result<Result<(), CapabilityError>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        access.with(|mut store| {
            store
                .data_mut()
                .effects
                .unbounded_send(crate::plugin_manager::Effect::Write(contents, tx))
        })?;
        Ok(rx
            .await
            .unwrap_or_else(|_| Err(CapabilityError::Failed("host closed".into()))))
    }
    async fn exit(
        access: &Accessor<MyState, Self>,
    ) -> wasmtime::Result<Result<(), CapabilityError>> {
        let (tx, rx) = futures::channel::oneshot::channel();
        access.with(|mut store| {
            store
                .data_mut()
                .effects
                .unbounded_send(crate::plugin_manager::Effect::Exit(tx))
        })?;
        Ok(rx
            .await
            .unwrap_or_else(|_| Err(CapabilityError::Failed("host closed".into()))))
    }
    fn timer(
        mut access: wasmtime::component::Access<'_, MyState>,
        interval: Duration,
    ) -> wasmtime::Result<Result<StreamReader<u64>, CapabilityError>> {
        let interval = match duration(interval, true) {
            Ok(interval) => interval,
            Err(error) => return Ok(Err(error)),
        };
        let recipe = iced::advanced::subscription::into_recipes(iced::time::every(interval))
            .pop()
            .expect("timer recipe");
        let stream = recipe
            .stream(futures::stream::pending().boxed())
            .map(|_| {
                u64::try_from(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis(),
                )
                .unwrap_or(u64::MAX)
            })
            .boxed();
        Ok(Ok(StreamReader::new(
            &mut access,
            Producer {
                stream,
                cleanup: None,
            },
        )?))
    }
    fn keyboard(
        mut access: wasmtime::component::Access<'_, MyState>,
        event: KeyEvent,
    ) -> wasmtime::Result<Result<StreamReader<String>, CapabilityError>> {
        let (tx, rx) = futures::channel::mpsc::channel(1);
        let keys = access.data_mut().keys.clone();
        keys.lock().unwrap().push((event, tx.clone()));
        let cleanup = Box::new(move || {
            keys.lock()
                .unwrap()
                .retain(|(_, sender)| !sender.same_receiver(&tx));
        });
        Ok(Ok(StreamReader::new(
            &mut access,
            Producer {
                stream: rx.boxed(),
                cleanup: Some(cleanup),
            },
        )?))
    }
}
