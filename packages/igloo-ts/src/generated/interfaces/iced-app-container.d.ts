/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/container@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type Horizontal = import('iced:app/alignment@0.1.0').Horizontal;
  export type Vertical = import('iced:app/alignment@0.1.0').Vertical;
  
  export class Container implements Disposable {
    constructor(content: Element)
    padding(p: Padding): void;
    width(w: Length): void;
    height(h: Length): void;
    maxWidth(max: Pixels): void;
    maxHeight(max: Pixels): void;
    centerX(w: Length): void;
    centerY(h: Length): void;
    center(l: Length): void;
    alignLeft(w: Length): void;
    alignRight(w: Length): void;
    alignTop(h: Length): void;
    alignBottom(h: Length): void;
    alignX(align: Horizontal): void;
    alignY(align: Vertical): void;
    clip(clip: boolean): void;
    static intoElement(widget: Container): Element;
    [Symbol.dispose](): void;
  }
}
