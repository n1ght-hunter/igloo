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
 * The child message type is inferred and accumulated as elements are pushed, so
 * `Column.new()` needs no annotation. Pass an explicit argument —
 * `Column.new<Msg>()` — to pin the type: each `push` is then checked against
 * `Msg` directly, so a stray message fails on the offending call.
 *
 * @example
 * ```typescript
 * const col = Column.new()
 *   .spacing(10)
 *   .push(Text.new('Item 1'))
 *   .push(Text.new('Item 2'));
 * ```
 *
 * @typeParam Msg - The accumulated message type of the column's children.
 * @typeParam Bound - Upper bound each child must satisfy; `unknown` when unpinned.
 */
export class Column<Msg = never, Bound = unknown> implements IntoElement<Msg> {
  private raw: WitColumn;

  private constructor() {
    this.raw = new WitColumn();
  }

  /** Create a new empty Column builder */
  static new(): Column<never, unknown>;
  /** Create a new empty Column builder pinned to the message type `B` */
  static new<B>(): Column<never, B>;
  static new(): Column<never, unknown> {
    return new Column();
  }

  /** Create a Column with the given elements */
  static with<const M>(elements: ElementLike<M>[]): Column<M, unknown> {
    return new Column<M, unknown>().extend(elements);
  }

  /** Add an element to the column */
  push<const M extends Bound>(element: ElementLike<M>): Column<Msg | M, Bound> {
    this.raw.push(toElement(element).inner);
    return this as unknown as Column<Msg | M, Bound>;
  }

  /** Add an element conditionally */
  pushIf<const M extends Bound>(
    condition: boolean,
    element: () => ElementLike<M>,
  ): Column<Msg | M, Bound> {
    if (condition) {
      this.raw.push(toElement(element()).inner);
    }
    return this as unknown as Column<Msg | M, Bound>;
  }

  /** Add multiple elements */
  extend<const M extends Bound>(elements: ElementLike<M>[]): Column<Msg | M, Bound> {
    for (const element of elements) {
      this.raw.push(toElement(element).inner);
    }
    return this as unknown as Column<Msg | M, Bound>;
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
  intoElement(): Element<Msg> {
    return new Element(WitColumn.intoElement(this.raw));
  }
}
