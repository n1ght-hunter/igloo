import { Column as WitColumn } from 'iced:app/column@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Horizontal } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, toElement, type IntoElement, type ElementLike } from '../element.js';

/**
 * Builder for creating Column layout widgets.
 * A Column arranges its children vertically.
 *
 * @example
 * ```typescript
 * const col = Column.new()
 *   .spacing(10)
 *   .push(Text.new('Item 1'))
 *   .push(Text.new('Item 2'));
 * ```
 */
export class Column implements IntoElement {
  private raw: WitColumn;

  private constructor() {
    this.raw = new WitColumn();
  }

  /** Create a new empty Column builder */
  static new(): Column {
    return new Column();
  }

  /** Create a Column with the given elements */
  static with(elements: ElementLike[]): Column {
    const col = new Column();
    return col.extend(elements);
  }

  /** Add an element to the column */
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

  /** Set the padding around the column */
  padding(padding: Padding): this {
    this.raw.padding(padding);
    return this;
  }

  /** Set the column width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the column height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Set the maximum width in pixels */
  maxWidth(maxWidth: Pixels): this {
    this.raw.maxWidth(maxWidth);
    return this;
  }

  /** Set horizontal alignment of children */
  alignX(align: Horizontal): this {
    this.raw.alignX(align);
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.raw.clip(clip);
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(WitColumn.intoElement(this.raw));
  }
}
