import type { Padding as WitPadding } from 'iced:app/padding@0.1.0';

export type { Padding as WitPadding } from 'iced:app/padding@0.1.0';

/**
 * Helper functions for creating Padding values.
 */
export const Padding = {
  /** Create padding with the same value on all sides */
  all(value: number): WitPadding {
    return { top: value, right: value, bottom: value, left: value };
  },

  /** Create padding with separate horizontal and vertical values */
  xy(x: number, y: number): WitPadding {
    return { top: y, right: x, bottom: y, left: x };
  },

  /** Create padding with separate values for each side */
  each(top: number, right: number, bottom: number, left: number): WitPadding {
    return { top, right, bottom, left };
  },

  /** No padding */
  none(): WitPadding {
    return { top: 0, right: 0, bottom: 0, left: 0 };
  },
} as const;
