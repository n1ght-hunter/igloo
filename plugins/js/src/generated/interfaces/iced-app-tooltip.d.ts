/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/tooltip@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  /**
   * # Variants
   * 
   * ## `"top"`
   * 
   * ## `"bottom"`
   * 
   * ## `"left"`
   * 
   * ## `"right"`
   * 
   * ## `"follow-cursor"`
   */
  export type Position = 'top' | 'bottom' | 'left' | 'right' | 'follow-cursor';
  export interface Tooltip {
    content: Element,
    tooltip: Element,
    position: Position,
    gap?: Pixels,
    padding?: Pixels,
    snapWithinViewport?: boolean,
  }
}
