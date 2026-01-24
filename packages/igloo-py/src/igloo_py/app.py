"""Application framework following the Elm architecture."""

from abc import ABC, abstractmethod
from typing import TypeVar, Generic, Callable
from dataclasses import dataclass
import sys

from .element import ElementLike, to_element, WitElement
from .message import MessageManager, MessageId, Message

Msg = TypeVar("Msg")
T = TypeVar("T", bound="App")


class App(ABC, Generic[Msg]):
    """
    Abstract base class for Igloo applications.

    State is stored on `self` - just use instance attributes.
    Override `update` to handle messages and mutate state.
    Override `view` to render the current state.
    """

    @abstractmethod
    def update(self, msg: Msg) -> None:
        """Update state based on a message. Mutate self directly."""
        ...

    @abstractmethod
    def view(self, messages: MessageManager[Msg]) -> ElementLike:
        """Render the current state as an ElementLike (widget or Element)."""
        ...


def igloo_app(cls: type[T]) -> type[T]:
    """
    Decorator that registers an App class and exports WitWorld and Message.

    Example:
        from igloo_py import App, igloo_app, Text, Column, Button, MessageManager, ElementLike

        @igloo_app
        class CounterApp(App[str]):
            def __init__(self):
                self.count = 0

            def update(self, msg: str) -> None:
                if msg == 'increment':
                    self.count += 1
                elif msg == 'decrement':
                    self.count -= 1

            def view(self, messages: MessageManager[str]) -> ElementLike:
                return Column.new().push(
                    Text.new(f"Count: {self.count}")
                ).push(
                    Button.new(Text.new("+")).on_press(messages, lambda: "increment")
                )

        # That's it! WitWorld and Message are automatically exported.
    """
    app = cls()
    msg_manager: MessageManager = MessageManager()

    def _update(msg_id: MessageId, message: Message) -> None:
        msg = msg_manager.dispatch(msg_id, message)
        if msg is not None:
            app.update(msg)

    def _view() -> WitElement:
        msg_manager.clear()
        return to_element(app.view(msg_manager)).inner

    class WitWorld:
        """WIT world implementation for componentize-py."""

        def update(self, message_id: int, message: Message) -> None:
            _update(message_id, message)

        def view(self) -> WitElement:
            return _view()

    class MessageExport:
        """WIT message export implementation."""

        def clone_message(self, message: int) -> int:
            return message

    # Inject WitWorld and Message into the caller's module
    frame = sys._getframe(1)
    frame.f_globals["WitWorld"] = WitWorld
    frame.f_globals["Message"] = MessageExport

    return cls


@dataclass
class AppExports:
    """The exports required by the WIT interface."""

    WitWorld: type
    Message: type
    update: Callable[[MessageId, Message], None]
    view: Callable[[], WitElement]


def create_app(app: App[Msg]) -> AppExports:
    """
    Create an Igloo application from an App instance.
    Returns the WitWorld and Message classes required by componentize-py.

    For simpler usage, consider using the @igloo_app decorator instead.
    """
    msg_manager: MessageManager[Msg] = MessageManager()

    def _update(msg_id: MessageId, message: Message) -> None:
        msg = msg_manager.dispatch(msg_id, message)
        if msg is not None:
            app.update(msg)

    def _view() -> WitElement:
        msg_manager.clear()
        return to_element(app.view(msg_manager)).inner

    class WitWorld:
        """WIT world implementation for componentize-py."""

        def update(self, message_id: int, message: Message) -> None:
            _update(message_id, message)

        def view(self) -> WitElement:
            return _view()

    class MessageExport:
        """WIT message export implementation."""

        def clone_message(self, message: int) -> int:
            return message

    return AppExports(
        WitWorld=WitWorld,
        Message=MessageExport,
        update=_update,
        view=_view,
    )
