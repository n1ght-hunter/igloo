use crate::Element;
use crate::bindings::iced::app::svg::Svg as WitSvg;

/// A vector graphics image.
pub struct Svg {
    raw: WitSvg,
}

impl Svg {
    /// Creates a new [`Svg`] from the given path.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            raw: WitSvg::new(&path.into()),
        }
    }
}

impl<Message: 'static> From<Svg> for Element<Message> {
    fn from(svg: Svg) -> Self {
        Element::new(move |_realize| WitSvg::into_element(svg.raw))
    }
}
