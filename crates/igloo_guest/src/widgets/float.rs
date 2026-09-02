use crate::Element;
use crate::bindings::iced::app::widgets::{FloatNode, Node, Translation};

/// Displays floating content on top of the application.
pub struct Float<Message> {
    content: Element<Message>,
    scale: Option<f32>,
    translation: Option<(f32, f32)>,
}

impl<Message: 'static> Float<Message> {
    /// Creates a new [`Float`] with the given content.
    pub fn new(content: impl Into<Element<Message>>) -> Self {
        Self {
            content: content.into(),
            scale: None,
            translation: None,
        }
    }

    /// Sets the scale to be applied to the contents of the [`Float`].
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = Some(scale);
        self
    }

    /// Sets the translation applied to the contents of the [`Float`].
    pub fn translation(mut self, x: f32, y: f32) -> Self {
        self.translation = Some((x, y));
        self
    }
}

impl<Message: 'static> From<Float<Message>> for Element<Message> {
    fn from(float: Float<Message>) -> Self {
        Element::new(move |realize, arena| {
            let content = float.content.build(realize, arena);
            let node = FloatNode {
                content,
                scale: float.scale,
                translation: float.translation.map(|(x, y)| Translation { x, y }),
            };
            arena.push(Node::Float(node))
        })
    }
}
