"""Application framework following the Elm architecture."""

import sys
from abc import ABC, abstractmethod
from types import ModuleType
from typing import Generic, TypeVar, cast

from wit_world.exports import AppInstance as WitAppInstance
from wit_world.exports.app_instance import Application as WitApplication
from wit_world.exports.app_instance import MessageValue

from .callbacks import Frame, resolve, with_frame
from .element import ElementLike, WitElement, to_element

Msg = TypeVar("Msg")
T = TypeVar("T", bound="App[object]")


class App(ABC, Generic[Msg]):
    """An Igloo application whose state is stored on the instance."""

    @abstractmethod
    def update(self, msg: Msg) -> None:
        """Update the application state from a message."""
        ...

    @abstractmethod
    def view(self) -> ElementLike:
        """Render the current application state."""
        ...


def create_application(app_type: type[App[Msg]]) -> type[WitApplication]:
    """Create the exported WIT application resource for an App type."""

    class Application:
        def __init__(self) -> None:
            erased_app_type = cast(type[App[object]], app_type)
            self._app: App[object] = erased_app_type()
            self._current: Frame[object] = Frame(0)
            self._previous: Frame[object] = Frame(0)

        def view(self) -> WitElement:
            next_frame: Frame[object] = Frame(self._current.next_base())
            with with_frame(next_frame):
                element = to_element(self._app.view()).inner
            self._previous = self._current
            self._current = next_frame
            return element

        def update(self, id: int, value: MessageValue) -> None:
            callback = self._current.get(id) or self._previous.get(id)
            if callback is None:
                return
            message = resolve(callback, value)
            if message is not None:
                self._app.update(message)

    return Application


def igloo_app(cls: type[T]) -> type[T]:
    """Export an App subclass through componentize-py's generated bindings."""
    application = create_application(cls)

    class AppInstance(WitAppInstance):
        pass

    module_globals = sys._getframe(1).f_globals
    module_globals["AppInstance"] = AppInstance
    module_globals["Application"] = application
    resource_module = ModuleType("app_instance")
    resource_module.__dict__["Application"] = application
    sys.modules["app_instance"] = resource_module
    return cls
