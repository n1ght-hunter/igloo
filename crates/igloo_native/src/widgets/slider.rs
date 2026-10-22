//! A horizontal bar that selects a value by dragging.

use stabby::option::Option as Opt;

use crate::Emit;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::Length;

/// A horizontal bar that selects a value by dragging.
#[stabby::stabby]
pub struct SliderNode {
    pub range_start: f32,
    pub range_end: f32,
    pub value: f32,
    pub on_change: crate::messages::EmitF32,
    pub default: Opt<f32>,
    pub on_release: Opt<Emit>,
    pub width: Opt<Length>,
    pub height: Opt<f32>,
    pub step: Opt<f32>,
    pub shift_step: Opt<f32>,
}

impl std::fmt::Debug for SliderNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SliderNode")
            .field("range_start", &self.range_start)
            .field("range_end", &self.range_end)
            .field("value", &self.value)
            .field("on_change", &"<opaque>")
            .field("default", &self.default)
            .field("on_release", &"<opaque>")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("step", &self.step)
            .field("shift_step", &self.shift_step)
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build(slider: SliderNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let emit = std::rc::Rc::new(slider.on_change);
    let mut widget =
        iced::widget::Slider::new(slider.range_start..=slider.range_end, slider.value, {
            let emit = emit.clone();
            move |value| crate::widgets::Emitted::f32(&emit, value)
        });
    if let Some(default) = Option::<f32>::from(slider.default) {
        widget = widget.default(default);
    }
    if let Some(emit) = Option::from(slider.on_release) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_release(crate::widgets::Emitted::fixed(&emit));
    }
    if let Some(width) = Option::<Length>::from(slider.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<f32>::from(slider.height) {
        widget = widget.height(height);
    }
    if let Some(step) = Option::<f32>::from(slider.step) {
        widget = widget.step(step);
    }
    if let Some(shift_step) = Option::<f32>::from(slider.shift_step) {
        widget = widget.shift_step(shift_step);
    }
    widget
}

/// A builder for a [`SliderNode`].
pub struct Slider<Message> {
    range_start: f32,
    range_end: f32,
    value: f32,
    on_change: Option<std::boxed::Box<dyn Fn(f32) -> Message>>,
    default: Option<f32>,
    on_release: Option<std::boxed::Box<dyn Fn() -> Message>>,
    width: Option<Length>,
    height: Option<f32>,
    step: Option<f32>,
    shift_step: Option<f32>,
}

impl<Message> Slider<Message> {
    /// Creates a slider over `range_start..=range_end`, at `value`.
    pub fn new(range_start: f32, range_end: f32, value: f32) -> Self {
        Self {
            range_start,
            range_end,
            value,
            on_change: None,
            default: None,
            on_release: None,
            width: None,
            height: None,
            step: None,
            shift_step: None,
        }
    }

    /// Sets the message emitted with the new value while dragging.
    pub fn on_change(mut self, emit: impl Fn(f32) -> Message + 'static) -> Self {
        self.on_change = Some(std::boxed::Box::new(emit));
        self
    }

    pub fn default(mut self, default: f32) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets the message emitted when the drag is released.
    pub fn on_release(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_release = Some(std::boxed::Box::new(emit));
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn shift_step(mut self, shift_step: f32) -> Self {
        self.shift_step = Some(shift_step);
        self
    }
}

impl<Message> std::fmt::Debug for Slider<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slider")
            .field("range_start", &self.range_start)
            .field("range_end", &self.range_end)
            .field("value", &self.value)
            .field("on_change", &self.on_change.as_ref().map(|_| "<opaque>"))
            .field("default", &self.default)
            .field("on_release", &self.on_release.as_ref().map(|_| "<opaque>"))
            .field("width", &self.width)
            .field("height", &self.height)
            .field("step", &self.step)
            .field("shift_step", &self.shift_step)
            .finish()
    }
}

impl<Message: 'static> From<Slider<Message>> for Element<Message> {
    fn from(slider: Slider<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_change = realize.f32_mapper(
                slider
                    .on_change
                    .expect("Slider::on_change must be set before it is built"),
            );
            let on_release = slider.on_release.map(|emit| realize.fixed(emit));
            arena.push(Node::Slider(SliderNode {
                range_start: slider.range_start,
                range_end: slider.range_end,
                value: slider.value,
                on_change,
                default: slider.default.into(),
                on_release: on_release.into(),
                width: slider.width.into(),
                height: slider.height.into(),
                step: slider.step.into(),
                shift_step: slider.shift_step.into(),
            }))
        })
    }
}

/// Creates a [`Slider`] over `range_start..=range_end`, at `value`.
pub fn slider<Message>(range_start: f32, range_end: f32, value: f32) -> Slider<Message> {
    Slider::new(range_start, range_end, value)
}
