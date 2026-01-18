use iced_core::Length;

use crate::{
    Element,
    bindings::iced::app::element::space_to_element,
    element::Widget,
};

/// An amount of empty space.
#[derive(Debug, Default)]
pub struct Space {
    width: Option<Length>,
    height: Option<Length>,
}

impl Space {
    /// Creates an amount of empty [`Space`] with no size.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>) -> Self {
        Self {
            width: Some(width.into()),
            height: None,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>) -> Self {
        Self {
            width: None,
            height: Some(height.into()),
        }
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

impl<'a, Message> Widget<Message> for Space {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        space_to_element(crate::bindings::iced::app::space::Space {
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
        })
    }
}

impl<'a, Message: 'a> From<Space> for Element<Message> {
    fn from(space: Space) -> Self {
        Element::new(Box::new(space))
    }
}
