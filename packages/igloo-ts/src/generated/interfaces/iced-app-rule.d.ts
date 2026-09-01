/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/rule@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  
  export class Rule implements Disposable {
    constructor(isHorizontal: boolean, thickness: Pixels)
    static intoElement(widget: Rule): Element;
    [Symbol.dispose](): void;
  }
}
