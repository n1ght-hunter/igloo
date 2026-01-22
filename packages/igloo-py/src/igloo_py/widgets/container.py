"""Container widget builder."""

from typing import Any, Optional, TYPE_CHECKING

from ..element import Element, ElementLike, to_element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength
    from ..types.padding import WitPadding
    from ..types.alignment import WitHorizontal, WitVertical


class Container(IntoElement):
    """
    Builder for creating Container widgets.
    A Container is a widget that wraps a single child with optional padding,
    sizing, and alignment.

    Example:
        centered = Container.new(Text.new("Centered")).center(Length.fill())
    """

    def __init__(self, content: ElementLike) -> None:
        self._content = to_element(content).inner
        self._padding: Any = None
        self._width: Any = None
        self._height: Any = None
        self._max_width: Optional[float] = None
        self._max_height: Optional[float] = None
        self._center_x: Any = None
        self._center_y: Any = None
        self._center: Any = None
        self._align_left: Any = None
        self._align_right: Any = None
        self._align_top: Any = None
        self._align_bottom: Any = None
        self._align_x: Any = None
        self._align_y: Any = None
        self._clip: Optional[bool] = None

    @classmethod
    def new(cls, content: ElementLike) -> "Container":
        """Create a new Container builder with the given content element."""
        return cls(content)

    def padding(self, padding: "WitPadding") -> "Container":
        """Set the padding around the content."""
        self._padding = padding
        return self

    def width(self, width: "WitLength") -> "Container":
        """Set the container width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Container":
        """Set the container height."""
        self._height = height
        return self

    def max_width(self, max_width: float) -> "Container":
        """Set the maximum width in pixels."""
        self._max_width = max_width
        return self

    def max_height(self, max_height: float) -> "Container":
        """Set the maximum height in pixels."""
        self._max_height = max_height
        return self

    def center_x(self, width: "WitLength") -> "Container":
        """Center the content horizontally with the given width."""
        self._center_x = width
        return self

    def center_y(self, height: "WitLength") -> "Container":
        """Center the content vertically with the given height."""
        self._center_y = height
        return self

    def center(self, size: "WitLength") -> "Container":
        """Center the content both horizontally and vertically."""
        self._center = size
        return self

    def align_left(self, width: "WitLength") -> "Container":
        """Align content to the left with the given width."""
        self._align_left = width
        return self

    def align_right(self, width: "WitLength") -> "Container":
        """Align content to the right with the given width."""
        self._align_right = width
        return self

    def align_top(self, height: "WitLength") -> "Container":
        """Align content to the top with the given height."""
        self._align_top = height
        return self

    def align_bottom(self, height: "WitLength") -> "Container":
        """Align content to the bottom with the given height."""
        self._align_bottom = height
        return self

    def align_x(self, align: "WitHorizontal") -> "Container":
        """Set horizontal alignment."""
        self._align_x = align
        return self

    def align_y(self, align: "WitVertical") -> "Container":
        """Set vertical alignment."""
        self._align_y = align
        return self

    def clip(self, clip: bool = True) -> "Container":
        """Enable or disable clipping of content."""
        self._clip = clip
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        try:
            from ..generated.wit_world.imports.container import Container as WitContainer
            from ..generated.wit_world.imports.element import container_to_element

            record = WitContainer(
                content=self._content,
                padding=self._padding,
                width=self._width,
                height=self._height,
                max_width=self._max_width,
                max_height=self._max_height,
                center_x=self._center_x,
                center_y=self._center_y,
                center=self._center,
                align_left=self._align_left,
                align_right=self._align_right,
                align_top=self._align_top,
                align_bottom=self._align_bottom,
                align_x=self._align_x,
                align_y=self._align_y,
                clip=self._clip,
            )
            return Element(container_to_element(record))
        except ImportError:
            return Element(None)
