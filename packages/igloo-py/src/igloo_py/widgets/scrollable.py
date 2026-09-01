"""Scrollable widget builder."""

from collections.abc import Callable
from typing import TYPE_CHECKING, Any, TypeVar

from wit_world.imports.message_types import Viewport
from wit_world.imports.scrollable import (
    Direction_Both,
    Direction_Horizontal,
    Direction_Vertical,
    Scrollbar,
)
from wit_world.imports.scrollable import (
    Scrollable as WitScrollable,
)

from ..callbacks import push_viewport
from ..element import Element, ElementLike, IntoElement, to_element

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
        self._raw = WitScrollable(to_element(content).inner)

    @classmethod
    def new(cls, content: ElementLike) -> "Scrollable":
        """Create a new Scrollable builder with the given content."""
        return cls(content)

    def width(self, width: "WitLength") -> "Scrollable":
        """Set the width."""
        self._raw.width(width)
        return self

    def height(self, height: "WitLength") -> "Scrollable":
        """Set the height."""
        self._raw.height(height)
        return self

    def on_scroll(self, mapper: Callable[[Viewport], Msg]) -> "Scrollable":
        """
        Set the message to emit when scrolling occurs.
        The Message will have tag 'viewport' with scroll position info.
        """
        self._raw.on_scroll(push_viewport(mapper))
        return self

    def direction(self, direction: Any) -> "Scrollable":
        """Set the scroll direction and scrollbar configuration."""
        self._raw.direction(direction)
        return self

    def vertical(self, scrollbar: dict[str, Any] | None = None) -> "Scrollable":
        """Configure for vertical scrolling only."""
        sb = Scrollbar(
            width=scrollbar.get("width") if scrollbar else None,
            margin=scrollbar.get("margin") if scrollbar else None,
            scroller_width=scrollbar.get("scroller_width") if scrollbar else None,
            anchor=scrollbar.get("anchor") if scrollbar else None,
            spacing=scrollbar.get("spacing") if scrollbar else None,
        )
        self._raw.direction(Direction_Vertical(sb))
        return self

    def horizontal(self, scrollbar: dict[str, Any] | None = None) -> "Scrollable":
        """Configure for horizontal scrolling only."""
        sb = Scrollbar(
            width=scrollbar.get("width") if scrollbar else None,
            margin=scrollbar.get("margin") if scrollbar else None,
            scroller_width=scrollbar.get("scroller_width") if scrollbar else None,
            anchor=scrollbar.get("anchor") if scrollbar else None,
            spacing=scrollbar.get("spacing") if scrollbar else None,
        )
        self._raw.direction(Direction_Horizontal(sb))
        return self

    def both(
        self,
        vertical_scrollbar: dict[str, Any] | None = None,
        horizontal_scrollbar: dict[str, Any] | None = None,
    ) -> "Scrollable":
        """Configure for both vertical and horizontal scrolling."""
        vsb = Scrollbar(
            width=vertical_scrollbar.get("width") if vertical_scrollbar else None,
            margin=vertical_scrollbar.get("margin") if vertical_scrollbar else None,
            scroller_width=vertical_scrollbar.get("scroller_width") if vertical_scrollbar else None,
            anchor=vertical_scrollbar.get("anchor") if vertical_scrollbar else None,
            spacing=vertical_scrollbar.get("spacing") if vertical_scrollbar else None,
        )
        hsb = Scrollbar(
            width=horizontal_scrollbar.get("width") if horizontal_scrollbar else None,
            margin=horizontal_scrollbar.get("margin") if horizontal_scrollbar else None,
            scroller_width=horizontal_scrollbar.get("scroller_width")
            if horizontal_scrollbar
            else None,
            anchor=horizontal_scrollbar.get("anchor") if horizontal_scrollbar else None,
            spacing=horizontal_scrollbar.get("spacing") if horizontal_scrollbar else None,
        )
        self._raw.direction(Direction_Both((vsb, hsb)))
        return self

    def into_element(self) -> Element:
        """Convert to Element."""
        return Element(WitScrollable.into_element(self._raw))
