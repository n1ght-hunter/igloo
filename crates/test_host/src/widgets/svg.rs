use super::Message;
use crate::{
    bindings::iced::app::svg::Svg,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Svg {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let handle = iced::advanced::svg::Handle::from_path(self.path);
        iced::widget::Svg::new(handle).into()
    }
}
