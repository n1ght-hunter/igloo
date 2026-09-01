use iced_core::{Length, Pixels, text};

use crate::Element;
use crate::bindings::iced::app::checkbox::Checkbox as WitCheckbox;

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
        Element::new(move |realize| {
            let raw = WitCheckbox::new(checkbox.is_checked);
            if let Some(label) = checkbox.label {
                raw.label(&label);
            }
            if let Some(on_toggle) = checkbox.on_toggle {
                raw.on_toggle(realize.bool_mapper(on_toggle));
            }
            if let Some(size) = checkbox.size {
                raw.size(size);
            }
            if let Some(width) = checkbox.width {
                raw.width(width.into());
            }
            if let Some(spacing) = checkbox.spacing {
                raw.spacing(spacing);
            }
            if let Some(text_size) = checkbox.text_size {
                raw.text_size(text_size);
            }
            if let Some(lh) = checkbox.text_line_height {
                raw.text_line_height(lh.into());
            }
            if let Some(wrapping) = checkbox.text_wrapping {
                raw.text_wrapping(wrapping.into());
            }
            if let Some(shaping) = checkbox.text_shaping {
                raw.text_shaping(shaping.into());
            }
            WitCheckbox::into_element(raw)
        })
    }
}
