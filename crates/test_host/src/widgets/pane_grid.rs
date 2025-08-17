use super::Message;
use crate::{
    bindings::iced::app::pane_grid::PaneGrid,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for PaneGrid {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let children: Vec<_> = self
            .children
            .into_iter()
            .map(|e| resource_table.delete(e).unwrap().to_element(resource_table))
            .collect();
        iced::widget::Column::with_children(children).into()
    }
}
