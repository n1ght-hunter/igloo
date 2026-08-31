use std::marker::PhantomData;

use iced_core::Pixels;

use crate::Element;
use crate::bindings::iced::app::grid::Grid as WitGrid;

/// A container that arranges its contents in a grid.
pub struct Grid<Message> {
    raw: WitGrid,
    children: Vec<Element<Message>>,
    _message: PhantomData<Message>,
}

impl<Message> Default for Grid<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Grid<Message> {
    /// Creates an empty [`Grid`].
    pub fn new() -> Self {
        Self {
            raw: WitGrid::new(),
            children: Vec::new(),
            _message: PhantomData,
        }
    }

    /// Creates a [`Grid`] with the given capacity.
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Creates a [`Grid`] with the given elements.
    pub fn with_children(children: impl IntoIterator<Item = Element<Message>>) -> Self {
        Self::new().extend(children)
    }

    /// Creates a [`Grid`] from an already allocated [`Vec`].
    pub fn from_vec(children: Vec<Element<Message>>) -> Self {
        Self::new().extend(children)
    }

    /// Adds an element to the [`Grid`].
    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Extends the [`Grid`] with the given children.
    pub fn extend(self, children: impl IntoIterator<Item = Element<Message>>) -> Self {
        children.into_iter().fold(self, Self::push)
    }

    /// Sets the spacing between cells in the [`Grid`].
    pub fn spacing(self, amount: impl Into<Pixels>) -> Self {
        self.raw.spacing(amount.into().0);
        self
    }

    /// Sets the width of the [`Grid`].
    pub fn width(self, width: impl Into<Pixels>) -> Self {
        self.raw.width(width.into().0);
        self
    }

    /// Sets the height of the [`Grid`].
    pub fn height(self, height: impl Into<Pixels>) -> Self {
        self.raw.height(height.into().0);
        self
    }

    /// Sets the number of columns of the [`Grid`].
    pub fn columns(self, columns: u64) -> Self {
        self.raw.columns(columns);
        self
    }

    /// Sets the fluid spacing of the [`Grid`].
    pub fn fluid(self, amount: impl Into<Pixels>) -> Self {
        self.raw.fluid(amount.into().0);
        self
    }
}

impl<Message: 'static> From<Grid<Message>> for Element<Message> {
    fn from(grid: Grid<Message>) -> Self {
        Element::new(move |realize| {
            for child in grid.children {
                grid.raw.push(child.build(realize));
            }
            WitGrid::into_element(grid.raw)
        })
    }
}
