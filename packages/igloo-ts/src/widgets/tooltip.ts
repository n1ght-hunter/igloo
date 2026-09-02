import { Tooltip as WitTooltip } from 'iced:app/tooltip@0.1.0';
import type { Position } from 'iced:app/tooltip@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

/**
 * Builder for creating Tooltip widgets.
 * A Tooltip displays additional information when hovering over content.
 *
 * @example
 * ```typescript
 * const withTooltip = Tooltip.new(
 *   Button.new(Text.new('?')),
 *   Text.new('Help information'),
 *   'top',
 * );
 * ```
 *
 * @typeParam Msg - The message type of the content and tooltip, inferred from `new`.
 */
export class Tooltip<Msg = never> implements IntoElement<Msg> {
  private raw: WitTooltip;

  private constructor(content: ElementLike<Msg>, tooltip: ElementLike<Msg>, position: Position) {
    this.raw = new WitTooltip(toElement(content).inner, toElement(tooltip).inner, position);
  }

  /**
   * Create a new Tooltip builder.
   * @param content - The element to wrap with a tooltip
   * @param tooltip - The tooltip content to display on hover
   * @param position - Where to position the tooltip relative to content
   */
  static new<const Msg = never>(
    content: ElementLike<Msg>,
    tooltip: ElementLike<Msg>,
    position: Position,
  ): Tooltip<Msg> {
    return new Tooltip(content, tooltip, position);
  }

  /** Set the gap between content and tooltip */
  gap(gap: Pixels): this {
    this.raw.gap(gap);
    return this;
  }

  /** Set the tooltip padding */
  padding(padding: Pixels): this {
    this.raw.padding(padding);
    return this;
  }

  /** Enable or disable snapping within viewport */
  snapWithinViewport(snap: boolean = true): this {
    this.raw.snapWithinViewport(snap);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitTooltip.intoElement(this.raw));
  }
}
