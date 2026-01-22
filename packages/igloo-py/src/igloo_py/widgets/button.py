"""Button widget builder."""

from typing import Any, TypeVar, Callable, Optional, TYPE_CHECKING

from ..element import Element, ElementLike, to_element, IntoElement
from ..message import MessageManager, Message
from wit_world.imports.button import Button as WitButton
from wit_world.imports.element import button_to_element

if TYPE_CHECKING:
    from ..types.length import WitLength
    from ..types.padding import WitPadding

Msg = TypeVar("Msg")


class Button(IntoElement):
    """
    Builder for creating Button widgets.

    Example:
        # Widgets can be passed directly - no into_element() needed
        button = Button.new(Text.new("Click me"))
            .on_press(messages, lambda: {"type": "clicked"})
            .padding(Padding.all(10))

        # Use in a Column directly
        Column.new().push(button)
    """

    def __init__(self, content: ElementLike) -> None:
        self._content = to_element(content).inner
        self._width: Any = None
        self._height: Any = None
        self._padding: Any = None
        self._on_press: Optional[int] = None
        self._clip: Optional[bool] = None

    @classmethod
    def new(cls, content: ElementLike) -> "Button":
        """Create a new Button builder with the given content."""
        return cls(content)

    def on_press_msg(
        self, messages: MessageManager[Msg], handler: Callable[[Message], Msg]
    ) -> "Button":
        """
        Set the message to emit when the button is pressed.
        The handler receives the Message and returns the app message.
        """
        self._on_press = messages.register(handler)
        return self

    def on_press(self, messages: MessageManager[Msg], msg: Callable[[], Msg]) -> "Button":
        """
        Set the message to emit when the button is pressed (simple version).
        The handler returns the message directly.
        """
        self._on_press = messages.on(msg)
        return self

    def width(self, width: "WitLength") -> "Button":
        """Set the button width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Button":
        """Set the button height."""
        self._height = height
        return self

    def padding(self, padding: "WitPadding") -> "Button":
        """Set the button padding."""
        self._padding = padding
        return self

    def clip(self, clip: bool = True) -> "Button":
        """Enable or disable clipping of content."""
        self._clip = clip
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        record = WitButton(
            content=self._content,
            width=self._width,
            height=self._height,
            padding=self._padding,
            on_press=self._on_press,
            clip=self._clip,
        )
        return Element(button_to_element(record))
