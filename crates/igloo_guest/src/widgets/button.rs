use iced_core::{Length, Padding};

use crate::Element;
use crate::bindings::iced::app::button::Button as WitButton;

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
        Element::new(move |realize| {
            let content = button.content.build(realize);
            let raw = WitButton::new(content);
            if let Some(width) = button.width {
                raw.width(width.into());
            }
            if let Some(height) = button.height {
                raw.height(height.into());
            }
            if let Some(padding) = button.padding {
                raw.padding(padding.into());
            }
            if let Some(msg) = button.on_press {
                raw.on_press(realize.fixed(msg));
            }
            if let Some(clip) = button.clip {
                raw.clip(clip);
            }
            WitButton::into_element(raw)
        })
    }
}

