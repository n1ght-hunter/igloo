/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/combo-box@0.1.0' {
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  /**
   * A widget allowing selection from a list of options.
   */
  export interface ComboBox {
    /**
     * The options displayed in the [`ComboBox`].
     */
    options: Array<string>,
    /**
     * The placeholder to display when no option is selected.
     */
    placeholder: string,
    /**
     * The index of the currently selected option.
     */
    selected?: string,
    /**
     * The message produced when an option is selected.
     */
    onSelected: MessageId,
    onInput?: MessageId,
    onOptionHovered?: MessageId,
    onOpen?: MessageId,
    onClose?: MessageId,
    padding?: Padding,
    size?: number,
    lineHeight?: LineHeight,
    width?: Length,
  }
}
