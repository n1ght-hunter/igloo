use std::ops::RangeInclusive;

use iced_core::Length;

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, ProgressBarNode};

/// A bar that displays progress.
pub struct ProgressBar {
    range_start: f32,
    range_end: f32,
    value: f32,
    length: Option<Length>,
    girth: Option<Length>,
    vertical: Option<bool>,
}

impl ProgressBar {
    /// Creates a new [`ProgressBar`].
    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        let (range_start, range_end) = range.into_inner();
        Self {
            range_start,
            range_end,
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

impl<Message: 'static> From<ProgressBar> for Element<Message> {
    fn from(progress_bar: ProgressBar) -> Self {
        Element::new(move |_realize, arena| {
            let node = ProgressBarNode {
                range_start: progress_bar.range_start,
                range_end: progress_bar.range_end,
                value: progress_bar.value,
                length: progress_bar.length.map(Into::into),
                girth: progress_bar.girth.map(Into::into),
                vertical: progress_bar.vertical,
            };
            arena.push(Node::ProgressBar(node))
        })
    }
}
