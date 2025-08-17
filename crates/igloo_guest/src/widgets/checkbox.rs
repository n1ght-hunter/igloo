use iced_core::{Length, Pixels, text};

use crate::{
    Element,
    bindings::iced::app::element::checkbox_to_element,
    element::{CreateMessage, Widget},
};

/// A box that can be checked.
pub struct Checkbox<Message> {
    label: String,
    is_checked: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + Send + Sync>>,
    size: Option<Pixels>,
    width: Option<Length>,
    height: Option<Length>,
    spacing: Option<Pixels>,
    text_size: Option<Pixels>,
    text_line_height: Option<text::LineHeight>,
    text_wrapping: Option<text::Wrapping>,
    text_shaping: Option<text::Shaping>,
}

impl<Message> Checkbox<Message> {
    /// Creates a new [`Checkbox`] with the given label and state.
    pub fn new(label: impl Into<String>, is_checked: bool) -> Self {
        Self {
            label: label.into(),
            is_checked,
            on_toggle: None,
            size: None,
            width: None,
            height: None,
            spacing: None,
            text_size: None,
            text_line_height: None,
            text_wrapping: None,
            text_shaping: None,
        }
    }

    /// Sets the message to produce when the [`Checkbox`] is toggled.
    pub fn on_toggle(mut self, message: impl Fn(bool) -> Message + Send + Sync + 'static) -> Self {
        self.on_toggle = Some(Box::new(message));
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = Some(line_height.into());
        self
    }

    pub fn text_wrapping(mut self, wrapping: impl Into<text::Wrapping>) -> Self {
        self.text_wrapping = Some(wrapping.into());
        self
    }

    pub fn text_shaping(mut self, shaping: impl Into<text::Shaping>) -> Self {
        self.text_shaping = Some(shaping.into());
        self
    }
}

impl<Message: 'static> Widget<Message> for Checkbox<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn CreateMessage<Message>,
    ) -> crate::bindings::Element {
        checkbox_to_element(&crate::bindings::iced::app::element::Checkbox {
            label: self.label,
            is_checked: self.is_checked,
            on_toggle: self.on_toggle.map(|f| {
                create_message.add_message_func(Box::new(move |msg| {
                    if let crate::bindings::Message::BoolType(value) = msg {
                        Some(f(value))
                    } else {
                        None
                    }
                }))
            }),
            size: self.size.map(Into::into),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            spacing: self.spacing.map(Into::into),
            text_size: self.text_size.map(Into::into),
            text_line_height: self.text_line_height.map(Into::into),
            text_wrapping: self.text_wrapping.map(Into::into),
            text_shaping: self.text_shaping.map(Into::into),
        })
    }
}

impl<Message: Clone + 'static> From<Checkbox<Message>> for Element<Message> {
    fn from(checkbox: Checkbox<Message>) -> Self {
        Element::new(Box::new(checkbox))
    }
}
