//! Wasm bindings for iced client

pub mod arena;
pub mod element;
pub mod mappers;
pub mod task;
mod utils;
pub mod widgets;

pub use iced_core::*;

#[doc(hidden)]
#[allow(unsafe_code)]
pub mod bindings;

#[doc(hidden)]
pub use wit_bindgen;

pub use element::Element;
pub use task::Task;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use element::Realize;
use mappers::{Callback, Frame};

type WitTask = bindings::iced::app::task::Task;

/// Ids for task-emitted messages carry this bit, keeping them clear of the
/// frame-scoped widget callback ids (which count up from zero).
const TASK_ID_BIT: u32 = 1 << 31;

pub trait Application: 'static {
    type Message: 'static;

    /// Build the initial state and the task to run on startup.
    fn new() -> (Self, Task<Self::Message>)
    where
        Self: Sized;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;
}

/// The [`Realize`] implementation used at the root of a view tree, where the
/// concrete `Application::Message` is known and every callback can be pushed
/// into the frame being built.
struct RootRealize<'a, A: Application> {
    frame: &'a RefCell<Frame<A::Message>>,
}

impl<A: Application + 'static> Realize<A::Message> for RootRealize<'_, A> {
    fn fixed(&self, msg: A::Message) -> u32 {
        self.frame.borrow_mut().push_fixed(msg)
    }

    fn bool_mapper(&self, f: Box<dyn Fn(bool) -> A::Message>) -> u32 {
        self.frame.borrow_mut().push_bool(f)
    }

    fn f32_mapper(&self, f: Box<dyn Fn(f32) -> A::Message>) -> u32 {
        self.frame.borrow_mut().push_f32(f)
    }

    fn f64_mapper(&self, f: Box<dyn Fn(f64) -> A::Message>) -> u32 {
        self.frame.borrow_mut().push_f64(f)
    }

    fn u64_mapper(&self, f: Box<dyn Fn(u64) -> A::Message>) -> u32 {
        self.frame.borrow_mut().push_u64(f)
    }

    fn string_mapper(&self, f: Box<dyn Fn(String) -> A::Message>) -> u32 {
        self.frame.borrow_mut().push_string(f)
    }

    fn viewport_mapper(
        &self,
        f: Box<dyn Fn(bindings::iced::app::message_types::Viewport) -> A::Message>,
    ) -> u32 {
        self.frame.borrow_mut().push_viewport(f)
    }
}

/// The `application` resource: owns the plugin's state for as long as the
/// plugin lives, replacing the old process-global `StateManager`.
///
/// `current` and `previous` are the last two frames of callbacks minted by
/// `view()`. Keeping two generations covers a dispatch that races a
/// re-render; anything older misses instead of resolving to a stale or wrong
/// callback. Dropping the outgoing `previous` on each rotation is what frees
/// the closures — the fix for the old design's per-frame leak.
#[allow(missing_debug_implementations)]
pub struct ApplicationResource<A: Application> {
    state: RefCell<A>,
    current: RefCell<Frame<A::Message>>,
    previous: RefCell<Frame<A::Message>>,
    /// Callbacks for the messages a task will emit, keyed by the id handed to
    /// the host. Removed on delivery; a miss (late or duplicate) is ignored,
    /// like a stale widget callback.
    pending: RefCell<HashMap<u32, Callback<A::Message>>>,
    next_task_id: Cell<u32>,
    /// The startup task, taken by the first `boot` call.
    boot: RefCell<Option<Task<A::Message>>>,
}

