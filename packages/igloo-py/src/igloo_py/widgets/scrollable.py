"""Scrollable widget builder."""

from typing import Any, TypeVar, Callable, Optional, TYPE_CHECKING

from ..element import Element, ElementLike, to_element, IntoElement
from ..message import MessageManager, Message

if TYPE_CHECKING:
    from ..types.length import WitLength

Msg = TypeVar("Msg")


class Scrollable(IntoElement):
    """
    Builder for creating Scrollable widgets.
    A Scrollable wraps content that can be scrolled.

    Example:
        scrollable = Scrollable.new(
            Column.new()
                .push(Text.new("Item 1"))
                .push(Text.new("Item 2"))
                # ... more items
        ).height(Length.fixed(300))
    """

    def __init__(self, content: ElementLike) -> None:
        self._content = to_element(content).inner
        self._width: Any = None
        self._height: Any = None
        self._on_scroll: Optional[int] = None
        self._direction: Any = None

    @classmethod
    def new(cls, content: ElementLike) -> "Scrollable":
        """Create a new Scrollable builder with the given content."""
        return cls(content)

    def width(self, width: "WitLength") -> "Scrollable":
        """Set the width."""
        self._width = width
        return self

    def height(self, height: "WitLength") -> "Scrollable":
        """Set the height."""
        self._height = height
        return self

    def on_scroll(
        self, messages: MessageManager[Msg], handler: Callable[[Message], Msg]
    ) -> "Scrollable":
        """
        Set the message to emit when scrolling occurs.
        The Message will have tag 'viewport' with scroll position info.
        """
        self._on_scroll = messages.register(handler)
        return self

    def direction(self, direction: Any) -> "Scrollable":
        """Set the scroll direction and scrollbar configuration."""
        self._direction = direction
        return self

    def vertical(self, scrollbar: dict[str, Any] | None = None) -> "Scrollable":
        """Configure for vertical scrolling only."""
        try:
            from ..generated.wit_world.imports.scrollable import Direction_Vertical, Scrollbar

            sb = Scrollbar(
                width=scrollbar.get("width") if scrollbar else None,
                margin=scrollbar.get("margin") if scrollbar else None,
                scroller_width=scrollbar.get("scroller_width") if scrollbar else None,
                alignment=scrollbar.get("alignment") if scrollbar else None,
                spacing=scrollbar.get("spacing") if scrollbar else None,
            )
            self._direction = Direction_Vertical(sb)
        except ImportError:
            self._direction = {"tag": "vertical", "val": scrollbar or {}}
        return self

    def horizontal(self, scrollbar: dict[str, Any] | None = None) -> "Scrollable":
        """Configure for horizontal scrolling only."""
        try:
            from ..generated.wit_world.imports.scrollable import Direction_Horizontal, Scrollbar

            sb = Scrollbar(
                width=scrollbar.get("width") if scrollbar else None,
                margin=scrollbar.get("margin") if scrollbar else None,
                scroller_width=scrollbar.get("scroller_width") if scrollbar else None,
                alignment=scrollbar.get("alignment") if scrollbar else None,
                spacing=scrollbar.get("spacing") if scrollbar else None,
            )
            self._direction = Direction_Horizontal(sb)
        except ImportError:
            self._direction = {"tag": "horizontal", "val": scrollbar or {}}
        return self

    def both(
        self,
        vertical_scrollbar: dict[str, Any] | None = None,
        horizontal_scrollbar: dict[str, Any] | None = None,
    ) -> "Scrollable":
        """Configure for both vertical and horizontal scrolling."""
        try:
            from ..generated.wit_world.imports.scrollable import Direction_Both, Scrollbar

            vsb = Scrollbar(
                width=vertical_scrollbar.get("width") if vertical_scrollbar else None,
                margin=vertical_scrollbar.get("margin") if vertical_scrollbar else None,
                scroller_width=vertical_scrollbar.get("scroller_width") if vertical_scrollbar else None,
                alignment=vertical_scrollbar.get("alignment") if vertical_scrollbar else None,
                spacing=vertical_scrollbar.get("spacing") if vertical_scrollbar else None,
            )
            hsb = Scrollbar(
                width=horizontal_scrollbar.get("width") if horizontal_scrollbar else None,
                margin=horizontal_scrollbar.get("margin") if horizontal_scrollbar else None,
                scroller_width=horizontal_scrollbar.get("scroller_width") if horizontal_scrollbar else None,
                alignment=horizontal_scrollbar.get("alignment") if horizontal_scrollbar else None,
                spacing=horizontal_scrollbar.get("spacing") if horizontal_scrollbar else None,
            )
            self._direction = Direction_Both((vsb, hsb))
        except ImportError:
            self._direction = {
                "tag": "both",
                "val": [vertical_scrollbar or {}, horizontal_scrollbar or {}],
            }
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        try:
            from ..generated.wit_world.imports.scrollable import Scrollable as WitScrollable
            from ..generated.wit_world.imports.element import scrollable_to_element

            record = WitScrollable(
                content=self._content,
                width=self._width,
                height=self._height,
                on_scroll=self._on_scroll,
                direction=self._direction,
            )
            return Element(scrollable_to_element(record))
        except ImportError:
            return Element(None)
