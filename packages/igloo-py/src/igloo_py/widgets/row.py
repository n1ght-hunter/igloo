"""Row layout widget builder."""

from collections.abc import Callable
from typing import TYPE_CHECKING

from wit_world.imports.row import Row as WitRow

from ..element import Element, ElementLike, IntoElement, to_element

if TYPE_CHECKING:
    from ..types.alignment import WitVertical
    from ..types.length import WitLength
    from ..types.padding import WitPadding


class Row(IntoElement):
    """
    Builder for creating Row layout widgets.
    A Row arranges its children horizontally.

    Example:
        row = Row.new()
            .spacing(10)
            .push(Text.new("Left"))
            .push(Text.new("Right"))
    """

    def __init__(self) -> None:
        self._raw = WitRow()

    @classmethod
    def new(cls) -> "Row":
        """Create a new empty Row builder."""
        return cls()

    @classmethod
    def with_elements(cls, elements: list[ElementLike]) -> "Row":
        """Create a Row with the given elements."""
        row = cls()
        row.extend(elements)
        return row

    def push(self, element: ElementLike) -> "Row":
        """Add an element to the row."""
        self._raw.push(to_element(element).inner)
        return self

    def push_if(self, condition: bool, element: Callable[[], ElementLike]) -> "Row":
        """Add an element conditionally."""
        if condition:
            self._raw.push(to_element(element()).inner)
        return self

    def extend(self, elements: list[ElementLike]) -> "Row":
        """Add multiple elements."""
        for element in elements:
            self._raw.push(to_element(element).inner)
        return self

    def spacing(self, spacing: float) -> "Row":
        """Set the spacing between elements in pixels."""
        self._raw.spacing(spacing)
        return self

    def padding(self, padding: "WitPadding") -> "Row":
        """Set the padding around the row."""
        self._raw.padding(padding)
        return self

    def width(self, width: "WitLength") -> "Row":
        """Set the row width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Row":
        """Set the row height."""
        self._raw.height(height)
        return self

    def align_y(self, align: "WitVertical") -> "Row":
        """Set vertical alignment of children."""
        self._raw.align_y(align)
        return self

    def clip(self, clip: bool = True) -> "Row":
        """Enable or disable clipping of content."""
        self._raw.clip(clip)
        return self

    def wrap(self, wrap: bool = True) -> "Row":
        """Enable or disable wrapping of elements to the next line."""
        self._raw.wrap(wrap)
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        return Element(WitRow.into_element(self._raw))
