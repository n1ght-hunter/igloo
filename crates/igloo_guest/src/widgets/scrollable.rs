use iced_core::{Length, Pixels};

use crate::{
    Element,
    bindings::iced::app::{
        element::scrollable_to_element,
        message::Viewport,
        scrollable::{Anchor, Direction, Scrollbar},
    },
    element::Widget,
};

/// A scrollable container.
pub struct Scrollable<Message> {
    content: Element<Message>,
    width: Option<Length>,
    height: Option<Length>,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message>>,
    direction: Option<Direction>,
}

impl<Message> Scrollable<Message> {
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            width: None,
            height: None,
            on_scroll: None,

            direction: None,
        }
    }

    /// Creates a new [`Scrollable`] with the given [`Direction`].
    pub fn with_direction(
        content: impl Into<Element<Message>>,
        direction: impl Into<Direction>,
    ) -> Self {
        Scrollable {
            content: content.into(),
            width: None,
            height: None,
            on_scroll: None,
            direction: Some(direction.into()),
        }
    }

    /// Makes the [`Scrollable`] scroll horizontally, with default [`Scrollbar`] settings.
    pub fn horizontal(self) -> Self {
        self.direction(Direction::Horizontal(Scrollbar {
            alignment: None,
            spacing: None,
            width: None,
            margin: None,
            scroller_width: None,
        }))
    }

    /// Sets the [`Direction`] of the [`Scrollable`].
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = Some(direction.into());
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
    ///
    /// The function takes the [`Viewport`] of the [`Scrollable`]
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
        let direction = if let Some(direction) = &mut self.direction {
            direction
        } else {
            self.direction = Some(Direction::Vertical(Scrollbar {
                alignment: None,
                spacing: None,
                width: None,
                margin: None,
                scroller_width: None,
            }));
            self.direction.as_mut().unwrap()
        };
        match direction {
            Direction::Horizontal(horizontal) | Direction::Both((horizontal, _)) => {
                horizontal.alignment = Some(alignment);
            }
            Direction::Vertical(_) => {}
        }

        self
    }

    /// Sets the [`Anchor`] of the vertical direction of the [`Scrollable`], if applicable.
    pub fn anchor_y(mut self, alignment: Anchor) -> Self {
        let direction = if let Some(direction) = &mut self.direction {
            direction
        } else {
            self.direction = Some(Direction::Vertical(Scrollbar {
                alignment: None,
                spacing: None,
                width: None,
                margin: None,
                scroller_width: None,
            }));
            self.direction.as_mut().unwrap()
        };
        match direction {
            Direction::Vertical(vertical) | Direction::Both((_, vertical)) => {
                vertical.alignment = Some(alignment);
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
        let direction = if let Some(direction) = &mut self.direction {
            direction
        } else {
            self.direction = Some(Direction::Vertical(Scrollbar {
                alignment: None,
                spacing: None,
                width: None,
                margin: None,
                scroller_width: None,
            }));
            self.direction.as_mut().unwrap()
        };
        match direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                scrollbar.spacing = Some(new_spacing.into().0);
            }
            Direction::Both(_) => {}
        }

        self
    }
}

impl<Message: 'static> Widget<Message> for Scrollable<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        scrollable_to_element(crate::bindings::iced::app::scrollable::Scrollable {
            content: self.content.as_element(create_message),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            direction: self.direction,
            on_scroll: self.on_scroll.map(|f| {
                create_message.add_message_func(Box::new(move |viewport| {
                    if let crate::bindings::Message::Viewport(v) = viewport {
                        Some(f(v))
                    } else {
                        None
                    }
                }))
            }),
        })
    }
}

impl<Message: 'static> From<Scrollable<Message>> for Element<Message> {
    fn from(scrollable: Scrollable<Message>) -> Self {
        Element::new(Box::new(scrollable))
    }
}
