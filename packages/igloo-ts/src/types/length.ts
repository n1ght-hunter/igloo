import type { Length as WitLength } from 'iced:app/length@0.1.0';

export type { Length as WitLength } from 'iced:app/length@0.1.0';

/**
 * Helper functions for creating Length values.
 * Length defines the strategy used to fill space in a specific dimension.
 */
export const Length = {
  /** Fill all the remaining space */
  fill(): WitLength {
    return { tag: 'fill' };
  },

  /** Fill the least amount of space */
  shrink(): WitLength {
    return { tag: 'shrink' };
  },

  /**
   * Fill a portion of the remaining space relative to other elements.
   * fill() is equivalent to fillPortion(1).
   */
  fillPortion(portion: number): WitLength {
    return { tag: 'fill-portion', val: portion };
  },

  /** Fill a fixed amount of space in pixels */
  fixed(pixels: number): WitLength {
    return { tag: 'fixed', val: pixels };
  },
} as const;
