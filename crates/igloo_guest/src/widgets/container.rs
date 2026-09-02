use iced_core::{
    Length, Padding, Pixels,
    alignment::{Horizontal, Vertical},
};

use crate::Element;
use crate::bindings::iced::app::widgets::{ContainerNode, Node};

/// A box that contains another element.
pub struct Container<Message> {
    content: Element<Message>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    center_x: Option<Length>,
    center_y: Option<Length>,
    center: Option<Length>,
    align_left: Option<Length>,
    align_right: Option<Length>,
    align_top: Option<Length>,
    align_bottom: Option<Length>,
    align_x: Option<Horizontal>,
    align_y: Option<Vertical>,
    clip: Option<bool>,
}

impl<Message: 'static> Container<Message> {
    /// Creates a [`Container`] with the given content.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            padding: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            center_x: None,
            center_y: None,
            center: None,
            align_left: None,
            align_right: None,
            align_top: None,
            align_bottom: None,
            align_x: None,
            align_y: None,
            clip: None,
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
        self.max_width = Some(max_width.into().0);
        self
    }

    /// Sets the maximum height of the [`Container`].
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = Some(max_height.into().0);
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

impl<Message: 'static> From<Container<Message>> for Element<Message> {
    fn from(container: Container<Message>) -> Self {
        Element::new(move |realize, arena| {
            let content = container.content.build(realize, arena);
            let node = ContainerNode {
                content,
                padding: container.padding.map(Into::into),
                width: container.width.map(Into::into),
                height: container.height.map(Into::into),
                max_width: container.max_width,
                max_height: container.max_height,
                align_x: container.align_x.map(Into::into),
                align_y: container.align_y.map(Into::into),
                clip: container.clip,
                center_x: container.center_x.map(Into::into),
                center_y: container.center_y.map(Into::into),
                center: container.center.map(Into::into),
                align_left: container.align_left.map(Into::into),
                align_right: container.align_right.map(Into::into),
                align_top: container.align_top.map(Into::into),
                align_bottom: container.align_bottom.map(Into::into),
            };
            arena.push(Node::Container(node))
        })
    }
}
