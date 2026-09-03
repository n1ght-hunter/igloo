//! A box that can be checked.

use stabby::option::Option as Opt;

use crate::messages::EmitBool;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Length, LineHeight, Shaping, Wrapping};

/// A box that can be checked.
#[stabby::stabby(no_opt)]
pub struct CheckboxNode {
    pub is_checked: bool,
    pub label: Opt<stabby::string::String>,
    pub on_toggle: Opt<EmitBool>,
    pub size: Opt<f32>,
    pub width: Opt<Length>,
    pub spacing: Opt<f32>,
    pub text_size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub wrapping: Opt<Wrapping>,
    pub shaping: Opt<Shaping>,
}

impl std::fmt::Debug for CheckboxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckboxNode")
            .field("is_checked", &self.is_checked)
            .field("label", &self.label)
            .field("on_toggle", &"<opaque>")
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
pub(crate) fn build(checkbox: CheckboxNode) -> iced::Element<'static, crate::widgets::Emitted> {
    let mut widget = iced::widget::Checkbox::new(checkbox.is_checked);
    if let Some(label) = Option::<stabby::string::String>::from(checkbox.label) {
        widget = widget.label(std::string::String::from(label));
    }
    if let Some(emit) = Option::from(checkbox.on_toggle) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_toggle(move |value| crate::widgets::Emitted::bool(&emit, value));
    }
    if let Some(size) = Option::<f32>::from(checkbox.size) {
        widget = widget.size(size);
    }
    if let Some(width) = Option::<Length>::from(checkbox.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(spacing) = Option::<f32>::from(checkbox.spacing) {
        widget = widget.spacing(spacing);
    }
    if let Some(text_size) = Option::<f32>::from(checkbox.text_size) {
        widget = widget.text_size(text_size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(checkbox.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(wrapping) = Option::<Wrapping>::from(checkbox.wrapping) {
        widget = widget.wrapping(iced::widget::text::Wrapping::from(wrapping));
    }
    if let Some(shaping) = Option::<Shaping>::from(checkbox.shaping) {
        widget = widget.shaping(iced::widget::text::Shaping::from(shaping));
    }
    widget.into()
}

/// A builder for a [`CheckboxNode`].
pub struct Checkbox<Message> {
    is_checked: bool,
    label: Option<std::string::String>,
    on_toggle: Option<std::boxed::Box<dyn Fn(bool) -> Message>>,
    size: Option<f32>,
    width: Option<Length>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    line_height: Option<LineHeight>,
    wrapping: Option<Wrapping>,
    shaping: Option<Shaping>,
}

impl<Message> Checkbox<Message> {
    /// Creates a checkbox in the given checked state.
    pub fn new(is_checked: bool) -> Self {
        Self {
            is_checked,
            label: None,
            on_toggle: None,
            size: None,
            width: None,
            spacing: None,
            text_size: None,
            line_height: None,
            wrapping: None,
            shaping: None,
        }
    }

    pub fn label(mut self, label: impl Into<std::string::String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the message emitted with the new state when the checkbox is toggled.
    pub fn on_toggle(mut self, emit: impl Fn(bool) -> Message + 'static) -> Self {
        self.on_toggle = Some(std::boxed::Box::new(emit));
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

impl<Message> std::fmt::Debug for Checkbox<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Checkbox")
            .field("is_checked", &self.is_checked)
            .field("label", &self.label)
            .field("on_toggle", &self.on_toggle.as_ref().map(|_| "<opaque>"))
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

impl<Message: 'static> From<Checkbox<Message>> for Element<Message> {
    fn from(checkbox: Checkbox<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_toggle = checkbox.on_toggle.map(|emit| realize.bool_mapper(emit));
            arena.push(Node::Checkbox(CheckboxNode {
                is_checked: checkbox.is_checked,
                label: checkbox.label.map(Into::into).into(),
                on_toggle: on_toggle.into(),
                size: checkbox.size.into(),
                width: checkbox.width.into(),
                spacing: checkbox.spacing.into(),
                text_size: checkbox.text_size.into(),
                line_height: checkbox.line_height.into(),
                wrapping: checkbox.wrapping.into(),
                shaping: checkbox.shaping.into(),
            }))
        })
    }
}

/// Creates a [`Checkbox`] in the given checked state.
pub fn checkbox<Message>(is_checked: bool) -> Checkbox<Message> {
    Checkbox::new(is_checked)
}
