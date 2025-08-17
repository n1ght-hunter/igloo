use std::ops::RangeInclusive;

use iced_core::Length;

use crate::{Element, bindings::iced::app::element::progress_bar_to_element, element::Widget};

/// A bar that displays progress.
#[derive(Debug)]
pub struct ProgressBar {
    range: RangeInclusive<f32>,
    value: f32,
    length: Option<Length>,
    girth: Option<Length>,
    vertical: Option<bool>,
}

impl ProgressBar {
    /// Creates a new [`ProgressBar`].
    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        Self {
            range,
            value,
            length: None,
            girth: None,
            vertical: None,
        }
    }

    pub fn length(mut self, length: impl Into<Length>) -> Self {
        self.length = Some(length.into());
        self
    }

    pub fn girth(mut self, girth: impl Into<Length>) -> Self {
        self.girth = Some(girth.into());
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = Some(vertical);
        self
    }
}

impl<'a, Message> Widget<Message> for ProgressBar {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        let (range_start, range_end) = self.range.into_inner();
        progress_bar_to_element(crate::bindings::iced::app::progress_bar::ProgressBar {
            range_start,
            range_end,
            value: self.value,
            length: self.length.map(Into::into),
            girth: self.girth.map(Into::into),
            vertical: self.vertical,
        })
    }
}

impl<'a, Message: 'a> From<ProgressBar> for Element<Message> {
    fn from(progress_bar: ProgressBar) -> Self {
        Element::new(Box::new(progress_bar))
    }
}
