//! A toggle switch.

use stabby::option::Option as Opt;

use crate::messages::EmitBool;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Horizontal, Length, LineHeight, Shaping, Wrapping};

/// A toggle switch.
#[stabby::stabby(no_opt)]
pub struct TogglerNode {
    pub is_toggled: bool,
    pub label: Opt<stabby::string::String>,
    pub on_toggle: Opt<EmitBool>,
    pub size: Opt<f32>,
    pub width: Opt<Length>,
    pub text_size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub alignment: Opt<Horizontal>,
    pub shaping: Opt<Shaping>,
    pub wrapping: Opt<Wrapping>,
    pub spacing: Opt<f32>,
}

impl std::fmt::Debug for TogglerNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TogglerNode")
            .field("is_toggled", &self.is_toggled)
            .field("label", &self.label)
            .field("on_toggle", &"<opaque>")
            .field("size", &self.size)
            .field("width", &self.width)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("alignment", &self.alignment)
            .field("shaping", &self.shaping)
            .field("wrapping", &self.wrapping)
            .field("spacing", &self.spacing)
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build(toggler: TogglerNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let mut widget = iced::widget::Toggler::new(toggler.is_toggled);
    if let Some(label) = Option::<stabby::string::String>::from(toggler.label) {
        widget = widget.label(std::string::String::from(label));
    }
    if let Some(emit) = Option::from(toggler.on_toggle) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_toggle(move |value| crate::widgets::Emitted::bool(&emit, value));
    }
    if let Some(size) = Option::<f32>::from(toggler.size) {
        widget = widget.size(size);
    }
    if let Some(width) = Option::<Length>::from(toggler.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(text_size) = Option::<f32>::from(toggler.text_size) {
        widget = widget.text_size(text_size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(toggler.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(alignment) = Option::<Horizontal>::from(toggler.alignment) {
        widget = widget.alignment(iced::alignment::Horizontal::from(alignment));
    }
    if let Some(shaping) = Option::<Shaping>::from(toggler.shaping) {
        widget = widget.shaping(iced::widget::text::Shaping::from(shaping));
    }
    if let Some(wrapping) = Option::<Wrapping>::from(toggler.wrapping) {
        widget = widget.wrapping(iced::widget::text::Wrapping::from(wrapping));
    }
    if let Some(spacing) = Option::<f32>::from(toggler.spacing) {
        widget = widget.spacing(spacing);
    }
    widget
}

/// A builder for a [`TogglerNode`].
pub struct Toggler<Message> {
    is_toggled: bool,
    label: Option<std::string::String>,
    on_toggle: Option<std::boxed::Box<dyn Fn(bool) -> Message>>,
    size: Option<f32>,
    width: Option<Length>,
    text_size: Option<f32>,
    line_height: Option<LineHeight>,
    alignment: Option<Horizontal>,
    shaping: Option<Shaping>,
    wrapping: Option<Wrapping>,
    spacing: Option<f32>,
}

impl<Message> Toggler<Message> {
    /// Creates a toggler in the given state.
    pub fn new(is_toggled: bool) -> Self {
        Self {
            is_toggled,
            label: None,
            on_toggle: None,
            size: None,
            width: None,
            text_size: None,
            line_height: None,
            alignment: None,
            shaping: None,
            wrapping: None,
            spacing: None,
        }
    }

    pub fn label(mut self, label: impl Into<std::string::String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the message emitted with the new state when the toggler is flipped.
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

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn alignment(mut self, alignment: Horizontal) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = Some(shaping);
        self
    }

    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = Some(wrapping);
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing);
        self
    }
}

impl<Message> std::fmt::Debug for Toggler<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toggler")
            .field("is_toggled", &self.is_toggled)
            .field("label", &self.label)
            .field("on_toggle", &self.on_toggle.as_ref().map(|_| "<opaque>"))
            .field("size", &self.size)
            .field("width", &self.width)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("alignment", &self.alignment)
            .field("shaping", &self.shaping)
            .field("wrapping", &self.wrapping)
            .field("spacing", &self.spacing)
            .finish()
    }
}

impl<Message: 'static> From<Toggler<Message>> for Element<Message> {
    fn from(toggler: Toggler<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_toggle = toggler.on_toggle.map(|emit| realize.bool_mapper(emit));
            arena.push(Node::Toggler(TogglerNode {
                is_toggled: toggler.is_toggled,
                label: toggler.label.map(Into::into).into(),
                on_toggle: on_toggle.into(),
                size: toggler.size.into(),
                width: toggler.width.into(),
                text_size: toggler.text_size.into(),
                line_height: toggler.line_height.into(),
                alignment: toggler.alignment.into(),
                shaping: toggler.shaping.into(),
                wrapping: toggler.wrapping.into(),
                spacing: toggler.spacing.into(),
            }))
        })
    }
}

/// Creates a [`Toggler`] in the given state.
pub fn toggler<Message>(is_toggled: bool) -> Toggler<Message> {
    Toggler::new(is_toggled)
}
