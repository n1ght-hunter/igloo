/// <reference path="./iced-app-callbacks.d.ts" />
/// <reference path="./iced-app-message-types.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/app-instance@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type CallbackId = import('iced:app/callbacks@0.1.0').CallbackId;
  export type Viewport = import('iced:app/message-types@0.1.0').Viewport;
  /**
   * The raw value produced by a widget interaction, if the callback it is being
   * dispatched to carries one. All-primitive, so it crosses the boundary as plain
   * data like `callback-id` does — no resource involved.
   */
  export type MessageValue = MessageValueFixed | MessageValueBoolValue | MessageValueF32Value | MessageValueF64Value | MessageValueU64Value | MessageValueStringValue | MessageValueViewportValue;
  export interface MessageValueFixed {
    tag: 'fixed',
  }
  export interface MessageValueBoolValue {
    tag: 'bool-value',
    val: boolean,
  }
  export interface MessageValueF32Value {
    tag: 'f32-value',
    val: number,
  }
  export interface MessageValueF64Value {
    tag: 'f64-value',
    val: number,
  }
  export interface MessageValueU64Value {
    tag: 'u64-value',
    val: bigint,
  }
  export interface MessageValueStringValue {
    tag: 'string-value',
    val: string,
  }
  export interface MessageValueViewportValue {
    tag: 'viewport-value',
    val: Viewport,
  }
  
  export class Application {
    constructor()
    view(): Element;
    update(id: CallbackId, value: MessageValue): void;
  }
}
