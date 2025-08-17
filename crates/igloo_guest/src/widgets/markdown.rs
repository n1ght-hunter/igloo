use crate::{
    bindings::iced::app::element::markdown_to_element,
    element::Widget,
};

/// A widget that can parse and display Markdown.
#[derive(Debug)]
pub struct Markdown {
    content: String,
}

impl Markdown {
    /// Creates a new [`Markdown`] widget from the provided source.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl<'a, Message> Widget<Message> for Markdown {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        markdown_to_element(&crate::bindings::iced::app::markdown::Markdown {
            content: self.content,
        })
    }
}
