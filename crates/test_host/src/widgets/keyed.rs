use super::Message;
use crate::{
    bindings::iced::app::keyed::KeyedColumn,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for KeyedColumn {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let KeyedColumn {
            keys,
            children,
            spacing,
            padding,
            width,
            height,
            max_width,
            align_items,
        } = self;

        let mut keyed_column = iced::widget::keyed::Column::from_vecs(
            keys,
            children
                .into_iter()
                .map(|c| resource_table.delete(c).unwrap().to_element(resource_table))
                .collect(),
        );

        if let Some(spacing) = spacing {
            keyed_column = keyed_column.spacing(spacing);
        }

        if let Some(padding) = padding {
            keyed_column = keyed_column.padding(padding);
        }

        if let Some(width) = width {
            keyed_column = keyed_column.width(width);
        }

        if let Some(height) = height {
            keyed_column = keyed_column.height(height);
        }

        if let Some(max_width) = max_width {
            keyed_column = keyed_column.max_width(max_width);
        }

        if let Some(align_items) = align_items {
            keyed_column = keyed_column.align_items(align_items.into());
        }

        keyed_column.into()
    }
}
