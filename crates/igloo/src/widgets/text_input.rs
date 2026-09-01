use crate::{
    bindings::iced::app::{
        alignment::Horizontal,
        length::Length,
        padding::Padding,
        shared::{Element, Pixels},
        text::LineHeight,
        text_input,
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct TextInputResource {
    pub placeholder: String,
    pub value: String,
    pub secure: Option<bool>,
    pub on_input: Option<u32>,
    pub on_submit: Option<u32>,
    pub on_paste: Option<u32>,
    pub width: Option<Length>,
    pub padding: Option<Padding>,
    pub size: Option<Pixels>,
    pub line_height: Option<LineHeight>,
    pub align_x: Option<Horizontal>,
}

impl TextInputResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mut input = iced::widget::TextInput::new(&self.placeholder, &self.value);

        if let Some(secure) = self.secure {
            input = input.secure(secure);
        }
        if let Some(mapper) = self.on_input {
            input = input.on_input(move |value| Message::String { mapper, value });
        }
        if let Some(rep) = self.on_submit {
            input = input.on_submit(Message::Fixed { rep });
        }
        if let Some(mapper) = self.on_paste {
            input = input.on_paste(move |value| Message::String { mapper, value });
        }
        if let Some(width) = self.width {
            input = input.width(width);
        }
        if let Some(padding) = self.padding {
            input = input.padding(padding);
        }
        if let Some(size) = self.size {
            input = input.size(size);
        }
        if let Some(line_height) = self.line_height {
            input = input.line_height(line_height);
        }
        if let Some(align_x) = self.align_x {
            input = input.align_x(align_x);
        }
        input.into()
    }
}

impl text_input::HostTextInput for MyState {
    fn new(&mut self, placeholder: String, value: String) -> Resource<text_input::TextInput> {
        self.table
            .push(TextInputResource {
                placeholder,
                value,
                secure: None,
                on_input: None,
                on_submit: None,
                on_paste: None,
                width: None,
                padding: None,
                size: None,
                line_height: None,
                align_x: None,
            })
            .unwrap()
    }

    fn secure(&mut self, self_: Resource<text_input::TextInput>, secure: bool) {
        self.table.get_mut(&self_).unwrap().secure = Some(secure);
    }

    fn on_input(&mut self, self_: Resource<text_input::TextInput>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_input = Some(mapper);
    }

    fn on_submit(&mut self, self_: Resource<text_input::TextInput>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_submit = Some(msg);
    }

    fn on_paste(&mut self, self_: Resource<text_input::TextInput>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_paste = Some(mapper);
    }

    fn width(&mut self, self_: Resource<text_input::TextInput>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn padding(&mut self, self_: Resource<text_input::TextInput>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn size(&mut self, self_: Resource<text_input::TextInput>, s: Pixels) {
        self.table.get_mut(&self_).unwrap().size = Some(s);
    }

    fn line_height(&mut self, self_: Resource<text_input::TextInput>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().line_height = Some(lh);
    }

    fn align_x(&mut self, self_: Resource<text_input::TextInput>, a: Horizontal) {
        self.table.get_mut(&self_).unwrap().align_x = Some(a);
    }

    fn into_element(&mut self, self_: Resource<text_input::TextInput>) -> Resource<Element> {
        let text_input = self.table.delete(self_).unwrap();
        self.table
            .push(super::Element::TextInput(text_input))
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<text_input::TextInput>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
