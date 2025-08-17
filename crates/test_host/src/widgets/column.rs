use crate::{
    bindings::iced::app::{column::Column},
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

use super::Message;

impl ToElement for Column {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Column {
            elements,
            spacing,
            padding,
            width,
            height,
            max_width,
            align_x,
            clip,
        } = self;

        let elements: Vec<iced::Element<'a, Message, Theme, Renderer>> = elements
            .into_iter()
            .map(|e| resource_table.delete(e).unwrap().to_element(resource_table))
            .collect();
        let mut column = iced::widget::Column::with_children(elements);

        if let Some(spacing) = spacing {
            column = column.spacing(spacing);
        }

        if let Some(padding) = padding {
            column = column.padding(padding);
        }

        if let Some(width) = width {
            column = column.width(width);
        }

        if let Some(height) = height {
            column = column.height(height);
        }

        if let Some(max_width) = max_width {
            column = column.max_width(max_width);
        }

        if let Some(align_x) = align_x {
            column = column.align_x(align_x);
        }

        if let Some(clip) = clip {
            column = column.clip(clip);
        }

        column.into()
    }
}
