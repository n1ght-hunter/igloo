use crate::{
    bindings::iced::app::widgets::TextNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Text` from its node.
pub fn build<'a, Theme, Renderer>(node: TextNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut text = iced::widget::Text::new(node.content);
    if let Some(size) = node.size {
        text = text.size(size);
    }
    if let Some(lh) = node.line_height {
        text = text.line_height(lh);
    }
    if let Some(w) = node.width {
        text = text.width(w);
    }
    if let Some(h) = node.height {
        text = text.height(h);
    }
    if node.center {
        text = text.center();
    }
    if let Some(align) = node.align_x {
        text = text.align_x(align);
    }
    if let Some(align) = node.align_y {
        text = text.align_y(align);
    }
    // TODO: color requires Theme::Class<'a>: From<StyleFn<'a, Theme>> bound
    // which is more complex to wire through generics
    text.into()
}
