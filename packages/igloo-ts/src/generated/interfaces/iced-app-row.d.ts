/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/row@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type Vertical = import('iced:app/alignment@0.1.0').Vertical;
  export interface Row {
    elements: Array<Element>,
    spacing?: Pixels,
    padding?: Padding,
    width?: Length,
    height?: Length,
    alignY?: Vertical,
    clip?: boolean,
    wrap?: boolean,
  }
}
