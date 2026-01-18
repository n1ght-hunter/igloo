/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/pick-list@0.1.0' {
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  /**
   * A widget for selecting a value from a set of options.
   */
  export interface PickList {
    /**
     * The options displayed in the [`PickList`].
     */
    options: Array<string>,
    /**
     * The index of the currently selected option.
     */
    selected?: string,
    onSelect: MessageId,
    /**
     * The placeholder to display when no option is selected.
     */
    placeholder?: string,
    width?: Length,
    padding?: Padding,
    textSize?: Pixels,
    textLineHeight?: LineHeight,
    textShaping?: Shaping,
    /**
     * The message produced when the [`PickList`] is opened.
     */
    onOpen?: MessageId,
    /**
     * The message produced when the [`PickList`] is closed.
     */
    onClose?: MessageId,
  }
}
