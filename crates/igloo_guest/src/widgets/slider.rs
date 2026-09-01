use std::ops::RangeInclusive;

use iced_core::{Length, Pixels};

use crate::Element;
use crate::bindings::iced::app::slider::Slider as WitSlider;

/// An interactive bar for selecting a value from a range.
pub struct Slider<Message> {
    range: (f32, f32),
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message>,
    on_release: Option<Message>,
    width: Option<Length>,
    height: Option<Pixels>,
    step: Option<f32>,
    shift_step: Option<f32>,
    default: Option<f32>,
}

impl<Message: 'static> Slider<Message> {
    /// Creates a new [`Slider`].
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

    /// Sets the message to produce when the [`Slider`] is released.
    pub fn on_release(mut self, message: Message) -> Self {
        self.on_release = Some(message);
        self
    }

    /// Sets the width of the [`Slider`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Slider`].
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the step size of the [`Slider`].
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Sets the step size of the [`Slider`] when the shift key is pressed.
    pub fn shift_step(mut self, step: f32) -> Self {
        self.shift_step = Some(step);
        self
    }

    /// Sets the default value of the [`Slider`].
    pub fn default(mut self, value: f32) -> Self {
        self.default = Some(value);
        self
    }
}

impl<Message: 'static> From<Slider<Message>> for Element<Message> {
    fn from(slider: Slider<Message>) -> Self {
        Element::new(move |realize| {
            let mapper = realize.f32_mapper(slider.on_change);
            let (start, end) = slider.range;
            let raw = WitSlider::new(start, end, slider.value, mapper);
            if let Some(msg) = slider.on_release {
                raw.on_release(realize.fixed(msg));
            }
            if let Some(width) = slider.width {
                raw.width(width.into());
            }
            if let Some(height) = slider.height {
                raw.height(height.0);
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
            WitSlider::into_element(raw)
        })
    }
}
