use crate::{
    bindings::iced::app::widgets::ScrollableNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Scrollable` from its node and its already-built content element.
pub fn build<'a, Theme, Renderer>(
    node: ScrollableNode,
    content: impl iced::Widget<Message, Theme, Renderer> + 'a,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut scrollable = iced::widget::Scrollable::new(content);
    if let Some(width) = node.width {
        scrollable = scrollable.width(width);
    }
    if let Some(height) = node.height {
        scrollable = scrollable.height(height);
    }
    if let Some(direction) = node.direction {
        scrollable = scrollable.direction(direction);
    }
    if let Some(mapper) = node.on_scroll {
        scrollable =
            scrollable.on_scroll(
                move |v: iced::widget::scrollable::Scroll| Message::Viewport {
                    mapper,
                    value: v.viewport.into(),
                },
            );
    }
    scrollable
}
