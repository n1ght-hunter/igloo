use crate::{
    bindings::iced::app::widgets::{Position, TooltipNode},
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

impl From<Position> for iced::widget::tooltip::Position {
    fn from(value: Position) -> Self {
        match value {
            Position::Top => iced::widget::tooltip::Position::Top,
            Position::Bottom => iced::widget::tooltip::Position::Bottom,
            Position::Left => iced::widget::tooltip::Position::Left,
            Position::Right => iced::widget::tooltip::Position::Right,
            Position::FollowCursor => iced::widget::tooltip::Position::FollowCursor,
        }
    }
}

/// Builds an `iced::Tooltip` from its node and its already-built content and overlay elements.
pub fn build<'a, Theme, Renderer>(
    node: TooltipNode,
    content: iced::Element<'a, Message, Theme, Renderer>,
    tooltip_content: iced::Element<'a, Message, Theme, Renderer>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut tooltip = iced::widget::Tooltip::new(content, tooltip_content, node.position.into());

    if let Some(gap) = node.gap {
        tooltip = tooltip.gap(gap);
    }
    if let Some(padding) = node.padding {
        tooltip = tooltip.padding(padding);
    }
    if let Some(snap) = node.snap_within_viewport {
        tooltip = tooltip.snap_within_viewport(snap);
    }

    tooltip.into()
}
