use crate::{
    bindings::iced::app::{
        length::Length,
        padding::Padding,
        pick_list,
        shared::Element,
        text::{LineHeight, Shaping},
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct PickListResource {
    pub options: Vec<String>,
    pub selected: Option<String>,
    pub on_select: u32,
    pub placeholder: Option<String>,
    pub width: Option<Length>,
    pub padding: Option<Padding>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<LineHeight>,
    pub text_shaping: Option<Shaping>,
    pub on_open: Option<u32>,
    pub on_close: Option<u32>,
}

impl PickListResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let on_select = self.on_select;
        let mut pick_list =
            iced::widget::PickList::new(self.options, self.selected, move |value| {
                Message::String {
                    mapper: on_select,
                    value,
                }
            });

        if let Some(placeholder) = self.placeholder {
            pick_list = pick_list.placeholder(placeholder);
        }
        if let Some(width) = self.width {
            pick_list = pick_list.width(width);
        }
        if let Some(padding) = self.padding {
            pick_list = pick_list.padding(padding);
        }
        if let Some(text_size) = self.text_size {
            pick_list = pick_list.text_size(text_size);
        }
        if let Some(text_line_height) = self.text_line_height {
            pick_list = pick_list.text_line_height(text_line_height);
        }
        if let Some(text_shaping) = self.text_shaping {
            pick_list = pick_list.text_shaping(text_shaping.into());
        }
        if let Some(rep) = self.on_open {
            pick_list = pick_list.on_open(Message::Fixed { rep });
        }
        if let Some(rep) = self.on_close {
            pick_list = pick_list.on_close(Message::Fixed { rep });
        }

        pick_list.into()
    }
}

impl pick_list::HostPickList for MyState {
    fn new(
        &mut self,
        options: Vec<String>,
        selected: Option<String>,
        on_select: u32,
    ) -> Resource<pick_list::PickList> {
        self.table
            .push(PickListResource {
                options,
                selected,
                on_select,
                placeholder: None,
                width: None,
                padding: None,
                text_size: None,
                text_line_height: None,
                text_shaping: None,
                on_open: None,
                on_close: None,
            })
            .unwrap()
    }

    fn placeholder(&mut self, self_: Resource<pick_list::PickList>, text: String) {
        self.table.get_mut(&self_).unwrap().placeholder = Some(text);
    }

    fn width(&mut self, self_: Resource<pick_list::PickList>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn padding(&mut self, self_: Resource<pick_list::PickList>, p: Padding) {
        self.table.get_mut(&self_).unwrap().padding = Some(p);
    }

    fn text_size(&mut self, self_: Resource<pick_list::PickList>, s: f32) {
        self.table.get_mut(&self_).unwrap().text_size = Some(s);
    }

    fn text_line_height(&mut self, self_: Resource<pick_list::PickList>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().text_line_height = Some(lh);
    }

    fn text_shaping(&mut self, self_: Resource<pick_list::PickList>, s: Shaping) {
        self.table.get_mut(&self_).unwrap().text_shaping = Some(s);
    }

    fn on_open(&mut self, self_: Resource<pick_list::PickList>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_open = Some(msg);
    }

    fn on_close(&mut self, self_: Resource<pick_list::PickList>, msg: u32) {
        self.table.get_mut(&self_).unwrap().on_close = Some(msg);
    }

    fn into_element(&mut self, self_: Resource<pick_list::PickList>) -> Resource<Element> {
        let pick_list = self.table.delete(self_).unwrap();
        self.table
            .push(super::Element::PickList(pick_list))
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<pick_list::PickList>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
