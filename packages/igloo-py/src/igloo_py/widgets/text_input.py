"""TextInput widget builder."""

from typing import Any, TypeVar, Callable, Optional, TYPE_CHECKING

from ..element import Element, IntoElement
from ..message import MessageManager, Message
from wit_world.imports.text_input import TextInput as WitTextInput
from wit_world.imports.element import text_input_to_element

if TYPE_CHECKING:
    from ..types.length import WitLength
    from ..types.padding import WitPadding
    from ..types.alignment import WitHorizontal

Msg = TypeVar("Msg")


class TextInput(IntoElement):
    """
    Builder for creating TextInput widgets.
    A TextInput is a field that can be filled with text.

    Example:
        input = TextInput.new("Enter name...", state.name)
            .on_input(messages, lambda msg: {"type": "name_changed", "value": get_string(msg)})
    """

    def __init__(self, placeholder: str, value: str) -> None:
        self._placeholder = placeholder
        self._value = value
        self._secure: Optional[bool] = None
        self._on_input: Optional[int] = None
        self._on_submit: Optional[int] = None
        self._on_paste: Optional[int] = None
        self._width: Any = None
        self._padding: Any = None
        self._size: Optional[float] = None
        self._line_height: Any = None
        self._align_x: Any = None

    @classmethod
    def new(cls, placeholder: str, value: str) -> "TextInput":
        """Create a new TextInput builder with placeholder and current value."""
        return cls(placeholder, value)

    def secure(self, secure: bool = True) -> "TextInput":
        """Make the text input secure (e.g., for passwords)."""
        self._secure = secure
        return self

    def on_input(
        self, messages: MessageManager[Msg], handler: Callable[[Message], Msg]
    ) -> "TextInput":
        """
        Set the message to emit when the text changes.
        The Message will have tag 'string-type' with the new text value.
        """
        self._on_input = messages.register(handler)
        return self

    def on_submit(self, messages: MessageManager[Msg], handler: Callable[[], Msg]) -> "TextInput":
        """Set the message to emit when the user submits (e.g., presses Enter)."""
        self._on_submit = messages.on(handler)
        return self

    def on_paste(
        self, messages: MessageManager[Msg], handler: Callable[[Message], Msg]
    ) -> "TextInput":
        """
        Set the message to emit when text is pasted.
        The Message will have tag 'string-type' with the pasted text.
        """
        self._on_paste = messages.register(handler)
        return self

    def width(self, width: "WitLength") -> "TextInput":
        """Set the width."""
        self._width = width
        return self

    def padding(self, padding: "WitPadding") -> "TextInput":
        """Set the padding."""
        self._padding = padding
        return self

    def size(self, size: float) -> "TextInput":
        """Set the text size in pixels."""
        self._size = size
        return self

    def line_height(self, line_height: Any) -> "TextInput":
        """Set the line height."""
        self._line_height = line_height
        return self

    def align_x(self, align: "WitHorizontal") -> "TextInput":
        """Set horizontal alignment of text."""
        self._align_x = align
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        record = WitTextInput(
            placeholder=self._placeholder,
            value=self._value,
            secure=self._secure,
            on_input=self._on_input,
            on_submit=self._on_submit,
            on_paste=self._on_paste,
            width=self._width,
            padding=self._padding,
            size=self._size,
            line_height=self._line_height,
            align_x=self._align_x,
        )
        return Element(text_input_to_element(record))
