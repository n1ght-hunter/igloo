/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/text-input@0.1.0' {
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Horizontal = import('iced:app/alignment@0.1.0').Horizontal;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A field that can be filled with text.
   */
  export interface TextInput {
    /**
     * The placeholder text of the [`TextInput`].
     */
    placeholder: string,
    /**
     * The current value of the [`TextInput`].
     */
    value: string,
    /**
     * Whether the [`TextInput`] is secure (e.g. password field).
     */
    secure?: boolean,
    /**
     * The message produced when the [`TextInput`] changes.
     */
    onInput?: MessageId,
    /**
     * The message produced when the [`TextInput`] is submitted.
     */
    onSubmit?: MessageId,
    onPaste?: MessageId,
    width?: Length,
    padding?: Padding,
    size?: Pixels,
    lineHeight?: LineHeight,
    alignX?: Horizontal,
  }
}
