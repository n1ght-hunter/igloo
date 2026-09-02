use crate::{
    bindings::iced::app::widgets::RowNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Row` from its node and its already-built children.
pub fn build<'a, Theme, Renderer>(
    node: RowNode,
    children: Vec<iced::Element<'a, Message, Theme, Renderer>>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut row = iced::widget::Row::with_children(children);
    if let Some(spacing) = node.spacing {
        row = row.spacing(spacing);
    }
    if let Some(padding) = node.padding {
        row = row.padding(padding);
    }
    if let Some(w) = node.width {
        row = row.width(w);
    }
    if let Some(h) = node.height {
        row = row.height(h);
    }
    if let Some(align_y) = node.align_y {
        row = row.align_y(align_y);
    }
    if let Some(clip) = node.clip {
        row = row.clip(clip);
    }

    // must be last as it returns a different element type
    if let Some(true) = node.wrap {
        return iced::Element::from(row.wrap());
    }

    row.into()
}
