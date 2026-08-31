use crate::{
    bindings::iced::app::{
        shared::{Element, Pixels},
        tooltip::{self, HostTooltip, Position},
    },
    plugin_manager::MyState,
    widgets::{Message, ToElement, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

impl From<Position> for iced::widget::tooltip::Position {
    fn from(value: Position) -> Self {
        match value {
            Position::Top => iced::widget::tooltip::Position::Top,
            Position::Bottom => iced::widget::tooltip::Position::Bottom,
            Position::Left => iced::widget::tooltip::Position::Left,
            Position::Right => iced::widget::tooltip::Position::Right,
            Position::FollowCursor => iced::widget::tooltip::Position::FollowCursor,
        }
    }
}

#[derive(Debug)]
pub struct TooltipResource {
    pub content: Resource<Element>,
    pub tooltip: Resource<Element>,
    pub position: Position,
    pub gap: Option<Pixels>,
    pub padding: Option<Pixels>,
    pub snap_within_viewport: Option<bool>,
}

impl TooltipResource {
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
        let tooltip_content = resource_table
            .delete(self.tooltip)
            .unwrap()
            .to_element(resource_table);

        let mut tooltip =
            iced::widget::Tooltip::new(content, tooltip_content, self.position.into());

        if let Some(gap) = self.gap {
            tooltip = tooltip.gap(gap);
        }
        if let Some(padding) = self.padding {
            tooltip = tooltip.padding(padding);
        }
        if let Some(snap) = self.snap_within_viewport {
            tooltip = tooltip.snap_within_viewport(snap);
        }

        tooltip.into()
    }
}

impl HostTooltip for MyState {
    fn new(
        &mut self,
        content: Resource<Element>,
        tooltip: Resource<Element>,
        position: Position,
    ) -> Resource<tooltip::Tooltip> {
        self.table
            .push(TooltipResource {
                content,
                tooltip,
                position,
                gap: None,
                padding: None,
                snap_within_viewport: None,
            })
            .unwrap()
    }

    fn gap(&mut self, self_: Resource<tooltip::Tooltip>, g: Pixels) {
        self.table.get_mut(&self_).unwrap().gap = Some(g);
    }

    fn padding(&mut self, self_: Resource<tooltip::Tooltip>, p: Pixels) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn snap_within_viewport(&mut self, self_: Resource<tooltip::Tooltip>, snap: bool) {
        self.table.get_mut(&self_).unwrap().snap_within_viewport = Some(snap);
    }

    fn into_element(&mut self, self_: Resource<tooltip::Tooltip>) -> Resource<Element> {
        let tooltip = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Tooltip(tooltip)).unwrap()
    }

    fn drop(&mut self, rep: Resource<tooltip::Tooltip>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
