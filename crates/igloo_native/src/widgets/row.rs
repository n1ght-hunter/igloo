//! A horizontal stack of child nodes.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Padding, Vertical};
use crate::widgets::{Node, NodeId};

/// A horizontal stack of child nodes.
#[stabby::stabby]
#[derive(Debug)]
pub struct RowNode {
    pub children: stabby::vec::Vec<NodeId>,
    pub spacing: Opt<f32>,
    pub padding: Opt<Padding>,
    pub width: Opt<crate::widgets::Length>,
    pub height: Opt<crate::widgets::Length>,
    pub align_y: Opt<Vertical>,
    pub clip: Opt<bool>,
    pub wrap: Opt<bool>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    row: RowNode,
) -> iced::Element<'a, crate::widgets::Emitted> {
    use crate::widgets::Length;

    let children: std::vec::Vec<_> = row
        .children
        .into_iter()
        .map(|child| crate::widgets::build_node(nodes, child))
        .collect();
    let mut widget = iced::widget::Row::with_children(children);
    if let Some(spacing) = Option::<f32>::from(row.spacing) {
        widget = widget.spacing(spacing);
    }
    if let Some(padding) = Option::<Padding>::from(row.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    if let Some(width) = Option::<Length>::from(row.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(row.height) {
        widget = widget.height(iced::Length::from(height));
    }
    if let Some(align_y) = Option::<Vertical>::from(row.align_y) {
        widget = widget.align_y(iced::alignment::Vertical::from(align_y));
    }
    if let Some(clip) = Option::<bool>::from(row.clip) {
        widget = widget.clip(clip);
    }
    if let Some(true) = Option::<bool>::from(row.wrap) {
        return widget.wrap().into();
    }
    widget.into()
}

/// A builder for a [`RowNode`].
pub struct Row<Message> {
    children: std::vec::Vec<Element<Message>>,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<crate::widgets::Length>,
    height: Option<crate::widgets::Length>,
    align_y: Option<Vertical>,
    clip: Option<bool>,
    wrap: Option<bool>,
}

impl<Message> Row<Message> {
    /// Creates an empty row.
    pub fn new() -> Self {
        Self {
            children: std::vec::Vec::new(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            align_y: None,
            clip: None,
            wrap: None,
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

    pub fn width(mut self, width: impl Into<crate::widgets::Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<crate::widgets::Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn align_y(mut self, align_y: Vertical) -> Self {
        self.align_y = Some(align_y);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }

    /// Turns this row into a wrapping row.
    pub fn wrap(mut self) -> Self {
        self.wrap = Some(true);
        self
    }
}

impl<Message> Default for Row<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> std::fmt::Debug for Row<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("children", &self.children)
            .field("spacing", &self.spacing)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("align_y", &self.align_y)
            .field("clip", &self.clip)
            .field("wrap", &self.wrap)
            .finish()
    }
}

impl<Message: 'static> From<Row<Message>> for Element<Message> {
    fn from(row: Row<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let children: stabby::vec::Vec<NodeId> = row
                .children
                .into_iter()
                .map(|child| child.build(realize, arena))
                .collect();
            arena.push(Node::Row(RowNode {
                children,
                spacing: row.spacing.into(),
                padding: row.padding.into(),
                width: row.width.into(),
                height: row.height.into(),
                align_y: row.align_y.into(),
                clip: row.clip.into(),
                wrap: row.wrap.into(),
            }))
        })
    }
}

/// Creates an empty [`Row`].
pub fn row<Message>() -> Row<Message> {
    Row::new()
}
