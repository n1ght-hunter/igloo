use super::Message;
use crate::{
    bindings::iced::app::markdown::Markdown,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};
use iced::{theme, widget::markdown};

impl ToElement for Markdown {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let items_vec: Vec<_> = markdown::parse(&self.content).collect();
        let items = Box::leak(items_vec.into_boxed_slice());
        let settings =
            markdown::Settings::with_style(markdown::Style::from_palette(theme::Palette::LIGHT));
        markdown::view(items.iter(), settings).map(|_| Message {
            id: 0,
            content: crate::bindings::Message::Empty,
        })
    }
}
