"""Column layout widget builder."""

from collections.abc import Callable
from typing import TYPE_CHECKING

from wit_world.imports.column import Column as WitColumn

from ..element import Element, ElementLike, IntoElement, to_element

if TYPE_CHECKING:
    from ..types.alignment import WitHorizontal
    from ..types.length import WitLength
    from ..types.padding import WitPadding


class Column(IntoElement):
    """
    Builder for creating Column layout widgets.
    A Column arranges its children vertically.

    Example:
        # Widgets can be passed directly - no into_element() needed
        col = Column.new()
            .spacing(10)
            .push(Text.new("Item 1"))
            .push(Text.new("Item 2"))
            .push(Button.new(Text.new("Click")).on_press(lambda: msg))
    """

    def __init__(self) -> None:
        self._raw = WitColumn()

    @classmethod
    def new(cls) -> "Column":
        """Create a new empty Column builder."""
        return cls()

    @classmethod
    def with_elements(cls, elements: list[ElementLike]) -> "Column":
        """Create a Column with the given elements."""
        col = cls()
        col.extend(elements)
        return col

    def push(self, element: ElementLike) -> "Column":
        """Add an element to the column."""
        self._raw.push(to_element(element).inner)
        return self

    def push_if(self, condition: bool, element: Callable[[], ElementLike]) -> "Column":
        """Add an element conditionally."""
        if condition:
            self._raw.push(to_element(element()).inner)
        return self

    def extend(self, elements: list[ElementLike]) -> "Column":
        """Add multiple elements."""
        for element in elements:
            self._raw.push(to_element(element).inner)
        return self

    def spacing(self, spacing: float) -> "Column":
        """Set the spacing between elements in pixels."""
        self._raw.spacing(spacing)
        return self

    def padding(self, padding: "WitPadding") -> "Column":
        """Set the padding around the column."""
        self._raw.padding(padding)
        return self

    def width(self, width: "WitLength") -> "Column":
        """Set the column width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Column":
        """Set the column height."""
        self._raw.height(height)
        return self

    def max_width(self, max_width: float) -> "Column":
        """Set the maximum width in pixels."""
        self._raw.max_width(max_width)
        return self

    def align_x(self, align: "WitHorizontal") -> "Column":
        """Set horizontal alignment of children."""
        self._raw.align_x(align)
        return self

    def clip(self, clip: bool = True) -> "Column":
        """Enable or disable clipping of content."""
        self._raw.clip(clip)
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        return Element(WitColumn.into_element(self._raw))
