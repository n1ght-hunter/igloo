use iced_core::Length;

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, TextNode};

pub struct Text {
    content: String,
    size: Option<f32>,
    line_height: Option<iced_core::text::LineHeight>,
    width: Option<Length>,
    height: Option<Length>,
    center: bool,
    align_x: Option<iced_core::text::Alignment>,
    align_y: Option<iced_core::alignment::Vertical>,
    color: Option<iced_core::Color>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: None,
            line_height: None,
            width: None,
            height: None,
            center: false,
            align_x: None,
            align_y: None,
            color: None,
        }
    }

    pub fn size(mut self, size: impl Into<f32>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn line_height(mut self, line_height: impl Into<iced_core::text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    pub fn align_x(mut self, align: impl Into<iced_core::text::Alignment>) -> Self {
        self.align_x = Some(align.into());
        self
    }

    pub fn align_y(mut self, align: impl Into<iced_core::alignment::Vertical>) -> Self {
        self.align_y = Some(align.into());
        self
    }

    pub fn color(mut self, color: impl Into<iced_core::Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl<Message: 'static> From<Text> for Element<Message> {
    fn from(text: Text) -> Self {
        Element::new(move |_realize, arena| {
            let node = TextNode {
                content: text.content,
                size: text.size,
                line_height: text.line_height.map(Into::into),
                width: text.width.map(Into::into),
                height: text.height.map(Into::into),
                center: text.center,
                align_x: text.align_x.map(Into::into),
                align_y: text.align_y.map(Into::into),
                color: text.color.map(Into::into),
            };
            arena.push(Node::Text(node))
        })
    }
}

impl<'a, Message: 'static> From<&'a str> for Element<Message> {
    fn from(s: &'a str) -> Self {
        Text::new(s).into()
    }
}
