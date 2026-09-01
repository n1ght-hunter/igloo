/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/text-input@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Horizontal = import('iced:app/alignment@0.1.0').Horizontal;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class TextInput implements Disposable {
    constructor(placeholder: string, value: string)
    secure(secure: boolean): void;
    onInput(mapper: CallbackId): void;
    onSubmit(msg: CallbackId): void;
    onPaste(mapper: CallbackId): void;
    width(w: Length): void;
    padding(p: Padding): void;
    size(s: Pixels): void;
    lineHeight(lh: LineHeight): void;
    alignX(a: Horizontal): void;
    static intoElement(widget: TextInput): Element;
    [Symbol.dispose](): void;
  }
}
