use iced_core::Length;

use crate::Element;
use crate::bindings::iced::app::text::Text as WitText;

pub struct Text {
    raw: WitText,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            raw: WitText::new(&content.into()),
        }
    }

    pub fn size(self, size: impl Into<f32>) -> Self {
        self.raw.size(size.into());
        self
    }

    pub fn line_height(self, line_height: impl Into<iced_core::text::LineHeight>) -> Self {
        self.raw.line_height(line_height.into().into());
        self
    }

    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    pub fn height(self, height: impl Into<Length>) -> Self {
        self.raw.height(height.into().into());
        self
    }

    pub fn center(self) -> Self {
        self.raw.center();
        self
    }

    pub fn align_x(self, align: impl Into<iced_core::text::Alignment>) -> Self {
        self.raw.align_x(align.into().into());
        self
    }

    pub fn align_y(self, align: impl Into<iced_core::alignment::Vertical>) -> Self {
        self.raw.align_y(align.into().into());
        self
    }

    pub fn color(self, color: impl Into<iced_core::Color>) -> Self {
        self.raw.color(color.into().into());
        self
    }
}

impl<Message: 'static> From<Text> for Element<Message> {
    fn from(text: Text) -> Self {
        Element::new(move |_realize| WitText::into_element(text.raw))
    }
}

impl<'a, Message: 'static> From<&'a str> for Element<Message> {
    fn from(s: &'a str) -> Self {
        Text::new(s).into()
    }
}
