//! A horizontal or vertical rule for dividing content.

use crate::widgets::Node;
use crate::widgets::element::{Arena, Element, Realize};

/// A horizontal or vertical rule for dividing content.
#[stabby::stabby]
#[derive(Debug)]
pub struct RuleNode {
    pub is_horizontal: bool,
    pub thickness: f32,
}

#[cfg(feature = "host")]
pub(crate) fn build(rule: RuleNode) -> iced::Element<'static, crate::widgets::Emitted> {
    if rule.is_horizontal {
        iced::widget::rule::horizontal(rule.thickness).into()
    } else {
        iced::widget::rule::vertical(rule.thickness).into()
    }
}

/// A builder for a [`RuleNode`].
#[derive(Debug)]
pub struct Rule {
    is_horizontal: bool,
    thickness: f32,
}

impl Rule {
    /// Creates a horizontal rule with the given thickness.
    pub fn horizontal(thickness: f32) -> Self {
        Self {
            is_horizontal: true,
            thickness,
        }
    }

    /// Creates a vertical rule with the given thickness.
    pub fn vertical(thickness: f32) -> Self {
        Self {
            is_horizontal: false,
            thickness,
        }
    }
}

impl<Message: 'static> From<Rule> for Element<Message> {
    fn from(rule: Rule) -> Self {
        Element::new(move |_realize: &dyn Realize<Message>, arena: &mut Arena| {
            arena.push(Node::Rule(RuleNode {
                is_horizontal: rule.is_horizontal,
                thickness: rule.thickness,
            }))
        })
    }
}

/// Creates a horizontal [`Rule`] with the given thickness.
pub fn horizontal_rule(thickness: f32) -> Rule {
    Rule::horizontal(thickness)
}

/// Creates a vertical [`Rule`] with the given thickness.
pub fn vertical_rule(thickness: f32) -> Rule {
    Rule::vertical(thickness)
}
