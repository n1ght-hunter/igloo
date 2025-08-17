use std::sync::Arc;

use iced_core::Color;

use crate::bindings::{MessageId, iced::app::element::explain};

pub type MessageFunction<Message> =
    Box<dyn Fn(crate::bindings::iced::app::message::Message) -> Option<Message>>;

pub trait CreateMessage<Message> {
    fn add_message_func(&self, message: MessageFunction<Message>) -> MessageId;

    fn add_message(&self, message: Message) -> MessageId;
}

pub trait Widget<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn CreateMessage<Message>,
    ) -> crate::bindings::Element;
}

pub struct Element<Message> {
    widget: Box<dyn Widget<Message>>,
}

impl<Message> std::fmt::Debug for Element<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element").finish()
    }
}

impl<Message> Element<Message> {
    pub fn new(widget: Box<dyn Widget<Message>>) -> Self {
        Self { widget }
    }

    pub fn as_element(
        self,
        create_message: &dyn CreateMessage<Message>,
    ) -> crate::bindings::Element {
        self.widget.as_element(create_message)
    }

    pub fn explain(self, color: Color) -> Element<Message>
    where
        Message: 'static,
    {
        Element::new(Box::new(Explain {
            color,
            widget: self.widget,
        }))
    }
}

impl<Message: 'static> Element<Message> {
    pub fn map<B: 'static>(self, f: impl Fn(Message) -> B + 'static) -> Element<B> {
        Element::new(Box::new(Mapper::new(self.widget, f)))
    }
}

struct Explain<Message> {
    color: Color,
    widget: Box<dyn Widget<Message>>,
}

impl<Message> Widget<Message> for Explain<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn CreateMessage<Message>,
    ) -> crate::bindings::Element {
        explain(self.widget.as_element(create_message), self.color.into())
    }
}

struct Mapper<MessageA, MessageB> {
    widget: Box<dyn Widget<MessageA>>,
    mapper: Arc<dyn Fn(MessageA) -> MessageB>,
}

impl<MessageA, MessageB> Mapper<MessageA, MessageB> {
    pub fn new<F>(widget: Box<dyn Widget<MessageA>>, mapper: F) -> Self
    where
        F: Fn(MessageA) -> MessageB + 'static,
    {
        Self {
            widget,
            mapper: Arc::new(mapper),
        }
    }
}

impl<MessageA: 'static, MessageB: 'static> Widget<MessageB> for Mapper<MessageA, MessageB> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn CreateMessage<MessageB>,
    ) -> crate::bindings::Element {
        let create_message = MapperMessageCreator::new(self.mapper, create_message);
        self.widget.as_element(&create_message)
    }
}

struct MapperMessageCreator<'a, MessageA, MessageB> {
    mapper: Arc<dyn Fn(MessageA) -> MessageB>,
    create_message: &'a dyn CreateMessage<MessageB>,
}

impl<'a, MessageA, MessageB> MapperMessageCreator<'a, MessageA, MessageB> {
    pub fn new(
        mapper: Arc<dyn Fn(MessageA) -> MessageB>,
        create_message: &'a dyn CreateMessage<MessageB>,
    ) -> Self {
        Self {
            mapper,
            create_message,
        }
    }
}

impl<'a, MessageA: 'static, MessageB: 'static> CreateMessage<MessageA>
    for MapperMessageCreator<'a, MessageA, MessageB>
{
    fn add_message_func(&self, message: MessageFunction<MessageA>) -> MessageId {
        let mapper = Arc::clone(&self.mapper);
        let mapped_message = Box::new(move |msg| message(msg).map(|msg| (mapper)(msg)));

        // Here we would use the mapper to transform the message before adding it.
        self.create_message.add_message_func(mapped_message)
    }

    fn add_message(&self, message: MessageA) -> MessageId {
        // Here we would use the mapper to transform the message before adding it.
        self.create_message.add_message((self.mapper)(message))
    }
}
