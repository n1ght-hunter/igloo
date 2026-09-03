//! ABI-stable tasks returned from a plugin's `update`.
//!
//! A [`Task`] is a list of [`Step`]s. Steps whose result is already known —
//! [`Step::Output`] and [`Step::Effect`] — carry a plain value and never touch an
//! executor. Only [`Step::Future`] and [`Step::Stream`] box a future, and only
//! because their result genuinely is not ready yet.
//!
//! The host maps a [`Task`] onto an [`iced::Task`] with [`Task::into_iced`],
//! gating every [`Action`] through a [`Permissions`] policy. Nothing here
//! reimplements iced's scheduling — it only carries the work, and the capability
//! check, across the FFI boundary.
//!
//! Mapping occurs in the typed guest task before erasure. The host cannot
//! transform an opaque payload without knowing its concrete Rust type; the
//! application adapter does that work on the plugin side of the boundary.

use core::future::Future;

use stabby::boxed::Box;
use stabby::future::DynFutureUnsync;
use stabby::option::Option as Opt;
use stabby::string::String;
use stabby::vec::Vec;

#[cfg(feature = "host")]
use crate::helpers::StreamDynMut;
use crate::helpers::{BoxStream, Stream};
use crate::messages::SendEmit;
pub use crate::messages::{Emit, Message};

/// A boxed future resolving to one [`Message`], usable across the FFI boundary.
pub type BoxFuture = DynFutureUnsync<'static, Message>;

/// A plugin-supplied recipe for producing a [`Message`] from the clipboard's
/// current text, if any.
pub type EmitOptString =
    stabby::dynptr!(Box<dyn stabby::closure::Call1<Message, Opt<String>> + Send>);

/// A host capability a plugin can invoke.
///
/// Fire-and-forget: nothing comes back. To act on the result of async work, have
/// a [`Step::Future`] yield a message and return the effect from the next
/// `update`.
#[stabby::stabby]
#[repr(C, u8)]
pub enum Action {
    /// Write the given text to the system clipboard.
    WriteClipboard(String),
    /// Read the current text from the system clipboard, then turn it into a message.
    ReadClipboard(EmitOptString),
    /// Load a font from its bytes.
    LoadFont(Vec<u8>),
    /// Ask the host to close the application.
    Exit,
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::WriteClipboard(contents) => f
                .debug_struct("Action::WriteClipboard")
                .field("contents", contents)
                .finish(),
            Action::ReadClipboard(_) => f.debug_struct("Action::ReadClipboard").finish(),
            Action::LoadFont(_) => f.debug_struct("Action::LoadFont").finish(),
            Action::Exit => f.debug_struct("Action::Exit").finish(),
        }
    }
}

/// One piece of work in a [`Task`].
#[stabby::stabby]
#[repr(C, u8)]
pub enum Step {
    /// A message that is already known; feed it straight back to `update`.
    Output(Message),
    /// A host effect that is already known.
    Effect(Action),
    /// Run a future to completion and feed its [`Message`] back to `update`.
    Future(BoxFuture),
    /// Run a stream and feed each [`Message`] it yields back to `update`.
    Stream(BoxStream),
    /// Run every step of `first`, then every step of `then`, in order.
    Sequence(Vec<Step>, Vec<Step>),
    /// Wait `millis` milliseconds, then feed [`Message`] back to `update`.
    Sleep(u64, SendEmit),
}

impl core::fmt::Debug for Step {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Step::Output(message) => f.debug_tuple("Step::Output").field(message).finish(),
            Step::Effect(action) => f.debug_tuple("Step::Effect").field(action).finish(),
            Step::Future(_) => f.debug_struct("Step::Future").finish_non_exhaustive(),
            Step::Stream(_) => f.debug_struct("Step::Stream").finish_non_exhaustive(),
            Step::Sequence(first, then) => f
                .debug_tuple("Step::Sequence")
                .field(&first.len())
                .field(&then.len())
                .finish(),
            Step::Sleep(millis, _) => f.debug_tuple("Step::Sleep").field(millis).finish(),
        }
    }
}

/// A set of [`Step`]s for the host runtime to perform, returned from `update`.
#[stabby::stabby]
pub struct Task {
    steps: Vec<Step>,
}

impl Task {
    /// A [`Task`] that does nothing.
    pub fn none() -> Self {
        Self { steps: Vec::new() }
    }

    /// A [`Task`] that instantly produces `message`.
    pub fn done(message: Message) -> Self {
        Self::step(Step::Output(message))
    }

    /// A [`Task`] that performs a single host [`Action`].
    pub fn effect(action: Action) -> Self {
        Self::step(Step::Effect(action))
    }

    /// A [`Task`] that runs the given future to completion and produces its output.
    pub fn future<F>(future: F) -> Self
    where
        F: Future<Output = Message> + Send + 'static,
    {
        Self::step(Step::Future(Box::new(future).into()))
    }

    /// A [`Task`] that runs the given stream to completion and produces each of its items.
    pub fn stream<S>(stream: S) -> Self
    where
        S: futures::Stream<Item = Message> + Send + 'static,
    {
        Self::step(Step::Stream(box_stream(stream)))
    }

    /// Combines `tasks` into one [`Task`] that runs all of them in parallel.
    pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self {
        Self {
            steps: tasks.into_iter().flat_map(|task| task.steps).collect(),
        }
    }

    /// A [`Task`] that runs every step of `self`, then every step of `then`, in order.
    pub fn chain(self, then: Self) -> Self {
        Self::step(Step::Sequence(self.steps, then.steps))
    }

