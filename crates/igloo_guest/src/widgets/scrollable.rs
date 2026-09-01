use iced_core::{Length, Pixels};

use crate::Element;
use crate::bindings::iced::app::message_types::Viewport;
use crate::bindings::iced::app::scrollable::{
    Anchor, Direction, Scrollable as WitScrollable, Scrollbar,
};

/// A scrollable container.
pub struct Scrollable<Message> {
    content: Element<Message>,
    direction: Option<Direction>,
    width: Option<Length>,
    height: Option<Length>,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message>>,
}

impl<Message: 'static> Scrollable<Message> {
    /// Creates a new [`Scrollable`] wrapping the given content.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            direction: None,
            width: None,
            height: None,
            on_scroll: None,
        }
    }

    /// Creates a new [`Scrollable`] with the given [`Direction`].
    pub fn with_direction(content: impl Into<Element<Message>>, direction: Direction) -> Self {
        let mut this = Self::new(content);
        this.direction = Some(direction);
        this
    }

    /// Makes the [`Scrollable`] scroll horizontally, with default [`Scrollbar`] settings.
    pub fn horizontal(self) -> Self {
        self.direction(Direction::Horizontal(Scrollbar::default()))
    }

    /// Sets the [`Direction`] of the [`Scrollable`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Sets the width of the [`Scrollable`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Scrollable`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets a function to call when the [`Scrollable`] is scrolled.
    pub fn on_scroll(mut self, f: impl Fn(Viewport) -> Message + 'static) -> Self {
        self.on_scroll = Some(Box::new(f));
        self
    }

    /// Anchors the vertical [`Scrollable`] direction to the top.
    pub fn anchor_top(self) -> Self {
        self.anchor_y(Anchor::Start)
    }

    /// Anchors the vertical [`Scrollable`] direction to the bottom.
    pub fn anchor_bottom(self) -> Self {
        self.anchor_y(Anchor::End)
    }

    /// Anchors the horizontal [`Scrollable`] direction to the left.
    pub fn anchor_left(self) -> Self {
        self.anchor_x(Anchor::Start)
    }

    /// Anchors the horizontal [`Scrollable`] direction to the right.
    pub fn anchor_right(self) -> Self {
        self.anchor_x(Anchor::End)
    }

    /// Sets the [`Anchor`] of the horizontal direction of the [`Scrollable`], if applicable.
    pub fn anchor_x(mut self, alignment: Anchor) -> Self {
        let direction = self
            .direction
            .get_or_insert_with(|| Direction::Vertical(Scrollbar::default()));

        match direction {
            Direction::Horizontal(horizontal) | Direction::Both((horizontal, _)) => {
                horizontal.anchor = Some(alignment);
            }
            Direction::Vertical(_) => {}
        }

        self
    }

    /// Sets the [`Anchor`] of the vertical direction of the [`Scrollable`], if applicable.
    pub fn anchor_y(mut self, alignment: Anchor) -> Self {
        let direction = self
            .direction
            .get_or_insert_with(|| Direction::Vertical(Scrollbar::default()));

        match direction {
            Direction::Vertical(vertical) | Direction::Both((_, vertical)) => {
                vertical.anchor = Some(alignment);
            }
            Direction::Horizontal(_) => {}
        }

        self
    }

    /// Embeds the [`Scrollbar`] into the [`Scrollable`], instead of floating on top of the
    /// content.
    ///
    /// The `spacing` provided will be used as space between the [`Scrollbar`] and the contents
    /// of the [`Scrollable`].
    pub fn spacing(mut self, new_spacing: impl Into<Pixels>) -> Self {
        let direction = self
            .direction
            .get_or_insert_with(|| Direction::Vertical(Scrollbar::default()));

        match direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                scrollbar.spacing = Some(new_spacing.into().0);
            }
            Direction::Both(_) => {}
        }

        self
    }
}

impl<Message: 'static> From<Scrollable<Message>> for Element<Message> {
    fn from(scrollable: Scrollable<Message>) -> Self {
        Element::new(move |realize| {
            let content = scrollable.content.build(realize);
            let raw = WitScrollable::new(content);
            if let Some(direction) = scrollable.direction {
                raw.direction(direction);
            }
            if let Some(width) = scrollable.width {
                raw.width(width.into());
            }
            if let Some(height) = scrollable.height {
                raw.height(height.into());
            }
            if let Some(on_scroll) = scrollable.on_scroll {
                raw.on_scroll(realize.viewport_mapper(on_scroll));
            }
            WitScrollable::into_element(raw)
        })
    }
}
