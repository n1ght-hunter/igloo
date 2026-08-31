use crate::{
    bindings::iced::app::{markdown, shared::Element},
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use iced::{theme, widget::markdown as iced_markdown};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct MarkdownResource {
    pub content: String,
    pub on_link_click: u32,
}

impl MarkdownResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        // Leaked deliberately: iced's markdown `view` borrows its parsed
        // `Item`s for the lifetime of the returned element, and this element
        // is rebuilt fresh from guest state on every `view()` call.
        let items_vec: Vec<_> = iced_markdown::parse(&self.content).collect();
        let items: &'static [iced_markdown::Item] = Box::leak(items_vec.into_boxed_slice());
        let settings = iced_markdown::Settings::with_style(iced_markdown::Style::from_palette(
            theme::Palette::LIGHT,
        ));

        let mapper = self.on_link_click;
        iced_markdown::view(items.iter(), settings)
            .map(move |uri| Message::String { mapper, value: uri })
    }
}

impl markdown::HostMarkdown for MyState {
    fn new(&mut self, content: String, on_link_click: u32) -> Resource<markdown::Markdown> {
        self.table
            .push(MarkdownResource {
                content,
                on_link_click,
            })
            .unwrap()
    }

    fn into_element(&mut self, self_: Resource<markdown::Markdown>) -> Resource<Element> {
        let markdown = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Markdown(markdown)).unwrap()
    }

    fn drop(&mut self, rep: Resource<markdown::Markdown>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
