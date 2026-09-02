use iced_core::Pixels;

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, TooltipNode};

/// The position of the tooltip. Defaults to following the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// The tooltip will appear on the top of the widget.
    #[default]
    Top,
    /// The tooltip will appear on the bottom of the widget.
    Bottom,
    /// The tooltip will appear on the left of the widget.
    Left,
    /// The tooltip will appear on the right of the widget.
    Right,
    /// The tooltip will follow the cursor.
    FollowCursor,
}

/// Displays a widget on top of another when hovered.
pub struct Tooltip<Message> {
    content: Element<Message>,
    tooltip: Element<Message>,
    position: Position,
    gap: Option<f32>,
    padding: Option<f32>,
    snap_within_viewport: Option<bool>,
}

impl<Message> Tooltip<Message> {
    /// Create a new tooltip.
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

    /// Set the gap between the content and its tooltip.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = Some(gap.into().0);
        self
    }

    /// Set the padding of the tooltip.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into().0);
        self
    }

    /// Sets whether the Tooltip is snapped within the viewport.
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = Some(snap);
        self
    }
}

impl<Message: 'static> From<Tooltip<Message>> for Element<Message> {
    fn from(tooltip: Tooltip<Message>) -> Self {
        Element::new(move |realize, arena| {
            let content = tooltip.content.build(realize, arena);
            let tooltip_content = tooltip.tooltip.build(realize, arena);
            let node = TooltipNode {
                content,
                tooltip: tooltip_content,
                position: tooltip.position.into(),
                gap: tooltip.gap,
                padding: tooltip.padding,
                snap_within_viewport: tooltip.snap_within_viewport,
            };
            arena.push(Node::Tooltip(node))
        })
    }
}
