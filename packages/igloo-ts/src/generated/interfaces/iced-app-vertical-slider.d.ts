/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/vertical-slider@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class VerticalSlider implements Disposable {
    constructor(rangeStart: number, rangeEnd: number, value: number, onChange: CallbackId)
    'default'(v: number): void;
    onRelease(msg: CallbackId): void;
    width(w: Pixels): void;
    height(h: Length): void;
    step(s: number): void;
    shiftStep(s: number): void;
    static intoElement(widget: VerticalSlider): Element;
    [Symbol.dispose](): void;
  }
}
