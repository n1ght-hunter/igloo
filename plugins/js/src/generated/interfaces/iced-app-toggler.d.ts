/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/toggler@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Message = import('iced:app/message@0.1.0').Message;
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
    onToggle?: Message,
  }
}
