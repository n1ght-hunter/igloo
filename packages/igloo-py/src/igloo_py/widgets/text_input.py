"""TextInput widget builder."""

from collections.abc import Callable
from typing import TYPE_CHECKING, Any, TypeVar

from wit_world.imports.text_input import TextInput as WitTextInput

from ..callbacks import push_fixed, push_string
from ..element import Element, IntoElement

if TYPE_CHECKING:
    from ..types.alignment import WitHorizontal
    from ..types.length import WitLength
    from ..types.padding import WitPadding

Msg = TypeVar("Msg")


class TextInput(IntoElement):
    """
    Builder for creating TextInput widgets.
    A TextInput is a field that can be filled with text.

    Example:
        input = TextInput.new("Enter name...", state.name)
            .on_input(lambda value: {"type": "name_changed", "value": value})
    """

    def __init__(self, placeholder: str, value: str) -> None:
        self._raw = WitTextInput(placeholder, value)

    @classmethod
    def new(cls, placeholder: str, value: str) -> "TextInput":
        """Create a new TextInput builder with placeholder and current value."""
        return cls(placeholder, value)

    def secure(self, secure: bool = True) -> "TextInput":
        """Make the text input secure (e.g., for passwords)."""
        self._raw.secure(secure)
        return self

    def on_input(self, mapper: Callable[[str], Msg]) -> "TextInput":
        """
        Set the message to emit when the text changes.
        The mapper receives the new text value.
        """
        self._raw.on_input(push_string(mapper))
        return self

    def on_submit(self, message: Callable[[], Msg]) -> "TextInput":
        """Set the message to emit when the user submits (e.g., presses Enter)."""
        self._raw.on_submit(push_fixed(message()))
        return self

    def on_paste(self, mapper: Callable[[str], Msg]) -> "TextInput":
        """
        Set the message to emit when text is pasted.
        The mapper receives the pasted text.
        """
        self._raw.on_paste(push_string(mapper))
        return self

    def width(self, width: "WitLength") -> "TextInput":
        """Set the width."""
        self._raw.width(width)
        return self

    def padding(self, padding: "WitPadding") -> "TextInput":
        """Set the padding."""
        self._raw.padding(padding)
        return self

    def size(self, size: float) -> "TextInput":
        """Set the text size in pixels."""
        self._raw.size(size)
        return self

    def line_height(self, line_height: Any) -> "TextInput":
        """Set the line height."""
        self._raw.line_height(line_height)
        return self

    def align_x(self, align: "WitHorizontal") -> "TextInput":
        """Set horizontal alignment of text."""
        self._raw.align_x(align)
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        return Element(WitTextInput.into_element(self._raw))
