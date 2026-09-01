/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/space@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  
  export class Space implements Disposable {
    constructor()
    width(w: Length): void;
    height(h: Length): void;
    static intoElement(widget: Space): Element;
    [Symbol.dispose](): void;
  }
}
