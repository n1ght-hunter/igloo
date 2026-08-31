/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/radio@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Wrapping = import('iced:app/text@0.1.0').Wrapping;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class Radio implements Disposable {
    constructor(label: string, isSelected: boolean, msg: CallbackId)
    size(s: Pixels): void;
    width(w: Length): void;
    spacing(s: Pixels): void;
    textSize(s: Pixels): void;
    textLineHeight(lh: LineHeight): void;
    textWrapping(w: Wrapping): void;
    textShaping(s: Shaping): void;
    static intoElement(widget: Radio): Element;
    [Symbol.dispose](): void;
  }
}
