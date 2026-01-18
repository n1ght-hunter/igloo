import type { Toggler as WitToggler } from 'iced:app/toggler@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Horizontal } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { togglerToElement } from 'iced:app/element@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating Toggler widgets.
 * A Toggler represents a setting that can be toggled on or off.
 *
 * @example
 * ```typescript
 * const toggler = Toggler.new(state.darkMode)
 *   .label('Dark Mode')
 *   .onToggle(messages, (msg) => {
 *     if (msg.tag === 'bool-type') {
 *       return { type: 'darkModeChanged', enabled: msg.val };
 *     }
 *     return { type: 'noop' };
 *   })
 *   .build();
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Toggler<Msg> implements IntoElement {
  private record: WitToggler;

  private constructor(isToggled: boolean) {
    this.record = { isToggled };
  }

  /** Create a new Toggler builder with the given toggled state */
  static new<Msg>(isToggled: boolean): Toggler<Msg> {
    return new Toggler(isToggled);
  }

  /** Set the toggler label */
  label(label: string): this {
    this.record.label = label;
    return this;
  }

  /**
   * Set the message to emit when the toggler is toggled.
   * The Message will have tag 'bool-type' with the new toggled state.
   */
  onToggle<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): Toggler<M> {
    this.record.onToggle = messages.register(handler);
    return this as unknown as Toggler<M>;
  }

  /** Set the toggler size in pixels */
  size(size: Pixels): this {
    this.record.size = size;
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the text size */
  textSize(size: Pixels): this {
    this.record.textSize = size;
    return this;
  }

  /** Set the text line height */
  textLineHeight(lineHeight: LineHeight): this {
    this.record.textLineHeight = lineHeight;
    return this;
  }

  /** Set the text alignment */
  textAlignment(alignment: Horizontal): this {
    this.record.textAlignment = alignment;
    return this;
  }

  /** Set the text shaping */
  textShaping(shaping: Shaping): this {
    this.record.textShaping = shaping;
    return this;
  }

  /** Set the text wrapping */
  textWrapping(wrapping: Wrapping): this {
    this.record.textWrapping = wrapping;
    return this;
  }

  /** Set the spacing between toggler and label */
  spacing(spacing: Pixels): this {
    this.record.spacing = spacing;
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(togglerToElement(this.record));
  }
}
