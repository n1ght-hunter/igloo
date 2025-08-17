use iced_core::{Length, Padding, Pixels, alignment};

use crate::Element;
use crate::bindings::iced::app::element::keyed_column_to_element;
use crate::element::Widget;

type Key = u64;

/// A container that keeps the state of its children using keys.
#[derive(Debug)]
pub struct KeyedColumn<Message> {
    keys: Vec<Key>,
    children: Vec<Element<Message>>,
    spacing: Option<Pixels>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<Pixels>,
    align_items: Option<alignment::Alignment>,
}

impl<Message> Default for KeyedColumn<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> KeyedColumn<Message> {
    /// Creates an empty [`KeyedColumn`].
    pub fn new() -> Self {
        Self::from_vecs(Vec::new(), Vec::new())
    }

    /// Creates a [`Column`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vecs(Vec::with_capacity(capacity), Vec::with_capacity(capacity))
    }

    pub fn from_vecs(keys: Vec<Key>, children: Vec<Element<Message>>) -> Self {
        KeyedColumn {
            keys,
            children,
            spacing: None,
            padding: None,
            width: None,
            height: None,
            max_width: None,
            align_items: None,
        }
    }

    /// Creates a [`KeyedColumn`] from an already allocated [`Vec`].
    pub fn with_children(children: Vec<(Key, Element<Message>)>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Adds a keyed element to the [`KeyedColumn`].
    pub fn push(mut self, key: Key, child: impl Into<Element<Message>>) -> Self {
        let child = child.into();
        self.keys.push(key);
        self.children.push(child);
        self
    }

    /// Extends the [`KeyedColumn`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = (u64, Element<Message>)>) -> Self {
        children
            .into_iter()
            .fold(self, |col, (key, elem)| col.push(key, elem))
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    pub fn align_items(mut self, alignment: impl Into<alignment::Alignment>) -> Self {
        self.align_items = Some(alignment.into());
        self
    }
}

impl<Message> Widget<Message> for KeyedColumn<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        keyed_column_to_element(crate::bindings::iced::app::element::KeyedColumn {
            keys: self.keys,
            children: self
                .children
                .into_iter()
                .map(|e| e.as_element(create_message))
                .collect(),
            spacing: self.spacing.map(Into::into),
            padding: self.padding.map(Into::into),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            max_width: self.max_width.map(Into::into),
            align_items: self.align_items.map(Into::into),
        })
    }
}

impl<Message: 'static> From<KeyedColumn<Message>> for Element<Message> {
    fn from(column: KeyedColumn<Message>) -> Self {
        Element::new(Box::new(column))
    }
}
