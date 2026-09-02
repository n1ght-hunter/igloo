use iced_core::{Length, Padding, Pixels, alignment, text};

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, TextInputNode};

/// A field that can be filled with text.
pub struct TextInput<Message> {
    placeholder: String,
    value: String,
    secure: Option<bool>,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    on_submit: Option<Message>,
    on_paste: Option<Box<dyn Fn(String) -> Message>>,
    width: Option<Length>,
    padding: Option<Padding>,
    size: Option<f32>,
    line_height: Option<text::LineHeight>,
    align_x: Option<alignment::Horizontal>,
}

impl<Message: 'static> TextInput<Message> {
    /// Creates a new [`TextInput`] with the given placeholder and value.
    pub fn new(placeholder: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            value: value.into(),
            secure: None,
            on_input: None,
            on_submit: None,
            on_paste: None,
            width: None,
            padding: None,
            size: None,
            line_height: None,
            align_x: None,
        }
    }

    /// Sets whether the [`TextInput`] should mask its contents.
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = Some(secure);
        self
    }

    /// Sets the message produced when the [`TextInput`] changes.
    pub fn on_input(mut self, message: impl Fn(String) -> Message + 'static) -> Self {
        self.on_input = Some(Box::new(message));
        self
    }

    /// Sets the message produced when the [`TextInput`] is submitted.
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the message produced when text is pasted into the [`TextInput`].
    pub fn on_paste(mut self, message: impl Fn(String) -> Message + 'static) -> Self {
        self.on_paste = Some(Box::new(message));
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().0);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
        self.align_x = Some(align.into());
        self
    }
}

impl<Message: 'static> From<TextInput<Message>> for Element<Message> {
    fn from(text_input: TextInput<Message>) -> Self {
        Element::new(move |realize, arena| {
            let node = TextInputNode {
                placeholder: text_input.placeholder,
                value: text_input.value,
                secure: text_input.secure,
                on_input: text_input.on_input.map(|f| realize.string_mapper(f)),
                on_submit: text_input.on_submit.map(|msg| realize.fixed(msg)),
                on_paste: text_input.on_paste.map(|f| realize.string_mapper(f)),
                width: text_input.width.map(Into::into),
                padding: text_input.padding.map(Into::into),
                size: text_input.size,
                line_height: text_input.line_height.map(Into::into),
                align_x: text_input.align_x.map(Into::into),
            };
            arena.push(Node::TextInput(node))
        })
    }
}
