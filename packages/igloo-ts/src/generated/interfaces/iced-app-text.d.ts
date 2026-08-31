/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/text@0.1.0' {
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Vertical = import('iced:app/alignment@0.1.0').Vertical;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  export type Color = import('iced:app/shared@0.1.0').Color;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type TextAlignment = TextAlignmentDefault | TextAlignmentLeft | TextAlignmentCenter | TextAlignmentRight | TextAlignmentJustified;
  export interface TextAlignmentDefault {
    tag: 'default',
  }
  export interface TextAlignmentLeft {
    tag: 'left',
  }
  export interface TextAlignmentCenter {
    tag: 'center',
  }
  export interface TextAlignmentRight {
    tag: 'right',
  }
  export interface TextAlignmentJustified {
    tag: 'justified',
  }
  export type LineHeight = LineHeightRelative | LineHeightAbsolute;
  export interface LineHeightRelative {
    tag: 'relative',
    val: number,
  }
  export interface LineHeightAbsolute {
    tag: 'absolute',
    val: Pixels,
  }
  /**
   * # Variants
   * 
   * ## `"basic"`
   * 
   * ## `"advanced"`
   * 
   * ## `"auto"`
   */
  export type Shaping = 'basic' | 'advanced' | 'auto';
  /**
   * # Variants
   * 
   * ## `"none"`
   * 
   * ## `"word"`
   * 
   * ## `"glyph"`
   * 
   * ## `"word-or-glyph"`
   */
  export type Wrapping = 'none' | 'word' | 'glyph' | 'word-or-glyph';
  
  export class Text implements Disposable {
    constructor(content: string)
    size(s: number): void;
    lineHeight(lh: LineHeight): void;
    width(w: Length): void;
    height(h: Length): void;
    center(): void;
    alignX(align: TextAlignment): void;
    alignY(align: Vertical): void;
    color(c: Color): void;
    static intoElement(widget: Text): Element;
    [Symbol.dispose](): void;
  }
}
