use crate::{
    bindings::iced::app::widgets::ButtonNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Button` from its node and its already-built content element.
pub fn build<'a, Theme, Renderer>(
    node: ButtonNode,
    content: iced::Element<'a, Message, Theme, Renderer>,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mut button = iced::widget::Button::new(content);

    if let Some(w) = node.width {
        button = button.width(w);
    }
    if let Some(h) = node.height {
        button = button.height(h);
    }
    if let Some(p) = node.padding {
        button = button.padding(p);
    }
    if let Some(rep) = node.on_press {
        button = button.on_press(Message::Fixed { rep });
    }
    if let Some(clip) = node.clip {
        button = button.clip(clip);
    }
    button.into()
}
