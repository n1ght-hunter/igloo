/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/image@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Rotation = import('iced:app/shared@0.1.0').Rotation;
  export type ContentFit = import('iced:app/shared@0.1.0').ContentFit;
  export type FilterMethod = import('iced:app/shared@0.1.0').FilterMethod;
  export type Length = import('iced:app/length@0.1.0').Length;
  
  export class Image implements Disposable {
    constructor(handle: string)
    width(w: Length): void;
    height(h: Length): void;
    expand(expand: boolean): void;
    contentFit(fit: ContentFit): void;
    filterMethod(method: FilterMethod): void;
    rotation(r: Rotation): void;
    opacity(o: number): void;
    scale(s: number): void;
    static intoElement(widget: Image): Element;
    [Symbol.dispose](): void;
  }
}
