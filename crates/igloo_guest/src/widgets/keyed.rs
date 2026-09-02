use std::marker::PhantomData;

use iced_core::{Length, Padding, Pixels, alignment};

use crate::Element;
use crate::bindings::iced::app::widgets::{KeyedColumnNode, Node};

/// The key of a keyed element.
pub type Key = u64;

/// A container that keeps track of its children by key, minimizing rebuilds.
pub struct KeyedColumn<Message> {
    children: Vec<(Key, Element<Message>)>,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<f32>,
    align_items: Option<alignment::Alignment>,
    _message: PhantomData<Message>,
}

impl<Message> Default for KeyedColumn<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> KeyedColumn<Message> {
    /// Creates an empty [`KeyedColumn`].
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            max_width: None,
            align_items: None,
            _message: PhantomData,
        }
    }

    /// Creates a [`KeyedColumn`] with the given capacity.
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Creates a [`KeyedColumn`] from already allocated keys and children.
    pub fn from_vecs(keys: Vec<Key>, children: Vec<Element<Message>>) -> Self {
        keys.into_iter()
            .zip(children)
            .fold(Self::new(), |column, (key, child)| column.push(key, child))
    }

    /// Creates a [`KeyedColumn`] with the given keyed children.
    pub fn with_children(children: Vec<(Key, Element<Message>)>) -> Self {
        Self::new().extend(children)
    }

    /// Adds a keyed element to the [`KeyedColumn`].
    pub fn push(mut self, key: Key, child: impl Into<Element<Message>>) -> Self {
        self.children.push((key, child.into()));
        self
    }

    /// Extends the [`KeyedColumn`] with the given keyed children.
    pub fn extend(self, children: impl IntoIterator<Item = (Key, Element<Message>)>) -> Self {
        children
            .into_iter()
            .fold(self, |column, (key, child)| column.push(key, child))
    }

    /// Sets the spacing between elements in the [`KeyedColumn`].
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into().0);
        self
    }

    /// Sets the padding of the [`KeyedColumn`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the width of the [`KeyedColumn`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`KeyedColumn`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the maximum width of the [`KeyedColumn`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into().0);
        self
    }

    /// Sets the alignment of the elements in the [`KeyedColumn`].
    pub fn align_items(mut self, align: impl Into<alignment::Alignment>) -> Self {
        self.align_items = Some(align.into());
        self
    }
}

impl<Message: 'static> From<KeyedColumn<Message>> for Element<Message> {
    fn from(column: KeyedColumn<Message>) -> Self {
        Element::new(move |realize, arena| {
            let mut keys = Vec::with_capacity(column.children.len());
            let mut children = Vec::with_capacity(column.children.len());
            for (key, child) in column.children {
                keys.push(key);
                children.push(child.build(realize, arena));
            }
            let node = KeyedColumnNode {
                keys,
                children,
                spacing: column.spacing,
                padding: column.padding.map(Into::into),
                width: column.width.map(Into::into),
                height: column.height.map(Into::into),
                max_width: column.max_width,
                align_items: column.align_items.map(Into::into),
            };
            arena.push(Node::KeyedColumn(node))
        })
    }
}
