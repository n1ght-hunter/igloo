//! A clickable button wrapping a single child node.

use stabby::option::Option as Opt;

use crate::Emit;
use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::{Length, Padding};

/// A clickable button wrapping a single child node.
#[stabby::stabby]
pub struct ButtonNode {
    pub content: crate::widgets::NodeId,
    pub on_press: Opt<Emit>,
    pub width: Opt<Length>,
    pub height: Opt<Length>,
    pub padding: Opt<Padding>,
    pub clip: Opt<bool>,
}

impl std::fmt::Debug for ButtonNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonNode")
            .field("content", &self.content)
            .field("on_press", &"<opaque>")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("clip", &self.clip)
            .finish()
    }
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    button: ButtonNode,
) -> iced::Element<'a, crate::widgets::Emitted> {
    let content = crate::widgets::build_node(nodes, button.content);
    let mut widget = iced::widget::button(content);
    if let Some(emit) = Option::from(button.on_press) {
        let emit = std::rc::Rc::new(emit);
        widget = widget.on_press_with(move || crate::widgets::Emitted::fixed(&emit));
    }
    if let Some(width) = Option::<Length>::from(button.width) {
        widget = widget.width(iced::Length::from(width));
    }
    if let Some(height) = Option::<Length>::from(button.height) {
        widget = widget.height(iced::Length::from(height));
    }
    if let Some(padding) = Option::<Padding>::from(button.padding) {
        widget = widget.padding(iced::Padding::from(padding));
    }
    if let Some(clip) = Option::<bool>::from(button.clip) {
        widget = widget.clip(clip);
    }
    widget.into()
}

/// A builder for a [`ButtonNode`].
pub struct Button<Message> {
    content: Element<Message>,
    on_press: Option<std::boxed::Box<dyn Fn() -> Message>>,
    width: Option<Length>,
    height: Option<Length>,
    padding: Option<Padding>,
    clip: Option<bool>,
}

impl<Message> Button<Message> {
    /// Creates a button wrapping `content`.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            on_press: None,
            width: None,
            height: None,
            padding: None,
            clip: None,
        }
    }

    /// Sets the message emitted when the button is pressed.
    pub fn on_press(mut self, emit: impl Fn() -> Message + 'static) -> Self {
        self.on_press = Some(std::boxed::Box::new(emit));
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

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = Some(clip);
        self
    }
}

impl<Message> std::fmt::Debug for Button<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("content", &self.content)
            .field("on_press", &self.on_press.as_ref().map(|_| "<opaque>"))
            .field("width", &self.width)
            .field("height", &self.height)
            .field("padding", &self.padding)
            .field("clip", &self.clip)
            .finish()
    }
}

impl<Message: 'static> From<Button<Message>> for Element<Message> {
    fn from(button: Button<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let content = button.content.build(realize, arena);
            let on_press = button.on_press.map(|emit| realize.fixed(emit));
            arena.push(Node::Button(ButtonNode {
                content,
                on_press: on_press.into(),
                width: button.width.into(),
                height: button.height.into(),
                padding: button.padding.into(),
                clip: button.clip.into(),
            }))
        })
    }
}

/// Creates a [`Button`] wrapping `content`.
pub fn button<Message>(content: impl Into<Element<Message>>) -> Button<Message> {
    Button::new(content)
}
