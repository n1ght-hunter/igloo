import type { Color as WitColor } from 'iced:app/shared@0.1.0';

export type { Color as WitColor } from 'iced:app/shared@0.1.0';

/**
 * Helper functions for creating Color values.
 * Colors are in the sRGB color space with components from 0.0 to 1.0.
 */
export const Color = {
  /** Create a color with RGB values (0.0-1.0) and full opacity */
  rgb(r: number, g: number, b: number): WitColor {
    return { r, g, b, a: 1.0 };
  },

  /** Create a color with RGBA values (0.0-1.0) */
  rgba(r: number, g: number, b: number, a: number): WitColor {
    return { r, g, b, a };
  },

  /** Create a color from 8-bit RGB values (0-255) */
  rgb8(r: number, g: number, b: number): WitColor {
    return { r: r / 255, g: g / 255, b: b / 255, a: 1.0 };
  },

  /** Create a color from 8-bit RGBA values (0-255) */
  rgba8(r: number, g: number, b: number, a: number): WitColor {
    return { r: r / 255, g: g / 255, b: b / 255, a: a / 255 };
  },

  /** Create a color from a hex string (e.g., "#ff0000" or "ff0000") */
  hex(hex: string): WitColor {
    const cleanHex = hex.replace('#', '');
    const r = parseInt(cleanHex.slice(0, 2), 16) / 255;
    const g = parseInt(cleanHex.slice(2, 4), 16) / 255;
    const b = parseInt(cleanHex.slice(4, 6), 16) / 255;
    const a = cleanHex.length === 8 ? parseInt(cleanHex.slice(6, 8), 16) / 255 : 1.0;
    return { r, g, b, a };
  },

  // Common colors
  white: { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } as WitColor,
  black: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } as WitColor,
  transparent: { r: 0.0, g: 0.0, b: 0.0, a: 0.0 } as WitColor,
  red: { r: 1.0, g: 0.0, b: 0.0, a: 1.0 } as WitColor,
  green: { r: 0.0, g: 1.0, b: 0.0, a: 1.0 } as WitColor,
  blue: { r: 0.0, g: 0.0, b: 1.0, a: 1.0 } as WitColor,
} as const;
