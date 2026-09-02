use iced_core::{Length, Pixels, text};

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, RadioNode};

/// A circular button representing an alternative.
pub struct Radio<Message> {
    label: String,
    is_selected: bool,
    on_select: Message,
    size: Option<f32>,
    width: Option<Length>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<text::LineHeight>,
    text_wrapping: Option<text::Wrapping>,
    text_shaping: Option<text::Shaping>,
}

impl<Message: 'static> Radio<Message> {
    /// Creates a new [`Radio`] with the given label and value.
    pub fn new<F, V>(label: impl Into<String>, value: V, selected: Option<V>, f: F) -> Self
    where
        V: Eq + Copy,
        F: FnOnce(V) -> Message,
    {
        Self {
            label: label.into(),
            is_selected: Some(value) == selected,
            on_select: f(value),
            size: None,
            width: None,
            spacing: None,
            text_size: None,
            text_line_height: None,
            text_wrapping: None,
            text_shaping: None,
        }
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }

    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into().0);
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

impl<Message: 'static> From<Radio<Message>> for Element<Message> {
    fn from(radio: Radio<Message>) -> Self {
        Element::new(move |realize, arena| {
            let node = RadioNode {
                label: radio.label,
                is_selected: radio.is_selected,
                on_select: realize.fixed(radio.on_select),
                size: radio.size,
                width: radio.width.map(Into::into),
                spacing: radio.spacing,
                text_size: radio.text_size,
                text_line_height: radio.text_line_height.map(Into::into),
                text_wrapping: radio.text_wrapping.map(Into::into),
                text_shaping: radio.text_shaping.map(Into::into),
            };
            arena.push(Node::Radio(node))
        })
    }
}
