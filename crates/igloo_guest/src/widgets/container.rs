use iced_core::{
    Length, Padding, Pixels,
    alignment::{Horizontal, Vertical},
};

use crate::Element;
use crate::bindings::iced::app::container::Container as WitContainer;

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
        Element::new(move |realize| {
            let content = container.content.build(realize);
            let raw = WitContainer::new(content);
            if let Some(padding) = container.padding {
                raw.padding(padding.into());
            }
            if let Some(width) = container.width {
                raw.width(width.into());
            }
            if let Some(height) = container.height {
                raw.height(height.into());
            }
            if let Some(max_width) = container.max_width {
                raw.max_width(max_width);
            }
            if let Some(max_height) = container.max_height {
                raw.max_height(max_height);
            }
            if let Some(width) = container.center_x {
                raw.center_x(width.into());
            }
            if let Some(height) = container.center_y {
                raw.center_y(height.into());
            }
            if let Some(length) = container.center {
                raw.center(length.into());
            }
            if let Some(width) = container.align_left {
                raw.align_left(width.into());
            }
            if let Some(width) = container.align_right {
                raw.align_right(width.into());
            }
            if let Some(height) = container.align_top {
                raw.align_top(height.into());
            }
            if let Some(height) = container.align_bottom {
                raw.align_bottom(height.into());
            }
            if let Some(align) = container.align_x {
                raw.align_x(align.into());
            }
            if let Some(align) = container.align_y {
                raw.align_y(align.into());
            }
            if let Some(clip) = container.clip {
                raw.clip(clip);
            }
            WitContainer::into_element(raw)
        })
    }
}
