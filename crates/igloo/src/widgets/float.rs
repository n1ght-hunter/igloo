use crate::{
    bindings::iced::app::{
        float::{self, HostFloat, Translation},
        shared::Element,
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use iced::Vector;
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct FloatResource {
    pub content: Resource<Element>,
    pub scale: Option<f32>,
    pub translation: Option<Translation>,
}

impl FloatResource {
    pub fn to_iced_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let content = resource_table
            .delete(self.content)
            .unwrap()
            .to_element(resource_table);

        let mut float = iced::widget::Float::new(content);

        if let Some(scale) = self.scale {
            float = float.scale(scale);
        }

        if let Some(translation) = self.translation {
            float = float.translate(move |_, _| Vector::new(translation.x, translation.y));
        }

        float.into()
    }
}

impl HostFloat for MyState {
    fn new(&mut self, content: Resource<Element>) -> Resource<float::Float> {
        self.table
            .push(FloatResource {
                content,
                scale: None,
                translation: None,
            })
            .unwrap()
    }

    fn scale(&mut self, self_: Resource<float::Float>, s: f32) {
        self.table.get_mut(&self_).unwrap().scale = Some(s);
    }

    fn translation(&mut self, self_: Resource<float::Float>, t: Translation) {
        self.table.get_mut(&self_).unwrap().translation = Some(t);
    }

    fn into_element(&mut self, self_: Resource<float::Float>) -> Resource<Element> {
        let float = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Float(float)).unwrap()
    }

    fn drop(&mut self, rep: Resource<float::Float>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
