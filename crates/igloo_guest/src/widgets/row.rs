use std::marker::PhantomData;

use iced_core::{Length, Padding, Pixels};

use crate::Element;
use crate::bindings::iced::app::row::Row as WitRow;

/// A container that distributes its contents horizontally.
pub struct Row<Message> {
    raw: WitRow,
    children: Vec<Element<Message>>,
    _message: PhantomData<Message>,
}

impl<Message> Default for Row<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Row<Message> {
    /// Creates an empty [`Row`].
    pub fn new() -> Self {
        Self {
            raw: WitRow::new(),
            children: Vec::new(),
            _message: PhantomData,
        }
    }

    /// Creates a [`Row`] with the given capacity.
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Creates a [`Row`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        Self::new().extend(children)
    }

    /// Sets the horizontal spacing _between_ elements.
    pub fn spacing(self, amount: impl Into<Pixels>) -> Self {
        self.raw.spacing(amount.into().0);
        self
    }

    /// Sets the [`Padding`] of the [`Row`].
    pub fn padding(self, padding: impl Into<Padding>) -> Self {
        self.raw.padding(padding.into().into());
        self
    }

    /// Sets the width of the [`Row`].
    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    /// Sets the height of the [`Row`].
    pub fn height(self, height: impl Into<Length>) -> Self {
        self.raw.height(height.into().into());
        self
    }

    /// Sets the vertical alignment of the contents of the [`Row`].
    pub fn align_y(self, align: impl Into<iced_core::alignment::Vertical>) -> Self {
        self.raw.align_y(align.into().into());
        self
    }

    /// Sets whether the contents of the [`Row`] should be clipped on overflow.
    pub fn clip(self, clip: bool) -> Self {
        self.raw.clip(clip);
        self
    }

    /// Turns the [`Row`] into a wrapping row.
    /// The original alignment of the [`Row`] is preserved per row wrapped.
    pub fn wrap(self) -> Self {
        self.raw.wrap(true);
        self
    }

    /// Creates a [`Row`] from an already allocated [`Vec`].
    pub fn from_vec(children: Vec<Element<Message>>) -> Self {
        Self::new().extend(children)
    }

    /// Adds an element to the [`Row`].
    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn push_maybe(self, child: Option<impl Into<Element<Message>>>) -> Self {
        match child {
            Some(child) => self.push(child),
            None => self,
        }
    }

    /// Extends the [`Row`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element<Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<Message: 'static> From<Row<Message>> for Element<Message> {
    fn from(row: Row<Message>) -> Self {
        Element::new(move |realize| {
            for child in row.children {
                row.raw.push(child.build(realize));
            }
            WitRow::into_element(row.raw)
        })
    }
}
