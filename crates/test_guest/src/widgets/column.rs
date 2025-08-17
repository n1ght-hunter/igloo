use iced_core::{Length, Padding, Pixels};

use crate::{
    Element,
    bindings::iced::app::{self, element::column_to_element},
    element::Widget,
};

/// A container that distributes its contents vertically.
#[derive(Debug)]
pub struct Column<Message> {
    elements: Vec<Element<Message>>,
    spacing: Option<Pixels>,
    padding: Option<Padding>,
    height: Option<Length>,
    width: Option<Length>,
    max_width: Option<Pixels>,
    align_x: Option<iced_core::alignment::Horizontal>,
    clip: Option<bool>,
}

impl<Message> Default for Column<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Column<Message> {
    /// Creates an empty [`Column`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Column`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Column`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        let iterator = children.into_iter();

        Self::with_capacity(iterator.size_hint().0).extend(iterator)
    }

    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = Some(amount.into());
        self
    }

    /// Sets the [`Padding`] of the [`Column`].
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the width of the [`Column`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Column`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the maximum width of the [`Column`].
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into());
        self
    }

    /// Sets the horizontal alignment of the contents of the [`Column`].
    pub fn align_x(mut self, align: impl Into<iced_core::alignment::Horizontal>) -> Self {
        self.align_x = Some(align.into());
        self
    }

    //// Sets whether the contents of the [`Column`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }

    /// Creates a [`Column`] from an already allocated [`Vec`].
    ///
    /// Keep in mind that the [`Column`] will not inspect the [`Vec`], which means
    /// it won't automatically adapt to the sizing strategy of its contents.
    ///
    /// If any of the children have a [`Length::Fill`] strategy, you will need to
    /// call [`Column::width`] or [`Column::height`] accordingly.
    pub fn from_vec(children: Vec<Element<Message>>) -> Self {
        Column {
            elements: children,
            spacing: None,
            padding: None,
            height: None,
            width: None,
            max_width: None,
            align_x: None,
            clip: None,
        }
    }

    /// Adds an element to the [`Column`].
    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        self.elements.push(child.into());

        self
    }

    /// Extends the [`Column`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element<Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<Message> Widget<Message> for Column<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        let column = app::element::Column {
            elements: self
                .elements
                .into_iter()
                .map(|e| e.as_element(create_message))
                .collect(),
            spacing: self.spacing.map(Into::into),
            padding: self.padding.map(Into::into),
            height: self.height.map(Into::into),
            width: self.width.map(Into::into),
            max_width: self.max_width.map(Into::into),
            align_x: self.align_x.map(Into::into),
            clip: self.clip,
        };

        column_to_element(column)
    }
}

impl<Message: Clone + 'static> From<Column<Message>> for Element<Message> {
    fn from(column: Column<Message>) -> Self {
        Element::new(Box::new(column))
    }
}
