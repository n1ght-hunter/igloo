use super::Message;
use crate::{
    bindings::iced::app::table::Table,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Table {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Table { columns, rows, .. } = self;

        let header_row = iced::widget::Row::from_iter(
            columns
                .into_iter()
                .map(|h| resource_table.delete(h).unwrap().to_element(resource_table)),
        );

        let mut column = iced::widget::Column::new().push(header_row);

        for row in rows {
            column = column.push(
                resource_table
                    .delete(row)
                    .unwrap()
                    .to_element(resource_table),
            );
        }

        column.into()
    }
}
