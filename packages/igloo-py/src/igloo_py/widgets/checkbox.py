"""Checkbox widget builder."""

from typing import Any, TypeVar, Callable, Optional, TYPE_CHECKING

from ..element import Element, IntoElement
from ..message import MessageManager, Message

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
            .on_toggle(messages, lambda: {"type": "toggled"})
    """

    def __init__(self, is_checked: bool) -> None:
        self._is_checked = is_checked
        self._label: Optional[str] = None
        self._on_toggle: Optional[int] = None
        self._size: Optional[float] = None
        self._width: Any = None
        self._height: Any = None
        self._spacing: Optional[float] = None
        self._text_size: Optional[float] = None
        self._text_line_height: Any = None
        self._text_wrapping: Any = None
        self._text_shaping: Any = None

    @classmethod
    def new(cls, is_checked: bool) -> "Checkbox":
        """Create a new Checkbox builder with the given checked state."""
        return cls(is_checked)

    def label(self, label: str) -> "Checkbox":
        """Set the checkbox label."""
        self._label = label
        return self

    def on_toggle(
        self, messages: MessageManager[Msg], handler: Callable[[], Msg]
    ) -> "Checkbox":
        """
        Set the message to emit when the checkbox is toggled.
        """
        self._on_toggle = messages.on(handler)
        return self

    def on_toggle_msg(
        self, messages: MessageManager[Msg], handler: Callable[[Message], Msg]
    ) -> "Checkbox":
        """
        Set the message to emit when the checkbox is toggled.
        The Message will have tag 'bool-type' with the new checked state.
        """
        self._on_toggle = messages.register(handler)
        return self

    def size(self, size: float) -> "Checkbox":
        """Set the checkbox size in pixels."""
        self._size = size
        return self

    def width(self, width: "WitLength") -> "Checkbox":
        """Set the width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Checkbox":
        """Set the height."""
        self._height = height
        return self

    def spacing(self, spacing: float) -> "Checkbox":
        """Set the spacing between checkbox and label."""
        self._spacing = spacing
        return self

    def text_size(self, size: float) -> "Checkbox":
        """Set the text size."""
        self._text_size = size
        return self

    def text_line_height(self, line_height: Any) -> "Checkbox":
        """Set the text line height."""
        self._text_line_height = line_height
        return self

    def text_wrapping(self, wrapping: Any) -> "Checkbox":
        """Set the text wrapping."""
        self._text_wrapping = wrapping
        return self

    def text_shaping(self, shaping: Any) -> "Checkbox":
        """Set the text shaping."""
        self._text_shaping = shaping
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        try:
            from ..generated.wit_world.imports.checkbox import Checkbox as WitCheckbox
            from ..generated.wit_world.imports.element import checkbox_to_element

            record = WitCheckbox(
                is_checked=self._is_checked,
                label=self._label,
                on_toggle=self._on_toggle,
                size=self._size,
                width=self._width,
                height=self._height,
                spacing=self._spacing,
                text_size=self._text_size,
                text_line_height=self._text_line_height,
                text_wrapping=self._text_wrapping,
                text_shaping=self._text_shaping,
            )
            return Element(checkbox_to_element(record))
        except ImportError:
            return Element(None)
