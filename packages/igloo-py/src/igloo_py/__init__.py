"""
Igloo Python SDK - Builder pattern APIs for WASM component UI widgets.

This SDK allows you to build Igloo plugins in Python following the Elm architecture.

Example:
    from igloo_py import (
        create_app,
        Text,
        Column,
        Button,
        Length,
        Padding,
        MessageManager,
        ElementLike,
    )

    class CounterApp:
        def init(self) -> int:
            return 0

        def update(self, state: int, msg: str) -> int:
            if msg == "increment":
                return state + 1
            elif msg == "decrement":
                return state - 1
            return state

        def view(self, state: int, messages: MessageManager[str]) -> ElementLike:
            return Column.new().spacing(10).push(
                Text.new(f"Count: {state}").size(24)
            ).push(
                Button.new(Text.new("+")).on_press(messages, lambda: "increment")
            ).push(
                Button.new(Text.new("-")).on_press(messages, lambda: "decrement")
            )

    app_exports = create_app(CounterApp())

    # Export for WIT interface
    update = app_exports.update
    view = app_exports.view
"""

# Core
from .element import Element, to_element, IntoElement, ElementLike
from .message import MessageManager, Message, MessageId
from .app import create_app, AppProtocol, AppExports

# Types
from .types import Length, Padding, Color, Horizontal, Vertical
from .types.color import ColorHelper

# Widgets
from .widgets import (
    Text,
    Button,
    Column,
    Row,
    Container,
    TextInput,
    Checkbox,
    ProgressBar,
    Rule,
    Space,
    Scrollable,
)

__all__ = [
    # Core
    "Element",
    "to_element",
    "IntoElement",
    "ElementLike",
    "MessageManager",
    "Message",
    "MessageId",
    "create_app",
    "AppProtocol",
    "AppExports",
    # Types
    "Length",
    "Padding",
    "Color",
    "ColorHelper",
    "Horizontal",
    "Vertical",
    # Widgets
    "Text",
    "Button",
    "Column",
    "Row",
    "Container",
    "TextInput",
    "Checkbox",
    "ProgressBar",
    "Rule",
    "Space",
    "Scrollable",
]

__version__ = "0.1.0"
