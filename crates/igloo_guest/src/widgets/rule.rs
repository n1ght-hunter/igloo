use iced_core::Pixels;

use crate::Element;
use crate::bindings::iced::app::widgets::{Node, RuleNode};

/// Display a horizontal or vertical rule for dividing content.
pub struct Rule {
    is_horizontal: bool,
    thickness: f32,
}

impl Rule {
    /// Creates a horizontal [`Rule`] with the given thickness.
    pub fn horizontal(thickness: impl Into<Pixels>) -> Self {
        Self {
            is_horizontal: true,
            thickness: thickness.into().0,
        }
    }

    /// Creates a vertical [`Rule`] with the given thickness.
    pub fn vertical(thickness: impl Into<Pixels>) -> Self {
        Self {
            is_horizontal: false,
            thickness: thickness.into().0,
        }
    }
}

impl<Message: 'static> From<Rule> for Element<Message> {
    fn from(rule: Rule) -> Self {
        Element::new(move |_realize, arena| {
            arena.push(Node::Rule(RuleNode {
                is_horizontal: rule.is_horizontal,
                thickness: rule.thickness,
            }))
        })
    }
}
