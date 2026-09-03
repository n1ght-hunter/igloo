//! A text widget.

use stabby::option::Option as Opt;

use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Color, Length, LineHeight, TextAlignment, Vertical};

/// A text widget.
#[stabby::stabby]
#[derive(Debug)]
pub struct TextNode {
    pub content: stabby::string::String,
    pub size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub center: bool,
    pub align_x: Opt<TextAlignment>,
    pub align_y: Opt<Vertical>,
    pub color: Opt<Color>,
}

#[cfg(feature = "host")]
pub(crate) fn build(text: TextNode) -> iced::Element<'static, crate::widgets::Emitted> {
    let mut widget = iced::widget::text(std::string::String::from(text.content));
    if let Some(size) = Option::<f32>::from(text.size) {
        widget = widget.size(size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(text.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(width) = Option::<Length>::from(text.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(text.height) {
        widget = widget.height(iced::Length::from(height));
    }
    if text.center {
        widget = widget.center();
    }
    if let Some(align_x) = Option::<TextAlignment>::from(text.align_x) {
        widget = widget.align_x(iced::widget::text::Alignment::from(align_x));
    }
    if let Some(align_y) = Option::<Vertical>::from(text.align_y) {
        widget = widget.align_y(iced::alignment::Vertical::from(align_y));
    }
    if let Some(color) = Option::<Color>::from(text.color) {
        widget = widget.color(iced::Color::from(color));
    }
    widget.into()
}

/// A builder for a [`TextNode`].
#[derive(Debug)]
pub struct Text {
    content: std::string::String,
    size: Option<f32>,
    line_height: Option<LineHeight>,
    width: Option<Length>,
    height: Option<Length>,
    center: bool,
    align_x: Option<TextAlignment>,
    align_y: Option<Vertical>,
    color: Option<Color>,
}

impl Text {
    /// Creates a text widget with the given content.
    pub fn new(content: impl Into<std::string::String>) -> Self {
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

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
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

    /// Centers the text within its bounds.
    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    pub fn align_x(mut self, align_x: TextAlignment) -> Self {
        self.align_x = Some(align_x);
        self
    }

    pub fn align_y(mut self, align_y: Vertical) -> Self {
        self.align_y = Some(align_y);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<Message: 'static> From<Text> for Element<Message> {
    fn from(text: Text) -> Self {
        Element::new(move |_realize: &dyn Realize<Message>, arena: &mut Arena| {
            arena.push(Node::Text(TextNode {
                content: text.content.into(),
                size: text.size.into(),
                line_height: text.line_height.into(),
                width: text.width.into(),
                height: text.height.into(),
                center: text.center,
                align_x: text.align_x.into(),
                align_y: text.align_y.into(),
                color: text.color.into(),
            }))
        })
    }
}

/// Creates a [`Text`] widget with the given content.
pub fn text(content: impl Into<std::string::String>) -> Text {
    Text::new(content)
}
