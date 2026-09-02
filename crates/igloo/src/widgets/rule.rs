use crate::{
    bindings::iced::app::widgets::RuleNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Rule` from its node.
pub fn build<'a, Theme, Renderer>(node: RuleNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    if node.is_horizontal {
        iced::widget::rule::horizontal(node.thickness).into()
    } else {
        iced::widget::rule::vertical(node.thickness).into()
    }
}
