/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/toggler@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Shaping = import('iced:app/text@0.1.0').Shaping;
  export type Wrapping = import('iced:app/text@0.1.0').Wrapping;
  export type Horizontal = import('iced:app/alignment@0.1.0').Horizontal;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class Toggler implements Disposable {
    constructor(isToggled: boolean)
    label(label: string): void;
    onToggle(mapper: CallbackId): void;
    size(s: Pixels): void;
    width(w: Length): void;
    textSize(s: Pixels): void;
    textLineHeight(lh: LineHeight): void;
    textAlignment(a: Horizontal): void;
    textShaping(s: Shaping): void;
    textWrapping(w: Wrapping): void;
    spacing(s: Pixels): void;
    static intoElement(widget: Toggler): Element;
    [Symbol.dispose](): void;
  }
}
