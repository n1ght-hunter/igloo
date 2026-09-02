use crate::Element;
use crate::bindings::iced::app::widgets::{Node, SvgNode};

/// A vector graphics image.
pub struct Svg {
    path: String,
}

impl Svg {
    /// Creates a new [`Svg`] from the given path.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl<Message: 'static> From<Svg> for Element<Message> {
    fn from(svg: Svg) -> Self {
        Element::new(move |_realize, arena| arena.push(Node::Svg(SvgNode { path: svg.path })))
    }
}
