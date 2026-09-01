# igloo-py

Python bindings for Igloo - builder pattern APIs for WASM component UI widgets.

## Installation

```bash
uv add igloo-py
```

## Usage

```python
from igloo_py import (
    App,
    igloo_app,
    Text,
    Column,
    Button,
    Length,
    Padding,
    ElementLike,
)


@igloo_app
class CounterApp(App[str]):
    def __init__(self) -> None:
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
```

## Building

```bash
# Generate WIT bindings
mise run gen

# Build WASM component
mise run //plugins/python:build
```
