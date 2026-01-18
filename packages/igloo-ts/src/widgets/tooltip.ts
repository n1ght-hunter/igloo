import type { Tooltip as WitTooltip, Position } from 'iced:app/tooltip@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { tooltipToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

export type { Position } from 'iced:app/tooltip@0.1.0';

/**
 * Builder for creating Tooltip widgets.
 * A Tooltip displays additional information when hovering over content.
 *
 * @example
 * ```typescript
 * const withTooltip = Tooltip.new(
 *   Button.new(Text.new('?').build()).build(),
 *   Text.new('Help information').build(),
 *   'top'
 * ).build();
 * ```
 */
export class Tooltip {
  private record: WitTooltip;

  private constructor(content: Element, tooltip: Element, position: Position) {
    this.record = { content: content.inner, tooltip: tooltip.inner, position };
  }

  /**
   * Create a new Tooltip builder.
   * @param content - The element to wrap with a tooltip
   * @param tooltip - The tooltip content to display on hover
   * @param position - Where to position the tooltip relative to content
   */
  static new(content: Element, tooltip: Element, position: Position): Tooltip {
    return new Tooltip(content, tooltip, position);
  }

  /** Set the gap between content and tooltip */
  gap(gap: Pixels): this {
    this.record.gap = gap;
    return this;
  }

  /** Set the tooltip padding */
  padding(padding: Pixels): this {
    this.record.padding = padding;
    return this;
  }

  /** Enable or disable snapping within viewport */
  snapWithinViewport(snap: boolean = true): this {
    this.record.snapWithinViewport = snap;
    return this;
  }

  /** Build the Tooltip widget into an Element */
  build(): Element {
    return new Element(tooltipToElement(this.record));
  }
}
