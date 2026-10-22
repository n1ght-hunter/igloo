use iced::{Widget, widget::markdown as iced_markdown};

use crate::{
    bindings::iced::app::widgets::MarkdownNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::markdown` view from its node.
pub fn build<'a, Theme, Renderer>(
    node: MarkdownNode,
    items: &'a [iced_markdown::Item],
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let settings = iced_markdown::Settings::default();

    let mapper = node.on_link_click;
    iced_markdown::view(
        items,
        settings,
        <Theme as iced::theme::Base>::default(iced::theme::Mode::default()),
    )
    .map(move |uri| Message::String { mapper, value: uri })
}
