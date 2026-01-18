/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/scrollable@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type Vertical = import('iced:app/alignment@0.1.0').Vertical;
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  /**
   * # Variants
   * 
   * ## `"start"`
   * 
   * ## `"end"`
   */
  export type Anchor = 'start' | 'end';
  export interface Scrollbar {
    width?: Pixels,
    margin?: Pixels,
    scrollerWidth?: Pixels,
    alignment?: Anchor,
    spacing?: Pixels,
  }
  export type Direction = DirectionVertical | DirectionHorizontal | DirectionBoth;
  export interface DirectionVertical {
    tag: 'vertical',
    val: Scrollbar,
  }
  export interface DirectionHorizontal {
    tag: 'horizontal',
    val: Scrollbar,
  }
  export interface DirectionBoth {
    tag: 'both',
    val: [Scrollbar, Scrollbar],
  }
  export interface Scrollable {
    content: Element,
    width?: Length,
    height?: Length,
    onScroll?: MessageId,
    direction?: Direction,
  }
}
