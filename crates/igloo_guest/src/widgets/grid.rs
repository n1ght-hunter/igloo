use iced_core::Pixels;

use crate::{
    Element,
    bindings::iced::app::element::grid_to_element,
    element::Widget,
};

/// A container that arranges its contents in a grid.
#[derive(Debug)]
pub struct Grid<Message> {
    elements: Vec<Element<Message>>,
    spacing: Option<Pixels>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    columns: Option<u64>,
    fluid: Option<Pixels>,
}

impl<Message> Default for Grid<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Grid<Message> {
    /// Creates an empty [`Grid`].
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    /// Creates a [`Grid`] with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Creates a [`Grid`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        let iter = children.into_iter();
        Self::with_capacity(iter.size_hint().0).extend(iter)
    }

    /// Sets the spacing between cells in the [`Grid`].
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = Some(amount.into());
        self
    }

    /// Sets the width of the [`Grid`].
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the height of the [`Grid`].
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the number of columns of the [`Grid`].
    pub fn columns(mut self, columns: u64) -> Self {
        self.columns = Some(columns);
        self
    }

    /// Sets the fluid spacing of the [`Grid`].
    pub fn fluid(mut self, amount: impl Into<Pixels>) -> Self {
        self.fluid = Some(amount.into());
        self
    }

    /// Creates a [`Grid`] from an already allocated [`Vec`].
    pub fn from_vec(children: Vec<Element<Message>>) -> Self {
        Grid {
            elements: children,
            spacing: None,
            width: None,
            height: None,
            columns: None,
            fluid: None,
        }
    }

    /// Adds an element to the [`Grid`].
    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        self.elements.push(child.into());
        self
    }

    /// Extends the [`Grid`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element<Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }
}

impl<Message> Widget<Message> for Grid<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        grid_to_element(crate::bindings::iced::app::element::Grid {
            elements: self
                .elements
                .into_iter()
                .map(|e| e.as_element(create_message))
                .collect(),
            spacing: self.spacing.map(Into::into),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            columns: self.columns,
            fluid: self.fluid.map(Into::into),
        })
    }
}

impl<Message: 'static> From<Grid<Message>> for Element<Message> {
    fn from(grid: Grid<Message>) -> Self {
        Element::new(Box::new(grid))
    }
}
