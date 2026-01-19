import type { Radio as WitRadio } from 'iced:app/radio@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { radioToElement } from 'iced:app/element@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating Radio button widgets.
 * A Radio is a circular button representing an alternative in a group.
 *
 * @example
 * ```typescript
 * const radio = Radio.new(
 *   'Option A',
 *   state.selected === 'a',
 *   messages,
 *   () => ({ type: 'selected', value: 'a' })
 * );
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Radio<Msg> implements IntoElement {
  private record: WitRadio;

  private constructor(label: string, isSelected: boolean, onSelect: bigint) {
    this.record = { label, isSelected, onSelect };
  }

  /**
   * Create a new Radio builder.
   * @param label - The radio button label
   * @param isSelected - Whether this radio is currently selected
   * @param messages - MessageManager instance
   * @param onSelect - Handler called when this radio is selected
   */
  static new<Msg>(
    label: string,
    isSelected: boolean,
    messages: MessageManager<Msg>,
    onSelect: () => Msg
  ): Radio<Msg> {
    return new Radio(label, isSelected, messages.on(onSelect));
  }

  /** Set the radio button size in pixels */
  size(size: Pixels): this {
    this.record.size = size;
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the spacing between radio button and label */
  spacing(spacing: Pixels): this {
    this.record.spacing = spacing;
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

  /** Set the text wrapping */
  textWrapping(wrapping: Wrapping): this {
    this.record.textWrapping = wrapping;
    return this;
  }

  /** Set the text shaping */
  textShaping(shaping: Shaping): this {
    this.record.textShaping = shaping;
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(radioToElement(this.record));
  }
}
