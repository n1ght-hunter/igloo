import type { Grid as WitGrid } from 'iced:app/grid@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { gridToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

/**
 * Builder for creating Grid layout widgets.
 * A Grid arranges its contents in a grid pattern.
 *
 * @example
 * ```typescript
 * const grid = Grid.new()
 *   .columns(3)
 *   .spacing(10)
 *   .push(Text.new('Cell 1').build())
 *   .push(Text.new('Cell 2').build())
 *   .push(Text.new('Cell 3').build())
 *   .push(Text.new('Cell 4').build())
 *   .build();
 * ```
 */
export class Grid {
  private record: WitGrid;

  private constructor() {
    this.record = { elements: [] };
  }

  /** Create a new empty Grid builder */
  static new(): Grid {
    return new Grid();
  }

  /** Create a Grid with the given elements */
  static with(elements: Element[]): Grid {
    const grid = new Grid();
    grid.record.elements = elements.map((e) => e.inner);
    return grid;
  }

  /** Add an element to the grid */
  push(element: Element): this {
    this.record.elements.push(element.inner);
    return this;
  }

  /** Add multiple elements */
  extend(elements: Element[]): this {
    for (const element of elements) {
      this.record.elements.push(element.inner);
    }
    return this;
  }

  /** Set the spacing between cells */
  spacing(spacing: Pixels): this {
    this.record.spacing = spacing;
    return this;
  }

  /** Set the grid width in pixels */
  width(width: Pixels): this {
    this.record.width = width;
    return this;
  }

  /** Set the grid height in pixels */
  height(height: Pixels): this {
    this.record.height = height;
    return this;
  }

  /** Set the number of columns */
  columns(columns: number | bigint): this {
    this.record.columns = BigInt(columns);
    return this;
  }

  /** Set fluid column width (columns resize based on content) */
  fluid(width: Pixels): this {
    this.record.fluid = width;
    return this;
  }

  /** Build the Grid widget into an Element */
  build(): Element {
    return new Element(gridToElement(this.record));
  }
}
