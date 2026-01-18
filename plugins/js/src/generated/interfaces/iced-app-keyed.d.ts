/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-padding.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/keyed@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Alignment = import('iced:app/alignment@0.1.0').Alignment;
  export type Padding = import('iced:app/padding@0.1.0').Padding;
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Key = bigint;
  /**
   * A container that keeps the state of its children using keys.
   */
  export interface KeyedColumn {
    keys: BigUint64Array,
    children: Array<Element>,
    spacing?: Pixels,
    padding?: Padding,
    width?: Length,
    height?: Length,
    maxWidth?: Pixels,
    alignItems?: Alignment,
  }
}
