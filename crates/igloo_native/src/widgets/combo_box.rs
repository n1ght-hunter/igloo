//! A widget allowing selection from a list of options with text input.

use stabby::option::Option as Opt;

use crate::Emit;
use crate::messages::EmitString;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Length, LineHeight, Padding};

/// A widget allowing selection from a list of options with text input.
#[stabby::stabby(no_opt)]
pub struct ComboBoxNode {
    pub options: stabby::vec::Vec<stabby::string::String>,
    pub placeholder: stabby::string::String,
    pub selected: Opt<stabby::string::String>,
    pub on_selected: EmitString,
    pub on_input: Opt<EmitString>,
    pub on_option_hovered: Opt<EmitString>,
    pub on_open: Opt<Emit>,
    pub on_close: Opt<Emit>,
    pub padding: Opt<Padding>,
    pub size: Opt<f32>,
    pub line_height: Opt<LineHeight>,
    pub width: Opt<Length>,
}

impl std::fmt::Debug for ComboBoxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComboBoxNode")
            .field("options", &self.options)
            .field("placeholder", &self.placeholder)
            .field("selected", &self.selected)
            .field("on_selected", &"<opaque>")
            .field("on_input", &self.on_input.as_ref().map(|_| "<opaque>"))
            .field(
                "on_option_hovered",
                &self.on_option_hovered.as_ref().map(|_| "<opaque>"),
            )
            .field("on_open", &self.on_open.as_ref().map(|_| "<opaque>"))
            .field("on_close", &self.on_close.as_ref().map(|_| "<opaque>"))
            .field("padding", &self.padding)
            .field("size", &self.size)
            .field("line_height", &self.line_height)
            .field("width", &self.width)
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    node: ComboBoxNode,
    state: &'a iced::widget::combo_box::State<std::string::String>,
) -> impl iced::Widget<crate::widgets::Emitted> + 'a {
    let placeholder = std::string::String::from(node.placeholder);
    let selected =
        Option::<stabby::string::String>::from(node.selected).map(std::string::String::from);

    // iced's `ComboBox` borrows its `State` for the element lifetime. The
    // prepared view owns that state so rebuilding a view does not leak it.
    let on_selected = std::rc::Rc::new(node.on_selected);
    let mut widget = iced::widget::ComboBox::new(state, placeholder, selected.as_ref(), {
        let on_selected = on_selected.clone();
        move |value| crate::widgets::Emitted::str(&on_selected, value)
    });

    if let Some(emit) = Option::from(node.on_input) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_input(move |value| crate::widgets::Emitted::str(&emit, value));
    }
    if let Some(emit) = Option::from(node.on_option_hovered) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_option_hovered(move |value| crate::widgets::Emitted::str(&emit, value));
    }
    if let Some(emit) = Option::from(node.on_open) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_open(crate::widgets::Emitted::fixed(&emit));
    }
    if let Some(emit) = Option::from(node.on_close) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_close(crate::widgets::Emitted::fixed(&emit));
    }
    if let Some(padding) = Option::<Padding>::from(node.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    if let Some(size) = Option::<f32>::from(node.size) {
        widget = widget.size(size);
    }
    if let Some(line_height) = Option::<LineHeight>::from(node.line_height) {
        widget = widget.line_height(iced::widget::text::LineHeight::from(line_height));
    }
    if let Some(width) = Option::<Length>::from(node.width) {
        widget = widget.width(iced::Length::from(width));
    }

    widget
}

/// A builder for a [`ComboBoxNode`].
pub struct ComboBox<Message> {
    options: std::vec::Vec<std::string::String>,
    placeholder: std::string::String,
    selected: Option<std::string::String>,
    on_selected: Option<std::boxed::Box<dyn Fn(std::string::String) -> Message>>,
    on_input: Option<std::boxed::Box<dyn Fn(std::string::String) -> Message>>,
    on_option_hovered: Option<std::boxed::Box<dyn Fn(std::string::String) -> Message>>,
    on_open: Option<std::boxed::Box<dyn Fn() -> Message>>,
    on_close: Option<std::boxed::Box<dyn Fn() -> Message>>,
    padding: Option<Padding>,
    size: Option<f32>,
    line_height: Option<LineHeight>,
    width: Option<Length>,
}

