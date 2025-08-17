use iced_core::{Length, Padding};

use crate::{Element, bindings::iced::app::element::button_to_element};

/// A generic widget that produces a message when pressed.
#[derive(Debug)]
pub struct Button<Message: Clone> {
    content: Element<Message>,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    on_press: Option<Message>,
    clip: Option<bool>,
}

impl<Message: Clone> Button<Message> {
    /// Creates a new [`Button`] with the given content.
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

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the message to produce when pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets whether the contents of the [`Button`] should be clipped on overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message: Clone> crate::element::Widget<Message> for Button<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        button_to_element(crate::bindings::iced::app::element::Button {
            content: self.content.as_element(create_message),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            padding: self.padding.map(Into::into),
            on_press: self.on_press.map(|msg| create_message.add_message(msg)),
            clip: self.clip,
        })
    }
}

impl<Message: Clone + 'static> From<Button<Message>> for Element<Message> {
    fn from(button: Button<Message>) -> Self {
        Element::new(Box::new(button))
    }
}

// impl From<Button> for Element {
//     fn from(value: Button) -> Self {
//         button_to_element(value.0)
//     }
// }
