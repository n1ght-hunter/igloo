declare module 'iced:app/progress-bar@0.1.0' {
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
  }
}
