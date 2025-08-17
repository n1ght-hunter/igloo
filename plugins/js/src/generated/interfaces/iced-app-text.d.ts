/// <reference path="./iced-app-alignment.d.ts" />
/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/text@0.1.0' {
  export type Length = import('iced:app/length@0.1.0').Length;
  export type Vertical = import('iced:app/alignment@0.1.0').Vertical;
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  /**
   * The horizontal alignment of some text.
   */
  export type Alignment = AlignmentDefault | AlignmentLeft | AlignmentCenter | AlignmentRight | AlignmentJustified;
  /**
   * No specific alignment.
   * 
   * Left-to-right text will be aligned to the left, while
   * right-to-left text will be aligned to the right.
   */
  export interface AlignmentDefault {
    tag: 'default',
  }
  /**
   * Align text to the left.
   */
  export interface AlignmentLeft {
    tag: 'left',
  }
  /**
   * Center text.
   */
  export interface AlignmentCenter {
    tag: 'center',
  }
  /**
   * Align text to the right.
   */
  export interface AlignmentRight {
    tag: 'right',
  }
  /**
   * Justify text.
   */
  export interface AlignmentJustified {
    tag: 'justified',
  }
  /**
   * The height of a line of text in a paragraph.
   */
  export type LineHeight = LineHeightRelative | LineHeightAbsolute;
  /**
   * A factor of the size of the text.
   */
  export interface LineHeightRelative {
    tag: 'relative',
    val: number,
  }
  /**
   * An absolute height in logical pixels.
   */
  export interface LineHeightAbsolute {
    tag: 'absolute',
    val: Pixels,
  }
  /**
   * The shaping strategy of some text.
   * # Variants
   * 
   * ## `"basic"`
   * 
   * No shaping and no font fallback.
   * 
   * This shaping strategy is very cheap, but it will not display complex
   * scripts properly nor try to find missing glyphs in your system fonts.
   * 
   * You should use this strategy when you have complete control of the text
   * and the font you are displaying in your application.
   * 
   * This is the default.
   * ## `"advanced"`
   * 
   * Advanced text shaping and font fallback.
   * 
   * You will need to enable this flag if the text contains a complex
   * script, the font used needs it, and/or multiple fonts in your system
   * may be needed to display all of the glyphs.
   * 
   * Advanced shaping is expensive! You should only enable it when necessary.
   */
  export type Shaping = 'basic' | 'advanced';
  /**
   * The wrapping strategy of some text.
   * # Variants
   * 
   * ## `"none"`
   * 
   * No wrapping.
   * ## `"word"`
   * 
   * Wraps at the word level.
   * 
   * This is the default.
   * ## `"glyph"`
   * 
   * Wraps at the glyph level.
   * ## `"word-or-glyph"`
   * 
   * Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself.
   */
  export type Wrapping = 'none' | 'word' | 'glyph' | 'word-or-glyph';
  export interface Text {
    text: string,
    size?: number,
    lineHeight?: LineHeight,
    width?: Length,
    height?: Length,
    center?: boolean,
    alignX?: Alignment,
    alignY?: Vertical,
    shaping?: Shaping,
    wrapping?: Wrapping,
  }
}
