use crate::{
    bindings::iced::app::{
        alignment::Vertical,
        length::Length,
        shared::{Color, Element},
        text::{self, HostText, LineHeight, TextAlignment},
    },
    plugin_manager::MyState,
    widgets::{Message, WrapperRenderer, WrapperTheme},
};
use wasmtime::component::Resource;

#[derive(Debug)]
pub struct TextResource {
    pub content: String,
    pub size: Option<f32>,
    pub line_height: Option<LineHeight>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub center: bool,
    pub align_x: Option<TextAlignment>,
    pub align_y: Option<Vertical>,
    pub color: Option<Color>,
}

impl TextResource {
    pub fn to_iced_element<'a, Theme, Renderer>(self) -> iced::Element<'a, Message, Theme, Renderer>
    where
        Theme: WrapperTheme + 'a,
        Renderer: WrapperRenderer + 'a,
    {
        let mut text = iced::widget::Text::new(self.content);
        if let Some(size) = self.size {
            text = text.size(size);
        }
        if let Some(lh) = self.line_height {
            text = text.line_height(lh);
        }
        if let Some(w) = self.width {
            text = text.width(w);
        }
        if let Some(h) = self.height {
            text = text.height(h);
        }
        if self.center {
            text = text.center();
        }
        if let Some(align) = self.align_x {
            text = text.align_x(align);
        }
        if let Some(align) = self.align_y {
            text = text.align_y(align);
        }
        // TODO: color requires Theme::Class<'a>: From<StyleFn<'a, Theme>> bound
        // which is more complex to wire through generics
        text.into()
    }
}

impl HostText for MyState {
    fn new(&mut self, content: String) -> Resource<text::Text> {
        self.table
            .push(TextResource {
                content,
                size: None,
                line_height: None,
                width: None,
                height: None,
                center: false,
                align_x: None,
                align_y: None,
                color: None,
            })
            .unwrap()
    }

    fn size(&mut self, self_: Resource<text::Text>, s: f32) {
        self.table.get_mut(&self_).unwrap().size = Some(s);
    }

    fn line_height(&mut self, self_: Resource<text::Text>, lh: LineHeight) {
        self.table.get_mut(&self_).unwrap().line_height = Some(lh);
    }

    fn width(&mut self, self_: Resource<text::Text>, w: Length) {
        self.table.get_mut(&self_).unwrap().width = Some(w);
    }

    fn height(&mut self, self_: Resource<text::Text>, h: Length) {
        self.table.get_mut(&self_).unwrap().height = Some(h);
    }

    fn center(&mut self, self_: Resource<text::Text>) {
        self.table.get_mut(&self_).unwrap().center = true;
    }

    fn align_x(&mut self, self_: Resource<text::Text>, align: TextAlignment) {
        self.table.get_mut(&self_).unwrap().align_x = Some(align);
    }

    fn align_y(&mut self, self_: Resource<text::Text>, align: Vertical) {
        self.table.get_mut(&self_).unwrap().align_y = Some(align);
    }

    fn color(&mut self, self_: Resource<text::Text>, c: Color) {
        self.table.get_mut(&self_).unwrap().color = Some(c);
    }

    fn into_element(&mut self, self_: Resource<text::Text>) -> Resource<Element> {
        let txt = self.table.delete(self_).unwrap();
        self.table.push(super::Element::Text(txt)).unwrap()
    }

    fn drop(&mut self, rep: Resource<text::Text>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
