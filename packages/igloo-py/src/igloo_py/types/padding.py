"""Padding type helpers."""

from typing import Any

# Try to import generated types, fall back to simple definitions
try:
    from ..generated.wit_world.imports.padding import Padding as WitPadding

    class _Padding:
        """Helper functions for creating Padding values."""

        @staticmethod
        def all(value: float) -> WitPadding:
            """Create padding with the same value on all sides."""
            return WitPadding(left=value, right=value, top=value, bottom=value)

        @staticmethod
        def xy(x: float, y: float) -> WitPadding:
            """Create padding with separate horizontal and vertical values."""
            return WitPadding(left=x, right=x, top=y, bottom=y)

        @staticmethod
        def each(top: float, right: float, bottom: float, left: float) -> WitPadding:
            """Create padding with separate values for each side."""
            return WitPadding(left=left, right=right, top=top, bottom=bottom)

        @staticmethod
        def none() -> WitPadding:
            """No padding."""
            return WitPadding(left=0, right=0, top=0, bottom=0)

except ImportError:
    # Fallback for when bindings aren't generated yet
    WitPadding = Any  # type: ignore

    class _Padding:
        @staticmethod
        def all(value: float) -> Any:
            return {"top": value, "right": value, "bottom": value, "left": value}

        @staticmethod
        def xy(x: float, y: float) -> Any:
            return {"top": y, "right": x, "bottom": y, "left": x}

        @staticmethod
        def each(top: float, right: float, bottom: float, left: float) -> Any:
            return {"top": top, "right": right, "bottom": bottom, "left": left}

        @staticmethod
        def none() -> Any:
            return {"top": 0, "right": 0, "bottom": 0, "left": 0}


# Singleton instance for convenient access
Padding = _Padding()
