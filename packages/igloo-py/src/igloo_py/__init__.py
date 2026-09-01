"""
Igloo Python SDK - Builder pattern APIs for WASM component UI widgets.

This SDK allows you to build Igloo plugins in Python following the Elm architecture.

Example:
    from igloo_py import App, igloo_app, Text, Column, Button, ElementLike

    @igloo_app
    class CounterApp(App[str]):
        def __init__(self):
            self.count = 0

        def update(self, msg: str) -> None:
            if msg == "increment":
                self.count += 1
            elif msg == "decrement":
                self.count -= 1

        def view(self) -> ElementLike:
            return Column.new().spacing(10).push(
                Text.new(f"Count: {self.count}").size(24)
            ).push(
                Button.new(Text.new("+")).on_press(lambda: "increment")
            ).push(
                Button.new(Text.new("-")).on_press(lambda: "decrement")
            )

    # AppInstance and Application are automatically exported.
"""

# Core
from .app import App, create_application, igloo_app
from .callbacks import (
    Frame,
    push_bool,
    push_f32,
    push_f64,
    push_fixed,
    push_string,
    push_u64,
    push_viewport,
    with_frame,
)
from .element import Element, ElementLike, IntoElement, to_element

# Types
from .types import Color, Horizontal, Length, Padding, Vertical
from .types.color import ColorHelper

# Widgets
from .widgets import (
    Button,
    Checkbox,
    Column,
    Container,
    ProgressBar,
    Row,
    Rule,
    Scrollable,
    Space,
    Text,
    TextInput,
)

__all__ = [
    # Core
    "Element",
    "to_element",
    "IntoElement",
    "ElementLike",
    "Frame",
    "with_frame",
    "push_fixed",
    "push_bool",
    "push_f32",
    "push_f64",
    "push_u64",
    "push_string",
    "push_viewport",
    "create_application",
    "igloo_app",
    "App",
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
