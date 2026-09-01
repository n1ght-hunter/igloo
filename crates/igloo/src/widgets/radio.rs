use crate::{
    bindings::iced::app::{
        length::Length,
        radio,
        shared::Element,
        text::{LineHeight, Shaping, Wrapping},
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct RadioResource {
    pub label: String,
    pub is_selected: bool,
    pub on_select: u32,
    pub size: Option<f32>,
    pub width: Option<Length>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<LineHeight>,
    pub text_wrapping: Option<Wrapping>,
    pub text_shaping: Option<Shaping>,
}

impl RadioResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let on_select = self.on_select;
        let value: u32 = 1;
        let selected = if self.is_selected { Some(value) } else { None };

        let mut radio = iced::widget::Radio::new(self.label, value, selected, move |_: u32| {
            Message::Fixed { rep: on_select }
        });

        if let Some(size) = self.size {
            radio = radio.size(size);
        }
        if let Some(width) = self.width {
            radio = radio.width(width);
        }
        if let Some(spacing) = self.spacing {
            radio = radio.spacing(spacing);
        }
        if let Some(text_size) = self.text_size {
            radio = radio.text_size(text_size);
        }
        if let Some(lh) = self.text_line_height {
            radio = radio.text_line_height(lh);
        }
        if let Some(wrapping) = self.text_wrapping {
            radio = radio.text_wrapping(wrapping.into());
        }
        if let Some(shaping) = self.text_shaping {
            radio = radio.text_shaping(shaping.into());
        }
        radio.into()
    }
}

impl radio::HostRadio for MyState {
    fn new(&mut self, label: String, is_selected: bool, msg: u32) -> Resource<radio::Radio> {
        self.table
            .push(RadioResource {
                label,
                is_selected,
                on_select: msg,
                size: None,
                width: None,
                spacing: None,
                text_size: None,
                text_line_height: None,
                text_wrapping: None,
                text_shaping: None,
            })
            .unwrap()
    }

    fn size(&mut self, self_: Resource<radio::Radio>, s: f32) {
        self.table.get_mut(&self_).unwrap().size = Some(s);
    }

    fn width(&mut self, self_: Resource<radio::Radio>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn spacing(&mut self, self_: Resource<radio::Radio>, s: f32) {
        self.table.get_mut(&self_).unwrap().spacing = Some(s);
    }

    fn text_size(&mut self, self_: Resource<radio::Radio>, s: f32) {
        self.table.get_mut(&self_).unwrap().text_size = Some(s);
    }

    fn text_line_height(&mut self, self_: Resource<radio::Radio>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().text_line_height = Some(lh);
    }

    fn text_wrapping(&mut self, self_: Resource<radio::Radio>, w: Wrapping) {
        self.table.get_mut(&self_).unwrap().text_wrapping = Some(w);
    }

    fn text_shaping(&mut self, self_: Resource<radio::Radio>, s: Shaping) {
        self.table.get_mut(&self_).unwrap().text_shaping = Some(s);
    }

    fn into_element(&mut self, self_: Resource<radio::Radio>) -> Resource<Element> {
        let radio = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Radio(radio)).unwrap()
    }

    fn drop(&mut self, rep: Resource<radio::Radio>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
