use crate::{
    Element,
    bindings::iced::app::element::svg_to_element,
    element::Widget,
};

/// A vector graphics image.
#[derive(Debug)]
pub struct Svg {
    path: String,
}

impl Svg {
    /// Creates a new [`Svg`] from the given path.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl<'a, Message> Widget<Message> for Svg {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        svg_to_element(&crate::bindings::iced::app::svg::Svg { path: self.path })
    }
}

impl<'a, Message: 'a> From<Svg> for Element<Message> {
    fn from(svg: Svg) -> Self {
        Element::new(Box::new(svg))
    }
}
