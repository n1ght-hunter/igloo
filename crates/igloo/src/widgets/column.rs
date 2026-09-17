use crate::{
    bindings::iced::app::widgets::ColumnNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Column` from its node and its already-built children.
pub fn build<'a, Theme, Renderer>(
    node: ColumnNode,
    children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut col = iced::widget::Column::with_children(children);
    if let Some(spacing) = node.spacing {
        col = col.spacing(spacing);
    }
    if let Some(padding) = node.padding {
        col = col.padding(padding);
    }
    let mut width: Option<iced::Length> = node.width.map(Into::into);
    if let Some(max) = node.max_width {
        width = Some(width.unwrap_or(iced::Length::Shrink).max(max));
    }
    if let Some(w) = width {
        col = col.width(w);
    }
    if let Some(h) = node.height {
        col = col.height(h);
    }
    if let Some(align) = node.align_x {
        col = col.align_x(align);
    }
    if let Some(clip) = node.clip {
        col = col.clip(clip);
    }
    col.into()
}
