"""Element wrapper and IntoElement abstract base class."""

from abc import ABC, abstractmethod
from typing import Union, Any

from wit_world.imports.element import explain as wit_explain
from wit_world.imports.shared import Color

# WitElement is the raw element type from WIT bindings
# After generation, this will be properly typed
WitElement = Any


class IntoElement(ABC):
    """Abstract base class for types that can be converted to an Element."""

    @abstractmethod
    def into_element(self) -> "Element":
        """Convert this widget into an Element."""
        ...


class Element:
    """
    Wrapper class for the WIT Element resource.
    Provides a convenient interface for working with UI elements.
    """

    def __init__(self, inner: WitElement) -> None:
        self.inner = inner

    def into_element(self) -> "Element":
        """Returns itself (Element already is an Element)."""
        return self

    def explain(self, color: Color) -> "Element":
        """
        Debug helper that draws a colored overlay on the element.
        Useful for visualizing element bounds during development.
        """
        return Element(wit_explain(self.inner, color))


# Type that can be used where an Element is expected
ElementLike = Union[Element, IntoElement]


def to_element(value: ElementLike) -> Element:
    """
    Convert an ElementLike to an Element.
    If already an Element, returns it. Otherwise calls into_element().
    """
    if isinstance(value, Element):
        return value
    return value.into_element()
