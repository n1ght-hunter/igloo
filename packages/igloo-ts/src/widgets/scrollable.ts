import type { Scrollable as WitScrollable, Direction, Scrollbar } from 'iced:app/scrollable@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { scrollableToElement } from 'iced:app/element@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';
import { MessageManager } from '../message.js';
import { Anchor } from '../types/enums.js';

export type { Direction, Scrollbar } from 'iced:app/scrollable@0.1.0';

/**
 * Builder for creating Scrollable widgets.
 * A Scrollable wraps content that can be scrolled.
 *
 * @example
 * ```typescript
 * const scrollable = Scrollable.new(
 *   Column.new()
 *     .push(Text.new('Item 1'))
 *     .push(Text.new('Item 2'))
 *     // ... more items
 * )
 *   .height(Length.fixed(300));
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Scrollable<Msg> implements IntoElement {
  private record: WitScrollable;

  private constructor(content: ElementLike) {
    this.record = { content: toElement(content).inner };
  }

  /** Create a new Scrollable builder with the given content */
  static new<Msg>(content: ElementLike): Scrollable<Msg> {
    return new Scrollable(content);
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /**
   * Set the message to emit when scrolling occurs.
   * The Message will have tag 'viewport' with scroll position info.
   */
  onScroll<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): Scrollable<M> {
    this.record.onScroll = messages.register(handler);
    return this as unknown as Scrollable<M>;
  }

  /** Set the scroll direction and scrollbar configuration */
  direction(direction: Direction): this {
    this.record.direction = direction;
    return this;
  }

  /** Configure for vertical scrolling only */
  vertical(scrollbar: Scrollbar = {}): this {
    this.record.direction = { tag: 'vertical', val: scrollbar };
    return this;
  }

  /** Configure for horizontal scrolling only */
  horizontal(scrollbar: Scrollbar = {}): this {
    this.record.direction = { tag: 'horizontal', val: scrollbar };
    return this;
  }

  /** Configure for both vertical and horizontal scrolling */
  both(verticalScrollbar: Scrollbar = {}, horizontalScrollbar: Scrollbar = {}): this {
    this.record.direction = { tag: 'both', val: [verticalScrollbar, horizontalScrollbar] };
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(scrollableToElement(this.record));
  }
}

/**
 * Helper to create scrollbar configuration
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
    alignment?: Anchor;
    spacing?: Pixels;
  }): Scrollbar {
    return options;
  },
} as const;
