/// <reference path="./iced-app-message-types.d.ts" />
declare module 'iced:app/message@0.1.0' {
  export function cloneMessage(message: MessageId): MessageId;
  export type Rectangle = import('iced:app/message-types@0.1.0').Rectangle;
  export type Pane = import('iced:app/message-types@0.1.0').Pane;
  export type DragEvent = import('iced:app/message-types@0.1.0').DragEvent;
  export type ResizeEvent = import('iced:app/message-types@0.1.0').ResizeEvent;
  export type Viewport = import('iced:app/message-types@0.1.0').Viewport;
  export type Size = import('iced:app/message-types@0.1.0').Size;
  /**
   * resource message {
     *     /// clone the message
     *     clone: func() -> message;
     * }
     */
    export type MessageId = bigint;
    export type Message = MessageEmpty | MessageBoolType | MessageStringType | MessageTranslateType | MessagePaneType | MessageDragEvent | MessageResizeEvent | MessageViewport | MessageSize | MessageF64Type | MessageF32Type | MessageU64Type;
    export interface MessageEmpty {
      tag: 'empty',
    }
    export interface MessageBoolType {
      tag: 'bool-type',
      val: boolean,
    }
    export interface MessageStringType {
      tag: 'string-type',
      val: string,
    }
    export interface MessageTranslateType {
      tag: 'translate-type',
      val: [Rectangle, Rectangle],
    }
    export interface MessagePaneType {
      tag: 'pane-type',
      val: Pane,
    }
    export interface MessageDragEvent {
      tag: 'drag-event',
      val: DragEvent,
    }
    export interface MessageResizeEvent {
      tag: 'resize-event',
      val: ResizeEvent,
    }
    export interface MessageViewport {
      tag: 'viewport',
      val: Viewport,
    }
    export interface MessageSize {
      tag: 'size',
      val: Size,
    }
    export interface MessageF64Type {
      tag: 'f64-type',
      val: number,
    }
    export interface MessageF32Type {
      tag: 'f32-type',
      val: number,
    }
    export interface MessageU64Type {
      tag: 'u64-type',
      val: bigint,
    }
  }
  