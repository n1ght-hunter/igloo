/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/table@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A grid-like visual representation of data distributed in columns and rows.
   */
  export interface Table {
    /**
     * Header cells of the [`Table`].
     */
    columns: Array<Element>,
    /**
     * Rows of cells for the [`Table`].
     */
    rows: Array<Element>,
    width?: Length,
    padding?: Pixels,
    paddingX?: Pixels,
    paddingY?: Pixels,
    separator?: Pixels,
    separatorX?: Pixels,
    separatorY?: Pixels,
  }
}
