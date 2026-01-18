/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/radio@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Wrapping = import('iced:app/text@0.1.0').Wrapping;
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  /**
   * A circular button representing an alternative.
   */
  export interface Radio {
    /**
     * The label of the radio.
     */
    label: string,
    /**
     * Whether the radio is currently selected.
     */
    isSelected: boolean,
    /**
     * The message produced when the radio is selected.
     */
    onSelect: MessageId,
    /**
     * Sets the size of the radio button
     */
    size?: Pixels,
    /**
     * sets the width of the radio button
     */
    width?: Length,
    /**
     * Sets the spacing between the Radio button and the text.
     */
    spacing?: Pixels,
    /**
     * Sets the text size of the Radio button.
     */
    textSize?: Pixels,
    /**
     * Sets the text line height of the Radio button.
     */
    textLineHeight?: LineHeight,
    /**
     * Sets the text wrapping of the Radio button.
     */
    textWrapping?: Wrapping,
    /**
     * Sets the text shaping of the Radio button.
     */
    textShaping?: Shaping,
  }
}
