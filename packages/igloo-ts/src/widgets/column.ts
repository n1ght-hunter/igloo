import type { Column as WitColumn } from 'iced:app/column@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Horizontal } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { columnToElement } from 'iced:app/element@0.1.0';
import { Element, toElement, type IntoElement, type ElementLike } from '../element.js';

/**
 * Builder for creating Column layout widgets.
 * A Column arranges its children vertically.
 *
 * @example
 * ```typescript
 * // Widgets can be passed directly - no .build() needed
 * const col = Column.new()
 *   .spacing(10)
 *   .push(Text.new('Item 1'))
 *   .push(Text.new('Item 2'))
 *   .push(Button.new(Text.new('Click')).onPress(messages, () => msg));
 * ```
 */
export class Column implements IntoElement {
  private record: WitColumn;

  private constructor() {
    this.record = { elements: [] };
  }

  /** Create a new empty Column builder */
  static new(): Column {
    return new Column();
  }

  /** Create a Column with the given elements */
  static with(elements: ElementLike[]): Column {
    const col = new Column();
    col.record.elements = elements.map((e) => toElement(e).inner);
    return col;
  }

  /** Add an element to the column */
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

  /** Set the padding around the column */
  padding(padding: Padding): this {
    this.record.padding = padding;
    return this;
  }

  /** Set the column width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the column height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /** Set the maximum width in pixels */
  maxWidth(maxWidth: Pixels): this {
    this.record.maxWidth = maxWidth;
    return this;
  }

  /** Set horizontal alignment of children */
  alignX(align: Horizontal): this {
    this.record.alignX = align;
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.record.clip = clip;
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(columnToElement(this.record));
  }

  /** @deprecated Use intoElement() instead */
  build(): Element {
    return this.intoElement();
  }
}
