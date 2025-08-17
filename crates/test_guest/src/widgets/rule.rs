use crate::{Element, bindings::iced::app::element::rule_to_element, element::Widget};
use iced_core::Pixels;

/// Display a horizontal or vertical rule for dividing content.
#[derive(Debug)]
pub struct Rule {
    is_horizontal: bool,
    thickness: Pixels,
}

impl Rule {
    /// Creates a horizontal [`Rule`] with the given thickness.
    pub fn horizontal(thickness: impl Into<Pixels>) -> Self {
        let pixels: Pixels = thickness.into();
        Self {
            is_horizontal: true,
            thickness: pixels,
        }
    }

    /// Creates a vertical [`Rule`] with the given thickness.
    pub fn vertical(thickness: impl Into<Pixels>) -> Self {
        let pixels: Pixels = thickness.into();
        Self {
            is_horizontal: false,
            thickness: pixels,
        }
    }
}

impl<'a, Message> Widget<Message> for Rule {
    fn as_element(
        self: Box<Self>,
        _: &dyn crate::element::CreateMessage<Message>,
    ) -> crate::bindings::Element {
        rule_to_element(crate::bindings::iced::app::rule::Rule {
            is_horizontal: self.is_horizontal,
            thickness: self.thickness.into(),
        })
    }
}

impl<'a, Message: 'a> From<Rule> for Element<Message> {
    fn from(rule: Rule) -> Self {
        Element::new(Box::new(rule))
    }
}
