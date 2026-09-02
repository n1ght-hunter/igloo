import { KeyedColumn as WitKeyedColumn } from 'iced:app/keyed@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Alignment } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

/**
 * Builder for creating KeyedColumn widgets.
 * A KeyedColumn is a column that preserves child state based on keys.
 *
 * @example
 * ```typescript
 * const keyedColumn = KeyedColumn.new()
 *   .spacing(10)
 *   .pushKeyed(1n, Text.new('Item 1'))
 *   .pushKeyed(2n, Text.new('Item 2'));
 * ```
 *
 * The child message type is inferred and accumulated as elements are pushed;
 * pass `KeyedColumn.new<Msg>()` to pin it so each push is checked against `Msg`.
 *
 * @typeParam Msg - The accumulated message type of the column's children.
 * @typeParam Bound - Upper bound each child must satisfy; `unknown` when unpinned.
 */
export class KeyedColumn<Msg = never, Bound = unknown> implements IntoElement<Msg> {
  private raw: WitKeyedColumn;

  private constructor() {
    this.raw = new WitKeyedColumn();
  }

  /** Create a new empty KeyedColumn builder */
  static new(): KeyedColumn<never, unknown>;
  /** Create a new empty KeyedColumn builder pinned to the message type `B` */
  static new<B>(): KeyedColumn<never, B>;
  static new(): KeyedColumn<never, unknown> {
    return new KeyedColumn();
  }

  /** Add a keyed element to the column */
  pushKeyed<const M extends Bound>(
    key: bigint,
    element: ElementLike<M>,
  ): KeyedColumn<Msg | M, Bound> {
    this.raw.push(key, toElement(element).inner);
    return this as unknown as KeyedColumn<Msg | M, Bound>;
  }

  /** Add multiple keyed elements */
  extendKeyed<const M extends Bound>(
    items: Array<[bigint, ElementLike<M>]>,
  ): KeyedColumn<Msg | M, Bound> {
    for (const [key, element] of items) {
      this.raw.push(key, toElement(element).inner);
    }
    return this as unknown as KeyedColumn<Msg | M, Bound>;
  }

  /** Set the spacing between elements */
  spacing(spacing: Pixels): this {
    this.raw.spacing(spacing);
    return this;
  }

  /** Set the padding */
  padding(padding: Padding): this {
    this.raw.padding(padding);
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

  /** Set the maximum width in pixels */
  maxWidth(maxWidth: Pixels): this {
    this.raw.maxWidth(maxWidth);
    return this;
  }

  /** Set the alignment of items */
  alignItems(alignment: Alignment): this {
    this.raw.alignItems(alignment);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitKeyedColumn.intoElement(this.raw));
  }
}
