//! A circular button representing one choice in a set.

use stabby::option::Option as Opt;

use crate::Emit;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Length, LineHeight, Shaping, Wrapping};

/// A circular button representing one choice in a set.
#[stabby::stabby(no_opt)]
pub struct RadioNode {
    pub label: stabby::string::String,
    pub is_selected: bool,
    pub on_select: Emit,
    pub size: Opt<f32>,
    pub width: Opt<Length>,
    pub spacing: Opt<f32>,
    pub text_size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub wrapping: Opt<Wrapping>,
    pub shaping: Opt<Shaping>,
}

impl std::fmt::Debug for RadioNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadioNode")
            .field("label", &self.label)
            .field("is_selected", &self.is_selected)
            .field("on_select", &"<opaque>")
            .field("size", &self.size)
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("wrapping", &self.wrapping)
            .field("shaping", &self.shaping)
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build(radio: RadioNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let selected = if radio.is_selected { Some(()) } else { None };
    let emit = std::rc::Rc::new(radio.on_select);
    let mut widget =
        iced::widget::Radio::new(std::string::String::from(radio.label), (), selected, {
            let emit = emit.clone();
            move |()| crate::widgets::Emitted::fixed(&emit)
        });
    if let Some(size) = Option::<f32>::from(radio.size) {
        widget = widget.size(size);
    }
    if let Some(width) = Option::<Length>::from(radio.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(spacing) = Option::<f32>::from(radio.spacing) {
        widget = widget.spacing(spacing);
    }
    if let Some(text_size) = Option::<f32>::from(radio.text_size) {
        widget = widget.text_size(text_size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(radio.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(wrapping) = Option::<Wrapping>::from(radio.wrapping) {
        widget = widget.wrapping(iced::widget::text::Wrapping::from(wrapping));
    }
    if let Some(shaping) = Option::<Shaping>::from(radio.shaping) {
        widget = widget.shaping(iced::widget::text::Shaping::from(shaping));
    }
    widget
}

/// A builder for a [`RadioNode`].
pub struct Radio<Message> {
    label: std::string::String,
    is_selected: bool,
    on_select: Option<std::boxed::Box<dyn Fn() -> Message>>,
    size: Option<f32>,
    width: Option<Length>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    line_height: Option<LineHeight>,
    wrapping: Option<Wrapping>,
    shaping: Option<Shaping>,
}

impl<Message> Radio<Message> {
    /// Creates a radio button with the given label and selection state.
    pub fn new(label: impl Into<std::string::String>, is_selected: bool) -> Self {
        Self {
            label: label.into(),
            is_selected,
            on_select: None,
            size: None,
            width: None,
            spacing: None,
            text_size: None,
            line_height: None,
            wrapping: None,
            shaping: None,
        }
    }

    /// Sets the message emitted when this radio button is selected.
    pub fn on_select(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_select = Some(std::boxed::Box::new(emit));
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = Some(wrapping);
        self
    }

    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = Some(shaping);
        self
    }
}

impl<Message> std::fmt::Debug for Radio<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Radio")
            .field("label", &self.label)
            .field("is_selected", &self.is_selected)
            .field("on_select", &self.on_select.as_ref().map(|_| "<opaque>"))
            .field("size", &self.size)
            .field("width", &self.width)
            .field("spacing", &self.spacing)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("wrapping", &self.wrapping)
            .field("shaping", &self.shaping)
            .finish()
    }
}

impl<Message: 'static> From<Radio<Message>> for Element<Message> {
    fn from(radio: Radio<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_select = realize.fixed(
                radio
                    .on_select
                    .expect("Radio::on_select must be set before it is built"),
            );
            arena.push(Node::Radio(RadioNode {
                label: radio.label.into(),
                is_selected: radio.is_selected,
                on_select,
                size: radio.size.into(),
                width: radio.width.into(),
                spacing: radio.spacing.into(),
                text_size: radio.text_size.into(),
                line_height: radio.line_height.into(),
                wrapping: radio.wrapping.into(),
                shaping: radio.shaping.into(),
            }))
        })
    }
}

/// Creates a [`Radio`] button with the given label and selection state.
pub fn radio<Message>(label: impl Into<std::string::String>, is_selected: bool) -> Radio<Message> {
    Radio::new(label, is_selected)
}
