use crate::{
    bindings::iced::app::widgets::RuleNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Rule` from its node.
pub fn build<'a, Theme, Renderer>(
    node: RuleNode,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    if node.is_horizontal {
        iced::widget::rule::horizontal(node.thickness)
    } else {
        iced::widget::rule::vertical(node.thickness)
    }
}
