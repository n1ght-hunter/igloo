import { Radio as WitRadio } from 'iced:app/radio@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushFixed } from '../callbacks.js';

/**
 * Builder for creating Radio button widgets.
 * A Radio is a circular button representing an alternative in a group.
 *
 * @example
 * ```typescript
 * const radio = Radio.new('Option A', state.selected === 'a', () => ({
 *   type: 'selected',
 *   value: 'a',
 * }));
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Radio<Msg> implements IntoElement {
  private raw: WitRadio;

  private constructor(label: string, isSelected: boolean, msg: CallbackId) {
    this.raw = new WitRadio(label, isSelected, msg);
  }

  /**
   * Create a new Radio builder.
   * @param onSelect - Handler called when this radio is selected
   */
  static new<Msg>(label: string, isSelected: boolean, onSelect: () => Msg): Radio<Msg> {
    return new Radio(label, isSelected, pushFixed(onSelect()));
  }

  /** Set the radio button size in pixels */
  size(size: Pixels): this {
    this.raw.size(size);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the spacing between radio button and label */
  spacing(spacing: Pixels): this {
    this.raw.spacing(spacing);
    return this;
  }

  /** Set the text size */
  textSize(size: Pixels): this {
    this.raw.textSize(size);
    return this;
  }

  /** Set the text line height */
  textLineHeight(lineHeight: LineHeight): this {
    this.raw.textLineHeight(lineHeight);
    return this;
  }

  /** Set the text wrapping */
  textWrapping(wrapping: Wrapping): this {
    this.raw.textWrapping(wrapping);
    return this;
  }

  /** Set the text shaping */
  textShaping(shaping: Shaping): this {
    this.raw.textShaping(shaping);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitRadio.intoElement(this.raw));
  }
}
