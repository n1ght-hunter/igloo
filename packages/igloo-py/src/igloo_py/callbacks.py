"""Guest-owned widget callback frames."""

from collections.abc import Callable, Iterator
from contextlib import contextmanager
from dataclasses import dataclass
from typing import Generic, Literal, TypeVar, cast

from wit_world.exports.app_instance import (
    MessageValue,
    MessageValue_BoolValue,
    MessageValue_F32Value,
    MessageValue_F64Value,
    MessageValue_Fixed,
    MessageValue_StringValue,
    MessageValue_U64Value,
    MessageValue_ViewportValue,
)
from wit_world.imports.message_types import Viewport

Msg = TypeVar("Msg")
Value = TypeVar("Value")


@dataclass
class FixedCallback(Generic[Msg]):
    """A callback carrying an application message directly."""

    kind: Literal["fixed"]
    message: Msg


@dataclass
class MapperCallback(Generic[Value, Msg]):
    """A callback mapping a widget value into an application message."""

    kind: Literal["bool", "f32", "f64", "u64", "string", "viewport"]
    mapper: Callable[[Value], Msg]


Callback = FixedCallback[Msg] | MapperCallback[object, Msg]


class Frame(Generic[Msg]):
    """One view call's callbacks, addressed by monotonically increasing IDs."""

    def __init__(self, base: int) -> None:
        self._base = base
        self._callbacks: list[Callback[Msg]] = []

    def next_base(self) -> int:
        """Return the first callback ID available to the next frame."""
        return self._base + len(self._callbacks)

    def push(self, callback: Callback[Msg]) -> int:
        """Append a callback and return its ID."""
        callback_id = self.next_base()
        self._callbacks.append(callback)
        return callback_id

    def get(self, callback_id: int) -> Callback[Msg] | None:
        """Return a callback when its ID belongs to this frame."""
        index = callback_id - self._base
        if index < 0 or index >= len(self._callbacks):
            return None
        return self._callbacks[index]


_current_frame: Frame[object] | None = None


@contextmanager
def with_frame(frame: Frame[Msg]) -> Iterator[None]:
    """Install a frame while a view tree is being built."""
    global _current_frame
    previous = _current_frame
    _current_frame = cast(Frame[object], frame)
    try:
        yield
    finally:
        _current_frame = previous


def _active_frame() -> Frame[object]:
    if _current_frame is None:
        raise RuntimeError(
            "igloo: a widget callback was registered outside of view(); "
            "build widgets inside your app view method"
        )
    return _current_frame


def push_fixed(message: Msg) -> int:
    """Register a fixed application message."""
    return _active_frame().push(FixedCallback(kind="fixed", message=message))


def _push_mapper(
    kind: Literal["bool", "f32", "f64", "u64", "string", "viewport"],
    mapper: Callable[[Value], Msg],
) -> int:
    callback = MapperCallback(kind=kind, mapper=mapper)
    return _active_frame().push(cast(Callback[object], callback))


def push_bool(mapper: Callable[[bool], Msg]) -> int:
    """Register a boolean-valued callback."""
    return _push_mapper("bool", mapper)


def push_f32(mapper: Callable[[float], Msg]) -> int:
    """Register an f32-valued callback."""
    return _push_mapper("f32", mapper)


def push_f64(mapper: Callable[[float], Msg]) -> int:
    """Register an f64-valued callback."""
    return _push_mapper("f64", mapper)


def push_u64(mapper: Callable[[int], Msg]) -> int:
    """Register a u64-valued callback."""
    return _push_mapper("u64", mapper)


def push_string(mapper: Callable[[str], Msg]) -> int:
    """Register a string-valued callback."""
    return _push_mapper("string", mapper)


def push_viewport(mapper: Callable[[Viewport], Msg]) -> int:
    """Register a viewport-valued callback."""
    return _push_mapper("viewport", mapper)


def resolve(callback: Callback[Msg], value: MessageValue) -> Msg | None:
    """Resolve a callback only when its expected value kind matches."""
    if callback.kind == "fixed" and isinstance(value, MessageValue_Fixed):
        return callback.message
    if not isinstance(callback, MapperCallback):
        return None
    if callback.kind == "bool" and isinstance(value, MessageValue_BoolValue):
        return callback.mapper(value.value)
    if callback.kind == "f32" and isinstance(value, MessageValue_F32Value):
        return callback.mapper(value.value)
    if callback.kind == "f64" and isinstance(value, MessageValue_F64Value):
        return callback.mapper(value.value)
    if callback.kind == "u64" and isinstance(value, MessageValue_U64Value):
        return callback.mapper(value.value)
    if callback.kind == "string" and isinstance(value, MessageValue_StringValue):
        return callback.mapper(value.value)
    if callback.kind == "viewport" and isinstance(value, MessageValue_ViewportValue):
        return callback.mapper(value.value)
    return None
