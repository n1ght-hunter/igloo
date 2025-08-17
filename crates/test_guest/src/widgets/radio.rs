use iced_core::{Length, Pixels, text};

use crate::{
    Element,
    bindings::iced::app::element::radio_to_element,
    element::Widget,
};

/// A circular button representing an alternative.
pub struct Radio<Message> {
    on_click: Message,
    is_selected: bool,
    label: String,
    width: Option<Length>,
    size: Option<Pixels>,
    spacing: Option<Pixels>,
    text_size: Option<Pixels>,
    text_line_height: Option<text::LineHeight>,
    text_shaping: Option<text::Shaping>,
    text_wrapping: Option<text::Wrapping>,
}

impl<Message> Radio<Message> {
    /// Creates a new [`Radio`] with the given label and value.
    pub fn new<F, V>(label: impl Into<String>, value: V, selected: Option<V>, f: F) -> Self
    where
        V: Eq + Copy,
        F: FnOnce(V) -> Message,
    {
        Self {
            is_selected: Some(value) == selected,
            on_click: f(value),
            label: label.into(),
            width: None,
            size: None,
            spacing: None,
            text_size: None,
            text_line_height: None,
            text_wrapping: None,
            text_shaping: None,
        }
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
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

impl<Message: Clone> Widget<Message> for Radio<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        radio_to_element(&crate::bindings::iced::app::radio::Radio {
            label: self.label,
            is_selected: self.is_selected,
            on_select: create_message.add_message(self.on_click),
            size: self.size.map(Into::into),
            width: self.width.map(Into::into),
            spacing: self.spacing.map(Into::into),
            text_size: self.text_size.map(Into::into),
            text_line_height: self.text_line_height.map(Into::into),
            text_wrapping: self.text_wrapping.map(Into::into),
            text_shaping: self.text_shaping.map(Into::into),
        })
    }
}

impl<Message: Clone + 'static> From<Radio<Message>> for Element<Message> {
    fn from(radio: Radio<Message>) -> Self {
        Element::new(Box::new(radio))
    }
}
