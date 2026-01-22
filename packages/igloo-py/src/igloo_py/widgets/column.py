"""Column layout widget builder."""

from typing import Any, Callable, Optional, List, TYPE_CHECKING

from ..element import Element, ElementLike, to_element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength
    from ..types.padding import WitPadding
    from ..types.alignment import WitHorizontal


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
            .push(Button.new(Text.new("Click")).on_press(messages, lambda: msg))
    """

    def __init__(self) -> None:
        self._elements: List[Any] = []
        self._spacing: Optional[float] = None
        self._padding: Any = None
        self._width: Any = None
        self._height: Any = None
        self._max_width: Optional[float] = None
        self._align_x: Any = None
        self._clip: Optional[bool] = None

    @classmethod
    def new(cls) -> "Column":
        """Create a new empty Column builder."""
        return cls()

    @classmethod
    def with_elements(cls, elements: list[ElementLike]) -> "Column":
        """Create a Column with the given elements."""
        col = cls()
        col._elements = [to_element(e).inner for e in elements]
        return col

    def push(self, element: ElementLike) -> "Column":
        """Add an element to the column."""
        self._elements.append(to_element(element).inner)
        return self

    def push_if(self, condition: bool, element: Callable[[], ElementLike]) -> "Column":
        """Add an element conditionally."""
        if condition:
            self._elements.append(to_element(element()).inner)
        return self

    def extend(self, elements: list[ElementLike]) -> "Column":
        """Add multiple elements."""
        for element in elements:
            self._elements.append(to_element(element).inner)
        return self

    def spacing(self, spacing: float) -> "Column":
        """Set the spacing between elements in pixels."""
        self._spacing = spacing
        return self

    def padding(self, padding: "WitPadding") -> "Column":
        """Set the padding around the column."""
        self._padding = padding
        return self

    def width(self, width: "WitLength") -> "Column":
        """Set the column width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Column":
        """Set the column height."""
        self._height = height
        return self

    def max_width(self, max_width: float) -> "Column":
        """Set the maximum width in pixels."""
        self._max_width = max_width
        return self

    def align_x(self, align: "WitHorizontal") -> "Column":
        """Set horizontal alignment of children."""
        self._align_x = align
        return self

    def clip(self, clip: bool = True) -> "Column":
        """Enable or disable clipping of content."""
        self._clip = clip
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        try:
            from ..generated.wit_world.imports.column import Column as WitColumn
            from ..generated.wit_world.imports.element import column_to_element

            record = WitColumn(
                elements=self._elements,
                spacing=self._spacing,
                padding=self._padding,
                height=self._height,
                width=self._width,
                max_width=self._max_width,
                align_x=self._align_x,
                clip=self._clip,
            )
            return Element(column_to_element(record))
        except ImportError:
            return Element(None)
