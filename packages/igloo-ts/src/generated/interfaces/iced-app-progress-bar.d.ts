/// <reference path="./iced-app-length.d.ts" />
declare module 'iced:app/progress-bar@0.1.0' {
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A bar that displays progress.
   */
  export interface ProgressBar {
    /**
     * Start value of the range of possible values.
     */
    rangeStart: number,
    /**
     * End value of the range of possible values.
     */
    rangeEnd: number,
    /**
     * Current value of the [`ProgressBar`].
     */
    value: number,
    length?: Length,
    girth?: Length,
    vertical?: boolean,
  }
}
