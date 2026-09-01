"""Container widget builder."""

from typing import TYPE_CHECKING

from wit_world.imports.container import Container as WitContainer

from ..element import Element, ElementLike, IntoElement, to_element

if TYPE_CHECKING:
    from ..types.alignment import WitHorizontal, WitVertical
    from ..types.length import WitLength
    from ..types.padding import WitPadding


class Container(IntoElement):
    """
    Builder for creating Container widgets.
    A Container is a widget that wraps a single child with optional padding,
    sizing, and alignment.

    Example:
        centered = Container.new(Text.new("Centered")).center(Length.fill())
    """

    def __init__(self, content: ElementLike) -> None:
        self._raw = WitContainer(to_element(content).inner)

    @classmethod
    def new(cls, content: ElementLike) -> "Container":
        """Create a new Container builder with the given content element."""
        return cls(content)

    def padding(self, padding: "WitPadding") -> "Container":
        """Set the padding around the content."""
        self._raw.padding(padding)
        return self

    def width(self, width: "WitLength") -> "Container":
        """Set the container width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Container":
        """Set the container height."""
        self._raw.height(height)
        return self

    def max_width(self, max_width: float) -> "Container":
        """Set the maximum width in pixels."""
        self._raw.max_width(max_width)
        return self

    def max_height(self, max_height: float) -> "Container":
        """Set the maximum height in pixels."""
        self._raw.max_height(max_height)
        return self

    def center_x(self, width: "WitLength") -> "Container":
        """Center the content horizontally with the given width."""
        self._raw.center_x(width)
        return self

    def center_y(self, height: "WitLength") -> "Container":
        """Center the content vertically with the given height."""
        self._raw.center_y(height)
        return self

    def center(self, size: "WitLength") -> "Container":
        """Center the content both horizontally and vertically."""
        self._raw.center(size)
        return self

    def align_left(self, width: "WitLength") -> "Container":
        """Align content to the left with the given width."""
        self._raw.align_left(width)
        return self

    def align_right(self, width: "WitLength") -> "Container":
        """Align content to the right with the given width."""
        self._raw.align_right(width)
        return self

    def align_top(self, height: "WitLength") -> "Container":
        """Align content to the top with the given height."""
        self._raw.align_top(height)
        return self

    def align_bottom(self, height: "WitLength") -> "Container":
        """Align content to the bottom with the given height."""
        self._raw.align_bottom(height)
        return self

    def align_x(self, align: "WitHorizontal") -> "Container":
        """Set horizontal alignment."""
        self._raw.align_x(align)
        return self

    def align_y(self, align: "WitVertical") -> "Container":
        """Set vertical alignment."""
        self._raw.align_y(align)
        return self

    def clip(self, clip: bool = True) -> "Container":
        """Enable or disable clipping of content."""
        self._raw.clip(clip)
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        return Element(WitContainer.into_element(self._raw))
