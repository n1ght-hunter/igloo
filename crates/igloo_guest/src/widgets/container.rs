use iced_core::{
    Length, Padding, Pixels,
    alignment::{Horizontal, Vertical},
};

use crate::{
    Element,
    bindings::iced::app::{self, element::container_to_element},
    element::Widget,
};

/// A box that contains another element.
#[derive(Debug)]
pub struct Container<Message> {
    content: Element<Message>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<Pixels>,
    max_height: Option<Pixels>,
    align_x: Option<Horizontal>,
    align_y: Option<Vertical>,
    clip: Option<bool>,
    center_x: Option<Length>,
    center_y: Option<Length>,
    center: Option<Length>,
    align_left: Option<Length>,
    align_right: Option<Length>,
    align_top: Option<Length>,
    align_bottom: Option<Length>,
}

impl<Message> Container<Message> {
    /// Creates a [`Container`] with the given content.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Container {
            content: content.into(),
            padding: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            align_x: None,
            align_y: None,
            clip: None,
            center_x: None,
            center_y: None,
            center: None,
            align_left: None,
            align_right: None,
            align_top: None,
            align_bottom: None,
        }
    }

    /// Sets the [`Padding`] of the [`Container`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the width of the [`Container`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Container`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the maximum width of the [`Container`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    /// Sets the maximum height of the [`Container`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    /// Sets the width of the [`Container`] and centers its contents horizontally.
    pub fn center_x(mut self, width: impl Into<Length>) -> Self {
        self.center_x = Some(width.into());
        self
    }

    /// Sets the height of the [`Container`] and centers its contents vertically.
    pub fn center_y(mut self, height: impl Into<Length>) -> Self {
        self.center_y = Some(height.into());
        self
    }

    /// Centers the contents in both the horizontal and vertical axes of the [`Container`].
    ///
    /// This is equivalent to chaining [`center_x`](Self::center_x) and [`center_y`](Self::center_y).
    pub fn center(mut self, length: impl Into<Length>) -> Self {
        self.center = Some(length.into());
        self
    }

    /// Aligns the contents of the [`Container`] to the left.
    pub fn align_left(mut self, width: impl Into<Length>) -> Self {
        self.align_left = Some(width.into());
        self
    }

    /// Aligns the contents of the [`Container`] to the right.
    pub fn align_right(mut self, width: impl Into<Length>) -> Self {
        self.align_right = Some(width.into());
        self
    }

    /// Aligns the contents of the [`Container`] to the top.
    pub fn align_top(mut self, height: impl Into<Length>) -> Self {
        self.align_top = Some(height.into());
        self
    }

    /// Aligns the contents of the [`Container`] to the bottom.
    pub fn align_bottom(mut self, height: impl Into<Length>) -> Self {
        self.align_bottom = Some(height.into());
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Container`].
    pub fn align_x(mut self, align: impl Into<Horizontal>) -> Self {
        self.align_x = Some(align.into());
        self
    }

    /// Sets the vertical alignment of the contents of the [`Container`].
    pub fn align_y(mut self, align: impl Into<Vertical>) -> Self {
        self.align_y = Some(align.into());
        self
    }

    /// Sets whether the contents of the [`Container`] should be clipped on overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message> Widget<Message> for Container<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        container_to_element(app::element::Container {
            content: self.content.as_element(create_message),
            padding: self.padding.map(Into::into),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            max_width: self.max_width.map(Into::into),
            max_height: self.max_height.map(Into::into),
            align_x: self.align_x.map(Into::into),
            align_y: self.align_y.map(Into::into),
            clip: self.clip,
            center_x: self.center_x.map(Into::into),
            center_y: self.center_y.map(Into::into),
            center: self.center.map(Into::into),
            align_left: self.align_left.map(Into::into),
            align_right: self.align_right.map(Into::into),
            align_top: self.align_top.map(Into::into),
            align_bottom: self.align_bottom.map(Into::into),
        })
    }
}

impl<Message: 'static> From<Container<Message>> for Element<Message> {
    fn from(container: Container<Message>) -> Self {
        Element::new(Box::new(container))
    }
}
