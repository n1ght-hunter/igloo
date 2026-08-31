use crate::{
    bindings::iced::app::{
        combo_box, length::Length,
        padding::Padding, shared::Element, text::LineHeight,
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct ComboBoxResource {
    pub options: Vec<String>,
    pub placeholder: String,
    pub selected: Option<String>,
    pub on_selected: u32,
    pub on_input: Option<u32>,
    pub on_option_hovered: Option<u32>,
    pub on_open: Option<u32>,
    pub on_close: Option<u32>,
    pub padding: Option<Padding>,
    pub size: Option<f32>,
    pub line_height: Option<LineHeight>,
    pub width: Option<Length>,
}

impl ComboBoxResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        // Leaked deliberately: iced's `ComboBox` widget borrows its `State` for
        // the lifetime of the returned element, and this element is rebuilt
        // fresh from guest state on every `view()` call.
        let state: &'static iced::widget::combo_box::State<String> =
            Box::leak(Box::new(iced::widget::combo_box::State::new(self.options)));

        let on_selected = self.on_selected;
        let mut combo_box = iced::widget::ComboBox::new(
            state,
            &self.placeholder,
            self.selected.as_ref(),
            move |value| Message::String {
                mapper: on_selected,
                value,
            },
        );

        if let Some(mapper) = self.on_input {
            combo_box = combo_box.on_input(move |value| Message::String { mapper, value });
        }
        if let Some(mapper) = self.on_option_hovered {
            combo_box = combo_box.on_option_hovered(move |value| Message::String { mapper, value });
        }
        if let Some(rep) = self.on_open {
            combo_box = combo_box.on_open(Message::Fixed { rep });
        }
        if let Some(rep) = self.on_close {
            combo_box = combo_box.on_close(Message::Fixed { rep });
        }
        if let Some(padding) = self.padding {
            combo_box = combo_box.padding(padding);
        }
        if let Some(size) = self.size {
            combo_box = combo_box.size(size);
        }
        if let Some(line_height) = self.line_height {
            combo_box = combo_box.line_height(line_height);
        }
        if let Some(width) = self.width {
            combo_box = combo_box.width(width);
        }

        combo_box.into()
    }
}

impl combo_box::HostComboBox for MyState {
    fn new(
        &mut self,
        options: Vec<String>,
        placeholder: String,
        selected: Option<String>,
        on_selected: u32,
    ) -> Resource<combo_box::ComboBox> {
        self.table
            .push(ComboBoxResource {
                options,
                placeholder,
                selected,
                on_selected,
                on_input: None,
                on_option_hovered: None,
                on_open: None,
                on_close: None,
                padding: None,
                size: None,
                line_height: None,
                width: None,
            })
            .unwrap()
    }

    fn on_input(&mut self, self_: Resource<combo_box::ComboBox>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_input = Some(mapper);
    }

    fn on_option_hovered(&mut self, self_: Resource<combo_box::ComboBox>, mapper: u32) {
        self.table.get_mut(&self_).unwrap().on_option_hovered = Some(mapper);
    }

    fn on_open(&mut self, self_: Resource<combo_box::ComboBox>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_open = Some(msg);
    }

    fn on_close(&mut self, self_: Resource<combo_box::ComboBox>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_close = Some(msg);
    }

    fn padding(&mut self, self_: Resource<combo_box::ComboBox>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn size(&mut self, self_: Resource<combo_box::ComboBox>, s: f32) {
        self.table.get_mut(&self_).unwrap().size = Some(s);
    }

    fn line_height(&mut self, self_: Resource<combo_box::ComboBox>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().line_height = Some(lh);
    }

    fn width(&mut self, self_: Resource<combo_box::ComboBox>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn into_element(&mut self, self_: Resource<combo_box::ComboBox>) -> Resource<Element> {
        let combo_box = self.table.delete(self_).unwrap();
        self.table
            .push(super::Element::ComboBox(combo_box))
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<combo_box::ComboBox>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
