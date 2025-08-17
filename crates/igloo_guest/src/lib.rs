//! Wasm bindings for iced client

pub mod element;
pub(crate) mod utils;
pub mod widgets;

pub use iced_core::*;

#[doc(hidden)]
#[allow(unsafe_code)]
pub mod bindings;

use std::{cell::RefCell, collections::HashMap};

pub use element::Element;

use crate::{bindings::MessageId, element::MessageFunction};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct StateManager<State, Message>
where
    State: Application<State, Message>,
{
    message_manager: MessageManager<Message>,
    state: RefCell<State>,
}

impl<State, Message> StateManager<State, Message>
where
    State: Application<State, Message>,
{
    pub fn new() -> Self {
        Self {
            message_manager: MessageManager::new(),
            state: RefCell::new(State::new()),
        }
    }

    pub fn view(&self) -> bindings::Element {
        let state = self.state.borrow();
        let view = state.view();
        let element = view.as_element(&self.message_manager);
        element
    }

    pub fn update(&self, message_id: u64, message: bindings::iced::app::message::Message) {
        let message = self.message_manager.get_message(message_id, message);
        if let Some(message) = message {
            self.state.borrow_mut().update(message);
        }
    }
}

pub trait Application<State, Message> {
    fn new() -> Self
    where
        Self: Sized;
    fn view(&self) -> Element<Message>;
    fn update(&mut self, message: Message);
}

struct MessageManager<Message>(std::sync::Mutex<MessageManagerInner<Message>>);

enum MessageTypes<Message> {
    Raw(Message),
    Function(MessageFunction<Message>),
}

struct MessageManagerInner<Message> {
    messages: HashMap<u64, MessageTypes<Message>>,
    id: u64,
}

impl<Message> MessageManager<Message> {
    const STARTING_ID: u64 = 1;

    fn new() -> Self {
        Self(std::sync::Mutex::new(MessageManagerInner {
            messages: HashMap::new(),
            id: Self::STARTING_ID,
        }))
    }

    fn new_id(current_id: u64) -> u64 {
        current_id.checked_add(1).unwrap_or(Self::STARTING_ID)
    }

    fn get_message(
        &self,
        id: MessageId,
        message: bindings::iced::app::message::Message,
    ) -> Option<Message> {
        let mut inner = self.0.lock().unwrap();
        let message_type = inner.messages.remove(&id)?;
        match message_type {
            MessageTypes::Raw(msg) => Some(msg),
            MessageTypes::Function(func) => func(message),
        }
    }
}

impl<Message> element::CreateMessage<Message> for MessageManager<Message> {
    fn add_message_func(&self, message: MessageFunction<Message>) -> MessageId {
        let mut inner = self.0.lock().unwrap();
        let id = inner.id;
        inner.messages.insert(id, MessageTypes::Function(message));
        inner.id = Self::new_id(inner.id);
        id
    }

    fn add_message(&self, message: Message) -> MessageId {
        let mut inner = self.0.lock().unwrap();
        let id = inner.id;
        inner.messages.insert(id, MessageTypes::Raw(message));
        inner.id = Self::new_id(inner.id);
        id
    }
}

#[macro_export]
macro_rules! export_guest {
    (
        $state:ident,
        $message:ident
    ) => {

        #[doc(hidden)]
        #[allow(missing_debug_implementations, unsafe_code)]
        mod guest_impl {
            type State = super::$state;
            type Message = super::$message;

            #[doc(hidden)]
            #[allow(missing_debug_implementations)]
            pub struct GuestComponent;

            impl igloo_guest::bindings::exports::iced::app::message::Guest for GuestComponent {
                #[allow(async_fn_in_trait)]
                fn clone_message(
                    message: igloo_guest::bindings::MessageId,
                ) -> igloo_guest::bindings::MessageId {
                    message
                }
            }

            static STATE: std::cell::LazyCell<igloo_guest::StateManager<State, Message>> = std::cell::LazyCell::new(|| igloo_guest::StateManager::<State, Message>::new());

            impl igloo_guest::bindings::Guest for GuestComponent {
                fn view() -> igloo_guest::bindings::Element {
                     STATE.view()
                }

                #[allow(async_fn_in_trait)]
                fn update(
                    message_id: u64,
                    message: igloo_guest::bindings::iced::app::message::Message,
                ) {
                    STATE.update(message_id, message);
                }
            }

            igloo_guest::bindings::export!(GuestComponent with_types_in igloo_guest::bindings);
    }
    };
}
