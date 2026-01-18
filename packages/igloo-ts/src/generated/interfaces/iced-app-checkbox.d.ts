/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/checkbox@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Wrapping = import('iced:app/text@0.1.0').Wrapping;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A box that can be checked.
   */
  export interface Checkbox {
    /**
     * Whether the [`Checkbox`] is currently checked.
     */
    isChecked: boolean,
    /**
     * The label of the [`Checkbox`].
     */
    label?: string,
    /**
     * The message produced when the [`Checkbox`] is toggled.
     */
    onToggle?: MessageId,
    size?: Pixels,
    width?: Length,
    height?: Length,
    spacing?: Pixels,
    textSize?: Pixels,
    textLineHeight?: LineHeight,
    textWrapping?: Wrapping,
    textShaping?: Shaping,
  }
}
