use iced_core::Pixels;

use crate::{Element, bindings::iced::app::element::tooltip_to_element, element::Widget};

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
#[derive(Debug)]
pub struct Tooltip<Message> {
    content: Element<Message>,
    tooltip: Element<Message>,
    position: Position,
    gap: Option<Pixels>,
    padding: Option<Pixels>,
    snap_within_viewport: Option<bool>,
}

impl<Message> Tooltip<Message> {
    /// Create a new tooltip.
    pub fn new(
        content: impl Into<Element<Message>>,
        tooltip: impl Into<Element<Message>>,
        position: Position,
    ) -> Self {
        Tooltip {
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
        self.gap = Some(gap.into());
        self
    }

    /// Set the padding of the tooltip.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets whether the Tooltip is snapped within the viewport.
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = Some(snap);
        self
    }
}

impl<Message> Widget<Message> for Tooltip<Message> {
    fn as_element<'b>(
        self: Box<Self>,
        create_message: &'b dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        tooltip_to_element(crate::bindings::iced::app::tooltip::Tooltip {
            content: self.content.as_element(create_message),
            tooltip: self.tooltip.as_element(create_message),
            position: self.position.into(),
            gap: self.gap.map(|g| g.into()),
            padding: self.padding.map(|p| p.into()),
            snap_within_viewport: self.snap_within_viewport,
        })
    }
}

impl<Message: 'static> From<Tooltip<Message>> for Element<Message> {
    fn from(tooltip: Tooltip<Message>) -> Self {
        Element::new(Box::new(tooltip))
    }
}
