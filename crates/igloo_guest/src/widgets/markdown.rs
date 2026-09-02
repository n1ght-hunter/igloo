use crate::Element;
use crate::bindings::iced::app::widgets::{MarkdownNode, Node};

/// A widget that can parse and display Markdown.
pub struct Markdown<Message> {
    content: String,
    on_link_click: Box<dyn Fn(String) -> Message>,
}

impl<Message: 'static> Markdown<Message> {
    /// Creates a new [`Markdown`] widget from the provided source, mapping
    /// clicked link URLs to a message through `on_link_click`.
    pub fn new(
        content: impl Into<String>,
        on_link_click: impl Fn(String) -> Message + 'static,
    ) -> Self {
        Self {
            content: content.into(),
            on_link_click: Box::new(on_link_click),
        }
    }
}

impl<Message: 'static> From<Markdown<Message>> for Element<Message> {
    fn from(markdown: Markdown<Message>) -> Self {
        Element::new(move |realize, arena| {
            let node = MarkdownNode {
                content: markdown.content,
                on_link_click: realize.string_mapper(markdown.on_link_click),
            };
            arena.push(Node::Markdown(node))
        })
    }
}
