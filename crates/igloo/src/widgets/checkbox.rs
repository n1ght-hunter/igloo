use crate::{
    bindings::iced::app::widgets::CheckboxNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Checkbox` from its node.
pub fn build<'a, Theme, Renderer>(node: CheckboxNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut checkbox = iced::widget::Checkbox::new(node.is_checked);

    if let Some(label) = node.label {
        checkbox = checkbox.label(label);
    }
    if let Some(mapper) = node.on_toggle {
        checkbox = checkbox.on_toggle(move |value| Message::Bool { mapper, value });
    }
    if let Some(size) = node.size {
        checkbox = checkbox.size(size);
    }
    if let Some(width) = node.width {
        checkbox = checkbox.width(width);
    }
    if let Some(spacing) = node.spacing {
        checkbox = checkbox.spacing(spacing);
    }
    if let Some(text_size) = node.text_size {
        checkbox = checkbox.text_size(text_size);
    }
    if let Some(lh) = node.line_height {
        checkbox = checkbox.line_height(lh);
    }
    if let Some(wrapping) = node.wrapping {
        checkbox = checkbox.wrapping(wrapping.into());
    }
    if let Some(shaping) = node.shaping {
        checkbox = checkbox.shaping(shaping.into());
    }
    checkbox.into()
}
