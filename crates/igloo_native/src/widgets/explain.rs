//! Wraps a node with a debug overlay showing its layout bounds.

use crate::widgets::element::{Arena, Realize};
use crate::widgets::types::Color;
use crate::widgets::{Node, NodeId};

/// Wraps a node with a debug overlay showing its layout bounds.
#[stabby::stabby]
#[derive(Debug)]
pub struct ExplainNode {
    pub content: NodeId,
    pub color: Color,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    explain: ExplainNode,
) -> iced::Element<'a, crate::widgets::Emitted> {
    let content = crate::widgets::build_node(nodes, explain.content);
    content.explain(iced::Color::from(explain.color))
}

impl<Message: 'static> super::Element<Message> {
    /// Wraps this element with a debug overlay of the given color.
    pub fn explain(self, color: Color) -> Self {
        super::Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let content = self.build(realize, arena);
            arena.push(Node::Explain(ExplainNode { content, color }))
        })
    }
}
