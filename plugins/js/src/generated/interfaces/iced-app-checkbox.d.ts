/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/checkbox@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Message = import('iced:app/message@0.1.0').Message;
  /**
   * A box that can be checked.
   */
  export interface Checkbox {
    /**
     * The label of the [`Checkbox`].
     */
    label: string,
    /**
     * Whether the [`Checkbox`] is currently checked.
     */
    isChecked: boolean,
    /**
     * The message produced when the [`Checkbox`] is toggled.
     */
    onToggle?: Message,
  }
}