impl<Message> ComboBox<Message> {
    /// Creates a combo box with the given options, placeholder and current selection.
    pub fn new(
        options: impl IntoIterator<Item = impl Into<std::string::String>>,
        placeholder: impl Into<std::string::String>,
        selected: Option<impl Into<std::string::String>>,
    ) -> Self {
        Self {
            options: options.into_iter().map(Into::into).collect(),
            placeholder: placeholder.into(),
            selected: selected.map(Into::into),
            on_selected: None,
            on_input: None,
            on_option_hovered: None,
            on_open: None,
            on_close: None,
            padding: None,
            size: None,
            line_height: None,
            width: None,
        }
    }

    /// Sets the message emitted when an option is selected.
    pub fn on_selected(mut self, emit: impl Fn(std::string::String) -> Message + 'static) -> Self {
        self.on_selected = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted on every keystroke while typing.
    pub fn on_input(mut self, emit: impl Fn(std::string::String) -> Message + 'static) -> Self {
        self.on_input = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted when an option is hovered.
    pub fn on_option_hovered(
        mut self,
        emit: impl Fn(std::string::String) -> Message + 'static,
    ) -> Self {
        self.on_option_hovered = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted when the combo box is opened.
    pub fn on_open(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_open = Some(std::boxed::Box::new(emit));
        self
    }

    /// Sets the message emitted when the combo box is closed.
    pub fn on_close(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_close = Some(std::boxed::Box::new(emit));
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

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
}

impl<Message> std::fmt::Debug for ComboBox<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComboBox")
            .field("options", &self.options)
            .field("placeholder", &self.placeholder)
            .field("selected", &self.selected)
            .field(
                "on_selected",
                &self.on_selected.as_ref().map(|_| "<opaque>"),
            )
            .field("on_input", &self.on_input.as_ref().map(|_| "<opaque>"))
            .field(
                "on_option_hovered",
                &self.on_option_hovered.as_ref().map(|_| "<opaque>"),
            )
            .field("on_open", &self.on_open.as_ref().map(|_| "<opaque>"))
            .field("on_close", &self.on_close.as_ref().map(|_| "<opaque>"))
            .field("padding", &self.padding)
            .field("size", &self.size)
            .field("line_height", &self.line_height)
            .field("width", &self.width)
            .finish()
    }
}

impl<Message: 'static> From<ComboBox<Message>> for Element<Message> {
    fn from(combo_box: ComboBox<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let on_selected = realize.string_mapper(
                combo_box
                    .on_selected
                    .expect("ComboBox::on_selected must be set before it is built"),
            );
            let on_input = combo_box.on_input.map(|emit| realize.string_mapper(emit));
            let on_option_hovered = combo_box
                .on_option_hovered
                .map(|emit| realize.string_mapper(emit));
            let on_open = combo_box.on_open.map(|emit| realize.fixed(emit));
            let on_close = combo_box.on_close.map(|emit| realize.fixed(emit));
            arena.push(Node::ComboBox(ComboBoxNode {
                options: combo_box.options.into_iter().map(Into::into).collect(),
                placeholder: combo_box.placeholder.into(),
                selected: combo_box.selected.map(Into::into).into(),
                on_selected,
                on_input: on_input.into(),
                on_option_hovered: on_option_hovered.into(),
                on_open: on_open.into(),
                on_close: on_close.into(),
                padding: combo_box.padding.into(),
                size: combo_box.size.into(),
                line_height: combo_box.line_height.into(),
                width: combo_box.width.into(),
            }))
        })
    }
}

/// Creates a [`ComboBox`] with the given options, placeholder and current selection.
pub fn combo_box<Message>(
    options: impl IntoIterator<Item = impl Into<std::string::String>>,
    placeholder: impl Into<std::string::String>,
    selected: Option<impl Into<std::string::String>>,
) -> ComboBox<Message> {
    ComboBox::new(options, placeholder, selected)
}
