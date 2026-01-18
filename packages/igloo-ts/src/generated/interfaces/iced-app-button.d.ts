/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/button@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export interface Button {
    content: Element,
    width?: Length,
    height?: Length,
    padding?: Padding,
    onPress?: MessageId,
    clip?: boolean,
  }
}
