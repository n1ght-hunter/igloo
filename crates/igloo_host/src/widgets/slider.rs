use super::Message;
use crate::{
    bindings::iced::app::slider::Slider,
    widgets::{ToElement, WrapperRenderer, WrapperTheme},
};

impl ToElement for Slider {
    fn to_element<'a, Theme, Renderer>(
        self,
        _resource_table: &mut wasmtime::component::ResourceTable,
    ) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let Slider {
            range_start,
            range_end,
            value,
            on_change,
            default,
            on_release,
            width,
            height: _height,
            step,
            shift_step,
        } = self;

        let mut slider =
            iced::widget::Slider::new(range_start..=range_end, value, move |v| Message {
                id: on_change,
                content: crate::bindings::Message::F32Type(v),
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

        if let Some(step) = step {
            slider = slider.step(step);
        }

        if let Some(shift_step) = shift_step {
            slider = slider.shift_step(shift_step);
        }

        slider.into()
    }
}
