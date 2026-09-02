use crate::{
    bindings::iced::app::widgets::SliderNode,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};

/// Builds an `iced::Slider` from its node.
pub fn build<'a, Theme, Renderer>(node: SliderNode) -> iced::Element<'a, Message, Theme, Renderer>
where
    Theme: WrapperTheme + 'a,
    Renderer: WrapperRenderer + 'a,
{
    let mapper = node.on_change;
    let mut slider = iced::widget::Slider::new(
        node.range_start..=node.range_end,
        node.value,
        move |value| Message::F32 { mapper, value },
    );

    if let Some(default) = node.default {
        slider = slider.default(default);
    }
    if let Some(rep) = node.on_release {
        slider = slider.on_release(Message::Fixed { rep });
    }
    if let Some(width) = node.width {
        slider = slider.width(width);
    }
    if let Some(height) = node.height {
        slider = slider.height(height);
    }
    if let Some(step) = node.step {
        slider = slider.step(step);
    }
    if let Some(shift_step) = node.shift_step {
        slider = slider.shift_step(shift_step);
    }
    slider.into()
}
