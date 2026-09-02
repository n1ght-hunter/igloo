use crate::{
    bindings::iced::app::widgets::RadioNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Radio` from its node.
pub fn build<'a, Theme, Renderer>(node: RadioNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let on_select = node.on_select;
    let value: u32 = 1;
    let selected = if node.is_selected { Some(value) } else { None };

    let mut radio = iced::widget::Radio::new(node.label, value, selected, move |_: u32| {
        Message::Fixed { rep: on_select }
    });

    if let Some(size) = node.size {
        radio = radio.size(size);
    }
    if let Some(width) = node.width {
        radio = radio.width(width);
    }
    if let Some(spacing) = node.spacing {
        radio = radio.spacing(spacing);
    }
    if let Some(text_size) = node.text_size {
        radio = radio.text_size(text_size);
    }
    if let Some(lh) = node.text_line_height {
        radio = radio.text_line_height(lh);
    }
    if let Some(wrapping) = node.text_wrapping {
        radio = radio.text_wrapping(wrapping.into());
    }
    if let Some(shaping) = node.text_shaping {
        radio = radio.text_shaping(shaping.into());
    }
    radio.into()
}
