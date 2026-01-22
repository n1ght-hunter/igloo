"""Row layout widget builder."""

from typing import Any, Callable, Optional, List, TYPE_CHECKING

from ..element import Element, ElementLike, to_element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength
    from ..types.padding import WitPadding
    from ..types.alignment import WitVertical


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
        self._elements: List[Any] = []
        self._spacing: Optional[float] = None
        self._padding: Any = None
        self._width: Any = None
        self._height: Any = None
        self._align_y: Any = None
        self._clip: Optional[bool] = None
        self._wrap: Optional[bool] = None

    @classmethod
    def new(cls) -> "Row":
        """Create a new empty Row builder."""
        return cls()

    @classmethod
    def with_elements(cls, elements: list[ElementLike]) -> "Row":
        """Create a Row with the given elements."""
        row = cls()
        row._elements = [to_element(e).inner for e in elements]
        return row

    def push(self, element: ElementLike) -> "Row":
        """Add an element to the row."""
        self._elements.append(to_element(element).inner)
        return self

    def push_if(self, condition: bool, element: Callable[[], ElementLike]) -> "Row":
        """Add an element conditionally."""
        if condition:
            self._elements.append(to_element(element()).inner)
        return self

    def extend(self, elements: list[ElementLike]) -> "Row":
        """Add multiple elements."""
        for element in elements:
            self._elements.append(to_element(element).inner)
        return self

    def spacing(self, spacing: float) -> "Row":
        """Set the spacing between elements in pixels."""
        self._spacing = spacing
        return self

    def padding(self, padding: "WitPadding") -> "Row":
        """Set the padding around the row."""
        self._padding = padding
        return self

    def width(self, width: "WitLength") -> "Row":
        """Set the row width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Row":
        """Set the row height."""
        self._height = height
        return self

    def align_y(self, align: "WitVertical") -> "Row":
        """Set vertical alignment of children."""
        self._align_y = align
        return self

    def clip(self, clip: bool = True) -> "Row":
        """Enable or disable clipping of content."""
        self._clip = clip
        return self

    def wrap(self, wrap: bool = True) -> "Row":
        """Enable or disable wrapping of elements to the next line."""
        self._wrap = wrap
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        try:
            from ..generated.wit_world.imports.row import Row as WitRow
            from ..generated.wit_world.imports.element import row_to_element

            record = WitRow(
                elements=self._elements,
                spacing=self._spacing,
                padding=self._padding,
                width=self._width,
                height=self._height,
                align_y=self._align_y,
                clip=self._clip,
                wrap=self._wrap,
            )
            return Element(row_to_element(record))
        except ImportError:
            return Element(None)
