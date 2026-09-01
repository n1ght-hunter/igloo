use iced_core::Pixels;

use crate::Element;
use crate::bindings::iced::app::rule::Rule as WitRule;

/// Display a horizontal or vertical rule for dividing content.
pub struct Rule {
    raw: WitRule,
}

impl Rule {
    /// Creates a horizontal [`Rule`] with the given thickness.
    pub fn horizontal(thickness: impl Into<Pixels>) -> Self {
        Self {
            raw: WitRule::new(true, thickness.into().0),
        }
    }

    /// Creates a vertical [`Rule`] with the given thickness.
    pub fn vertical(thickness: impl Into<Pixels>) -> Self {
        Self {
            raw: WitRule::new(false, thickness.into().0),
        }
    }
}

impl<Message: 'static> From<Rule> for Element<Message> {
    fn from(rule: Rule) -> Self {
        Element::new(move |_realize| WitRule::into_element(rule.raw))
    }
}
