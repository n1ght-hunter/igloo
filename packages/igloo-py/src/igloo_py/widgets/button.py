"""Button widget builder."""

from collections.abc import Callable
from typing import TYPE_CHECKING, TypeVar

from wit_world.imports.button import Button as WitButton

from ..callbacks import push_fixed
from ..element import Element, ElementLike, IntoElement, to_element

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
            .on_press(lambda: {"type": "clicked"})
            .padding(Padding.all(10))

        # Use in a Column directly
        Column.new().push(button)
    """

    def __init__(self, content: ElementLike) -> None:
        self._raw = WitButton(to_element(content).inner)

    @classmethod
    def new(cls, content: ElementLike) -> "Button":
        """Create a new Button builder with the given content."""
        return cls(content)

    def on_press(self, message: Callable[[], Msg]) -> "Button":
        """Set the message to emit when the button is pressed."""
        self._raw.on_press(push_fixed(message()))
        return self

    def width(self, width: "WitLength") -> "Button":
        """Set the button width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Button":
        """Set the button height."""
        self._raw.height(height)
        return self

    def padding(self, padding: "WitPadding") -> "Button":
        """Set the button padding."""
        self._raw.padding(padding)
        return self

    def clip(self, clip: bool = True) -> "Button":
        """Enable or disable clipping of content."""
        self._raw.clip(clip)
        return self

    def into_element(self) -> Element:
        """Convert to Element (implements IntoElement)."""
        return Element(WitButton.into_element(self._raw))
