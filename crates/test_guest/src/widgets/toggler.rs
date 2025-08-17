use iced_core::{Length, Pixels, alignment, text};

use crate::{
    bindings::iced::app::element::toggler_to_element,
    element::Widget, Element,
};

/// Togglers let users make binary choices by toggling a switch.
pub struct Toggler<Message> {
    is_toggled: bool,
    label: Option<String>,
    on_toggle: Option<Box<dyn Fn(bool) -> Message>>,
    size: Option<Pixels>,
    width: Option<Length>,
    text_size: Option<Pixels>,
    text_line_height: Option<text::LineHeight>,
    text_alignment: Option<alignment::Horizontal>,
    text_shaping: Option<text::Shaping>,
    text_wrapping: Option<text::Wrapping>,
    spacing: Option<Pixels>,
}

impl<Message> Toggler<Message> {
    /// Creates a new [`Toggler`] with the given state.
    pub fn new(is_toggled: bool) -> Self {
        Self {
            is_toggled,
            label: None,
            on_toggle: None,
            size: None,
            width: None,
            text_size: None,
            text_line_height: None,
            text_alignment: None,
            text_shaping: None,
            text_wrapping: None,
            spacing: None,
        }
    }

    /// Sets the label of the [`Toggler`].
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the message to produce when the [`Toggler`] is toggled.
    pub fn on_toggle(mut self, message: impl Fn(bool) -> Message + 'static) -> Self {
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

    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = Some(line_height.into());
        self
    }

    pub fn text_alignment(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.text_alignment = Some(alignment.into());
        self
    }

    pub fn text_shaping(mut self, shaping: impl Into<text::Shaping>) -> Self {
        self.text_shaping = Some(shaping.into());
        self
    }

    pub fn text_wrapping(mut self, wrapping: impl Into<text::Wrapping>) -> Self {
        self.text_wrapping = Some(wrapping.into());
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }
}

impl<Message: 'static> Widget<Message> for Toggler<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        toggler_to_element(&crate::bindings::iced::app::toggler::Toggler {
            is_toggled: self.is_toggled,
            label: self.label,
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
            text_size: self.text_size.map(Into::into),
            text_line_height: self.text_line_height.map(Into::into),
            text_alignment: self.text_alignment.map(Into::into),
            text_shaping: self.text_shaping.map(Into::into),
            text_wrapping: self.text_wrapping.map(Into::into),
            spacing: self.spacing.map(Into::into),
        })
    }
}

impl<Message: 'static> From<Toggler<Message>> for Element<Message> {
    fn from(toggler: Toggler<Message>) -> Self {
        Element::new(Box::new(toggler))
    }
}