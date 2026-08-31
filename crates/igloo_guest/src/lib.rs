//! Wasm bindings for iced client

pub mod element;
pub mod mappers;
mod utils;
pub mod widgets;

pub use iced_core::*;

#[doc(hidden)]
#[allow(unsafe_code)]
pub mod bindings;

#[doc(hidden)]
pub use wit_bindgen;

pub use element::Element;

use std::cell::RefCell;

use element::Realize;
use mappers::Frame;

pub trait Application: 'static {
    type Message: 'static;

    fn new() -> Self
    where
        Self: Sized;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
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
}

impl<A: Application + 'static> ApplicationResource<A> {
    pub(crate) fn new() -> Self {
        Self {
            state: RefCell::new(A::new()),
            current: RefCell::new(Frame::new(0)),
            previous: RefCell::new(Frame::new(0)),
        }
    }

    pub(crate) fn view(&self) -> bindings::iced::app::shared::Element {
        let next = RefCell::new(Frame::new(self.current.borrow().next_base()));
        let realize = RootRealize::<A> { frame: &next };
        let element = self.state.borrow().view().build(&realize);
        self.previous.replace(self.current.replace(next.into_inner()));
        element
    }

    /// Resolves `id` against `current`, then `previous`, and dispatches the
    /// `Application::Message` it (and `value`) produce, if any. A miss — an id
    /// from further back than one generation, or a variant/callback kind
    /// mismatch — is ignored rather than treated as an error.
    pub(crate) fn update(&self, id: u32, value: bindings::exports::iced::app::app_instance::MessageValue) {
        let msg = if let Some(cb) = self.current.borrow_mut().get_mut(id) {
            Self::resolve(cb, value)
        } else if let Some(cb) = self.previous.borrow_mut().get_mut(id) {
            Self::resolve(cb, value)
        } else {
            None
        };

        if let Some(msg) = msg {
            self.state.borrow_mut().update(msg);
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

impl<A: Application + 'static> bindings::exports::iced::app::app_instance::GuestApplication
    for ApplicationResource<A>
{
    fn new() -> Self {
        ApplicationResource::new()
    }

    fn view(&self) -> bindings::iced::app::shared::Element {
        ApplicationResource::view(self)
    }

    fn update(&self, id: u32, value: bindings::exports::iced::app::app_instance::MessageValue) {
        ApplicationResource::update(self, id, value);
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

            impl igloo_guest::bindings::exports::iced::app::app_instance::Guest for GuestComponent {
                type Application = igloo_guest::ApplicationResource<super::$app>;
            }

            igloo_guest::bindings::export!(GuestComponent with_types_in igloo_guest::bindings);
        }
    };
}
