/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/button@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class Button implements Disposable {
    constructor(content: Element)
    onPress(msg: CallbackId): void;
    width(w: Length): void;
    height(h: Length): void;
    padding(p: Padding): void;
    clip(clip: boolean): void;
    static intoElement(widget: Button): Element;
    [Symbol.dispose](): void;
  }
}
