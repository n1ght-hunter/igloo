import type { PaneGrid as WitPaneGrid } from 'iced:app/pane-grid@0.1.0';
import { paneGridToElement } from 'iced:app/element@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

/**
 * Builder for creating PaneGrid widgets.
 * A PaneGrid is a layout that can dynamically split its content into panes.
 *
 * @example
 * ```typescript
 * const paneGrid = PaneGrid.new()
 *   .push(Text.new('Pane 1'))
 *   .push(Text.new('Pane 2'));
 * ```
 */
export class PaneGrid implements IntoElement {
  private record: WitPaneGrid;

  private constructor() {
    this.record = { children: [] };
  }

  /** Create a new empty PaneGrid builder */
  static new(): PaneGrid {
    return new PaneGrid();
  }

  /** Create a PaneGrid with the given elements */
  static with(children: ElementLike[]): PaneGrid {
    const grid = new PaneGrid();
    grid.record.children = children.map((e) => toElement(e).inner);
    return grid;
  }

  /** Add a pane to the grid */
  push(element: ElementLike): this {
    this.record.children.push(toElement(element).inner);
    return this;
  }

  /** Add multiple panes */
  extend(elements: ElementLike[]): this {
    for (const element of elements) {
      this.record.children.push(toElement(element).inner);
    }
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(paneGridToElement(this.record));
  }
}