    /// A [`Task`] that waits `millis` milliseconds, then produces the message
    /// `on_complete` mints.
    pub fn sleep(millis: u64, on_complete: SendEmit) -> Self {
        Self::step(Step::Sleep(millis, on_complete))
    }

    /// The amount of work "units" of the [`Task`].
    pub fn units(&self) -> usize {
        self.steps.len()
    }

    fn step(step: Step) -> Self {
        let mut steps = Vec::new();
        steps.push(step);
        Self { steps }
    }

    /// Maps this task onto an [`iced::Task`], running each [`Action`] `permissions` allows.
    ///
    /// Denied actions are dropped silently.
    #[cfg(feature = "host")]
    pub fn into_iced(self, permissions: Permissions) -> iced::Task<Message> {
        let mut tasks = std::vec::Vec::new();

        for step in self.steps {
            match step {
                Step::Output(message) => tasks.push(iced::Task::done(message)),
                Step::Future(future) => tasks.push(iced::Task::future(future)),
                Step::Stream(stream) => tasks.push(iced::Task::stream(unbox(stream))),
                Step::Effect(action) => tasks.extend(dispatch(action, permissions)),
                Step::Sequence(first, then) => {
                    let first = Task { steps: first }.into_iced(permissions);
                    let then = Task { steps: then }.into_iced(permissions);
                    tasks.push(first.chain(then));
                }
                Step::Sleep(millis, on_complete) => {
                    use stabby::closure::Call0Dyn;
                    tasks.push(iced::Task::future(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
                        on_complete.call()
                    }));
                }
            }
        }

        match tasks.len() {
            0 => iced::Task::none(),
            1 => tasks.remove(0),
            _ => iced::Task::batch(tasks),
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self::none()
    }
}

impl From<()> for Task {
    fn from((): ()) -> Self {
        Self::none()
    }
}

impl core::fmt::Debug for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Task")
            .field("units", &self.steps.len())
            .finish()
    }
}

#[cfg(feature = "host")]
bitflags::bitflags! {
    /// Which host capabilities a plugin's [`Action`]s are allowed to use.
    ///
    /// An action whose flag is not set is dropped silently by [`Task::into_iced`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct Permissions: u8 {
        /// Allows [`Action::WriteClipboard`].
        const WRITE_CLIPBOARD = 1 << 0;
        /// Allows [`Action::ReadClipboard`].
        const READ_CLIPBOARD = 1 << 1;
        /// Allows [`Action::LoadFont`].
        const FONTS = 1 << 2;
        /// Allows [`Action::Exit`].
        const EXIT = 1 << 3;
    }
}

#[cfg(feature = "host")]
fn dispatch(action: Action, permissions: Permissions) -> Option<iced::Task<Message>> {
    Some(match action {
        Action::WriteClipboard(text) if permissions.contains(Permissions::WRITE_CLIPBOARD) => {
            iced::clipboard::write(std::string::String::from(text)).discard()
        }
        Action::ReadClipboard(emit) if permissions.contains(Permissions::READ_CLIPBOARD) => {
            use stabby::closure::Call1Dyn;
            iced::clipboard::read_text().map(move |contents| {
                emit.call(
                    contents
                        .ok()
                        .map(|text| String::from((*text).clone()))
                        .into(),
                )
            })
        }
        Action::LoadFont(bytes) if permissions.contains(Permissions::FONTS) => {
            iced::font::load(bytes.as_slice().to_vec()).discard()
        }
        Action::Exit if permissions.contains(Permissions::EXIT) => iced::exit(),
        _ => return None,
    })
}

#[cfg(feature = "host")]
pub(crate) fn unbox(stream: BoxStream) -> impl futures::Stream<Item = Message> + Send {
    futures::stream::unfold(Some(stream), |state| async move {
        let mut stream = state?;
        let message = Option::from(stream.next().await)?;
        Some((message, Some(stream)))
    })
}

pub(crate) fn box_stream(
    stream: impl futures::Stream<Item = Message> + Send + 'static,
) -> BoxStream {
    Box::new(Guest(std::boxed::Box::pin(stream))).into()
}

struct Guest(core::pin::Pin<std::boxed::Box<dyn futures::Stream<Item = Message> + Send>>);

impl Stream for Guest {
    extern "C" fn next<'a>(&'a mut self) -> DynFutureUnsync<'a, Opt<Message>> {
        use futures::StreamExt;
        Box::new(async move { self.0.next().await.into() }).into()
    }
}

#[cfg(all(test, feature = "host"))]
mod tests {
    use super::*;
    #[test]
    fn denied_clipboard_action_drops_callback_without_calling_it() {
        let capture = std::sync::Arc::new(());
        let state = capture.clone();
        let callback: EmitOptString = Box::new(move |_value: Opt<String>| {
            let _ = &state;
            panic!("denied clipboard callback must not run")
        })
        .into();
        assert!(dispatch(Action::ReadClipboard(callback), Permissions::empty()).is_none());
        assert_eq!(std::sync::Arc::strong_count(&capture), 1);
    }
    #[test]
    fn permissions_gate_each_host_capability() {
        assert!(dispatch(Action::WriteClipboard("text".into()), Permissions::empty()).is_none());
        assert!(
            dispatch(
                Action::WriteClipboard("text".into()),
                Permissions::WRITE_CLIPBOARD
            )
            .is_some()
        );
        assert!(dispatch(Action::LoadFont(Vec::new()), Permissions::empty()).is_none());
        assert!(dispatch(Action::LoadFont(Vec::new()), Permissions::FONTS).is_some());
        assert!(dispatch(Action::Exit, Permissions::empty()).is_none());
        assert!(dispatch(Action::Exit, Permissions::EXIT).is_some());
    }
}
