# igloo-py

Python bindings for Igloo - builder pattern APIs for WASM component UI widgets.

## Installation

```bash
uv add igloo-py
```

## Usage

```python
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
```

## Building

```bash
# Generate WIT bindings
uv run componentize-py --wit-path ../../wit bindings src/igloo_py/generated

# Build WASM component
uv run componentize-py --wit-path ../../wit --world app --output app.wasm src/app
```
