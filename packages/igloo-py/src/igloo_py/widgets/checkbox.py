"""Checkbox widget builder."""

from collections.abc import Callable
from typing import TYPE_CHECKING, Any, TypeVar

from wit_world.imports.checkbox import Checkbox as WitCheckbox

from ..callbacks import push_bool
from ..element import Element, IntoElement

if TYPE_CHECKING:
    from ..types.length import WitLength

Msg = TypeVar("Msg")


class Checkbox(IntoElement):
    """
    Builder for creating Checkbox widgets.
    A Checkbox is a box that can be checked.

    Example:
        checkbox = Checkbox.new(state.is_enabled)
            .label("Enable feature")
            .on_toggle(lambda checked: {"type": "toggled", "checked": checked})
    """

    def __init__(self, is_checked: bool) -> None:
        self._raw = WitCheckbox(is_checked)

    @classmethod
    def new(cls, is_checked: bool) -> "Checkbox":
        """Create a new Checkbox builder with the given checked state."""
        return cls(is_checked)

    def label(self, label: str) -> "Checkbox":
        """Set the checkbox label."""
        self._raw.label(label)
        return self

    def on_toggle(self, mapper: Callable[[bool], Msg]) -> "Checkbox":
        """Set the message to emit when the checked state changes."""
        self._raw.on_toggle(push_bool(mapper))
        return self

    def size(self, size: float) -> "Checkbox":
        """Set the checkbox size in pixels."""
        self._raw.size(size)
        return self

    def width(self, width: "WitLength") -> "Checkbox":
        """Set the width."""
        self._raw.width(width)
        return self

    def spacing(self, spacing: float) -> "Checkbox":
        """Set the spacing between checkbox and label."""
        self._raw.spacing(spacing)
        return self

    def text_size(self, size: float) -> "Checkbox":
        """Set the text size."""
        self._raw.text_size(size)
        return self

    def text_line_height(self, line_height: Any) -> "Checkbox":
        """Set the text line height."""
        self._raw.text_line_height(line_height)
        return self

    def text_wrapping(self, wrapping: Any) -> "Checkbox":
        """Set the text wrapping."""
        self._raw.text_wrapping(wrapping)
        return self

    def text_shaping(self, shaping: Any) -> "Checkbox":
        """Set the text shaping."""
        self._raw.text_shaping(shaping)
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        return Element(WitCheckbox.into_element(self._raw))
