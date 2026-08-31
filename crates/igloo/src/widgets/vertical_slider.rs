use crate::{
    bindings::iced::app::{
        length::Length,
        shared::{Element, Pixels},
        vertical_slider,
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct VerticalSliderResource {
    pub range_start: f32,
    pub range_end: f32,
    pub value: f32,
    pub on_change: u32,
    pub default: Option<f32>,
    pub on_release: Option<u32>,
    pub width: Option<Pixels>,
    pub height: Option<Length>,
    pub step: Option<f32>,
    pub shift_step: Option<f32>,
}

impl VerticalSliderResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mapper = self.on_change;
        let mut slider = iced::widget::VerticalSlider::new(
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

impl vertical_slider::HostVerticalSlider for MyState {
    fn new(
        &mut self,
        range_start: f32,
        range_end: f32,
        value: f32,
        on_change: u32,
    ) -> Resource<vertical_slider::VerticalSlider> {
        self.table
            .push(VerticalSliderResource {
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

    fn default(&mut self, self_: Resource<vertical_slider::VerticalSlider>, v: f32) {
        self.table.get_mut(&self_).unwrap().default = Some(v);
    }

    fn on_release(&mut self, self_: Resource<vertical_slider::VerticalSlider>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_release = Some(msg);
    }

    fn width(&mut self, self_: Resource<vertical_slider::VerticalSlider>, w: Pixels) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<vertical_slider::VerticalSlider>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn step(&mut self, self_: Resource<vertical_slider::VerticalSlider>, s: f32) {
        self.table.get_mut(&self_).unwrap().step = Some(s);
    }

    fn shift_step(&mut self, self_: Resource<vertical_slider::VerticalSlider>, s: f32) {
        self.table.get_mut(&self_).unwrap().shift_step = Some(s);
    }

    fn into_element(
        &mut self,
        self_: Resource<vertical_slider::VerticalSlider>,
    ) -> Resource<Element> {
        let slider = self.table.delete(self_).unwrap();
        self.table
            .push(super::Element::VerticalSlider(slider))
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<vertical_slider::VerticalSlider>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
