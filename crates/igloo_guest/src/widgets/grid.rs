use std::marker::PhantomData;

use iced_core::Pixels;

use crate::Element;
use crate::bindings::iced::app::widgets::{GridNode, Node};

/// A container that arranges its contents in a grid.
pub struct Grid<Message> {
    children: Vec<Element<Message>>,
    spacing: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    columns: Option<u64>,
    fluid: Option<f32>,
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
            children: Vec::new(),
            spacing: None,
            width: None,
            height: None,
            columns: None,
            fluid: None,
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
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = Some(amount.into().0);
        self
    }

    /// Sets the width of the [`Grid`].
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().0);
        self
    }

    /// Sets the height of the [`Grid`].
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into().0);
        self
    }

    /// Sets the number of columns of the [`Grid`].
    pub fn columns(mut self, columns: u64) -> Self {
        self.columns = Some(columns);
        self
    }

    /// Sets the fluid spacing of the [`Grid`].
    pub fn fluid(mut self, amount: impl Into<Pixels>) -> Self {
        self.fluid = Some(amount.into().0);
        self
    }
}

impl<Message: 'static> From<Grid<Message>> for Element<Message> {
    fn from(grid: Grid<Message>) -> Self {
        Element::new(move |realize, arena| {
            let elements = grid
                .children
                .into_iter()
                .map(|child| child.build(realize, arena))
                .collect();
            let node = GridNode {
                elements,
                spacing: grid.spacing,
                width: grid.width,
                height: grid.height,
                columns: grid.columns,
                fluid: grid.fluid,
            };
            arena.push(Node::Grid(node))
        })
    }
}
