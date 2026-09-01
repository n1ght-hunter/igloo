"""Text widget builder."""

from typing import TYPE_CHECKING, Any

from wit_world.imports.text import Text as WitText

from ..element import Element, IntoElement

if TYPE_CHECKING:
    from ..types.color import Color
    from ..types.length import WitLength


class Text(IntoElement):
    """
    Builder for creating Text widgets.

    Example:
        # Can be used directly where Element is expected
        Column.new().push(Text.new("Hello, World!").size(24))

        # Or explicitly converted
        label = Text.new("Hello, World!").into_element()
    """

    def __init__(self, text: str) -> None:
        self._raw = WitText(text)

    @classmethod
    def new(cls, text: str) -> "Text":
        """Create a new Text builder with the given content."""
        return cls(text)

    def size(self, size: float) -> "Text":
        """Set the text size in pixels."""
        self._raw.size(size)
        return self

    def line_height(self, line_height: Any) -> "Text":
        """Set the line height."""
        self._raw.line_height(line_height)
        return self

    def width(self, width: "WitLength") -> "Text":
        """Set the width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Text":
        """Set the height."""
        self._raw.height(height)
        return self

    def center(self, center: bool = True) -> "Text":
        """Center the text (shorthand for align_x: center and align_y: center)."""
        if center:
            self._raw.center()
        return self

    def align_x(self, align: Any) -> "Text":
        """Set horizontal text alignment."""
        self._raw.align_x(align)
        return self

    def align_y(self, align: Any) -> "Text":
        """Set vertical text alignment."""
        self._raw.align_y(align)
        return self

    def color(self, color: "Color") -> "Text":
        """Set the text color."""
        self._raw.color(color)
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        return Element(WitText.into_element(self._raw))
