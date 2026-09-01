/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/grid@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  
  export class Grid implements Disposable {
    constructor()
    push(child: Element): void;
    spacing(amount: Pixels): void;
    width(w: Pixels): void;
    height(h: Pixels): void;
    columns(columns: bigint): void;
    fluid(amount: Pixels): void;
    static intoElement(widget: Grid): Element;
    [Symbol.dispose](): void;
  }
}
