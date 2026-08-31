use crate::{
    bindings::iced::app::{
        length::Length,
        shared::Element, slider,
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct SliderResource {
    pub range_start: f32,
    pub range_end: f32,
    pub value: f32,
    pub on_change: u32,
    pub default: Option<f32>,
    pub on_release: Option<u32>,
    pub width: Option<Length>,
    pub height: Option<f32>,
    pub step: Option<f32>,
    pub shift_step: Option<f32>,
}

impl SliderResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mapper = self.on_change;
        let mut slider = iced::widget::Slider::new(
            self.range_start..=self.range_end,
            self.value,
            move |value| Message::F32 { mapper, value },
        );

        if let Some(default) = self.default {
            slider = slider.default(default);
        }
        if let Some(rep) = self.on_release {
            slider = slider.on_release(Message::Fixed { rep });
        }
        if let Some(width) = self.width {
            slider = slider.width(width);
        }
        if let Some(height) = self.height {
            slider = slider.height(height);
        }
        if let Some(step) = self.step {
            slider = slider.step(step);
        }
        if let Some(shift_step) = self.shift_step {
            slider = slider.shift_step(shift_step);
        }
        slider.into()
    }
}

impl slider::HostSlider for MyState {
    fn new(
        &mut self,
        range_start: f32,
        range_end: f32,
        value: f32,
        on_change: u32,
    ) -> Resource<slider::Slider> {
        self.table
            .push(SliderResource {
                range_start,
                range_end,
                value,
                on_change,
                default: None,
                on_release: None,
                width: None,
                height: None,
                step: None,
                shift_step: None,
            })
            .unwrap()
    }

    fn default(&mut self, self_: Resource<slider::Slider>, v: f32) {
        self.table.get_mut(&self_).unwrap().default = Some(v);
    }

    fn on_release(&mut self, self_: Resource<slider::Slider>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_release = Some(msg);
    }

    fn width(&mut self, self_: Resource<slider::Slider>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<slider::Slider>, h: f32) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn step(&mut self, self_: Resource<slider::Slider>, s: f32) {
        self.table.get_mut(&self_).unwrap().step = Some(s);
    }

    fn shift_step(&mut self, self_: Resource<slider::Slider>, s: f32) {
        self.table.get_mut(&self_).unwrap().shift_step = Some(s);
    }

    fn into_element(&mut self, self_: Resource<slider::Slider>) -> Resource<Element> {
        let slider = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Slider(slider)).unwrap()
    }

    fn drop(&mut self, rep: Resource<slider::Slider>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
