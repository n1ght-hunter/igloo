use crate::{
    bindings::iced::app::{float::Float},
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};
use iced::Vector;
use super::Message;

impl ToElement for Float {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Float {
            content,
            scale,
            translation,
        } = self;

        let content = resource_table
            .delete(content)
            .unwrap()
            .to_element(resource_table);

        let mut float = iced::widget::Float::new(content);

        if let Some(scale) = scale {
            float = float.scale(scale);
        }

        if let Some(translation) = translation {
            float = float.translate(move |_, _| Vector::new(translation.x, translation.y));
        }

        float.into()
    }
}
