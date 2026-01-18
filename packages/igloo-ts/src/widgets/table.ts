import type { Table as WitTable } from 'iced:app/table@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { tableToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

/**
 * Builder for creating Table widgets.
 * A Table displays data in a grid of columns and rows.
 *
 * @example
 * ```typescript
 * const table = Table.new()
 *   .columns([
 *     Text.new('Name').build(),
 *     Text.new('Age').build(),
 *   ])
 *   .pushRow([
 *     Text.new('Alice').build(),
 *     Text.new('30').build(),
 *   ])
 *   .pushRow([
 *     Text.new('Bob').build(),
 *     Text.new('25').build(),
 *   ])
 *   .build();
 * ```
 */
export class Table {
  private record: WitTable;

  private constructor() {
    this.record = { columns: [], rows: [] };
  }

  /** Create a new empty Table builder */
  static new(): Table {
    return new Table();
  }

  /** Set the header columns */
  columns(columns: Element[]): this {
    this.record.columns = columns.map((e) => e.inner);
    return this;
  }

  /** Add a row of cells */
  pushRow(cells: Element[]): this {
    // Rows are flattened - each cell is added sequentially
    for (const cell of cells) {
      this.record.rows.push(cell.inner);
    }
    return this;
  }

  /** Set all rows at once (flattened array of cells) */
  rows(rows: Element[]): this {
    this.record.rows = rows.map((e) => e.inner);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set padding on all sides */
  padding(padding: Pixels): this {
    this.record.padding = padding;
    return this;
  }

  /** Set horizontal padding */
  paddingX(padding: Pixels): this {
    this.record.paddingX = padding;
    return this;
  }

  /** Set vertical padding */
  paddingY(padding: Pixels): this {
    this.record.paddingY = padding;
    return this;
  }

  /** Set separator thickness on all sides */
  separator(thickness: Pixels): this {
    this.record.separator = thickness;
    return this;
  }

  /** Set horizontal separator thickness */
  separatorX(thickness: Pixels): this {
    this.record.separatorX = thickness;
    return this;
  }

  /** Set vertical separator thickness */
  separatorY(thickness: Pixels): this {
    this.record.separatorY = thickness;
    return this;
  }

  /** Build the Table widget into an Element */
  build(): Element {
    return new Element(tableToElement(this.record));
  }
}
