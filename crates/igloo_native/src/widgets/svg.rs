//! A vector graphics image.

use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};

/// A vector graphics image.
#[stabby::stabby]
#[derive(Debug)]
pub struct SvgNode {
    pub path: stabby::string::String,
}

#[cfg(feature = "host")]
pub(crate) fn build(svg: SvgNode) -> impl iced::Widget<crate::widgets::Emitted> + 'static {
    let handle = iced::advanced::svg::Handle::from_path(std::string::String::from(svg.path));
    iced::widget::Svg::new(handle)
}

/// A builder for an [`SvgNode`].
#[derive(Debug)]
pub struct Svg {
    path: std::string::String,
}

impl Svg {
    /// Creates a new svg from the given path.
    pub fn new(path: impl Into<std::string::String>) -> Self {
        Self { path: path.into() }
    }
}

impl<Message: 'static> From<Svg> for Element<Message> {
    fn from(svg: Svg) -> Self {
        Element::new(move |_realize: &dyn Realize<Message>, arena: &mut Arena| {
            arena.push(Node::Svg(SvgNode {
                path: svg.path.into(),
            }))
        })
    }
}

/// Creates an [`Svg`] from the given path.
pub fn svg(path: impl Into<std::string::String>) -> Svg {
    Svg::new(path)
}
