use super::Message;
use crate::{
    bindings::iced::app::button::Button,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Button {
    fn to_element<'a, Theme, Renderer>(
        self,
        resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Button {
            content,
            width,
            height,
            padding,
            on_press,
            clip,
        } = self;

        let content = resource_table
            .delete(content)
            .unwrap()
            .to_element(resource_table);

        let mut button = iced::widget::Button::new(content);

        if let Some(width) = width {
            button = button.width(width);
        }

        if let Some(height) = height {
            button = button.height(height);
        }

        if let Some(padding) = padding {
            button = button.padding(padding);
        }

        if let Some(message) = on_press {
            button = button.on_press(message.into());
        }

        if let Some(clip) = clip {
            button = button.clip(clip);
        }

        button.into()
    }
}
