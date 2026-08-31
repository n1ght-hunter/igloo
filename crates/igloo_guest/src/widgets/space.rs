use iced_core::Length;

use crate::Element;
use crate::bindings::iced::app::space::Space as WitSpace;

/// An amount of empty space.
pub struct Space {
    raw: WitSpace,
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
            raw: WitSpace::new(),
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
    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    /// Sets the height of the [`Space`].
    pub fn height(self, height: impl Into<Length>) -> Self {
        self.raw.height(height.into().into());
        self
    }
}

impl<Message: 'static> From<Space> for Element<Message> {
    fn from(space: Space) -> Self {
        Element::new(move |_realize| WitSpace::into_element(space.raw))
    }
}
