use crate::{
    bindings::iced::app::widgets::KeyedColumnNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::keyed::Column` from its node and its already-built children.
pub fn build<'a, Theme, Renderer>(
    node: KeyedColumnNode,
    children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut keyed_column = iced::widget::keyed::Column::from_vecs(node.keys, children);
    if let Some(spacing) = node.spacing {
        keyed_column = keyed_column.spacing(spacing);
    }
    if let Some(padding) = node.padding {
        keyed_column = keyed_column.padding(padding);
    }
    if let Some(w) = node.width {
        keyed_column = keyed_column.width(w);
    }
    if let Some(h) = node.height {
        keyed_column = keyed_column.height(h);
    }
    if let Some(max) = node.max_width {
        keyed_column = keyed_column.max_width(max);
    }
    if let Some(align) = node.align_items {
        keyed_column = keyed_column.align_items(align.into());
    }
    keyed_column.into()
}
