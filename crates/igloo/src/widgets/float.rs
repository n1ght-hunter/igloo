use iced::Vector;

use crate::{
    bindings::iced::app::widgets::FloatNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Float` from its node and its already-built content element.
pub fn build<'a, Theme, Renderer>(
    node: FloatNode,
    content: impl iced::Widget<Message, Theme, Renderer> + 'a,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut float = iced::widget::Float::new(content);

    if let Some(scale) = node.scale {
        float = float.scale(scale);
    }

    if let Some(translation) = node.translation {
        float = float.translate(move |_, _| Vector::new(translation.x, translation.y));
    }

    float
}
