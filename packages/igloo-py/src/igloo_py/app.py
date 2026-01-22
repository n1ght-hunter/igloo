"""Application framework following the Elm architecture."""

from abc import ABC, abstractmethod
from typing import TypeVar, Generic, Callable, Any
from dataclasses import dataclass

from .element import Element, ElementLike, to_element, WitElement
from .message import MessageManager, MessageId, Message

State = TypeVar("State")
Msg = TypeVar("Msg")


class App(ABC, Generic[State, Msg]):
    """Abstract base class defining the Elm architecture interface."""

    @abstractmethod
    def init(self) -> State:
        """Initialize the application state."""
        ...

    @abstractmethod
    def update(self, state: State, msg: Msg) -> State:
        """Update the state based on a message."""
        ...

    @abstractmethod
    def view(self, state: State, messages: MessageManager[Msg]) -> ElementLike:
        """Render the current state as an ElementLike (widget or Element)."""
        ...


@dataclass
class AppExports:
    """The exports required by the WIT interface."""

    update: Callable[[MessageId, Message], None]
    view: Callable[[], WitElement]


def create_app(app: App[State, Msg]) -> AppExports:
    """
    Create an Igloo application from an App definition.
    Returns the exports required by the WIT interface.

    Example:
        class CounterApp:
            def init(self) -> int:
                return 0

            def update(self, state: int, msg: str) -> int:
                if msg == 'increment':
                    return state + 1
                elif msg == 'decrement':
                    return state - 1
                return state

            def view(self, state: int, messages: MessageManager[str]) -> ElementLike:
                return Column.new().push(
                    Text.new(f"Count: {state}")
                ).push(
                    Button.new(Text.new("+")).on_press(messages, lambda: "increment")
                )

        app_exports = create_app(CounterApp())
        update = app_exports.update
        view = app_exports.view
    """
    state_holder: list[Any] = [app.init()]
    messages: MessageManager[Msg] = MessageManager()

    def update(msg_id: MessageId, message: Message) -> None:
        msg = messages.dispatch(msg_id, message)
        if msg is not None:
            state_holder[0] = app.update(state_holder[0], msg)

    def view() -> WitElement:
        messages.clear()
        return to_element(app.view(state_holder[0], messages)).inner

    return AppExports(update=update, view=view)
