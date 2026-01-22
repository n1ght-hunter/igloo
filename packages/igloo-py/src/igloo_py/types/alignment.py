"""Alignment type helpers."""

from typing import Any

# Try to import generated types, fall back to simple definitions
try:
    from ..generated.wit_world.imports.alignment import (
        Horizontal as WitHorizontal,
        Vertical as WitVertical,
    )

    class _Horizontal:
        """Helper for creating Horizontal alignment values."""

        @staticmethod
        def left() -> WitHorizontal:
            """Align to the left."""
            return WitHorizontal.LEFT

        @staticmethod
        def center() -> WitHorizontal:
            """Align to the center."""
            return WitHorizontal.CENTER

        @staticmethod
        def right() -> WitHorizontal:
            """Align to the right."""
            return WitHorizontal.RIGHT

    class _Vertical:
        """Helper for creating Vertical alignment values."""

        @staticmethod
        def top() -> WitVertical:
            """Align to the top."""
            return WitVertical.TOP

        @staticmethod
        def center() -> WitVertical:
            """Align to the center."""
            return WitVertical.CENTER

        @staticmethod
        def bottom() -> WitVertical:
            """Align to the bottom."""
            return WitVertical.BOTTOM

except ImportError:
    # Fallback for when bindings aren't generated yet
    WitHorizontal = Any  # type: ignore
    WitVertical = Any  # type: ignore

    class _Horizontal:
        @staticmethod
        def left() -> Any:
            return {"tag": "left"}

        @staticmethod
        def center() -> Any:
            return {"tag": "center"}

        @staticmethod
        def right() -> Any:
            return {"tag": "right"}

    class _Vertical:
        @staticmethod
        def top() -> Any:
            return {"tag": "top"}

        @staticmethod
        def center() -> Any:
            return {"tag": "center"}

        @staticmethod
        def bottom() -> Any:
            return {"tag": "bottom"}


# Singleton instances for convenient access
Horizontal = _Horizontal()
Vertical = _Vertical()
