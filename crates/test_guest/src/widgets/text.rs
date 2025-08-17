use iced_core::{Color, Pixels};

use crate::{
    Element,
    bindings::iced::app::element::text_to_element,
    element::Widget,
};

/// A bunch of text.
#[derive(Debug)]
pub struct Text {
    text: String,
    size: Option<Pixels>,
    line_height: Option<iced_core::text::LineHeight>,
    width: Option<iced_core::Length>,
    height: Option<iced_core::Length>,
    center: Option<bool>,
    align_x: Option<iced_core::text::Alignment>,
    align_y: Option<iced_core::alignment::Vertical>,
    shaping: Option<iced_core::text::Shaping>,
    wrapping: Option<iced_core::text::Wrapping>,
    color: Option<Color>,
}

impl Text {
    /// Creates a new [`Text`] widget that displays the provided content.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: None,
            line_height: None,
            width: None,
            height: None,
            center: None,
            align_x: None,
            align_y: None,
            shaping: None,
            wrapping: None,
            color: None,
        }
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn line_height(mut self, line_height: impl Into<iced_core::text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn width(mut self, width: impl Into<iced_core::Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<iced_core::Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn center(mut self) -> Self {
        self.center = Some(true);
        self
    }

    pub fn align_x(mut self, alignment: impl Into<iced_core::text::Alignment>) -> Self {
        self.align_x = Some(alignment.into());
        self
    }

    pub fn align_y(mut self, alignment: impl Into<iced_core::alignment::Vertical>) -> Self {
        self.align_y = Some(alignment.into());
        self
    }

    pub fn shaping(mut self, shaping: impl Into<iced_core::text::Shaping>) -> Self {
        self.shaping = Some(shaping.into());
        self
    }

    pub fn wrapping(mut self, wrapping: impl Into<iced_core::text::Wrapping>) -> Self {
        self.wrapping = Some(wrapping.into());
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl<'a, Message> Widget<Message> for Text {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        text_to_element(&crate::bindings::iced::app::text::Text {
            text: self.text,
            size: self.size.map(Into::into),
            line_height: self.line_height.map(Into::into),
            width: self.width.map(Into::into),
            height: self.height.map(Into::into),
            center: self.center,
            align_x: self.align_x.map(Into::into),
            align_y: self.align_y.map(Into::into),
            shaping: self.shaping.map(Into::into),
            wrapping: self.wrapping.map(Into::into),
            color: self.color.map(Into::into),
        })
    }
}

impl<'a, Message: 'a> From<Text> for Element<Message> {
    fn from(text: Text) -> Self {
        Element::new(Box::new(text))
    }
}
