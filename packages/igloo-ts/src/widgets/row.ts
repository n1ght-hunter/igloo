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
 *
 * The child message type is inferred and accumulated as elements are pushed;
 * pass `Row.new<Msg>()` to pin it so each `push` is checked against `Msg`.
 *
 * @typeParam Msg - The accumulated message type of the row's children.
 * @typeParam Bound - Upper bound each child must satisfy; `unknown` when unpinned.
 */
export class Row<Msg = never, Bound = unknown> implements IntoElement<Msg> {
  private raw: WitRow;

  private constructor() {
    this.raw = new WitRow();
  }

  /** Create a new empty Row builder */
  static new(): Row<never, unknown>;
  /** Create a new empty Row builder pinned to the message type `B` */
  static new<B>(): Row<never, B>;
  static new(): Row<never, unknown> {
    return new Row();
  }

  /** Create a Row with the given elements */
  static with<const M>(elements: ElementLike<M>[]): Row<M, unknown> {
    return new Row<M, unknown>().extend(elements);
  }

  /** Add an element to the row */
  push<const M extends Bound>(element: ElementLike<M>): Row<Msg | M, Bound> {
    this.raw.push(toElement(element).inner);
    return this as unknown as Row<Msg | M, Bound>;
  }

  /** Add an element conditionally */
  pushIf<const M extends Bound>(
    condition: boolean,
    element: () => ElementLike<M>,
  ): Row<Msg | M, Bound> {
    if (condition) {
      this.raw.push(toElement(element()).inner);
    }
    return this as unknown as Row<Msg | M, Bound>;
  }

  /** Add multiple elements */
  extend<const M extends Bound>(elements: ElementLike<M>[]): Row<Msg | M, Bound> {
    for (const element of elements) {
      this.raw.push(toElement(element).inner);
    }
    return this as unknown as Row<Msg | M, Bound>;
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
  intoElement(): Element<Msg> {
    return new Element(WitRow.intoElement(this.raw));
  }
}
