//! The guest view of an `iced::Task`.
//!
//! Like [`crate::Element`], a `Task` is a deferred builder: it records what the
//! plugin asked for and only turns into host resources in
//! [`crate::ApplicationResource`], where the concrete `Application::Message` is
//! known. Each message a task will emit is stashed in that resource's pending
//! registry through the shared [`Realize`] trait and handed to the host as an
//! opaque id, exactly as widget callbacks are.

use std::rc::Rc;

use crate::bindings::iced::app::task::Task as WitTask;
use crate::element::{MapRealize, Realize};

/// A deferred `iced::Task` whose message callbacks are not yet registered with
/// the host.
#[allow(missing_debug_implementations)]
pub struct Task<Message> {
    build: Box<dyn FnOnce(&dyn Realize<Message>) -> WitTask>,
}

impl<Message: 'static> Task<Message> {
    fn new(build: impl FnOnce(&dyn Realize<Message>) -> WitTask + 'static) -> Self {
        Self {
            build: Box::new(build),
        }
    }

    pub(crate) fn build(self, realize: &dyn Realize<Message>) -> WitTask {
        (self.build)(realize)
    }

    /// A task that does nothing.
    pub fn none() -> Self {
        Self::new(|_| WitTask::none())
    }

    /// Emit `msg` immediately.
    pub fn done(msg: Message) -> Self {
        Self::new(move |realize| WitTask::done(realize.fixed(msg)))
    }

    /// Emit `on_complete` after `millis` milliseconds, without blocking the UI.
    pub fn sleep(millis: u64, on_complete: Message) -> Self {
        Self::new(move |realize| WitTask::sleep(millis, realize.fixed(on_complete)))
    }

    /// Read the system clipboard, then emit the message `on_complete` maps its
    /// contents to (empty string when the clipboard is empty).
    pub fn read_clipboard(on_complete: impl Fn(String) -> Message + 'static) -> Self {
        Self::new(move |realize| {
            WitTask::read_clipboard(realize.string_mapper(Box::new(on_complete)))
        })
    }

    /// Write `contents` to the system clipboard.
    pub fn write_clipboard(contents: impl Into<String>) -> Self {
        let contents = contents.into();
        Self::new(move |_| WitTask::write_clipboard(&contents))
    }

    /// Close the host application.
    pub fn exit() -> Self {
        Self::new(|_| WitTask::exit())
    }

    /// Run several tasks concurrently.
    pub fn batch(tasks: impl IntoIterator<Item = Task<Message>>) -> Self {
        let tasks: Vec<Task<Message>> = tasks.into_iter().collect();
        Self::new(move |realize| {
            let handles: Vec<WitTask> = tasks.into_iter().map(|task| task.build(realize)).collect();
            WitTask::batch(handles)
        })
    }

    /// Run `self` to completion, then `next`.
    pub fn chain(self, next: Task<Message>) -> Self {
        Self::new(move |realize| {
            let first = self.build(realize);
            let next = next.build(realize);
            first.chain(next)
        })
    }

    /// Transform the messages this task emits.
    pub fn map<B: 'static>(self, f: impl Fn(Message) -> B + 'static) -> Task<B> {
        let f: Rc<dyn Fn(Message) -> B> = Rc::new(f);
        Task::new(move |realize: &dyn Realize<B>| {
            let adapter = MapRealize { inner: realize, f };
            self.build(&adapter)
        })
    }
}
