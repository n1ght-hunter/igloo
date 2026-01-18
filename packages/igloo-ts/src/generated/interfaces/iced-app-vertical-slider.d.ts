/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/vertical-slider@0.1.0' {
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A vertical bar for selecting a value from a range of floats.
   */
  export interface VerticalSlider {
    /**
     * Start value of the range of possible values.
     */
    rangeStart: number,
    /**
     * End value of the range of possible values.
     */
    rangeEnd: number,
    /**
     * The current value of the [`VerticalSlider`].
     */
    value: number,
    /**
     * The message produced when the [`VerticalSlider`] is dragged.
     */
    onChange: MessageId,
    /**
     * The default value of the [`VerticalSlider`].
     */
    'default'?: number,
    /**
     * The message produced when the [`VerticalSlider`] is released.
     */
    onRelease?: MessageId,
    width?: Pixels,
    height?: Length,
    step?: number,
    shiftStep?: number,
  }
}
