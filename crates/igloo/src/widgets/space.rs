use crate::{
    bindings::iced::app::{length::Length, shared::Element, space},
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug, Default)]
pub struct SpaceResource {
    pub width: Option<Length>,
    pub height: Option<Length>,
}

impl SpaceResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mut space = iced::widget::Space::new();
        if let Some(w) = self.width {
            space = space.width(w);
        }
        if let Some(h) = self.height {
            space = space.height(h);
        }
        space.into()
    }
}

impl space::HostSpace for MyState {
    fn new(&mut self) -> Resource<space::Space> {
        self.table.push(SpaceResource::default()).unwrap()
    }

    fn width(&mut self, self_: Resource<space::Space>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<space::Space>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn into_element(&mut self, self_: Resource<space::Space>) -> Resource<Element> {
        let space = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Space(space)).unwrap()
    }

    fn drop(&mut self, rep: Resource<space::Space>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
