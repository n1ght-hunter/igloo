use iced_core::Length;

use crate::{
    Element,
    bindings::iced::app::element::space_to_element,
    element::Widget,
};

/// An interactive bar for selecting a value from a range.
#[derive(Debug)]
pub struct Space {
    width: Length,
    height: Length,
}

impl Space {
    /// Creates an amount of empty [`Space`] with the given width and height.
    pub fn new(width: impl Into<Length>, height: impl Into<Length>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>) -> Self {
        Self {
            width: width.into(),
            height: Length::Shrink,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>) -> Self {
        Self {
            width: Length::Shrink,
            height: height.into(),
        }
    }

    /// Sets the width of the [`Space`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Space`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Space {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        space_to_element(crate::bindings::iced::app::space::Space {
            width: self.width.into(),
            height: self.height.into(),
        })
    }
}

impl<'a, Message: 'a> From<Space> for Element<Message> {
    fn from(space: Space) -> Self {
        Element::new(Box::new(space))
    }
}
