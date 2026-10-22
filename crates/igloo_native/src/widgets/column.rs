//! A vertical stack of child nodes.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Horizontal, Length, Padding};
use crate::widgets::{Node, NodeId};

/// A vertical stack of child nodes.
#[stabby::stabby]
#[derive(Debug)]
pub struct ColumnNode {
    pub children: stabby::vec::Vec<NodeId>,
    pub spacing: Opt<f32>,
    pub padding: Opt<Padding>,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub max_width: Opt<f32>,
    pub align_x: Opt<Horizontal>,
    pub clip: Opt<bool>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    column: ColumnNode,
) -> impl iced::Widget<crate::widgets::Emitted> + 'a {
    let children: std::vec::Vec<_> = column
        .children
        .into_iter()
        .map(|child| crate::widgets::build_node(nodes, child))
        .collect();
    let mut widget = iced::widget::Column::with_children(children);
    if let Some(spacing) = Option::<f32>::from(column.spacing) {
        widget = widget.spacing(spacing);
    }
    if let Some(padding) = Option::<Padding>::from(column.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    let mut width: Option<iced::Length> = Option::<Length>::from(column.width).map(Into::into);
    if let Some(max_width) = Option::<f32>::from(column.max_width) {
        width = Some(width.unwrap_or(iced::Length::Shrink).max(max_width));
    }
    if let Some(width) = width {
        widget = widget.width(width);
    }
    if let Some(height) = Option::<Length>::from(column.height) {
        widget = widget.height(iced::Length::from(height));
    }
    if let Some(align_x) = Option::<Horizontal>::from(column.align_x) {
        widget = widget.align_x(iced::alignment::Horizontal::from(align_x));
    }
    if let Some(clip) = Option::<bool>::from(column.clip) {
        widget = widget.clip(clip);
    }
    widget
}

/// A builder for a [`ColumnNode`].
pub struct Column<Message> {
    children: std::vec::Vec<Element<Message>>,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<f32>,
    align_x: Option<Horizontal>,
    clip: Option<bool>,
}

impl<Message> Column<Message> {
    /// Creates an empty column.
    pub fn new() -> Self {
        Self {
            children: std::vec::Vec::new(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            max_width: None,
            align_x: None,
            clip: None,
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

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn align_x(mut self, align_x: Horizontal) -> Self {
        self.align_x = Some(align_x);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message> Default for Column<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> std::fmt::Debug for Column<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Column")
            .field("children", &self.children)
            .field("spacing", &self.spacing)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("max_width", &self.max_width)
            .field("align_x", &self.align_x)
            .field("clip", &self.clip)
            .finish()
    }
}

impl<Message: 'static> From<Column<Message>> for Element<Message> {
    fn from(column: Column<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let children: stabby::vec::Vec<NodeId> = column
                .children
                .into_iter()
                .map(|child| child.build(realize, arena))
                .collect();
            arena.push(Node::Column(ColumnNode {
                children,
                spacing: column.spacing.into(),
                padding: column.padding.into(),
                width: column.width.into(),
                height: column.height.into(),
                max_width: column.max_width.into(),
                align_x: column.align_x.into(),
                clip: column.clip.into(),
            }))
        })
    }
}

/// Creates an empty [`Column`].
pub fn column<Message>() -> Column<Message> {
    Column::new()
}
