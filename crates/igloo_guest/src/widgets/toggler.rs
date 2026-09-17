use iced_core::{Length, Pixels, alignment, text};

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, TogglerNode};

/// Togglers let users make binary choices by toggling a switch.
pub struct Toggler<Message> {
    is_toggled: bool,
    label: Option<String>,
    on_toggle: Option<Box<dyn Fn(bool) -> Message>>,
    size: Option<f32>,
    width: Option<Length>,
    text_size: Option<f32>,
    line_height: Option<text::LineHeight>,
    alignment: Option<alignment::Horizontal>,
    shaping: Option<text::Shaping>,
    wrapping: Option<text::Wrapping>,
    spacing: Option<f32>,
}

impl<Message: 'static> Toggler<Message> {
    /// Creates a new [`Toggler`] with the given state.
    pub fn new(is_toggled: bool) -> Self {
        Self {
            is_toggled,
            label: None,
            on_toggle: None,
            size: None,
            width: None,
            text_size: None,
            line_height: None,
            alignment: None,
            shaping: None,
            wrapping: None,
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
        self.size = Some(size.into().0);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
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

    pub fn alignment(mut self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.alignment = Some(alignment.into());
        self
    }

    pub fn shaping(mut self, shaping: impl Into<text::Shaping>) -> Self {
        self.shaping = Some(shaping.into());
        self
    }

    pub fn wrapping(mut self, wrapping: impl Into<text::Wrapping>) -> Self {
        self.wrapping = Some(wrapping.into());
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }
}

impl<Message: 'static> From<Toggler<Message>> for Element<Message> {
    fn from(toggler: Toggler<Message>) -> Self {
        Element::new(move |realize, arena| {
            let node = TogglerNode {
                is_toggled: toggler.is_toggled,
                label: toggler.label,
                on_toggle: toggler.on_toggle.map(|f| realize.bool_mapper(f)),
                size: toggler.size,
                width: toggler.width.map(Into::into),
                text_size: toggler.text_size,
                line_height: toggler.line_height.map(Into::into),
                alignment: toggler.alignment.map(Into::into),
                shaping: toggler.shaping.map(Into::into),
                wrapping: toggler.wrapping.map(Into::into),
                spacing: toggler.spacing,
            };
            arena.push(Node::Toggler(node))
        })
    }
}
