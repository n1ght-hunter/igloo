use crate::{
    bindings::iced::app::{
        checkbox,
        length::Length,
        shared::Element,
        text::{LineHeight, Shaping, Wrapping},
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct CheckboxResource {
    pub is_checked: bool,
    pub label: Option<String>,
    pub on_toggle: Option<u32>,
    pub size: Option<f32>,
    pub width: Option<Length>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<LineHeight>,
    pub text_wrapping: Option<Wrapping>,
    pub text_shaping: Option<Shaping>,
}

impl CheckboxResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mut checkbox = iced::widget::Checkbox::new(self.is_checked);

        if let Some(label) = self.label {
            checkbox = checkbox.label(label);
        }
        if let Some(mapper) = self.on_toggle {
            checkbox = checkbox.on_toggle(move |value| Message::Bool { mapper, value });
        }
        if let Some(size) = self.size {
            checkbox = checkbox.size(size);
        }
        if let Some(width) = self.width {
            checkbox = checkbox.width(width);
        }
        if let Some(spacing) = self.spacing {
            checkbox = checkbox.spacing(spacing);
        }
        if let Some(text_size) = self.text_size {
            checkbox = checkbox.text_size(text_size);
        }
        if let Some(lh) = self.text_line_height {
            checkbox = checkbox.text_line_height(lh);
        }
        if let Some(wrapping) = self.text_wrapping {
            checkbox = checkbox.text_wrapping(wrapping.into());
        }
        if let Some(shaping) = self.text_shaping {
            checkbox = checkbox.text_shaping(shaping.into());
        }
        checkbox.into()
    }
}

impl checkbox::HostCheckbox for MyState {
    fn new(&mut self, is_checked: bool) -> Resource<checkbox::Checkbox> {
        self.table
            .push(CheckboxResource {
                is_checked,
                label: None,
                on_toggle: None,
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

    fn label(&mut self, self_: Resource<checkbox::Checkbox>, label: String) {
        self.table.get_mut(&self_).unwrap().label = Some(label);
    }

    fn on_toggle(&mut self, self_: Resource<checkbox::Checkbox>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_toggle = Some(mapper);
    }

    fn size(&mut self, self_: Resource<checkbox::Checkbox>, s: f32) {
        self.table.get_mut(&self_).unwrap().size = Some(s);
    }

    fn width(&mut self, self_: Resource<checkbox::Checkbox>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn spacing(&mut self, self_: Resource<checkbox::Checkbox>, s: f32) {
        self.table.get_mut(&self_).unwrap().spacing = Some(s);
    }

    fn text_size(&mut self, self_: Resource<checkbox::Checkbox>, s: f32) {
        self.table.get_mut(&self_).unwrap().text_size = Some(s);
    }

    fn text_line_height(&mut self, self_: Resource<checkbox::Checkbox>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().text_line_height = Some(lh);
    }

    fn text_wrapping(&mut self, self_: Resource<checkbox::Checkbox>, w: Wrapping) {
        self.table.get_mut(&self_).unwrap().text_wrapping = Some(w);
    }

    fn text_shaping(&mut self, self_: Resource<checkbox::Checkbox>, s: Shaping) {
        self.table.get_mut(&self_).unwrap().text_shaping = Some(s);
    }

    fn into_element(&mut self, self_: Resource<checkbox::Checkbox>) -> Resource<Element> {
        let checkbox = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Checkbox(checkbox)).unwrap()
    }

    fn drop(&mut self, rep: Resource<checkbox::Checkbox>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
