"""Space widget builder."""

from typing import TYPE_CHECKING

from wit_world.imports.space import Space as WitSpace

from ..element import Element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength


class Space(IntoElement):
    """
    Builder for creating Space widgets.
    A Space is an amount of empty space.

    Example:
        # Fixed size space
        fixed_space = Space.new().width(Length.fixed(20)).height(Length.fixed(10))

        # Flexible space that fills remaining width
        flex_space = Space.new().width(Length.fill())
    """

    def __init__(self) -> None:
        self._raw = WitSpace()

    @classmethod
    def new(cls) -> "Space":
        """Create a new Space builder."""
        return cls()

    @classmethod
    def with_size(cls, width: "WitLength", height: "WitLength") -> "Space":
        """Create a space with the given width and height."""
        space = cls()
        space.width(width)
        space.height(height)
        return space

    def width(self, width: "WitLength") -> "Space":
        """Set the width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Space":
        """Set the height."""
        self._raw.height(height)
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        return Element(WitSpace.into_element(self._raw))
