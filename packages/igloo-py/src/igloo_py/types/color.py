"""Color type helpers."""

from typing import TypedDict


class WitColor(TypedDict):
    """RGBA color."""

    r: float
    g: float
    b: float
    a: float


# Type alias for convenience
Color = WitColor


class _Color:
    """Helper functions for creating Color values."""

    @staticmethod
    def rgba(r: float, g: float, b: float, a: float = 1.0) -> Color:
        """Create a color from RGBA values (0.0-1.0)."""
        return {"r": r, "g": g, "b": b, "a": a}

    @staticmethod
    def rgb(r: float, g: float, b: float) -> Color:
        """Create an opaque color from RGB values (0.0-1.0)."""
        return {"r": r, "g": g, "b": b, "a": 1.0}

    @staticmethod
    def hex(hex_str: str) -> Color:
        """
        Create a color from a hex string.
        Supports formats: #RGB, #RGBA, #RRGGBB, #RRGGBBAA
        """
        hex_str = hex_str.lstrip("#")

        if len(hex_str) == 3:
            r = int(hex_str[0] * 2, 16) / 255.0
            g = int(hex_str[1] * 2, 16) / 255.0
            b = int(hex_str[2] * 2, 16) / 255.0
            return {"r": r, "g": g, "b": b, "a": 1.0}
        elif len(hex_str) == 4:
            r = int(hex_str[0] * 2, 16) / 255.0
            g = int(hex_str[1] * 2, 16) / 255.0
            b = int(hex_str[2] * 2, 16) / 255.0
            a = int(hex_str[3] * 2, 16) / 255.0
            return {"r": r, "g": g, "b": b, "a": a}
        elif len(hex_str) == 6:
            r = int(hex_str[0:2], 16) / 255.0
            g = int(hex_str[2:4], 16) / 255.0
            b = int(hex_str[4:6], 16) / 255.0
            return {"r": r, "g": g, "b": b, "a": 1.0}
        elif len(hex_str) == 8:
            r = int(hex_str[0:2], 16) / 255.0
            g = int(hex_str[2:4], 16) / 255.0
            b = int(hex_str[4:6], 16) / 255.0
            a = int(hex_str[6:8], 16) / 255.0
            return {"r": r, "g": g, "b": b, "a": a}
        else:
            raise ValueError(f"Invalid hex color: #{hex_str}")

    # Common colors
    @staticmethod
    def white() -> Color:
        return {"r": 1.0, "g": 1.0, "b": 1.0, "a": 1.0}

    @staticmethod
    def black() -> Color:
        return {"r": 0.0, "g": 0.0, "b": 0.0, "a": 1.0}

    @staticmethod
    def red() -> Color:
        return {"r": 1.0, "g": 0.0, "b": 0.0, "a": 1.0}

    @staticmethod
    def green() -> Color:
        return {"r": 0.0, "g": 1.0, "b": 0.0, "a": 1.0}

    @staticmethod
    def blue() -> Color:
        return {"r": 0.0, "g": 0.0, "b": 1.0, "a": 1.0}

    @staticmethod
    def transparent() -> Color:
        return {"r": 0.0, "g": 0.0, "b": 0.0, "a": 0.0}


# Singleton instance for convenient access
ColorHelper = _Color()
