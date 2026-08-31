/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
declare module 'iced:app/combo-box@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type LineHeight = import('iced:app/text@0.1.0').LineHeight;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class ComboBox implements Disposable {
    constructor(options: Array<string>, placeholder: string, selected: string | undefined, onSelected: CallbackId)
    onInput(mapper: CallbackId): void;
    onOptionHovered(mapper: CallbackId): void;
    onOpen(msg: CallbackId): void;
    onClose(msg: CallbackId): void;
    padding(p: Padding): void;
    size(s: number): void;
    lineHeight(lh: LineHeight): void;
    width(w: Length): void;
    static intoElement(widget: ComboBox): Element;
    [Symbol.dispose](): void;
  }
}
