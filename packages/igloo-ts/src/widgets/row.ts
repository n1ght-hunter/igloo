import { Row as WitRow } from 'iced:app/row@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Vertical } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, toElement, type IntoElement, type ElementLike } from '../element.js';

/**
 * Builder for creating Row layout widgets.
 * A Row arranges its children horizontally.
 *
 * @example
 * ```typescript
 * const row = Row.new()
 *   .spacing(10)
 *   .push(Text.new('Left'))
 *   .push(Text.new('Right'));
 * ```
 */
export class Row implements IntoElement {
  private raw: WitRow;

  private constructor() {
    this.raw = new WitRow();
  }

  /** Create a new empty Row builder */
  static new(): Row {
    return new Row();
  }

  /** Create a Row with the given elements */
  static with(elements: ElementLike[]): Row {
    const row = new Row();
    return row.extend(elements);
  }

  /** Add an element to the row */
  push(element: ElementLike): this {
    this.raw.push(toElement(element).inner);
    return this;
  }

  /** Add an element conditionally */
  pushIf(condition: boolean, element: () => ElementLike): this {
    if (condition) {
      this.raw.push(toElement(element()).inner);
    }
    return this;
  }

  /** Add multiple elements */
  extend(elements: ElementLike[]): this {
    for (const element of elements) {
      this.raw.push(toElement(element).inner);
    }
    return this;
  }

  /** Set the spacing between elements in pixels */
  spacing(spacing: Pixels): this {
    this.raw.spacing(spacing);
    return this;
  }

  /** Set the padding around the row */
  padding(padding: Padding): this {
    this.raw.padding(padding);
    return this;
  }

  /** Set the row width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the row height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Set vertical alignment of children */
  alignY(align: Vertical): this {
    this.raw.alignY(align);
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.raw.clip(clip);
    return this;
  }

  /** Enable or disable wrapping of elements to the next line */
  wrap(wrap: boolean = true): this {
    this.raw.wrap(wrap);
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(WitRow.intoElement(this.raw));
  }
}
