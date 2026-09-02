use std::marker::PhantomData;

use iced_core::{Length, Padding, Pixels};

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, RowNode};

/// A container that distributes its contents horizontally.
pub struct Row<Message> {
    children: Vec<Element<Message>>,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    align_y: Option<iced_core::alignment::Vertical>,
    clip: Option<bool>,
    wrap: Option<bool>,
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
            children: Vec::new(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            align_y: None,
            clip: None,
            wrap: None,
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
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = Some(amount.into().0);
        self
    }

    /// Sets the [`Padding`] of the [`Row`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the width of the [`Row`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Row`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the vertical alignment of the contents of the [`Row`].
    pub fn align_y(mut self, align: impl Into<iced_core::alignment::Vertical>) -> Self {
        self.align_y = Some(align.into());
        self
    }

    /// Sets whether the contents of the [`Row`] should be clipped on overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }

    /// Turns the [`Row`] into a wrapping row.
    /// The original alignment of the [`Row`] is preserved per row wrapped.
    pub fn wrap(mut self) -> Self {
        self.wrap = Some(true);
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
        Element::new(move |realize, arena| {
            let children = row
                .children
                .into_iter()
                .map(|child| child.build(realize, arena))
                .collect();
            let node = RowNode {
                children,
                spacing: row.spacing,
                padding: row.padding.map(Into::into),
                width: row.width.map(Into::into),
                height: row.height.map(Into::into),
                align_y: row.align_y.map(Into::into),
                clip: row.clip,
                wrap: row.wrap,
            };
            arena.push(Node::Row(node))
        })
    }
}
