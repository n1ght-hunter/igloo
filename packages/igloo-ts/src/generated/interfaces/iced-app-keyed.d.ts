/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/keyed@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type Alignment = import('iced:app/alignment@0.1.0').Alignment;
  
  export class KeyedColumn implements Disposable {
    constructor()
    push(key: bigint, child: Element): void;
    spacing(amount: Pixels): void;
    padding(p: Padding): void;
    width(w: Length): void;
    height(h: Length): void;
    maxWidth(amount: Pixels): void;
    alignItems(align: Alignment): void;
    static intoElement(widget: KeyedColumn): Element;
    [Symbol.dispose](): void;
  }
}
