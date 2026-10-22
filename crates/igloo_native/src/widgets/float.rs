//! Displays floating content on top of the application.

use stabby::option::Option as Opt;

use crate::widgets::element::{Arena, Element, Realize};
use crate::widgets::types::Translation;
use crate::widgets::{Node, NodeId};

/// Displays floating content on top of the application.
#[stabby::stabby]
#[derive(Debug)]
pub struct FloatNode {
    pub content: NodeId,
    pub scale: Opt<f32>,
    pub translation: Opt<Translation>,
}

#[cfg(feature = "host")]
pub(crate) fn build<'a>(
    nodes: &mut crate::widgets::RenderArena<'a>,
    float: FloatNode,
) -> impl iced::Widget<crate::widgets::Emitted> + 'a {
    let content = crate::widgets::build_node(nodes, float.content);
    let mut widget = iced::widget::Float::new(content);
    if let Some(scale) = Option::<f32>::from(float.scale) {
        widget = widget.scale(scale);
    }
    if let Some(translation) = Option::<Translation>::from(float.translation) {
        widget = widget.translate(move |_, _| iced::Vector::new(translation.x, translation.y));
    }
    widget
}

/// A builder for a [`FloatNode`].
pub struct Float<Message> {
    content: Element<Message>,
    scale: Option<f32>,
    translation: Option<Translation>,
}

impl<Message> Float<Message> {
    /// Creates a new float wrapping `content`.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            scale: None,
            translation: None,
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn translation(mut self, x: f32, y: f32) -> Self {
        self.translation = Some(Translation { x, y });
        self
    }
}

impl<Message> std::fmt::Debug for Float<Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Float")
            .field("content", &self.content)
            .field("scale", &self.scale)
            .field("translation", &self.translation)
            .finish()
    }
}

impl<Message: 'static> From<Float<Message>> for Element<Message> {
    fn from(float: Float<Message>) -> Self {
        Element::new(move |realize: &dyn Realize<Message>, arena: &mut Arena| {
            let content = float.content.build(realize, arena);
            arena.push(Node::Float(FloatNode {
                content,
                scale: float.scale.into(),
                translation: float.translation.into(),
            }))
        })
    }
}

/// Creates a [`Float`] wrapping `content`.
pub fn float<Message>(content: impl Into<Element<Message>>) -> Float<Message> {
    Float::new(content)
}
