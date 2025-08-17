use super::Message;
use crate::{
    bindings::iced::app::vertical_slider::VerticalSlider,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for VerticalSlider {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let VerticalSlider {
            range_start,
            range_end,
            value,
            on_change,
            default,
            on_release,
            width,
            height,
            step,
            shift_step,
        } = self;

        let mut slider =
            iced::widget::VerticalSlider::new(range_start..=range_end, value, move |v| Message {
                id: on_change,
                content: crate::bindings::Message::F64Type(v.into()),
            });

        if let Some(default) = default {
            slider = slider.default(default);
        }

        if let Some(message) = on_release {
            slider = slider.on_release(message.into());
        }

        if let Some(width) = width {
            slider = slider.width(width);
        }

        if let Some(height) = height {
            slider = slider.height(height);
        }

        if let Some(step) = step {
            slider = slider.step(step);
        }

        if let Some(shift_step) = shift_step {
            slider = slider.shift_step(shift_step);
        }

        slider.into()
    }
}
