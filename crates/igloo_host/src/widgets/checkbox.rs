use crate::{
    bindings::iced::app::checkbox::Checkbox,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

use super::Message;

impl ToElement for Checkbox {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Checkbox {
            label,
            is_checked,
            on_toggle,
            size,
            width,
            height: _height,
            spacing,
            text_size,
            text_line_height,
            text_wrapping,
            text_shaping,
        } = self;

        let mut checkbox = iced::widget::Checkbox::new(label, is_checked);

        if let Some(message) = on_toggle {
            checkbox = checkbox.on_toggle(move |v| Message {
                id: message,
                content: crate::bindings::Message::BoolType(v),
            });
        }

        if let Some(size) = size {
            checkbox = checkbox.size(size);
        }

        if let Some(width) = width {
            checkbox = checkbox.width(width);
        }

        if let Some(spacing) = spacing {
            checkbox = checkbox.spacing(spacing);
        }

        if let Some(text_size) = text_size {
            checkbox = checkbox.text_size(text_size);
        }

        if let Some(text_line_height) = text_line_height {
            checkbox = checkbox.text_line_height(text_line_height);
        }

        if let Some(text_wrapping) = text_wrapping {
            checkbox = checkbox.text_wrapping(text_wrapping.into());
        }

        if let Some(text_shaping) = text_shaping {
            checkbox = checkbox.text_shaping(text_shaping.into());
        }

        checkbox.into()
    }
}
