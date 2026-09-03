//! Displays a widget on top of another when hovered.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::Position;
use crate::widgets::{Node, NodeId};

/// Displays a widget on top of another when hovered.
#[stabby::stabby(no_opt)]
#[derive(Debug)]
pub struct TooltipNode {
    pub content: NodeId,
    pub tooltip: NodeId,
    pub position: Position,
    pub gap: Opt<f32>,
    pub padding: Opt<f32>,
    pub snap_within_viewport: Opt<bool>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    tooltip: TooltipNode,
) -> iced::Element<'a, crate::widgets::Emitted> {
    let content = crate::widgets::build_node(nodes, tooltip.content);
    let tooltip_content = crate::widgets::build_node(nodes, tooltip.tooltip);
    let mut widget = iced::widget::Tooltip::new(
        content,
        tooltip_content,
        iced::widget::tooltip::Position::from(tooltip.position),
    );
    if let Some(gap) = Option::<f32>::from(tooltip.gap) {
        widget = widget.gap(gap);
    }
    if let Some(padding) = Option::<f32>::from(tooltip.padding) {
        widget = widget.padding(padding);
    }
    if let Some(snap) = Option::<bool>::from(tooltip.snap_within_viewport) {
        widget = widget.snap_within_viewport(snap);
    }
    widget.into()
}

/// A builder for a [`TooltipNode`].
pub struct Tooltip<Message> {
    content: Element<Message>,
    tooltip: Element<Message>,
    position: Position,
    gap: Option<f32>,
    padding: Option<f32>,
    snap_within_viewport: Option<bool>,
}

impl<Message> Tooltip<Message> {
    /// Creates a tooltip showing `tooltip` over `content`.
    pub fn new(
        content: impl Into<Element<Message>>,
        tooltip: impl Into<Element<Message>>,
        position: Position,
    ) -> Self {
        Self {
            content: content.into(),
            tooltip: tooltip.into(),
            position,
            gap: None,
            padding: None,
            snap_within_viewport: None,
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = Some(gap);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = Some(snap);
        self
    }
}

impl<Message> std::fmt::Debug for Tooltip<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tooltip")
            .field("content", &self.content)
            .field("tooltip", &self.tooltip)
            .field("position", &self.position)
            .field("gap", &self.gap)
            .field("padding", &self.padding)
            .field("snap_within_viewport", &self.snap_within_viewport)
            .finish()
    }
}

impl<Message: 'static> From<Tooltip<Message>> for Element<Message> {
    fn from(tooltip: Tooltip<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let content = tooltip.content.build(realize, arena);
            let tooltip_content = tooltip.tooltip.build(realize, arena);
            arena.push(Node::Tooltip(TooltipNode {
                content,
                tooltip: tooltip_content,
                position: tooltip.position,
                gap: tooltip.gap.into(),
                padding: tooltip.padding.into(),
                snap_within_viewport: tooltip.snap_within_viewport.into(),
            }))
        })
    }
}

/// Creates a [`Tooltip`] showing `tooltip` over `content`.
pub fn tooltip<Message>(
    content: impl Into<Element<Message>>,
    tooltip: impl Into<Element<Message>>,
    position: Position,
) -> Tooltip<Message> {
    Tooltip::new(content, tooltip, position)
}
