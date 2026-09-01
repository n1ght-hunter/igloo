use std::marker::PhantomData;

use iced_core::{Length, Padding, Pixels, alignment};

use crate::Element;
use crate::bindings::iced::app::keyed::KeyedColumn as WitKeyedColumn;

/// The key of a keyed element.
pub type Key = u64;

/// A container that keeps track of its children by key, minimizing rebuilds.
pub struct KeyedColumn<Message> {
    raw: WitKeyedColumn,
    children: Vec<(Key, Element<Message>)>,
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
            raw: WitKeyedColumn::new(),
            children: Vec::new(),
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
    pub fn spacing(self, spacing: impl Into<Pixels>) -> Self {
        self.raw.spacing(spacing.into().0);
        self
    }

    /// Sets the padding of the [`KeyedColumn`].
    pub fn padding(self, padding: impl Into<Padding>) -> Self {
        self.raw.padding(padding.into().into());
        self
    }

    /// Sets the width of the [`KeyedColumn`].
    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    /// Sets the height of the [`KeyedColumn`].
    pub fn height(self, height: impl Into<Length>) -> Self {
        self.raw.height(height.into().into());
        self
    }

    /// Sets the maximum width of the [`KeyedColumn`].
    pub fn max_width(self, max_width: impl Into<Pixels>) -> Self {
        self.raw.max_width(max_width.into().0);
        self
    }

    /// Sets the alignment of the elements in the [`KeyedColumn`].
    pub fn align_items(self, align: impl Into<alignment::Alignment>) -> Self {
        self.raw.align_items(align.into().into());
        self
    }
}

impl<Message: 'static> From<KeyedColumn<Message>> for Element<Message> {
    fn from(column: KeyedColumn<Message>) -> Self {
        Element::new(move |realize| {
            for (key, child) in column.children {
                column.raw.push(key, child.build(realize));
            }
            WitKeyedColumn::into_element(column.raw)
        })
    }
}
