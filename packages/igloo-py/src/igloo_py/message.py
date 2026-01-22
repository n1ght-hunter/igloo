"""Message manager for the Elm-architecture pattern."""

from typing import TypeVar, Generic, Callable, Optional

# These types will be available after generating bindings
# For now, we use simple type aliases
MessageId = int
Message = dict

Msg = TypeVar("Msg")


class MessageManager(Generic[Msg]):
    """
    Manages message registration and dispatch for the Elm-architecture pattern.

    The MessageManager maps MessageId values to handler functions that
    produce application messages when events occur (e.g., button clicks).
    """

    def __init__(self) -> None:
        self._next_id: int = 0
        self._handlers: dict[int, Callable[[Message], Msg]] = {}

    def register(self, handler: Callable[[Message], Msg]) -> MessageId:
        """
        Register a handler that produces a message when triggered.
        Returns a MessageId that can be passed to widgets.
        """
        msg_id = self._next_id
        self._next_id += 1
        self._handlers[msg_id] = handler
        return msg_id

    def on(self, handler: Callable[[], Msg]) -> MessageId:
        """
        Register a simple handler that ignores the Message payload.
        Useful for simple button clicks that don't need event data.
        """
        return self.register(lambda _: handler())

    def dispatch(self, msg_id: MessageId, message: Message) -> Optional[Msg]:
        """
        Dispatch a message by its ID, returning the application message if found.
        """
        handler = self._handlers.get(msg_id)
        return handler(message) if handler else None

    def clear(self) -> None:
        """
        Clear all registered handlers.
        Called before each view() to avoid stale handlers.
        """
        self._handlers.clear()
        self._next_id = 0
