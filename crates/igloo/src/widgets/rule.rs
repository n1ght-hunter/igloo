use crate::{
    bindings::iced::app::{rule, shared::Element},
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct RuleResource {
    pub is_horizontal: bool,
    pub thickness: f32,
}

impl RuleResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        if self.is_horizontal {
            iced::widget::rule::horizontal(self.thickness).into()
        } else {
            iced::widget::rule::vertical(self.thickness).into()
        }
    }
}

impl rule::HostRule for MyState {
    fn new(&mut self, is_horizontal: bool, thickness: f32) -> Resource<rule::Rule> {
        self.table
            .push(RuleResource {
                is_horizontal,
                thickness,
            })
            .unwrap()
    }

    fn into_element(&mut self, self_: Resource<rule::Rule>) -> Resource<Element> {
        let rule = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Rule(rule)).unwrap()
    }

    fn drop(&mut self, rep: Resource<rule::Rule>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
