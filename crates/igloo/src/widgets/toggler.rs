use crate::{
    bindings::iced::app::widgets::TogglerNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Toggler` from its node.
pub fn build<'a, Theme, Renderer>(
    node: TogglerNode,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
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
    if let Some(lh) = node.line_height {
        toggler = toggler.line_height(lh);
    }
    if let Some(alignment) = node.alignment {
        let alignment: iced::alignment::Horizontal = alignment.into();
        toggler = toggler.alignment(alignment);
    }
    if let Some(shaping) = node.shaping {
        toggler = toggler.shaping(shaping.into());
    }
    if let Some(wrapping) = node.wrapping {
        toggler = toggler.wrapping(wrapping.into());
    }
    if let Some(spacing) = node.spacing {
        toggler = toggler.spacing(spacing);
    }
    toggler
}
