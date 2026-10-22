use crate::{
    bindings::iced::app::widgets::SpaceNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Space` from its node.
pub fn build<'a, Theme, Renderer>(
    node: SpaceNode,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut space = iced::widget::Space::new();
    if let Some(w) = node.width {
        space = space.width(w);
    }
    if let Some(h) = node.height {
        space = space.height(h);
    }
    space
}
