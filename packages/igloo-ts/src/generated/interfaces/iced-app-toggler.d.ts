/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/toggler@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Wrapping = import('iced:app/text@0.1.0').Wrapping;
  export type Horizontal = import('iced:app/alignment@0.1.0').Horizontal;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A widget representing a setting that can be toggled on or off.
   */
  export interface Toggler {
    /**
     * Whether the [`Toggler`] is currently toggled.
     */
    isToggled: boolean,
    /**
     * The optional label of the [`Toggler`].
     */
    label?: string,
    /**
     * The message produced when the [`Toggler`] is toggled.
     */
    onToggle?: MessageId,
    size?: Pixels,
    width?: Length,
    textSize?: Pixels,
    textLineHeight?: LineHeight,
    textAlignment?: Horizontal,
    textShaping?: Shaping,
    textWrapping?: Wrapping,
    spacing?: Pixels,
  }
}
