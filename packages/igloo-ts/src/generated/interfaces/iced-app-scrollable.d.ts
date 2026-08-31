/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/scrollable@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  /**
   * The anchor of the scroller of a scrollable relative to its viewport.
   * # Variants
   * 
   * ## `"start"`
   * 
   * ## `"end"`
   */
  export type Anchor = 'start' | 'end';
  export interface Scrollbar {
    width?: Pixels,
    margin?: Pixels,
    scrollerWidth?: Pixels,
    anchor?: Anchor,
    spacing?: Pixels,
  }
  export type Direction = DirectionVertical | DirectionHorizontal | DirectionBoth;
  export interface DirectionVertical {
    tag: 'vertical',
    val: Scrollbar,
  }
  export interface DirectionHorizontal {
    tag: 'horizontal',
    val: Scrollbar,
  }
  export interface DirectionBoth {
    tag: 'both',
    val: [Scrollbar, Scrollbar],
  }
  
  export class Scrollable implements Disposable {
    constructor(content: Element)
    width(w: Length): void;
    height(h: Length): void;
    direction(d: Direction): void;
    onScroll(mapper: CallbackId): void;
    static intoElement(widget: Scrollable): Element;
    [Symbol.dispose](): void;
  }
}
