use super::Message;
use crate::{
    bindings::iced::app::toggler::Toggler,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Toggler {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Toggler {
            is_toggled,
            label,
            on_toggle,
            size,
            width,
            text_size,
            text_line_height,
            text_alignment,
            text_shaping,
            text_wrapping,
            spacing,
        } = self;

        let mut toggler = iced::widget::Toggler::new(is_toggled);

        if let Some(label) = label {
            toggler = toggler.label(label);
        }

        if let Some(message) = on_toggle {
            toggler = toggler.on_toggle(move |v| Message {
                id: message,
                content: crate::bindings::Message::BoolType(v),
            });
        }

        if let Some(size) = size {
            toggler = toggler.size(size);
        }

        if let Some(width) = width {
            toggler = toggler.width(width);
        }

        if let Some(text_size) = text_size {
            toggler = toggler.text_size(text_size);
        }

        if let Some(text_line_height) = text_line_height {
            toggler = toggler.text_line_height(text_line_height);
        }

        if let Some(text_alignment) = text_alignment {
            toggler = toggler.text_alignment(text_alignment);
        }

        if let Some(text_shaping) = text_shaping {
            toggler = toggler.text_shaping(text_shaping.into());
        }

        if let Some(text_wrapping) = text_wrapping {
            toggler = toggler.text_wrapping(text_wrapping.into());
        }

        if let Some(spacing) = spacing {
            toggler = toggler.spacing(spacing);
        }

        toggler.into()
    }
}
