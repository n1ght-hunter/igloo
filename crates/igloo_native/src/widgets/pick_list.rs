//! A widget for selecting a value from a set of options.

use stabby::option::Option as Opt;

use crate::Emit;
use crate::messages::EmitString;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Length, LineHeight, Padding, Shaping};

/// A widget for selecting a value from a set of options.
#[stabby::stabby(no_opt)]
pub struct PickListNode {
    pub options: stabby::vec::Vec<stabby::string::String>,
    pub selected: Opt<stabby::string::String>,
    pub on_select: EmitString,
    pub placeholder: Opt<stabby::string::String>,
    pub width: Opt<Length>,
    pub padding: Opt<Padding>,
    pub text_size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub shaping: Opt<Shaping>,
    pub on_open: Opt<Emit>,
    pub on_close: Opt<Emit>,
}

impl std::fmt::Debug for PickListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PickListNode")
            .field("options", &self.options)
            .field("selected", &self.selected)
            .field("on_select", &"<opaque>")
            .field("placeholder", &self.placeholder)
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("shaping", &self.shaping)
            .field("on_open", &self.on_open.as_ref().map(|_| "<opaque>"))
            .field("on_close", &self.on_close.as_ref().map(|_| "<opaque>"))
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build(node: PickListNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let options: std::vec::Vec<std::string::String> = node
        .options
        .into_iter()
        .map(std::string::String::from)
        .collect();
    let selected =
        Option::<stabby::string::String>::from(node.selected).map(std::string::String::from);

    let emit = std::rc::Rc::new(node.on_select);
    let mut widget =
        iced::widget::PickList::new(selected, options, |value: &std::string::String| {
            value.clone()
        })
        .on_select({
            let emit = emit.clone();
            move |value| crate::widgets::Emitted::str(&emit, value)
        });

    if let Some(placeholder) = Option::<stabby::string::String>::from(node.placeholder) {
        widget = widget.placeholder(std::string::String::from(placeholder));
    }
    if let Some(width) = Option::<Length>::from(node.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(padding) = Option::<Padding>::from(node.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    if let Some(text_size) = Option::<f32>::from(node.text_size) {
        widget = widget.text_size(text_size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(node.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(shaping) = Option::<Shaping>::from(node.shaping) {
        widget = widget.shaping(iced::widget::text::Shaping::from(shaping));
    }
    if let Some(emit) = Option::from(node.on_open) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_open(crate::widgets::Emitted::fixed(&emit));
    }
    if let Some(emit) = Option::from(node.on_close) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_close(crate::widgets::Emitted::fixed(&emit));
    }

    widget
}

/// A builder for a [`PickListNode`].
pub struct PickList<Message> {
    options: std::vec::Vec<std::string::String>,
    selected: Option<std::string::String>,
    on_select: Option<std::boxed::Box<dyn Fn(std::string::String) -> Message>>,
    placeholder: Option<std::string::String>,
    width: Option<Length>,
    padding: Option<Padding>,
    text_size: Option<f32>,
    line_height: Option<LineHeight>,
    shaping: Option<Shaping>,
    on_open: Option<std::boxed::Box<dyn Fn() -> Message>>,
    on_close: Option<std::boxed::Box<dyn Fn() -> Message>>,
}

impl<Message> PickList<Message> {
    /// Creates a pick list with the given options and current selection.
    pub fn new(
        options: impl IntoIterator<Item = impl Into<std::string::String>>,
        selected: Option<impl Into<std::string::String>>,
    ) -> Self {
        Self {
            options: options.into_iter().map(Into::into).collect(),
            selected: selected.map(Into::into),
            on_select: None,
            placeholder: None,
            width: None,
            padding: None,
            text_size: None,
            line_height: None,
            shaping: None,
            on_open: None,
            on_close: None,
        }
    }

    /// Sets the message emitted with the chosen option when a selection is made.
    pub fn on_select(mut self, emit: impl Fn(std::string::String) -> Message + 'static) -> Self {
        self.on_select = Some(std::boxed::Box::new(emit));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<std::string::String>) -> Self {
        self.placeholder = Some(placeholder.into());
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

    pub fn text_size(mut self, text_size: f32) -> Self {
        self.text_size = Some(text_size);
        self
    }

    pub fn line_height(mut self, line_height: impl Into<LineHeight>) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = Some(shaping);
        self
    }

    /// Sets the message emitted when the pick list is opened.
    pub fn on_open(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_open = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted when the pick list is closed.
    pub fn on_close(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_close = Some(std::boxed::Box::new(emit));
        self
    }
}

impl<Message> std::fmt::Debug for PickList<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PickList")
            .field("options", &self.options)
            .field("selected", &self.selected)
            .field("on_select", &self.on_select.as_ref().map(|_| "<opaque>"))
            .field("placeholder", &self.placeholder)
            .field("width", &self.width)
            .field("padding", &self.padding)
            .field("text_size", &self.text_size)
            .field("line_height", &self.line_height)
            .field("shaping", &self.shaping)
            .field("on_open", &self.on_open.as_ref().map(|_| "<opaque>"))
            .field("on_close", &self.on_close.as_ref().map(|_| "<opaque>"))
            .finish()
    }
}

impl<Message: 'static> From<PickList<Message>> for Element<Message> {
    fn from(pick_list: PickList<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_select = realize.string_mapper(
                pick_list
                    .on_select
                    .expect("PickList::on_select must be set before it is built"),
            );
            let on_open = pick_list.on_open.map(|emit| realize.fixed(emit));
            let on_close = pick_list.on_close.map(|emit| realize.fixed(emit));
            arena.push(Node::PickList(PickListNode {
                options: pick_list.options.into_iter().map(Into::into).collect(),
                selected: pick_list.selected.map(Into::into).into(),
                on_select,
                placeholder: pick_list.placeholder.map(Into::into).into(),
                width: pick_list.width.into(),
                padding: pick_list.padding.into(),
                text_size: pick_list.text_size.into(),
                line_height: pick_list.line_height.into(),
                shaping: pick_list.shaping.into(),
                on_open: on_open.into(),
                on_close: on_close.into(),
            }))
        })
    }
}

/// Creates a [`PickList`] with the given options and current selection.
pub fn pick_list<Message>(
    options: impl IntoIterator<Item = impl Into<std::string::String>>,
    selected: Option<impl Into<std::string::String>>,
) -> PickList<Message> {
    PickList::new(options, selected)
}
