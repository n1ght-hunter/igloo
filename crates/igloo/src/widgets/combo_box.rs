use crate::{
    bindings::iced::app::widgets::ComboBoxNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::ComboBox` from its node.
pub fn build<'a, Theme, Renderer>(node: ComboBoxNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    // Leaked deliberately: iced's `ComboBox` widget borrows its `State` for
    // the lifetime of the returned element, and this element is rebuilt
    // fresh from guest state on every `view()` call.
    let state: &'static iced::widget::combo_box::State<String> =
        Box::leak(Box::new(iced::widget::combo_box::State::new(node.options)));

    let on_selected = node.on_selected;
    let mut combo_box = iced::widget::ComboBox::new(
        state,
        &node.placeholder,
        node.selected.as_ref(),
        move |value| Message::String {
            mapper: on_selected,
            value,
        },
    );

    if let Some(mapper) = node.on_input {
        combo_box = combo_box.on_input(move |value| Message::String { mapper, value });
    }
    if let Some(mapper) = node.on_option_hovered {
        combo_box = combo_box.on_option_hovered(move |value| Message::String { mapper, value });
    }
    if let Some(rep) = node.on_open {
        combo_box = combo_box.on_open(Message::Fixed { rep });
    }
    if let Some(rep) = node.on_close {
        combo_box = combo_box.on_close(Message::Fixed { rep });
    }
    if let Some(padding) = node.padding {
        combo_box = combo_box.padding(padding);
    }
    if let Some(size) = node.size {
        combo_box = combo_box.size(size);
    }
    if let Some(line_height) = node.line_height {
        combo_box = combo_box.line_height(line_height);
    }
    if let Some(width) = node.width {
        combo_box = combo_box.width(width);
    }

    combo_box.into()
}
