//! Rust guest bindings for the iced component API.

pub mod arena;
pub mod element;
mod job;
pub mod keyboard;
pub mod mappers;
pub mod subscription;
pub mod task;
pub mod time;
mod utils;
pub mod widgets;

pub use iced_core::*;
#[doc(hidden)]
#[allow(unsafe_code)]
pub mod bindings;
pub use element::Element;
#[doc(hidden)]
pub use job::JobResource;
pub use subscription::Subscription;
pub use task::Task;
#[doc(hidden)]
pub use wit_bindgen;

use bindings::exports::iced::app::app_instance::{
    self as api, Event as JobEvent, GuestApplication, Job,
};
use element::Realize;
use mappers::Frame;
use std::{cell::RefCell, rc::Rc};

pub trait Application: 'static {
    type Message: 'static;
    fn new() -> (Self, Task<Self::Message>)
    where
        Self: Sized;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

pub(crate) fn duration(value: std::time::Duration) -> bindings::iced::app::capabilities::Duration {
    bindings::iced::app::capabilities::Duration {
        seconds: value.as_secs(),
        nanoseconds: value.subsec_nanos(),
    }
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

/// Owns application state, widget callbacks and the messages its jobs emit.
pub struct ApplicationResource<A: Application> {
    state: RefCell<A>,
    current: RefCell<Frame<A::Message>>,
    previous: RefCell<Frame<A::Message>>,
    pending: Rc<job::Registry<A::Message>>,
    boot: RefCell<Option<Task<A::Message>>>,
}

impl<A: Application> std::fmt::Debug for ApplicationResource<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationResource")
            .finish_non_exhaustive()
    }
}

impl<A: Application> ApplicationResource<A> {
    fn new() -> Self {
        let (state, boot) = A::new();
        Self {
            state: RefCell::new(state),
            current: RefCell::new(Frame::new(0)),
            previous: RefCell::new(Frame::new(0)),
            pending: Rc::new(job::Registry::default()),
            boot: RefCell::new(Some(boot)),
        }
    }
    fn realize_task(&self, task: Task<A::Message>) -> Option<Job> {
        task.make
            .map(|make| Job::new(JobResource::new(make, self.pending.clone())))
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

    fn subscription(&self) -> Vec<api::Subscription> {
        self.state
            .borrow()
            .subscription()
            .entries
            .into_iter()
            .filter_map(|entry| {
                self.realize_task(entry.task).map(|job| api::Subscription {
                    key: entry.key,
                    revision: entry.revision,
                    job,
                })
            })
            .collect()
    }
    fn boot(&self) -> Option<Job> {
        self.boot
            .borrow_mut()
            .take()
            .and_then(|task| self.realize_task(task))
    }
    fn update(&self, event: JobEvent) -> Option<Job> {
        let message = match event {
            JobEvent::Message(id) => self.pending.take(id),
            JobEvent::Callback(event) => {
                if let Some(callback) = self.current.borrow_mut().get_mut(event.id) {
                    Self::resolve(callback, event.value)
                } else if let Some(callback) = self.previous.borrow_mut().get_mut(event.id) {
                    Self::resolve(callback, event.value)
                } else {
                    None
                }
            }
        };
        message.and_then(|message| {
            let task = self.state.borrow_mut().update(message);
            self.realize_task(task)
        })
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

impl<A: Application> GuestApplication for ApplicationResource<A> {
    fn new() -> Self {
        Self::new()
    }
    fn boot(&self) -> Option<Job> {
        self.boot()
    }
    fn view(&self) -> bindings::iced::app::widgets::ViewTree {
        self.view()
    }
    fn update(&self, event: JobEvent) -> Option<Job> {
        self.update(event)
    }
    fn subscription(&self) -> Vec<api::Subscription> {
        self.subscription()
    }
}

/// Exports the application and its owned jobs as a component.
#[macro_export]
macro_rules! export_guest {
    ($app:ident) => {
        #[doc(hidden)]
        #[allow(missing_debug_implementations, unsafe_code)]
        mod guest_impl {
            pub struct GuestComponent;
            impl $crate::bindings::exports::iced::app::app_instance::Guest for GuestComponent {
                type Application = $crate::ApplicationResource<super::$app>;
                type Job = $crate::JobResource<<super::$app as $crate::Application>::Message>;
            }
            $crate::bindings::export!(GuestComponent with_types_in $crate::bindings);
        }
    };
}
