"""Alignment type helpers."""

from wit_world.imports.alignment import (
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


# Singleton instances for convenient access
Horizontal = _Horizontal()
Vertical = _Vertical()
