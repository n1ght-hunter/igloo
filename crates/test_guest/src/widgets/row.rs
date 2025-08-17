use iced_core::{Length, Padding, Pixels};

use crate::{
    Element,
    bindings::iced::app::element::row_to_element,
    element::Widget,
};

/// A container that distributes its contents horizontally.
#[derive(Debug)]
pub struct Row<Message> {
    children: Vec<Element<Message>>,
    spacing: Option<Pixels>,
    padding: Option<Padding>,
    height: Option<Length>,
    width: Option<Length>,
    align_y: Option<iced_core::alignment::Vertical>,
    clip: Option<bool>,
    wrap: Option<bool>,
}

impl<Message> Default for Row<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Row<Message> {
    /// Creates an empty [`Row`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Row`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Row`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Sets the horizontal spacing _between_ elements.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = Some(amount.into());
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
        Row {
            children,
            spacing: None,
            padding: None,
            height: None,
            width: None,
            align_y: None,
            clip: None,
            wrap: None,
        }
    }

    /// Adds an element to the [`Row`].
    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        let child = child.into();
        self.children.push(child);
        self
    }

    pub fn push_maybe(mut self, child: Option<impl Into<Element<Message>>>) -> Self {
        if let Some(child) = child {
            self.children.push(child.into());
        }
        self
    }

    /// Extends the [`Row`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element<Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<Message> Widget<Message> for Row<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        row_to_element(crate::bindings::iced::app::row::Row {
            elements: self
                .children
                .into_iter()
                .map(|c| c.as_element(create_message))
                .collect(),
            spacing: self.spacing.map(Into::into),
            padding: self.padding.map(Into::into),
            height: self.height.map(Into::into),
            width: self.width.map(Into::into),
            align_y: self.align_y.map(Into::into),
            clip: self.clip,
            wrap: self.wrap,
        })
    }
}

impl<Message: 'static> From<Row<Message>> for Element<Message> {
    fn from(row: Row<Message>) -> Self {
        Element::new(Box::new(row))
    }
}
