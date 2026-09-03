//! Lazy guest work. Only an exported job polls a task; composition has no effects.

use crate::bindings::exports::iced::app::app_instance::JobError;
use crate::bindings::iced::app::capabilities;
use futures::{Stream, StreamExt, stream::LocalBoxStream};
use std::future::Future;

pub use capabilities::CapabilityError;
type Output<M> = Result<M, JobError>;
pub(crate) type Factory<M> = Box<dyn FnOnce() -> LocalBoxStream<'static, Output<M>>>;

/// Work executed once by the host through an owned job.
pub struct Task<Message> {
    pub(crate) make: Option<Factory<Message>>,
}

impl<M> std::fmt::Debug for Task<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("empty", &self.make.is_none())
            .finish()
    }
}

impl<M: 'static> Task<M> {
    pub(crate) fn new(make: impl FnOnce() -> LocalBoxStream<'static, Output<M>> + 'static) -> Self {
        Self {
            make: Some(Box::new(make)),
        }
    }

    pub(crate) fn into_stream(self) -> LocalBoxStream<'static, Output<M>> {
        match self.make {
            Some(make) => make(),
            None => futures::stream::empty().boxed_local(),
        }
    }

    pub fn none() -> Self {
        Self { make: None }
    }
    pub fn done(message: M) -> Self {
        Self::new(move || futures::stream::once(async move { Ok(message) }).boxed_local())
    }

    pub fn perform<T: 'static>(
        future: impl Future<Output = T> + 'static,
        map: impl FnOnce(T) -> M + 'static,
    ) -> Self {
        Self::new(move || futures::stream::once(async move { Ok(map(future.await)) }).boxed_local())
    }

    pub fn run<T: 'static>(
        stream: impl Stream<Item = T> + 'static,
        map: impl Fn(T) -> M + 'static,
    ) -> Self {
        Self::new(move || stream.map(move |value| Ok(map(value))).boxed_local())
    }

    pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self {
        let tasks: Vec<_> = tasks.into_iter().collect();
        Self::new(move || {
            futures::stream::select_all(tasks.into_iter().map(Self::into_stream)).boxed_local()
        })
    }

    /// Starts the next task only after this task's output and effects finish.
    pub fn chain(self, next: Self) -> Self {
        Self::new(move || {
            self.into_stream()
                .chain(futures::stream::once(async move { next.into_stream() }).flatten())
                .boxed_local()
        })
    }

    pub fn map<B: 'static>(self, map: impl Fn(M) -> B + 'static) -> Task<B> {
        Task::new(move || {
            self.into_stream()
                .map(move |value| value.map(&map))
                .boxed_local()
        })
    }

    pub fn sleep(delay: std::time::Duration, message: M) -> Self {
        Self::new(move || {
            futures::stream::once(async move {
                capabilities::sleep(crate::duration(delay))
                    .await
                    .map(|()| message)
                    .map_err(failed)
            })
            .boxed_local()
        })
    }

    pub fn read_clipboard(
        map: impl FnOnce(Result<Option<String>, CapabilityError>) -> M + 'static,
    ) -> Self {
        Self::perform(capabilities::read_clipboard(), map)
    }

    pub fn write_clipboard(contents: impl Into<String>) -> Self {
        let contents = contents.into();
        Self::effect(async move { capabilities::write_clipboard(contents).await })
    }

    pub fn exit() -> Self {
        Self::effect(capabilities::exit())
    }

    fn effect(future: impl Future<Output = Result<(), CapabilityError>> + 'static) -> Self {
        Self::new(move || {
            futures::stream::once(async move { future.await.err().map(failed) })
                .filter_map(|error| async move { error.map(Err) })
                .boxed_local()
        })
    }
}

pub(crate) fn failed(error: CapabilityError) -> JobError {
    JobError::Failed(format!("{error:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn chain_does_not_poll_second_future_until_first_stream_finishes() {
        let started = Rc::new(Cell::new(false));
        let observed = started.clone();
        let task = Task::run(futures::stream::iter([1, 2]), |value| value).chain(Task::perform(
            async move {
                observed.set(true);
                3
            },
            |value| value,
        ));
        assert!(!started.get());
        let mut stream = task.into_stream();
        assert!(!started.get());
        futures::executor::block_on(async {
            assert_eq!(stream.next().await.unwrap().unwrap(), 1);
            assert!(!started.get());
            assert_eq!(stream.next().await.unwrap().unwrap(), 2);
            assert!(!started.get());
            assert_eq!(stream.next().await.unwrap().unwrap(), 3);
            assert!(started.get());
            assert!(stream.next().await.is_none());
        });
    }

    #[test]
    fn nested_mapping_covers_batch_and_chain() {
        let task = Task::batch([Task::done(1), Task::done(2)])
            .chain(Task::done(3))
            .map(|value| value * 2)
            .map(|value| value + 1);
        let values = futures::executor::block_on(task.into_stream().collect::<Vec<_>>());
        let mut values: Vec<_> = values.into_iter().map(Result::unwrap).collect();
        assert_eq!(values.pop(), Some(7));
        values.sort();
        assert_eq!(values, [3, 5]);
    }

    #[test]
    fn dropping_unpolled_work_releases_captured_state() {
        let value = Rc::new(());
        let captured = value.clone();
        let task = Task::perform(
            async move {
                drop(captured);
                1
            },
            |value| value,
        );
        assert_eq!(Rc::strong_count(&value), 2);
        drop(task);
        assert_eq!(Rc::strong_count(&value), 1);
    }
}
