use iced_core::Length;

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, SpaceNode};

/// An amount of empty space.
pub struct Space {
    width: Option<Length>,
    height: Option<Length>,
}

impl Default for Space {
    fn default() -> Self {
        Self::new()
    }
}

impl Space {
    /// Creates an amount of empty [`Space`] with no size.
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
        }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>) -> Self {
        Self::new().width(width)
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>) -> Self {
        Self::new().height(height)
    }

    /// Sets the width of the [`Space`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Space`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }
}

impl<Message: 'static> From<Space> for Element<Message> {
    fn from(space: Space) -> Self {
        Element::new(move |_realize, arena| {
            arena.push(Node::Space(SpaceNode {
                width: space.width.map(Into::into),
                height: space.height.map(Into::into),
            }))
        })
    }
}
