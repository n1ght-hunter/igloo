use super::Message;
use crate::{
    bindings::iced::app::row::Row,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Row {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Row {
            elements,
            spacing,
            padding,
            width,
            height,
            align_y,
            clip,
            wrap,
        } = self;

        let elements = elements
            .into_iter()
            .map(|e| resource_table.delete(e).unwrap().to_element(resource_table));
        let mut row = iced::widget::Row::with_children(elements);

        if let Some(spacing) = spacing {
            row = row.spacing(spacing);
        }

        if let Some(padding) = padding {
            row = row.padding(padding);
        }

        if let Some(width) = width {
            row = row.width(width);
        }

        if let Some(height) = height {
            row = row.height(height);
        }

        if let Some(align_y) = align_y {
            row = row.align_y(align_y);
        }

        if let Some(clip) = clip {
            row = row.clip(clip);
        }

        // must be last as it returns a different element type
        if let Some(true) = wrap {
            return iced::Element::from(row.wrap());
        }

        row.into()
    }
}
