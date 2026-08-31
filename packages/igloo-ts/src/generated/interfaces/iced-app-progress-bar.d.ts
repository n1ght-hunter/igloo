/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/progress-bar@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  
  export class ProgressBar implements Disposable {
    constructor(rangeStart: number, rangeEnd: number, value: number)
    length(l: Length): void;
    girth(g: Length): void;
    vertical(v: boolean): void;
    static intoElement(widget: ProgressBar): Element;
    [Symbol.dispose](): void;
  }
}
