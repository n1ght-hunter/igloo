import { Grid as WitGrid } from 'iced:app/grid@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

/**
 * Builder for creating Grid layout widgets.
 * A Grid arranges its contents in a grid pattern.
 *
 * @example
 * ```typescript
 * const grid = Grid.new()
 *   .columns(3)
 *   .spacing(10)
 *   .push(Text.new('Cell 1'))
 *   .push(Text.new('Cell 2'));
 * ```
 *
 * The child message type is inferred and accumulated as elements are pushed;
 * pass `Grid.new<Msg>()` to pin it so each `push` is checked against `Msg`.
 *
 * @typeParam Msg - The accumulated message type of the grid's children.
 * @typeParam Bound - Upper bound each child must satisfy; `unknown` when unpinned.
 */
export class Grid<Msg = never, Bound = unknown> implements IntoElement<Msg> {
  private raw: WitGrid;

  private constructor() {
    this.raw = new WitGrid();
  }

  /** Create a new empty Grid builder */
  static new(): Grid<never, unknown>;
  /** Create a new empty Grid builder pinned to the message type `B` */
  static new<B>(): Grid<never, B>;
  static new(): Grid<never, unknown> {
    return new Grid();
  }

  /** Create a Grid with the given elements */
  static with<const M>(elements: ElementLike<M>[]): Grid<M, unknown> {
    return new Grid<M, unknown>().extend(elements);
  }

  /** Add an element to the grid */
  push<const M extends Bound>(element: ElementLike<M>): Grid<Msg | M, Bound> {
    this.raw.push(toElement(element).inner);
    return this as unknown as Grid<Msg | M, Bound>;
  }

  /** Add multiple elements */
  extend<const M extends Bound>(elements: ElementLike<M>[]): Grid<Msg | M, Bound> {
    for (const element of elements) {
      this.raw.push(toElement(element).inner);
    }
    return this as unknown as Grid<Msg | M, Bound>;
  }

  /** Set the spacing between cells */
  spacing(spacing: Pixels): this {
    this.raw.spacing(spacing);
    return this;
  }

  /** Set the grid width in pixels */
  width(width: Pixels): this {
    this.raw.width(width);
    return this;
  }

  /** Set the grid height in pixels */
  height(height: Pixels): this {
    this.raw.height(height);
    return this;
  }

  /** Set the number of columns */
  columns(columns: number | bigint): this {
    this.raw.columns(BigInt(columns));
    return this;
  }

  /** Set fluid column width (columns resize based on content) */
  fluid(width: Pixels): this {
    this.raw.fluid(width);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitGrid.intoElement(this.raw));
  }
}
