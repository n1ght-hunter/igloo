use std::ops::RangeInclusive;

use iced_core::{Length, Pixels};

use crate::{
    Element,
    bindings::iced::app::element::vertical_slider_to_element,
    element::Widget,
};

/// An interactive vertical bar for selecting a value from a range.
pub struct VerticalSlider<Message> {
    range: RangeInclusive<f32>,
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message>,
    default: Option<f32>,
    on_release: Option<Message>,
    width: Option<Pixels>,
    height: Option<Length>,
    step: Option<f32>,
    shift_step: Option<f32>,
}

impl<Message> VerticalSlider<Message> {
    /// Creates a new [`VerticalSlider`].
    pub fn new(
        range: RangeInclusive<f32>,
        value: f32,
        on_change: impl Fn(f32) -> Message + 'static,
    ) -> Self {
        Self {
            range,
            value,
            on_change: Box::new(on_change),
            default: None,
            on_release: None,
            width: None,
            height: None,
            step: None,
            shift_step: None,
        }
    }

    /// Sets the message to produce when the [`Slider`] is released.
    pub fn on_release(mut self, message: Message) -> Self {
        self.on_release = Some(message);
        self
    }

    /// Sets the width of the [`Slider`].
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Slider`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
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

impl<Message: Clone + 'static> Widget<Message> for VerticalSlider<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        let (range_start, range_end) = self.range.into_inner();
        let f = self.on_change;
        vertical_slider_to_element(
            crate::bindings::iced::app::vertical_slider::VerticalSlider {
                range_start,
                range_end,
                value: self.value,
                on_change: create_message.add_message_func(Box::new(move |msg| {
                    if let crate::bindings::Message::F32Type(value) = msg {
                        Some(f(value))
                    } else {
                        None
                    }
                })),
                default: self.default,
                on_release: if let Some(message) = self.on_release {
                    Some(create_message.add_message(message))
                } else {
                    None
                },
                width: self.width.map(Into::into),
                height: self.height.map(Into::into),
                step: self.step,
                shift_step: self.shift_step,
            },
        )
    }
}

impl<'a, Message: Clone + 'static> From<VerticalSlider<Message>> for Element<Message> {
    fn from(value: VerticalSlider<Message>) -> Self {
        Element::new(Box::new(value))
    }
}
