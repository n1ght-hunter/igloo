/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/slider@0.1.0' {
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A horizontal bar for selecting a value from a range of floats.
   */
  export interface Slider {
    /**
     * Start value of the range of possible values.
     */
    rangeStart: number,
    /**
     * End value of the range of possible values.
     */
    rangeEnd: number,
    /**
     * The current value of the [`Slider`].
     */
    value: number,
    onChange: MessageId,
    'default'?: number,
    /**
     * The message produced when the [`Slider`] is released.
     */
    onRelease?: MessageId,
    width?: Length,
    height?: Pixels,
    step?: number,
    shiftStep?: number,
  }
}
