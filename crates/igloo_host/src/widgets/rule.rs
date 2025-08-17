use crate::{
    bindings::iced::app::rule::Rule,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};
use super::Message;

/// Display a horizontal or vertical rule for dividing content.
impl ToElement for Rule {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Rule {
            is_horizontal,
            thickness,
        } = self;
        if is_horizontal {
            iced::widget::horizontal_rule(thickness).into()
        } else {
            iced::widget::vertical_rule(thickness).into()
        }
    }
}
