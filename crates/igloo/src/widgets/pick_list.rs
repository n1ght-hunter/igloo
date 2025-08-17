use super::Message;
use crate::{
    bindings::iced::app::pick_list::PickList,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for PickList {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let PickList {
            options,
            selected,
            on_select,
            placeholder,
            width,
            padding,
            text_size,
            text_line_height,
            text_shaping,
            on_open,
            on_close,
        } = self;

        let mut pick_list = iced::widget::PickList::new(options, selected, move |v| Message {
            id: on_select,
            content: crate::bindings::Message::StringType(v),
        });

        if let Some(placeholder) = placeholder {
            pick_list = pick_list.placeholder(placeholder);
        }

        if let Some(width) = width {
            pick_list = pick_list.width(width);
        }

        if let Some(padding) = padding {
            pick_list = pick_list.padding(padding);
        }

        if let Some(text_size) = text_size {
            pick_list = pick_list.text_size(text_size);
        }

        if let Some(text_line_height) = text_line_height {
            pick_list = pick_list.text_line_height(text_line_height);
        }

        if let Some(text_shaping) = text_shaping {
            pick_list = pick_list.text_shaping(text_shaping.into());
        }

        if let Some(on_open) = on_open {
            pick_list = pick_list.on_open(on_open.into());
        }

        if let Some(on_close) = on_close {
            pick_list = pick_list.on_close(on_close.into());
        }

        pick_list.into()
    }
}
