"""Rule widget builder."""

from ..element import Element, IntoElement
from wit_world.imports.rule import Rule as WitRule
from wit_world.imports.element import rule_to_element


class Rule(IntoElement):
    """
    Builder for creating Rule widgets.
    A Rule is a horizontal or vertical line for dividing content.

    Example:
        # Horizontal rule
        hr = Rule.horizontal(1)

        # Vertical rule
        vr = Rule.vertical(2)
    """

    def __init__(self, is_horizontal: bool, thickness: float) -> None:
        self._is_horizontal = is_horizontal
        self._thickness = thickness

    @classmethod
    def horizontal(cls, thickness: float) -> "Rule":
        """Create a horizontal rule with the given thickness."""
        return cls(True, thickness)

    @classmethod
    def vertical(cls, thickness: float) -> "Rule":
        """Create a vertical rule with the given thickness."""
        return cls(False, thickness)

    def into_element(self) -> Element:
        """Convert to Element."""
        record = WitRule(
            is_horizontal=self._is_horizontal,
            thickness=self._thickness,
        )
        return Element(rule_to_element(record))
