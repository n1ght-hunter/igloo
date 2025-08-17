use crate::{
    bindings::iced::app::combo_box::ComboBox,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

use super::Message;

impl ToElement for ComboBox {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let ComboBox {
            options,
            placeholder,
            selected,
            on_selected,
            on_input,
            on_option_hovered,
            on_open,
            on_close,
            padding,
            size,
            line_height,
            width,
        } = self;

        let state: &'static iced::widget::combo_box::State<String> =
            Box::leak(Box::new(iced::widget::combo_box::State::new(options)));

        let mut combo_box =
            iced::widget::ComboBox::new(state, &placeholder, selected.as_ref(), move |v| Message {
                id: on_selected,
                content: crate::bindings::Message::StringType(v),
            });

        if let Some(id) = on_input {
            combo_box = combo_box.on_input(move |v| Message {
                id,
                content: crate::bindings::Message::StringType(v),
            });
        }

        if let Some(id) = on_option_hovered {
            combo_box = combo_box.on_option_hovered(move |v| Message {
                id,
                content: crate::bindings::Message::StringType(v),
            });
        }

        if let Some(id) = on_open {
            combo_box = combo_box.on_open(id.into());
        }

        if let Some(id) = on_close {
            combo_box = combo_box.on_close(id.into());
        }

        if let Some(padding) = padding {
            combo_box = combo_box.padding(padding);
        }

        if let Some(size) = size {
            combo_box = combo_box.size(size);
        }

        if let Some(line_height) = line_height {
            combo_box = combo_box.line_height(line_height);
        }

        if let Some(width) = width {
            combo_box = combo_box.width(width);
        }

        combo_box.into()
    }
}
