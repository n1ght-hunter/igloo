/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/pick-list@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class PickList implements Disposable {
    constructor(options: Array<string>, selected: string | undefined, onSelect: CallbackId)
    placeholder(text: string): void;
    width(w: Length): void;
    padding(p: Padding): void;
    textSize(s: Pixels): void;
    textLineHeight(lh: LineHeight): void;
    textShaping(s: Shaping): void;
    onOpen(msg: CallbackId): void;
    onClose(msg: CallbackId): void;
    static intoElement(widget: PickList): Element;
    [Symbol.dispose](): void;
  }
}
