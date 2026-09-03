//! A container that arranges its contents in a grid.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::{Node, NodeId};

/// A container that arranges its contents in a grid.
#[stabby::stabby]
#[derive(Debug)]
pub struct GridNode {
    pub elements: stabby::vec::Vec<NodeId>,
    pub spacing: Opt<f32>,
    pub width: Opt<f32>,
    pub height: Opt<f32>,
    pub columns: Opt<u64>,
    pub fluid: Opt<f32>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    grid: GridNode,
) -> iced::Element<'a, crate::widgets::Emitted> {
    let children: std::vec::Vec<_> = grid
        .elements
        .into_iter()
        .map(|child| crate::widgets::build_node(nodes, child))
        .collect();
    let mut widget = iced::widget::Grid::with_children(children);
    if let Some(spacing) = Option::<f32>::from(grid.spacing) {
        widget = widget.spacing(spacing);
    }
    if let Some(width) = Option::<f32>::from(grid.width) {
        widget = widget.width(width);
    }
    if let Some(height) = Option::<f32>::from(grid.height) {
        widget = widget.height(height);
    }
    if let Some(columns) = Option::<u64>::from(grid.columns) {
        widget = widget.columns(columns as usize);
    }
    if let Some(fluid) = Option::<f32>::from(grid.fluid) {
        widget = widget.fluid(fluid);
    }
    widget.into()
}

/// A builder for a [`GridNode`].
pub struct Grid<Message> {
    children: std::vec::Vec<Element<Message>>,
    spacing: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    columns: Option<u64>,
    fluid: Option<f32>,
}

impl<Message> Grid<Message> {
    /// Creates an empty grid.
    pub fn new() -> Self {
        Self {
            children: std::vec::Vec::new(),
            spacing: None,
            width: None,
            height: None,
            columns: None,
            fluid: None,
        }
    }

    /// Appends a child.
    pub fn push(mut self, child: impl Into<Element<Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Appends every child produced by `children`.
    pub fn extend(
        mut self,
        children: impl IntoIterator<Item = impl Into<Element<Message>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn columns(mut self, columns: u64) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn fluid(mut self, fluid: f32) -> Self {
        self.fluid = Some(fluid);
        self
    }
}

impl<Message> Default for Grid<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> std::fmt::Debug for Grid<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("children", &self.children)
            .field("spacing", &self.spacing)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("columns", &self.columns)
            .field("fluid", &self.fluid)
            .finish()
    }
}

impl<Message: 'static> From<Grid<Message>> for Element<Message> {
    fn from(grid: Grid<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let elements: stabby::vec::Vec<NodeId> = grid
                .children
                .into_iter()
                .map(|child| child.build(realize, arena))
                .collect();
            arena.push(Node::Grid(GridNode {
                elements,
                spacing: grid.spacing.into(),
                width: grid.width.into(),
                height: grid.height.into(),
                columns: grid.columns.into(),
                fluid: grid.fluid.into(),
            }))
        })
    }
}

/// Creates an empty [`Grid`].
pub fn grid<Message>() -> Grid<Message> {
    Grid::new()
}
