"""ProgressBar widget builder."""

from typing import Any, Optional, TYPE_CHECKING

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
        self._range_start = range_start
        self._range_end = range_end
        self._value = value
        self._length: Any = None
        self._girth: Any = None
        self._vertical: Optional[bool] = None

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
        self._length = length
        return self

    def girth(self, girth: "WitLength") -> "ProgressBar":
        """Set the girth (height for horizontal, width for vertical)."""
        self._girth = girth
        return self

    def vertical(self, vertical: bool = True) -> "ProgressBar":
        """Make the progress bar vertical."""
        self._vertical = vertical
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        try:
            from ..generated.wit_world.imports.progress_bar import ProgressBar as WitProgressBar
            from ..generated.wit_world.imports.element import progress_bar_to_element

            record = WitProgressBar(
                range_start=self._range_start,
                range_end=self._range_end,
                value=self._value,
                length=self._length,
                girth=self._girth,
                vertical=self._vertical,
            )
            return Element(progress_bar_to_element(record))
        except ImportError:
            return Element(None)
