//! An amount of empty space.

use stabby::option::Option as Opt;

use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::Length;

/// An amount of empty space.
#[stabby::stabby]
#[derive(Debug)]
pub struct SpaceNode {
    pub width: Opt<Length>,
    pub height: Opt<Length>,
}

#[cfg(feature = "host")]
pub(crate) fn build(space: SpaceNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let mut widget = iced::widget::Space::new();
    if let Some(width) = Option::<Length>::from(space.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(space.height) {
        widget = widget.height(iced::Length::from(height));
    }
    widget
}

/// A builder for a [`SpaceNode`].
#[derive(Debug, Default)]
pub struct Space {
    width: Option<Length>,
    height: Option<Length>,
}

impl Space {
    /// Creates an amount of empty space with no size.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }
}

impl<Message: 'static> From<Space> for Element<Message> {
    fn from(space: Space) -> Self {
        Element::new(move |_realize: &dyn Realize<Message>, arena: &mut Arena| {
            arena.push(Node::Space(SpaceNode {
                width: space.width.into(),
                height: space.height.into(),
            }))
        })
    }
}

/// Creates an amount of empty [`Space`].
pub fn space() -> Space {
    Space::new()
}
