"""Length type helpers."""

from typing import Union, Any

# Try to import generated types, fall back to simple definitions
try:
    from ..generated.wit_world.imports.length import (
        Length as WitLength,
        Length_Fill,
        Length_FillPortion,
        Length_Shrink,
        Length_Fixed,
    )

    class _Length:
        """
        Helper functions for creating Length values.
        Length defines the strategy used to fill space in a specific dimension.
        """

        @staticmethod
        def fill() -> WitLength:
            """Fill all the remaining space."""
            return Length_Fill()

        @staticmethod
        def shrink() -> WitLength:
            """Fill the least amount of space."""
            return Length_Shrink()

        @staticmethod
        def fill_portion(portion: int) -> WitLength:
            """
            Fill a portion of the remaining space relative to other elements.
            fill() is equivalent to fill_portion(1).
            """
            return Length_FillPortion(portion)

        @staticmethod
        def fixed(pixels: float) -> WitLength:
            """Fill a fixed amount of space in pixels."""
            return Length_Fixed(pixels)

except ImportError:
    # Fallback for when bindings aren't generated yet
    WitLength = Any  # type: ignore

    class _Length:
        @staticmethod
        def fill() -> Any:
            return {"tag": "fill"}

        @staticmethod
        def shrink() -> Any:
            return {"tag": "shrink"}

        @staticmethod
        def fill_portion(portion: int) -> Any:
            return {"tag": "fill-portion", "val": portion}

        @staticmethod
        def fixed(pixels: float) -> Any:
            return {"tag": "fixed", "val": pixels}


# Singleton instance for convenient access
Length = _Length()
