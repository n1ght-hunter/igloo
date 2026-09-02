use crate::{
    bindings::iced::app::widgets::TogglerNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Toggler` from its node.
pub fn build<'a, Theme, Renderer>(node: TogglerNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut toggler = iced::widget::Toggler::new(node.is_toggled);

    if let Some(label) = node.label {
        toggler = toggler.label(label);
    }
    if let Some(mapper) = node.on_toggle {
        toggler = toggler.on_toggle(move |value| Message::Bool { mapper, value });
    }
    if let Some(size) = node.size {
        toggler = toggler.size(size);
    }
    if let Some(width) = node.width {
        toggler = toggler.width(width);
    }
    if let Some(text_size) = node.text_size {
        toggler = toggler.text_size(text_size);
    }
    if let Some(lh) = node.text_line_height {
        toggler = toggler.text_line_height(lh);
    }
    if let Some(alignment) = node.text_alignment {
        let alignment: iced::alignment::Horizontal = alignment.into();
        toggler = toggler.text_alignment(alignment);
    }
    if let Some(shaping) = node.text_shaping {
        toggler = toggler.text_shaping(shaping.into());
    }
    if let Some(wrapping) = node.text_wrapping {
        toggler = toggler.text_wrapping(wrapping.into());
    }
    if let Some(spacing) = node.spacing {
        toggler = toggler.spacing(spacing);
    }
    toggler.into()
}
