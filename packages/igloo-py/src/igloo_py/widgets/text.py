"""Text widget builder."""

from typing import Any, Optional, TYPE_CHECKING

from ..element import Element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength
    from ..types.color import Color


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
        self._text = text
        self._size: Optional[float] = None
        self._line_height: Any = None
        self._width: Any = None
        self._height: Any = None
        self._center: Optional[bool] = None
        self._align_x: Any = None
        self._align_y: Any = None
        self._shaping: Any = None
        self._wrapping: Any = None
        self._color: Any = None

    @classmethod
    def new(cls, text: str) -> "Text":
        """Create a new Text builder with the given content."""
        return cls(text)

    def size(self, size: float) -> "Text":
        """Set the text size in pixels."""
        self._size = size
        return self

    def line_height(self, line_height: Any) -> "Text":
        """Set the line height."""
        self._line_height = line_height
        return self

    def width(self, width: "WitLength") -> "Text":
        """Set the width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Text":
        """Set the height."""
        self._height = height
        return self

    def center(self, center: bool = True) -> "Text":
        """Center the text (shorthand for align_x: center and align_y: center)."""
        self._center = center
        return self

    def align_x(self, align: Any) -> "Text":
        """Set horizontal text alignment."""
        self._align_x = align
        return self

    def align_y(self, align: Any) -> "Text":
        """Set vertical text alignment."""
        self._align_y = align
        return self

    def shaping(self, shaping: Any) -> "Text":
        """Set the text shaping strategy."""
        self._shaping = shaping
        return self

    def wrapping(self, wrapping: Any) -> "Text":
        """Set the text wrapping strategy."""
        self._wrapping = wrapping
        return self

    def color(self, color: "Color") -> "Text":
        """Set the text color."""
        self._color = color
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        try:
            from ..generated.wit_world.imports.text import Text as WitText
            from ..generated.wit_world.imports.element import text_to_element

            record = WitText(
                text=self._text,
                size=self._size,
                line_height=self._line_height,
                width=self._width,
                height=self._height,
                center=self._center,
                align_x=self._align_x,
                align_y=self._align_y,
                shaping=self._shaping,
                wrapping=self._wrapping,
                color=self._color,
            )
            return Element(text_to_element(record))
        except ImportError:
            # If bindings not generated yet, return a placeholder
            return Element(None)
