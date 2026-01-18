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
  export interface Container {
    content: Element,
    padding?: Padding,
    width?: Length,
    height?: Length,
    maxWidth?: Pixels,
    maxHeight?: Pixels,
    centerX?: Length,
    centerY?: Length,
    center?: Length,
    alignLeft?: Length,
    alignRight?: Length,
    alignTop?: Length,
    alignBottom?: Length,
    alignX?: Horizontal,
    alignY?: Vertical,
    clip?: boolean,
  }
}
