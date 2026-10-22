use crate::{
    bindings::iced::app::widgets::PickListNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::PickList` from its node.
pub fn build<'a, Theme, Renderer>(
    node: PickListNode,
) -> impl iced::Widget<Message, Theme, Renderer> + 'a
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let on_select = node.on_select;
    let mut pick_list =
        iced::widget::PickList::new(node.selected, node.options, |value: &String| value.clone())
            .on_select(move |value| Message::String {
                mapper: on_select,
                value,
            });

    if let Some(placeholder) = node.placeholder {
        pick_list = pick_list.placeholder(placeholder);
    }
    if let Some(width) = node.width {
        pick_list = pick_list.width(width);
    }
    if let Some(padding) = node.padding {
        pick_list = pick_list.padding(padding);
    }
    if let Some(text_size) = node.text_size {
        pick_list = pick_list.text_size(text_size);
    }
    if let Some(line_height) = node.line_height {
        pick_list = pick_list.line_height(line_height);
    }
    if let Some(shaping) = node.shaping {
        pick_list = pick_list.shaping(shaping.into());
    }
    if let Some(rep) = node.on_open {
        pick_list = pick_list.on_open(Message::Fixed { rep });
    }
    if let Some(rep) = node.on_close {
        pick_list = pick_list.on_close(Message::Fixed { rep });
    }

    pick_list
}
