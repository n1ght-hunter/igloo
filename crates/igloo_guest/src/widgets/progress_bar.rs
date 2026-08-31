use std::ops::RangeInclusive;

use iced_core::Length;

use crate::Element;
use crate::bindings::iced::app::progress_bar::ProgressBar as WitProgressBar;

/// A bar that displays progress.
pub struct ProgressBar {
    raw: WitProgressBar,
}

impl ProgressBar {
    /// Creates a new [`ProgressBar`].
    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        let (range_start, range_end) = range.into_inner();
        Self {
            raw: WitProgressBar::new(range_start, range_end, value),
        }
    }

    pub fn length(self, length: impl Into<Length>) -> Self {
        self.raw.length(length.into().into());
        self
    }

    pub fn girth(self, girth: impl Into<Length>) -> Self {
        self.raw.girth(girth.into().into());
        self
    }

    pub fn vertical(self, vertical: bool) -> Self {
        self.raw.vertical(vertical);
        self
    }
}

impl<Message: 'static> From<ProgressBar> for Element<Message> {
    fn from(progress_bar: ProgressBar) -> Self {
        Element::new(move |_realize| WitProgressBar::into_element(progress_bar.raw))
    }
}
