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
        let Space { width, height } = self;

        let mut space = iced::widget::Space::new();

        if let Some(width) = width {
            space = space.width(width);
        }

        if let Some(height) = height {
            space = space.height(height);
        }

        space.into()
    }
}
