use iced::{theme, widget::markdown as iced_markdown};

use crate::{
    bindings::iced::app::widgets::MarkdownNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::markdown` view from its node.
pub fn build<'a, Theme, Renderer>(node: MarkdownNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    // Leaked deliberately: iced's markdown `view` borrows its parsed
    // `Item`s for the lifetime of the returned element, and this element
    // is rebuilt fresh from guest state on every `view()` call.
    let items_vec: Vec<_> = iced_markdown::parse(&node.content).collect();
    let items: &'static [iced_markdown::Item] = Box::leak(items_vec.into_boxed_slice());
    let settings = iced_markdown::Settings::with_style(iced_markdown::Style::from_palette(
        theme::Palette::LIGHT,
    ));

    let mapper = node.on_link_click;
    iced_markdown::view(items.iter(), settings)
        .map(move |uri| Message::String { mapper, value: uri })
}
