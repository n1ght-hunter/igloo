"""ProgressBar widget builder."""

from typing import TYPE_CHECKING

from wit_world.imports.progress_bar import ProgressBar as WitProgressBar

from ..element import Element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength


class ProgressBar(IntoElement):
    """
    Builder for creating ProgressBar widgets.
    A ProgressBar displays progress within a range.

    Example:
        progress_bar = ProgressBar.new(0, 100, state.progress).length(Length.fill())
    """

    def __init__(self, range_start: float, range_end: float, value: float) -> None:
        self._raw = WitProgressBar(range_start, range_end, value)

    @classmethod
    def new(cls, range_start: float, range_end: float, value: float) -> "ProgressBar":
        """
        Create a new ProgressBar builder.

        Args:
            range_start: Start of the value range
            range_end: End of the value range
            value: Current progress value
        """
        return cls(range_start, range_end, value)

    def length(self, length: "WitLength") -> "ProgressBar":
        """Set the length (width for horizontal, height for vertical)."""
        self._raw.length(length)
        return self

    def girth(self, girth: "WitLength") -> "ProgressBar":
        """Set the girth (height for horizontal, width for vertical)."""
        self._raw.girth(girth)
        return self

    def vertical(self, vertical: bool = True) -> "ProgressBar":
        """Make the progress bar vertical."""
        self._raw.vertical(vertical)
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        return Element(WitProgressBar.into_element(self._raw))
