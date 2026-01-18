import type { Row as WitRow } from 'iced:app/row@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Vertical } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { rowToElement } from 'iced:app/element@0.1.0';
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
  private record: WitRow;

  private constructor() {
    this.record = { elements: [] };
  }

  /** Create a new empty Row builder */
  static new(): Row {
    return new Row();
  }

  /** Create a Row with the given elements */
  static with(elements: ElementLike[]): Row {
    const row = new Row();
    row.record.elements = elements.map((e) => toElement(e).inner);
    return row;
  }

  /** Add an element to the row */
  push(element: ElementLike): this {
    this.record.elements.push(toElement(element).inner);
    return this;
  }

  /** Add an element conditionally */
  pushIf(condition: boolean, element: () => ElementLike): this {
    if (condition) {
      this.record.elements.push(toElement(element()).inner);
    }
    return this;
  }

  /** Add multiple elements */
  extend(elements: ElementLike[]): this {
    for (const element of elements) {
      this.record.elements.push(toElement(element).inner);
    }
    return this;
  }

  /** Set the spacing between elements in pixels */
  spacing(spacing: Pixels): this {
    this.record.spacing = spacing;
    return this;
  }

  /** Set the padding around the row */
  padding(padding: Padding): this {
    this.record.padding = padding;
    return this;
  }

  /** Set the row width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the row height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /** Set vertical alignment of children */
  alignY(align: Vertical): this {
    this.record.alignY = align;
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.record.clip = clip;
    return this;
  }

  /** Enable or disable wrapping of elements to the next line */
  wrap(wrap: boolean = true): this {
    this.record.wrap = wrap;
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(rowToElement(this.record));
  }

  /** @deprecated Use intoElement() instead */
  build(): Element {
    return this.intoElement();
  }
}
