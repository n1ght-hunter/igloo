use super::Message;
use crate::{
    bindings::iced::app::space::Space,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Space {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Space {
            width,
            height,
        } = self;

       iced::widget::Space::new(width, height).into()
    }
}
