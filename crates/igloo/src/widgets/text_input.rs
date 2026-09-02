use crate::{
    bindings::iced::app::widgets::TextInputNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::TextInput` from its node.
pub fn build<'a, Theme, Renderer>(
    node: TextInputNode,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut input = iced::widget::TextInput::new(&node.placeholder, &node.value);

    if let Some(secure) = node.secure {
        input = input.secure(secure);
    }
    if let Some(mapper) = node.on_input {
        input = input.on_input(move |value| Message::String { mapper, value });
    }
    if let Some(rep) = node.on_submit {
        input = input.on_submit(Message::Fixed { rep });
    }
    if let Some(mapper) = node.on_paste {
        input = input.on_paste(move |value| Message::String { mapper, value });
    }
    if let Some(width) = node.width {
        input = input.width(width);
    }
    if let Some(padding) = node.padding {
        input = input.padding(padding);
    }
    if let Some(size) = node.size {
        input = input.size(size);
    }
    if let Some(line_height) = node.line_height {
        input = input.line_height(line_height);
    }
    if let Some(align_x) = node.align_x {
        input = input.align_x(align_x);
    }
    input.into()
}
