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
    line_height: Option<text::LineHeight>,
    wrapping: Option<text::Wrapping>,
    shaping: Option<text::Shaping>,
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
            line_height: None,
            wrapping: None,
            shaping: None,
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

    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn wrapping(mut self, wrapping: impl Into<text::Wrapping>) -> Self {
        self.wrapping = Some(wrapping.into());
        self
    }

    pub fn shaping(mut self, shaping: impl Into<text::Shaping>) -> Self {
        self.shaping = Some(shaping.into());
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
                line_height: checkbox.line_height.map(Into::into),
                wrapping: checkbox.wrapping.map(Into::into),
                shaping: checkbox.shaping.map(Into::into),
            };
            arena.push(Node::Checkbox(node))
        })
    }
}
