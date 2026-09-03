//! A bar that displays progress.

use stabby::option::Option as Opt;

use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::Length;

/// A bar that displays progress.
#[stabby::stabby]
#[derive(Debug)]
pub struct ProgressBarNode {
    pub range_start: f32,
    pub range_end: f32,
    pub value: f32,
    pub length: Opt<Length>,
    pub girth: Opt<Length>,
    pub vertical: Opt<bool>,
}

#[cfg(feature = "host")]
pub(crate) fn build(bar: ProgressBarNode) -> iced::Element<'static, crate::widgets::Emitted> {
    let mut widget = iced::widget::ProgressBar::new(bar.range_start..=bar.range_end, bar.value);
    if let Some(length) = Option::<Length>::from(bar.length) {
        widget = widget.length(iced::Length::from(length));
    }
    if let Some(girth) = Option::<Length>::from(bar.girth) {
        widget = widget.girth(iced::Length::from(girth));
    }
    if let Some(vertical) = Option::<bool>::from(bar.vertical) {
        widget = widget.vertical(vertical);
    }
    widget.into()
}

/// A builder for a [`ProgressBarNode`].
#[derive(Debug)]
pub struct ProgressBar {
    range_start: f32,
    range_end: f32,
    value: f32,
    length: Option<Length>,
    girth: Option<Length>,
    vertical: Option<bool>,
}

impl ProgressBar {
    /// Creates a new progress bar over `range_start..=range_end`, at `value`.
    pub fn new(range_start: f32, range_end: f32, value: f32) -> Self {
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
    fn from(bar: ProgressBar) -> Self {
        Element::new(move |_realize: &dyn Realize<Message>, arena: &mut Arena| {
            arena.push(Node::ProgressBar(ProgressBarNode {
                range_start: bar.range_start,
                range_end: bar.range_end,
                value: bar.value,
                length: bar.length.into(),
                girth: bar.girth.into(),
                vertical: bar.vertical.into(),
            }))
        })
    }
}

/// Creates a [`ProgressBar`] over `range_start..=range_end`, at `value`.
pub fn progress_bar(range_start: f32, range_end: f32, value: f32) -> ProgressBar {
    ProgressBar::new(range_start, range_end, value)
}
