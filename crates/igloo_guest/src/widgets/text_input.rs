use iced_core::{Length, Padding, Pixels, alignment, text};

use crate::Element;
use crate::bindings::iced::app::text_input::TextInput as WitTextInput;

/// A field that can be filled with text.
pub struct TextInput<Message> {
    raw: WitTextInput,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    on_submit: Option<Message>,
    on_paste: Option<Box<dyn Fn(String) -> Message>>,
}

impl<Message: 'static> TextInput<Message> {
    /// Creates a new [`TextInput`] with the given placeholder and value.
    pub fn new(placeholder: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            raw: WitTextInput::new(&placeholder.into(), &value.into()),
            on_input: None,
            on_submit: None,
            on_paste: None,
        }
    }

    /// Sets whether the [`TextInput`] should mask its contents.
    pub fn secure(self, secure: bool) -> Self {
        self.raw.secure(secure);
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

    pub fn width(self, width: impl Into<Length>) -> Self {
        self.raw.width(width.into().into());
        self
    }

    pub fn padding(self, padding: impl Into<Padding>) -> Self {
        self.raw.padding(padding.into().into());
        self
    }

    pub fn size(self, size: impl Into<Pixels>) -> Self {
        self.raw.size(size.into().0);
        self
    }

    pub fn line_height(self, line_height: impl Into<text::LineHeight>) -> Self {
        self.raw.line_height(line_height.into().into());
        self
    }

    pub fn align_x(self, align: impl Into<alignment::Horizontal>) -> Self {
        self.raw.align_x(align.into().into());
        self
    }
}

impl<Message: 'static> From<TextInput<Message>> for Element<Message> {
    fn from(text_input: TextInput<Message>) -> Self {
        Element::new(move |realize| {
            if let Some(on_input) = text_input.on_input {
                text_input.raw.on_input(realize.string_mapper(on_input));
            }
            if let Some(msg) = text_input.on_submit {
                text_input.raw.on_submit(realize.fixed(msg));
            }
            if let Some(on_paste) = text_input.on_paste {
                text_input.raw.on_paste(realize.string_mapper(on_paste));
            }
            WitTextInput::into_element(text_input.raw)
        })
    }
}
