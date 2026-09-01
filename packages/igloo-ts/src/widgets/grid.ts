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
 */
export class Grid implements IntoElement {
  private raw: WitGrid;

  private constructor() {
    this.raw = new WitGrid();
  }

  /** Create a new empty Grid builder */
  static new(): Grid {
    return new Grid();
  }

  /** Create a Grid with the given elements */
  static with(elements: ElementLike[]): Grid {
    return new Grid().extend(elements);
  }

  /** Add an element to the grid */
  push(element: ElementLike): this {
    this.raw.push(toElement(element).inner);
    return this;
  }

  /** Add multiple elements */
  extend(elements: ElementLike[]): this {
    for (const element of elements) {
      this.raw.push(toElement(element).inner);
    }
    return this;
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
  intoElement(): Element {
    return new Element(WitGrid.intoElement(this.raw));
  }
}
