use crate::{
    bindings::iced::app::{shared::Element, svg},
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct SvgResource {
    pub path: String,
}

impl SvgResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let handle = iced::advanced::svg::Handle::from_path(self.path);
        iced::widget::Svg::new(handle).into()
    }
}

impl svg::HostSvg for MyState {
    fn new(&mut self, path: String) -> Resource<svg::Svg> {
        self.table.push(SvgResource { path }).unwrap()
    }

    fn into_element(&mut self, self_: Resource<svg::Svg>) -> Resource<Element> {
        let svg = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Svg(svg)).unwrap()
    }

    fn drop(&mut self, rep: Resource<svg::Svg>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
