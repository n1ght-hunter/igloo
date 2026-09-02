import { Scrollable as WitScrollable } from 'iced:app/scrollable@0.1.0';
import type { Direction, Scrollbar, Anchor } from 'iced:app/scrollable@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Viewport } from 'iced:app/message-types@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';
import { pushViewport } from '../callbacks.js';

export type { Direction, Scrollbar } from 'iced:app/scrollable@0.1.0';

/**
 * Builder for creating Scrollable widgets.
 * A Scrollable wraps content that can be scrolled.
 *
 * @example
 * ```typescript
 * const scrollable = Scrollable.new(
 *   Column.new().push(Text.new('Item 1')).push(Text.new('Item 2')),
 * ).height(Length.fixed(300));
 * ```
 *
 * @typeParam Msg - The message type of the content plus `onScroll`, inferred from `new`.
 */
export class Scrollable<Msg = never> implements IntoElement<Msg> {
  private raw: WitScrollable;

  private constructor(content: ElementLike<Msg>) {
    this.raw = new WitScrollable(toElement(content).inner);
  }

  /** Create a new Scrollable builder with the given content */
  static new<const Msg = never>(content: ElementLike<Msg>): Scrollable<Msg> {
    return new Scrollable(content);
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Set the message to emit when scrolling occurs, given the new viewport */
  onScroll<const M>(mapper: (viewport: Viewport) => M): Scrollable<Msg | M> {
    this.raw.onScroll(pushViewport(mapper));
    return this as unknown as Scrollable<Msg | M>;
  }

  /** Set the scroll direction and scrollbar configuration */
  direction(direction: Direction): this {
    this.raw.direction(direction);
    return this;
  }

  /** Configure for vertical scrolling only */
  vertical(scrollbar: Scrollbar = {}): this {
    this.raw.direction({ tag: 'vertical', val: scrollbar });
    return this;
  }

  /** Configure for horizontal scrolling only */
  horizontal(scrollbar: Scrollbar = {}): this {
    this.raw.direction({ tag: 'horizontal', val: scrollbar });
    return this;
  }

  /** Configure for both vertical and horizontal scrolling */
  both(verticalScrollbar: Scrollbar = {}, horizontalScrollbar: Scrollbar = {}): this {
    this.raw.direction({ tag: 'both', val: [verticalScrollbar, horizontalScrollbar] });
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitScrollable.intoElement(this.raw));
  }
}

/**
 * Helper to create scrollbar configuration.
 */
export const ScrollbarConfig = {
  /** Create a default scrollbar config */
  default(): Scrollbar {
    return {};
  },

  /** Create a scrollbar config with custom settings */
  custom(options: {
    width?: Pixels;
    margin?: Pixels;
    scrollerWidth?: Pixels;
    anchor?: Anchor;
    spacing?: Pixels;
  }): Scrollbar {
    return options;
  },
} as const;
