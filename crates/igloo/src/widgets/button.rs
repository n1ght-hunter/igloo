use crate::{
    bindings::iced::app::{
        button::{self, HostButton},
        length::Length,
        padding::Padding,
        shared::Element,
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::{Resource, ResourceTable};

#[derive(Debug)]
pub struct ButtonResource {
    pub content: Resource<Element>,
    pub on_press: Option<u32>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub padding: Option<Padding>,
    pub clip: Option<bool>,
}

impl ButtonResource {
    pub fn to_iced_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let content = resource_table
            .delete(self.content)
            .unwrap()
            .to_element(resource_table);

        let mut button = iced::widget::Button::new(content);

        if let Some(w) = self.width {
            button = button.width(w);
        }
        if let Some(h) = self.height {
            button = button.height(h);
        }
        if let Some(p) = self.padding {
            button = button.padding(p);
        }
        if let Some(rep) = self.on_press {
            button = button.on_press(Message::Fixed { rep });
        }
        if let Some(clip) = self.clip {
            button = button.clip(clip);
        }
        button.into()
    }
}

impl HostButton for MyState {
    fn new(&mut self, content: Resource<Element>) -> Resource<button::Button> {
        self.table
            .push(ButtonResource {
                content,
                on_press: None,
                width: None,
                height: None,
                padding: None,
                clip: None,
            })
            .unwrap()
    }

    fn on_press(&mut self, self_: Resource<button::Button>, msg: u32) {
        let btn = self.table.get_mut(&self_).unwrap();
        btn.on_press = Some(msg);
    }

    fn width(&mut self, self_: Resource<button::Button>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<button::Button>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn padding(&mut self, self_: Resource<button::Button>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn clip(&mut self, self_: Resource<button::Button>, clip: bool) {
        self.table.get_mut(&self_).unwrap().clip = Some(clip);
    }

    fn into_element(&mut self, self_: Resource<button::Button>) -> Resource<Element> {
        let btn = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Button(btn)).unwrap()
    }

    fn drop(&mut self, rep: Resource<button::Button>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
