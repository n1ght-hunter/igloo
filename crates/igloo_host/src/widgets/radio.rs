use super::Message;
use crate::{
    bindings::iced::app::radio::Radio,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Radio {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Radio {
            label,
            on_select,
            size,
            width,
            spacing,
            text_size,
            text_line_height,
            text_wrapping,
            text_shaping,
            is_selected
        } = self;

        let value1 = 1;

        let value2 = if is_selected {
            1
        } else {
            2
        };

        let mut radio = iced::widget::Radio::new(label, value1, Some(value2), move |v| Message {
            id: on_select,
            content: crate::bindings::Message::U64Type(v),
        });

        if let Some(size) = size {
            radio = radio.size(size);
        }

        if let Some(width) = width {
            radio = radio.width(width);
        }

        if let Some(spacing) = spacing {
            radio = radio.spacing(spacing);
        }

        if let Some(text_size) = text_size {
            radio = radio.text_size(text_size);
        }

        if let Some(text_line_height) = text_line_height {
            radio = radio.text_line_height(text_line_height);
        }

        if let Some(text_wrapping) = text_wrapping {
            radio = radio.text_wrapping(text_wrapping.into());
        }

        if let Some(text_shaping) = text_shaping {
            radio = radio.text_shaping(text_shaping.into());
        }

        radio.into()
    }
}
