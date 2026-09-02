use crate::{
    bindings::iced::app::widgets::GridNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Grid` from its node and its already-built children.
pub fn build<'a, Theme, Renderer>(
    node: GridNode,
    children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut grid = iced::widget::Grid::with_children(children);

    if let Some(spacing) = node.spacing {
        grid = grid.spacing(spacing);
    }
    if let Some(width) = node.width {
        grid = grid.width(width);
    }
    if let Some(height) = node.height {
        grid = grid.height(height);
    }
    if let Some(columns) = node.columns {
        grid = grid.columns(columns as usize);
    }
    if let Some(fluid) = node.fluid {
        grid = grid.fluid(fluid);
    }

    grid.into()
}