impl<A: Application + 'static> ApplicationResource<A> {
    pub(crate) fn new() -> Self {
        let (state, boot) = A::new();
        Self {
            state: RefCell::new(state),
            current: RefCell::new(Frame::new(0)),
            previous: RefCell::new(Frame::new(0)),
            pending: RefCell::new(HashMap::new()),
            next_task_id: Cell::new(0),
            boot: RefCell::new(Some(boot)),
        }
    }

    pub(crate) fn view(&self) -> bindings::iced::app::widgets::ViewTree {
        let mut arena = arena::Arena::new();
        let next = RefCell::new(Frame::new(self.current.borrow().next_base()));
        let realize = RootRealize::<A> { frame: &next };
        let root = self.state.borrow().view().build(&realize, &mut arena);
        self.previous
            .replace(self.current.replace(next.into_inner()));
        bindings::iced::app::widgets::ViewTree {
            root,
            nodes: arena.into_nodes(),
        }
    }

    pub(crate) fn boot(&self) -> WitTask {
        match self.boot.borrow_mut().take() {
            Some(task) => self.realize_task(task),
            None => WitTask::none(),
        }
    }

    fn realize_task(&self, task: Task<A::Message>) -> WitTask {
        let realize = PendingRealize::<A> { app: self };
        task.build(&realize)
    }

    /// Stashes `cb` in the pending registry under a fresh task-scoped id.
    fn register_pending(&self, cb: Callback<A::Message>) -> u32 {
        let raw = self.next_task_id.get();
        self.next_task_id.set(raw.wrapping_add(1) & !TASK_ID_BIT);
        let id = TASK_ID_BIT | raw;
        self.pending.borrow_mut().insert(id, cb);
        id
    }

    /// Resolves `id` against `current`, then `previous`, and dispatches the
    /// `Application::Message` it (and `value`) produce, if any. A miss — an id
    /// from further back than one generation, or a variant/callback kind
    /// mismatch — is ignored rather than treated as an error.
    pub(crate) fn update(
        &self,
        id: u32,
        value: bindings::exports::iced::app::app_instance::MessageValue,
    ) -> WitTask {
        let msg = if id & TASK_ID_BIT != 0 {
            self.pending
                .borrow_mut()
                .remove(&id)
                .and_then(|mut cb| Self::resolve(&mut cb, value))
        } else if let Some(cb) = self.current.borrow_mut().get_mut(id) {
            Self::resolve(cb, value)
        } else if let Some(cb) = self.previous.borrow_mut().get_mut(id) {
            Self::resolve(cb, value)
        } else {
            None
        };

        match msg {
            Some(msg) => {
                let task = self.state.borrow_mut().update(msg);
                self.realize_task(task)
            }
            None => WitTask::none(),
        }
    }

    /// Matches a callback against the value the interaction produced, calling it
    /// (or taking it, for `Fixed`) if the kinds line up. A mismatch — which
    /// should not happen, since the host always pairs an id with the value kind
    /// the widget that minted it expects — is treated as a miss, not a panic.
    fn resolve(
        cb: &mut mappers::Callback<A::Message>,
        value: bindings::exports::iced::app::app_instance::MessageValue,
    ) -> Option<A::Message> {
        use bindings::exports::iced::app::app_instance::MessageValue;
        use mappers::Callback;

        match (cb, value) {
            (Callback::Fixed(slot), MessageValue::Fixed) => slot.take(),
            (Callback::Bool(f), MessageValue::BoolValue(v)) => Some(f(v)),
            (Callback::F32(f), MessageValue::F32Value(v)) => Some(f(v)),
            (Callback::F64(f), MessageValue::F64Value(v)) => Some(f(v)),
            (Callback::U64(f), MessageValue::U64Value(v)) => Some(f(v)),
            (Callback::Str(f), MessageValue::StringValue(v)) => Some(f(v)),
            (Callback::Viewport(f), MessageValue::ViewportValue(v)) => Some(f(v)),
            _ => None,
        }
    }
}

/// The [`Realize`] implementation used at the root of a task tree. Unlike
/// [`RootRealize`], a task's output can land many frames after it was built, so
/// its callbacks go into the resource's frame-independent pending registry
/// under task-scoped ids rather than into a frame.
struct PendingRealize<'a, A: Application> {
    app: &'a ApplicationResource<A>,
}

impl<A: Application + 'static> Realize<A::Message> for PendingRealize<'_, A> {
    fn fixed(&self, msg: A::Message) -> u32 {
        self.app.register_pending(Callback::Fixed(Some(msg)))
    }

    fn bool_mapper(&self, f: Box<dyn Fn(bool) -> A::Message>) -> u32 {
        self.app.register_pending(Callback::Bool(f))
    }

    fn f32_mapper(&self, f: Box<dyn Fn(f32) -> A::Message>) -> u32 {
        self.app.register_pending(Callback::F32(f))
    }

    fn f64_mapper(&self, f: Box<dyn Fn(f64) -> A::Message>) -> u32 {
        self.app.register_pending(Callback::F64(f))
    }

    fn u64_mapper(&self, f: Box<dyn Fn(u64) -> A::Message>) -> u32 {
        self.app.register_pending(Callback::U64(f))
    }

    fn string_mapper(&self, f: Box<dyn Fn(String) -> A::Message>) -> u32 {
        self.app.register_pending(Callback::Str(f))
    }

    fn viewport_mapper(
        &self,
        f: Box<dyn Fn(bindings::iced::app::message_types::Viewport) -> A::Message>,
    ) -> u32 {
        self.app.register_pending(Callback::Viewport(f))
    }
}

impl<A: Application + 'static> bindings::exports::iced::app::app_instance::GuestApplication
    for ApplicationResource<A>
{
    fn new() -> Self {
        ApplicationResource::new()
    }

    fn boot(&self) -> WitTask {
        ApplicationResource::boot(self)
    }

    fn view(&self) -> bindings::iced::app::widgets::ViewTree {
        ApplicationResource::view(self)
    }

    fn update(
        &self,
        id: u32,
        value: bindings::exports::iced::app::app_instance::MessageValue,
    ) -> WitTask {
        ApplicationResource::update(self, id, value)
    }
}

/// Export macro — generates the WIT component glue for a guest application.
#[macro_export]
macro_rules! export_guest {
    ($app:ident) => {
        #[doc(hidden)]
        #[allow(missing_debug_implementations, unsafe_code)]
        mod guest_impl {
            #[doc(hidden)]
            #[allow(missing_debug_implementations)]
            pub struct GuestComponent;

            impl $crate::bindings::exports::iced::app::app_instance::Guest for GuestComponent {
                type Application = $crate::ApplicationResource<super::$app>;
            }

            $crate::bindings::export!(GuestComponent with_types_in $crate::bindings);
        }
    };
}
