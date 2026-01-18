import type { Text as WitText, Alignment, LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Vertical } from 'iced:app/alignment@0.1.0';
import type { Color } from 'iced:app/shared@0.1.0';
import { textToElement } from 'iced:app/element@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating Text widgets.
 *
 * @example
 * ```typescript
 * // Can be used directly where Element is expected
 * Column.new().push(Text.new('Hello, World!').size(24));
 *
 * // Or explicitly converted
 * const label = Text.new('Hello, World!').intoElement();
 * ```
 */
export class Text implements IntoElement {
  private record: WitText;

  private constructor(text: string) {
    this.record = { text };
  }

  /** Create a new Text builder with the given content */
  static new(text: string): Text {
    return new Text(text);
  }

  /** Set the text size in pixels */
  size(size: number): this {
    this.record.size = size;
    return this;
  }

  /** Set the line height */
  lineHeight(lineHeight: LineHeight): this {
    this.record.lineHeight = lineHeight;
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /** Center the text (shorthand for alignX: center and alignY: center) */
  center(center: boolean = true): this {
    this.record.center = center;
    return this;
  }

  /** Set horizontal text alignment */
  alignX(align: Alignment): this {
    this.record.alignX = align;
    return this;
  }

  /** Set vertical text alignment */
  alignY(align: Vertical): this {
    this.record.alignY = align;
    return this;
  }

  /** Set the text shaping strategy */
  shaping(shaping: Shaping): this {
    this.record.shaping = shaping;
    return this;
  }

  /** Set the text wrapping strategy */
  wrapping(wrapping: Wrapping): this {
    this.record.wrapping = wrapping;
    return this;
  }

  /** Set the text color */
  color(color: Color): this {
    this.record.color = color;
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(textToElement(this.record));
  }

  /** @deprecated Use intoElement() instead */
  build(): Element {
    return this.intoElement();
  }
}
