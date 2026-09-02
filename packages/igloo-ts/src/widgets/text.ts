import { Text as WitText } from 'iced:app/text@0.1.0';
import type { TextAlignment, LineHeight } from 'iced:app/text@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Vertical } from 'iced:app/alignment@0.1.0';
import type { Color } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating Text widgets.
 *
 * @example
 * ```typescript
 * Column.new().push(Text.new('Hello, World!').size(24));
 * ```
 */
export class Text implements IntoElement<never> {
  private raw: WitText;

  private constructor(text: string) {
    this.raw = new WitText(text);
  }

  /** Create a new Text builder with the given content */
  static new(text: string): Text {
    return new Text(text);
  }

  /** Set the text size in pixels */
  size(size: number): this {
    this.raw.size(size);
    return this;
  }

  /** Set the line height */
  lineHeight(lineHeight: LineHeight): this {
    this.raw.lineHeight(lineHeight);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Center the text horizontally and vertically */
  center(): this {
    this.raw.center();
    return this;
  }

  /** Set horizontal text alignment */
  alignX(align: TextAlignment): this {
    this.raw.alignX(align);
    return this;
  }

  /** Set vertical text alignment */
  alignY(align: Vertical): this {
    this.raw.alignY(align);
    return this;
  }

  /** Set the text color */
  color(color: Color): this {
    this.raw.color(color);
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element<never> {
    return new Element(WitText.intoElement(this.raw));
  }
}
