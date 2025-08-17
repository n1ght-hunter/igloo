use super::Message;
use crate::{
    bindings::iced::app::grid::Grid,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Grid {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Grid {
            elements,
            spacing,
            width,
            height,
            columns,
            fluid,
        } = self;

        let children: Vec<_> = elements
            .into_iter()
            .map(|e| resource_table.delete(e).unwrap().to_element(resource_table))
            .collect();

        let mut grid = iced::widget::Grid::with_children(children);

        if let Some(spacing) = spacing {
            grid = grid.spacing(spacing);
        }

        if let Some(width) = width {
            grid = grid.width(width);
        }

        if let Some(height) = height {
            grid = grid.height(height);
        }

        if let Some(columns) = columns {
            grid = grid.columns(columns as usize);
        }

        if let Some(fluid) = fluid {
            grid = grid.fluid(fluid);
        }

        grid.into()
    }
}
