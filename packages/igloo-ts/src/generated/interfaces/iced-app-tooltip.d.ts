/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/tooltip@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  /**
   * # Variants
   * 
   * ## `"top"`
   * 
   * ## `"bottom"`
   * 
   * ## `"left"`
   * 
   * ## `"right"`
   * 
   * ## `"follow-cursor"`
   */
  export type Position = 'top' | 'bottom' | 'left' | 'right' | 'follow-cursor';
  
  export class Tooltip implements Disposable {
    constructor(content: Element, tooltip: Element, position: Position)
    gap(g: Pixels): void;
    padding(p: Pixels): void;
    snapWithinViewport(snap: boolean): void;
    static intoElement(widget: Tooltip): Element;
    [Symbol.dispose](): void;
  }
}
