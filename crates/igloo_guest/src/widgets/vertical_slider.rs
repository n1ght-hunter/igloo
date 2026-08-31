use std::ops::RangeInclusive;

use iced_core::{Length, Pixels};

use crate::Element;
use crate::bindings::iced::app::vertical_slider::VerticalSlider as WitVerticalSlider;

/// An interactive vertical bar for selecting a value from a range.
pub struct VerticalSlider<Message> {
    range: (f32, f32),
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message>,
    on_release: Option<Message>,
    width: Option<Pixels>,
    height: Option<Length>,
    step: Option<f32>,
    shift_step: Option<f32>,
    default: Option<f32>,
}

impl<Message: 'static> VerticalSlider<Message> {
    /// Creates a new [`VerticalSlider`].
    pub fn new(
        range: RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
    ) -> Self {
        Self {
            range: range.into_inner(),
            value,
            on_change: Box::new(on_change),
            on_release: None,
            width: None,
            height: None,
            step: None,
            shift_step: None,
            default: None,
        }
    }

    /// Sets the message to produce when the [`VerticalSlider`] is released.
    pub fn on_release(mut self, message: Message) -> Self {
        self.on_release = Some(message);
        self
    }

    /// Sets the width of the [`VerticalSlider`].
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`VerticalSlider`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the step size of the [`VerticalSlider`].
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Sets the step size of the [`VerticalSlider`] when the shift key is pressed.
    pub fn shift_step(mut self, step: f32) -> Self {
        self.shift_step = Some(step);
        self
    }

    /// Sets the default value of the [`VerticalSlider`].
    pub fn default(mut self, value: f32) -> Self {
        self.default = Some(value);
        self
    }
}

impl<Message: 'static> From<VerticalSlider<Message>> for Element<Message> {
    fn from(slider: VerticalSlider<Message>) -> Self {
        Element::new(move |realize| {
            let mapper = realize.f32_mapper(slider.on_change);
            let (start, end) = slider.range;
            let raw = WitVerticalSlider::new(start, end, slider.value, mapper);
            if let Some(msg) = slider.on_release {
                raw.on_release(realize.fixed(msg));
            }
            if let Some(width) = slider.width {
                raw.width(width.0);
            }
            if let Some(height) = slider.height {
                raw.height(height.into());
            }
            if let Some(step) = slider.step {
                raw.step(step);
            }
            if let Some(step) = slider.shift_step {
                raw.shift_step(step);
            }
            if let Some(value) = slider.default {
                raw.default(value);
            }
            WitVerticalSlider::into_element(raw)
        })
    }
}
