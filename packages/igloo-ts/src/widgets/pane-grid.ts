import type { PaneGrid as WitPaneGrid } from 'iced:app/pane-grid@0.1.0';
import { paneGridToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

/**
 * Builder for creating PaneGrid widgets.
 * A PaneGrid is a layout that can dynamically split its content into panes.
 *
 * @example
 * ```typescript
 * const paneGrid = PaneGrid.new()
 *   .push(Text.new('Pane 1').build())
 *   .push(Text.new('Pane 2').build())
 *   .build();
 * ```
 */
export class PaneGrid {
  private record: WitPaneGrid;

  private constructor() {
    this.record = { children: [] };
  }

  /** Create a new empty PaneGrid builder */
  static new(): PaneGrid {
    return new PaneGrid();
  }

  /** Create a PaneGrid with the given elements */
  static with(children: Element[]): PaneGrid {
    const grid = new PaneGrid();
    grid.record.children = children.map((e) => e.inner);
    return grid;
  }

  /** Add a pane to the grid */
  push(element: Element): this {
    this.record.children.push(element.inner);
    return this;
  }

  /** Add multiple panes */
  extend(elements: Element[]): this {
    for (const element of elements) {
      this.record.children.push(element.inner);
    }
    return this;
  }

  /** Build the PaneGrid widget into an Element */
  build(): Element {
    return new Element(paneGridToElement(this.record));
  }
}
