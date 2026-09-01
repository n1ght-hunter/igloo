/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/svg@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  
  export class Svg implements Disposable {
    constructor(path: string)
    static intoElement(widget: Svg): Element;
    [Symbol.dispose](): void;
  }
}
