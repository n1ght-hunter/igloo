use iced_core::{Length, Pixels, text};

use crate::Element;
use crate::bindings::iced::app::widgets::{CheckboxNode, Node};

/// A box that can be checked.
pub struct Checkbox<Message> {
    is_checked: bool,
    label: Option<String>,
    on_toggle: Option<Box<dyn Fn(bool) -> Message>>,
    size: Option<f32>,
    width: Option<Length>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<text::LineHeight>,
    text_wrapping: Option<text::Wrapping>,
    text_shaping: Option<text::Shaping>,
}

impl<Message: 'static> Checkbox<Message> {
    /// Creates a new [`Checkbox`] with the given checked state.
    pub fn new(is_checked: bool) -> Self {
        Self {
            is_checked,
            label: None,
            on_toggle: None,
            size: None,
            width: None,
            spacing: None,
            text_size: None,
            text_line_height: None,
            text_wrapping: None,
            text_shaping: None,
        }
    }

    /// Sets the label of the [`Checkbox`].
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the message to produce when the [`Checkbox`] is toggled.
    pub fn on_toggle(mut self, message: impl Fn(bool) -> Message + 'static) -> Self {
        self.on_toggle = Some(Box::new(message));
        self
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

impl<Message: 'static> From<Checkbox<Message>> for Element<Message> {
    fn from(checkbox: Checkbox<Message>) -> Self {
        Element::new(move |realize, arena| {
            let node = CheckboxNode {
                is_checked: checkbox.is_checked,
                label: checkbox.label,
                on_toggle: checkbox.on_toggle.map(|f| realize.bool_mapper(f)),
                size: checkbox.size,
                width: checkbox.width.map(Into::into),
                spacing: checkbox.spacing,
                text_size: checkbox.text_size,
                text_line_height: checkbox.text_line_height.map(Into::into),
                text_wrapping: checkbox.text_wrapping.map(Into::into),
                text_shaping: checkbox.text_shaping.map(Into::into),
            };
            arena.push(Node::Checkbox(node))
        })
    }
}
