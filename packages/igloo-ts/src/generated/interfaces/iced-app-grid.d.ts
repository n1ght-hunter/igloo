/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/grid@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  /**
   * A container that arranges its contents in a grid.
   */
  export interface Grid {
    /**
     * The elements of the [`Grid`].
     */
    elements: Array<Element>,
    /**
     * Spacing between cells in the [`Grid`].
     */
    spacing?: Pixels,
    /**
     * Width of the [`Grid`].
     */
    width?: Pixels,
    /**
     * Height of the [`Grid`].
     */
    height?: Pixels,
    /**
     * Number of columns in the [`Grid`].
     */
    columns?: bigint,
    fluid?: Pixels,
  }
}
