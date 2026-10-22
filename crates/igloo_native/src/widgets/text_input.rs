//! A field for text input.

use stabby::option::Option as Opt;

use crate::Emit;
use crate::messages::EmitString;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Horizontal, Length, LineHeight, Padding};

/// A field for text input.
#[stabby::stabby(no_opt)]
pub struct TextInputNode {
    pub placeholder: stabby::string::String,
    pub value: stabby::string::String,
    pub secure: Opt<bool>,
    pub on_input: Opt<EmitString>,
    pub on_submit: Opt<Emit>,
    pub on_paste: Opt<EmitString>,
    pub width: Opt<Length>,
    pub padding: Opt<Padding>,
    pub size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub align_x: Opt<Horizontal>,
}

impl std::fmt::Debug for TextInputNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextInputNode")
            .field("placeholder", &self.placeholder)
            .field("value", &self.value)
            .field("secure", &self.secure)
            .field("on_input", &"<opaque>")
            .field("on_submit", &"<opaque>")
            .field("on_paste", &"<opaque>")
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("size", &self.size)
            .field("line_height", &self.line_height)
            .field("align_x", &self.align_x)
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build(input: TextInputNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let placeholder = std::string::String::from(input.placeholder);
    let value = std::string::String::from(input.value);
    let mut widget = iced::widget::TextInput::new(placeholder, value);
    if let Some(secure) = Option::<bool>::from(input.secure) {
        widget = widget.secure(secure);
    }
    if let Some(emit) = Option::from(input.on_input) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_input(move |value| crate::widgets::Emitted::str(&emit, value));
    }
    if let Some(emit) = Option::from(input.on_submit) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_submit(crate::widgets::Emitted::fixed(&emit));
    }
    if let Some(emit) = Option::from(input.on_paste) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_paste(move |value| crate::widgets::Emitted::str(&emit, value));
    }
    if let Some(width) = Option::<Length>::from(input.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(padding) = Option::<Padding>::from(input.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    if let Some(size) = Option::<f32>::from(input.size) {
        widget = widget.size(size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(input.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(align_x) = Option::<Horizontal>::from(input.align_x) {
        widget = widget.align_x(iced::alignment::Horizontal::from(align_x));
    }
    widget
}

/// A builder for a [`TextInputNode`].
pub struct TextInput<Message> {
    placeholder: std::string::String,
    value: std::string::String,
    secure: Option<bool>,
    on_input: Option<std::boxed::Box<dyn Fn(std::string::String) -> Message>>,
    on_submit: Option<std::boxed::Box<dyn Fn() -> Message>>,
    on_paste: Option<std::boxed::Box<dyn Fn(std::string::String) -> Message>>,
    width: Option<Length>,
    padding: Option<Padding>,
    size: Option<f32>,
    line_height: Option<LineHeight>,
    align_x: Option<Horizontal>,
}

impl<Message> TextInput<Message> {
    /// Creates a text input with the given placeholder and current value.
    pub fn new(
        placeholder: impl Into<std::string::String>,
        value: impl Into<std::string::String>,
    ) -> Self {
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

    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = Some(secure);
        self
    }

    /// Sets the message emitted with the new value on every keystroke.
    pub fn on_input(mut self, emit: impl Fn(std::string::String) -> Message + 'static) -> Self {
        self.on_input = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted when Enter is pressed.
    pub fn on_submit(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_submit = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted with the pasted text.
    pub fn on_paste(mut self, emit: impl Fn(std::string::String) -> Message + 'static) -> Self {
        self.on_paste = Some(std::boxed::Box::new(emit));
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

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn align_x(mut self, align_x: Horizontal) -> Self {
        self.align_x = Some(align_x);
        self
    }
}

impl<Message> std::fmt::Debug for TextInput<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextInput")
            .field("placeholder", &self.placeholder)
            .field("value", &self.value)
            .field("secure", &self.secure)
            .field("on_input", &self.on_input.as_ref().map(|_| "<opaque>"))
            .field("on_submit", &self.on_submit.as_ref().map(|_| "<opaque>"))
            .field("on_paste", &self.on_paste.as_ref().map(|_| "<opaque>"))
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("size", &self.size)
            .field("line_height", &self.line_height)
            .field("align_x", &self.align_x)
            .finish()
    }
}

impl<Message: 'static> From<TextInput<Message>> for Element<Message> {
    fn from(input: TextInput<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_input = input.on_input.map(|emit| realize.string_mapper(emit));
            let on_submit = input.on_submit.map(|emit| realize.fixed(emit));
            let on_paste = input.on_paste.map(|emit| realize.string_mapper(emit));
            arena.push(Node::TextInput(TextInputNode {
                placeholder: input.placeholder.into(),
                value: input.value.into(),
                secure: input.secure.into(),
                on_input: on_input.into(),
                on_submit: on_submit.into(),
                on_paste: on_paste.into(),
                width: input.width.into(),
                padding: input.padding.into(),
                size: input.size.into(),
                line_height: input.line_height.into(),
                align_x: input.align_x.into(),
            }))
        })
    }
}

/// Creates a [`TextInput`] with the given placeholder and current value.
pub fn text_input<Message>(
    placeholder: impl Into<std::string::String>,
    value: impl Into<std::string::String>,
) -> TextInput<Message> {
    TextInput::new(placeholder, value)
}
