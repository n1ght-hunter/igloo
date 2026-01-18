import type { KeyedColumn as WitKeyedColumn, Key } from 'iced:app/keyed@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Alignment } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { keyedColumnToElement } from 'iced:app/element@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

export type { Key } from 'iced:app/keyed@0.1.0';

/**
 * Builder for creating KeyedColumn widgets.
 * A KeyedColumn is a column that preserves state based on keys.
 * Useful for lists where items can be reordered, added, or removed.
 *
 * @example
 * ```typescript
 * const keyedColumn = KeyedColumn.new()
 *   .spacing(10)
 *   .pushKeyed(1n, Text.new('Item 1').build())
 *   .pushKeyed(2n, Text.new('Item 2').build())
 *   .pushKeyed(3n, Text.new('Item 3').build())
 *   .build();
 * ```
 */
export class KeyedColumn implements IntoElement {
  private keys: bigint[] = [];
  private children: Element[] = [];
  private _spacing?: Pixels;
  private _padding?: Padding;
  private _width?: Length;
  private _height?: Length;
  private _maxWidth?: Pixels;
  private _alignItems?: Alignment;

  private constructor() {}

  /** Create a new empty KeyedColumn builder */
  static new(): KeyedColumn {
    return new KeyedColumn();
  }

  /** Add a keyed element to the column */
  pushKeyed(key: Key, element: ElementLike): this {
    this.keys.push(key);
    this.children.push(toElement(element));
    return this;
  }

  /** Add multiple keyed elements */
  extendKeyed(items: Array<[Key, ElementLike]>): this {
    for (const [key, element] of items) {
      this.keys.push(key);
      this.children.push(toElement(element));
    }
    return this;
  }

  /** Set the spacing between elements */
  spacing(spacing: Pixels): this {
    this._spacing = spacing;
    return this;
  }

  /** Set the padding */
  padding(padding: Padding): this {
    this._padding = padding;
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this._width = width;
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this._height = height;
    return this;
  }

  /** Set the maximum width in pixels */
  maxWidth(maxWidth: Pixels): this {
    this._maxWidth = maxWidth;
    return this;
  }

  /** Set the alignment of items */
  alignItems(alignment: Alignment): this {
    this._alignItems = alignment;
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    const record: WitKeyedColumn = {
      keys: new BigUint64Array(this.keys),
      children: this.children.map((e) => e.inner),
      spacing: this._spacing,
      padding: this._padding,
      width: this._width,
      height: this._height,
      maxWidth: this._maxWidth,
      alignItems: this._alignItems,
    };
    return new Element(keyedColumnToElement(record));
  }
}
