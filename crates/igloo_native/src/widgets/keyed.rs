//! A container that keeps track of its children by key, minimizing rebuilds.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Alignment, Length, Padding};
use crate::widgets::{Node, NodeId};

/// The key of a keyed element.
pub type Key = u64;

/// A container that keeps track of its children by key, minimizing rebuilds.
#[stabby::stabby]
#[derive(Debug)]
pub struct KeyedColumnNode {
    pub keys: stabby::vec::Vec<Key>,
    pub children: stabby::vec::Vec<NodeId>,
    pub spacing: Opt<f32>,
    pub padding: Opt<Padding>,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub max_width: Opt<f32>,
    pub align_items: Opt<Alignment>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    column: KeyedColumnNode,
) -> impl iced::Widget<crate::widgets::Emitted> + 'a {
    let keys: std::vec::Vec<_> = column.keys.into_iter().collect();
    let children: std::vec::Vec<_> = column
        .children
        .into_iter()
        .map(|child| crate::widgets::build_node(nodes, child))
        .collect();
    let mut widget = iced::widget::keyed::Column::from_vecs(keys, children);
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
    if let Some(align_items) = Option::<Alignment>::from(column.align_items) {
        widget = widget.align_items(iced::alignment::Alignment::from(align_items));
    }
    widget
}

/// A builder for a [`KeyedColumnNode`].
pub struct KeyedColumn<Message> {
    children: std::vec::Vec<(Key, Element<Message>)>,
    spacing: Option<f32>,
    padding: Option<Padding>,
    width: Option<Length>,
    height: Option<Length>,
    max_width: Option<f32>,
    align_items: Option<Alignment>,
}

impl<Message> KeyedColumn<Message> {
    /// Creates an empty keyed column.
    pub fn new() -> Self {
        Self {
            children: std::vec::Vec::new(),
            spacing: None,
            padding: None,
            width: None,
            height: None,
            max_width: None,
            align_items: None,
        }
    }

    /// Appends a keyed child.
    pub fn push(mut self, key: Key, child: impl Into<Element<Message>>) -> Self {
        self.children.push((key, child.into()));
        self
    }

    /// Appends every keyed child produced by `children`.
    pub fn extend(
        mut self,
        children: impl IntoIterator<Item = (Key, impl Into<Element<Message>>)>,
    ) -> Self {
        self.children
            .extend(children.into_iter().map(|(key, child)| (key, child.into())));
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

    pub fn align_items(mut self, align: Alignment) -> Self {
        self.align_items = Some(align);
        self
    }
}

impl<Message> Default for KeyedColumn<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> std::fmt::Debug for KeyedColumn<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyedColumn")
            .field("children", &self.children)
            .field("spacing", &self.spacing)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("max_width", &self.max_width)
            .field("align_items", &self.align_items)
            .finish()
    }
}

impl<Message: 'static> From<KeyedColumn<Message>> for Element<Message> {
    fn from(column: KeyedColumn<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let mut keys = stabby::vec::Vec::new();
            let mut children = stabby::vec::Vec::new();
            for (key, child) in column.children {
                keys.push(key);
                children.push(child.build(realize, arena));
            }
            arena.push(Node::KeyedColumn(KeyedColumnNode {
                keys,
                children,
                spacing: column.spacing.into(),
                padding: column.padding.into(),
                width: column.width.into(),
                height: column.height.into(),
                max_width: column.max_width.into(),
                align_items: column.align_items.into(),
            }))
        })
    }
}

/// Creates an empty [`KeyedColumn`].
pub fn keyed_column<Message>() -> KeyedColumn<Message> {
    KeyedColumn::new()
}
