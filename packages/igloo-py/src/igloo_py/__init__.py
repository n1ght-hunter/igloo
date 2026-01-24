"""
Igloo Python SDK - Builder pattern APIs for WASM component UI widgets.

This SDK allows you to build Igloo plugins in Python following the Elm architecture.

Example:
    from igloo_py import App, igloo_app, Text, Column, Button, MessageManager, ElementLike

    @igloo_app
    class CounterApp(App[str]):
        def __init__(self):
            self.count = 0

        def update(self, msg: str) -> None:
            if msg == "increment":
                self.count += 1
            elif msg == "decrement":
                self.count -= 1

        def view(self, messages: MessageManager[str]) -> ElementLike:
            return Column.new().spacing(10).push(
                Text.new(f"Count: {self.count}").size(24)
            ).push(
                Button.new(Text.new("+")).on_press(messages, lambda: "increment")
            ).push(
                Button.new(Text.new("-")).on_press(messages, lambda: "decrement")
            )

    # That's it! WitWorld and Message are automatically exported.
"""

# Core
from .element import Element, to_element, IntoElement, ElementLike
from .message import MessageManager, Message, MessageId
from .app import create_app, igloo_app, App, AppExports

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
    "igloo_app",
    "App",
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
