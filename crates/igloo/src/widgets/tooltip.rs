use super::Message;
use crate::{
    bindings::iced::app::tooltip::Tooltip,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Tooltip {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Tooltip {
            content,
            tooltip,
            position,
            gap,
            padding,
            snap_within_viewport,
        } = self;

        let content = resource_table
            .delete(content)
            .unwrap()
            .to_element(resource_table);

        let tooltip = resource_table
            .delete(tooltip)
            .unwrap()
            .to_element(resource_table);

        let mut tooltip = iced::widget::Tooltip::new(content, tooltip, position.into());

        if let Some(gap) = gap {
            tooltip = tooltip.gap(gap);
        }

        if let Some(padding) = padding {
            tooltip = tooltip.padding(padding);
        }

        if let Some(snap_within_viewport) = snap_within_viewport {
            tooltip = tooltip.snap_within_viewport(snap_within_viewport);
        }

        tooltip.into()
    }
}
