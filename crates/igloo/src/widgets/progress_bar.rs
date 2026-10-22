use crate::{
    bindings::iced::app::widgets::ProgressBarNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::ProgressBar` from its node.
pub fn build<'a, Theme, Renderer>(
    node: ProgressBarNode,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut bar = iced::widget::ProgressBar::new(node.range_start..=node.range_end, node.value);

    if let Some(length) = node.length {
        bar = bar.length(length);
    }
    if let Some(girth) = node.girth {
        bar = bar.girth(girth);
    }
    if let Some(vertical) = node.vertical {
        bar = bar.vertical(vertical);
    }
    bar
}
