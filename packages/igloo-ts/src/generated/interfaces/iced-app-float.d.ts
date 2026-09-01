/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/float@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  /**
   * A translation applied to content.
   */
  export interface Translation {
    /**
     * Horizontal offset.
     */
    x: number,
    /**
     * Vertical offset.
     */
    y: number,
  }
  
  export class Float implements Disposable {
    constructor(content: Element)
    scale(s: number): void;
    translation(t: Translation): void;
    static intoElement(widget: Float): Element;
    [Symbol.dispose](): void;
  }
}
