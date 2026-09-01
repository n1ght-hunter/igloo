use iced_core::{Length, Pixels, alignment, text};

use crate::Element;
use crate::bindings::iced::app::toggler::Toggler as WitToggler;

/// Togglers let users make binary choices by toggling a switch.
pub struct Toggler<Message> {
    raw: WitToggler,
    on_toggle: Option<Box<dyn Fn(bool) -> Message>>,
}

impl<Message: 'static> Toggler<Message> {
    /// Creates a new [`Toggler`] with the given state.
    pub fn new(is_toggled: bool) -> Self {
        Self {
            raw: WitToggler::new(is_toggled),
            on_toggle: None,
        }
    }

    /// Sets the label of the [`Toggler`].
    pub fn label(self, label: impl Into<String>) -> Self {
        self.raw.label(&label.into());
        self
    }

    /// Sets the message to produce when the [`Toggler`] is toggled.
    pub fn on_toggle(mut self, message: impl Fn(bool) -> Message + 'static) -> Self {
        self.on_toggle = Some(Box::new(message));
        self
    }

    pub fn size(self, size: impl Into<Pixels>) -> Self {
        self.raw.size(size.into().0);
        self
    }

    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    pub fn text_size(self, size: impl Into<Pixels>) -> Self {
        self.raw.text_size(size.into().0);
        self
    }

    pub fn text_line_height(self, line_height: impl Into<text::LineHeight>) -> Self {
        self.raw.text_line_height(line_height.into().into());
        self
    }

    pub fn text_alignment(self, alignment: impl Into<alignment::Horizontal>) -> Self {
        self.raw.text_alignment(alignment.into().into());
        self
    }

    pub fn text_shaping(self, shaping: impl Into<text::Shaping>) -> Self {
        self.raw.text_shaping(shaping.into().into());
        self
    }

    pub fn text_wrapping(self, wrapping: impl Into<text::Wrapping>) -> Self {
        self.raw.text_wrapping(wrapping.into().into());
        self
    }

    pub fn spacing(self, spacing: impl Into<Pixels>) -> Self {
        self.raw.spacing(spacing.into().0);
        self
    }
}

impl<Message: 'static> From<Toggler<Message>> for Element<Message> {
    fn from(toggler: Toggler<Message>) -> Self {
        Element::new(move |realize| {
            if let Some(on_toggle) = toggler.on_toggle {
                toggler.raw.on_toggle(realize.bool_mapper(on_toggle));
            }
            WitToggler::into_element(toggler.raw)
        })
    }
}
