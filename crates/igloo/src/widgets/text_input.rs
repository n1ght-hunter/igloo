use super::Message;
use crate::{
    bindings::iced::app::text_input::TextInput,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for TextInput {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let TextInput {
            placeholder,
            value,
            secure,
            on_input,
            on_submit,
            on_paste,
            width,
            padding,
            size,
            line_height,
            align_x,
        } = self;

        let mut input = iced::widget::TextInput::new(&placeholder, &value);

        if let Some(is_secure) = secure {
            input = input.secure(is_secure);
        }

        if let Some(id) = on_input {
            input = input.on_input(move |v| Message {
                id,
                content: crate::bindings::Message::StringType(v),
            });
        }

        if let Some(id) = on_submit {
            input = input.on_submit(id.into());
        }

        if let Some(id) = on_paste {
            input = input.on_paste(move |v| Message {
                id,
                content: crate::bindings::Message::StringType(v),
            });
        }

        if let Some(width) = width {
            input = input.width(width);
        }

        if let Some(padding) = padding {
            input = input.padding(padding);
        }

        if let Some(size) = size {
            input = input.size(size);
        }

        if let Some(line_height) = line_height {
            input = input.line_height(line_height);
        }

        if let Some(align_x) = align_x {
            input = input.align_x(align_x);
        }

        input.into()
    }
}
