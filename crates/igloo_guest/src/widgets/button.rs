use iced_core::{Length, Padding};

use crate::Element;
use crate::bindings::iced::app::widgets::{ButtonNode, Node};

pub struct Button<Message> {
    content: Element<Message>,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    on_press: Option<Message>,
    clip: Option<bool>,
}

impl<Message: 'static> Button<Message> {
    pub fn new(element: impl Into<Element<Message>>) -> Self {
        Self {
            content: element.into(),
            width: None,
            height: None,
            padding: None,
            on_press: None,
            clip: None,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message: 'static> From<Button<Message>> for Element<Message> {
    fn from(button: Button<Message>) -> Self {
        Element::new(move |realize, arena| {
            let content = button.content.build(realize, arena);
            let node = ButtonNode {
                content,
                on_press: button.on_press.map(|msg| realize.fixed(msg)),
                width: button.width.map(Into::into),
                height: button.height.map(Into::into),
                padding: button.padding.map(Into::into),
                clip: button.clip,
            };
            arena.push(Node::Button(node))
        })
    }
}
