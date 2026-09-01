/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/markdown@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  
  export class Markdown implements Disposable {
    /**
    * Creates a new [`Markdown`] widget from the given source, mapping
    * clicked link URLs to a message through `on-link-click`.
    */
    constructor(content: string, onLinkClick: CallbackId)
    static intoElement(widget: Markdown): Element;
    [Symbol.dispose](): void;
  }
}
