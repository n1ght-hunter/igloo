use crate::{
    Element,
    bindings::iced::app::{self, element::float_to_element},
    element::Widget,
};

/// Displays floating content on top of the application.
#[derive(Debug)]
pub struct Float<Message> {
    content: Element<Message>,
    scale: Option<f32>,
    translation: Option<app::float::Translation>,
}

impl<Message> Float<Message> {
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
        self.translation = Some(app::float::Translation { x, y });
        self
    }
}

impl<Message> Widget<Message> for Float<Message> {
    fn as_element(
        self: Box<Self>,
        create_message: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        float_to_element(app::float::Float {
            content: self.content.as_element(create_message),
            scale: self.scale,
            translation: self.translation,
        })
    }
}

impl<Message: 'static> From<Float<Message>> for Element<Message> {
    fn from(float: Float<Message>) -> Self {
        Element::new(Box::new(float))
    }
}
