/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/button@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Message = import('iced:app/message@0.1.0').Message;
  export interface Button {
    content: Element,
    onClick?: Message,
  }
}
