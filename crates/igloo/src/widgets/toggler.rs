use crate::{
    bindings::iced::app::{
        alignment::Horizontal,
        length::Length,
        shared::Element,
        text::{LineHeight, Shaping, Wrapping},
        toggler,
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct TogglerResource {
    pub is_toggled: bool,
    pub label: Option<String>,
    pub on_toggle: Option<u32>,
    pub size: Option<f32>,
    pub width: Option<Length>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<LineHeight>,
    pub text_alignment: Option<Horizontal>,
    pub text_shaping: Option<Shaping>,
    pub text_wrapping: Option<Wrapping>,
    pub spacing: Option<f32>,
}

impl TogglerResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mut toggler = iced::widget::Toggler::new(self.is_toggled);

        if let Some(label) = self.label {
            toggler = toggler.label(label);
        }
        if let Some(mapper) = self.on_toggle {
            toggler = toggler.on_toggle(move |value| Message::Bool { mapper, value });
        }
        if let Some(size) = self.size {
            toggler = toggler.size(size);
        }
        if let Some(width) = self.width {
            toggler = toggler.width(width);
        }
        if let Some(text_size) = self.text_size {
            toggler = toggler.text_size(text_size);
        }
        if let Some(lh) = self.text_line_height {
            toggler = toggler.text_line_height(lh);
        }
        if let Some(alignment) = self.text_alignment {
            let alignment: iced::alignment::Horizontal = alignment.into();
            toggler = toggler.text_alignment(alignment);
        }
        if let Some(shaping) = self.text_shaping {
            toggler = toggler.text_shaping(shaping.into());
        }
        if let Some(wrapping) = self.text_wrapping {
            toggler = toggler.text_wrapping(wrapping.into());
        }
        if let Some(spacing) = self.spacing {
            toggler = toggler.spacing(spacing);
        }
        toggler.into()
    }
}

impl toggler::HostToggler for MyState {
    fn new(&mut self, is_toggled: bool) -> Resource<toggler::Toggler> {
        self.table
            .push(TogglerResource {
                is_toggled,
                label: None,
                on_toggle: None,
                size: None,
                width: None,
                text_size: None,
                text_line_height: None,
                text_alignment: None,
                text_shaping: None,
                text_wrapping: None,
                spacing: None,
            })
            .unwrap()
    }

    fn label(&mut self, self_: Resource<toggler::Toggler>, label: String) {
        self.table.get_mut(&self_).unwrap().label = Some(label);
    }

    fn on_toggle(&mut self, self_: Resource<toggler::Toggler>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_toggle = Some(mapper);
    }

    fn size(&mut self, self_: Resource<toggler::Toggler>, s: f32) {
        self.table.get_mut(&self_).unwrap().size = Some(s);
    }

    fn width(&mut self, self_: Resource<toggler::Toggler>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn text_size(&mut self, self_: Resource<toggler::Toggler>, s: f32) {
        self.table.get_mut(&self_).unwrap().text_size = Some(s);
    }

    fn text_line_height(&mut self, self_: Resource<toggler::Toggler>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().text_line_height = Some(lh);
    }

    fn text_alignment(&mut self, self_: Resource<toggler::Toggler>, a: Horizontal) {
        self.table.get_mut(&self_).unwrap().text_alignment = Some(a);
    }

    fn text_shaping(&mut self, self_: Resource<toggler::Toggler>, s: Shaping) {
        self.table.get_mut(&self_).unwrap().text_shaping = Some(s);
    }

    fn text_wrapping(&mut self, self_: Resource<toggler::Toggler>, w: Wrapping) {
        self.table.get_mut(&self_).unwrap().text_wrapping = Some(w);
    }

    fn spacing(&mut self, self_: Resource<toggler::Toggler>, s: f32) {
        self.table.get_mut(&self_).unwrap().spacing = Some(s);
    }

    fn into_element(&mut self, self_: Resource<toggler::Toggler>) -> Resource<Element> {
        let toggler = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Toggler(toggler)).unwrap()
    }

    fn drop(&mut self, rep: Resource<toggler::Toggler>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
