declare module 'iced:app/message-types@0.1.0' {
  export interface Rectangle {
    x: number,
    y: number,
    width: number,
    height: number,
  }
  export interface AbsoluteOffset {
    x: number,
    y: number,
  }
  export interface RelativeOffset {
    x: number,
    y: number,
  }
  /**
   * The current Viewport of the Scrollable.
   */
  export interface Viewport {
    /**
     * Returns the AbsoluteOffset of the current Viewport.
     */
    absoluteOffset: AbsoluteOffset,
    /**
     * Returns the AbsoluteOffset of the current Viewport, but with its alignment reversed.
     * This method can be useful to switch the alignment of a Scrollable while maintaining its scrolling position.
     */
    absoluteOffsetReversed: AbsoluteOffset,
    /**
     * Returns the RelativeOffset of the current Viewport.
     */
    relativeOffset: RelativeOffset,
    /**
     * Returns the bounds of the current Viewport.
     */
    bounds: Rectangle,
    /**
     * Returns the content bounds of the current Viewport.
     */
    contentBounds: Rectangle,
  }
}
