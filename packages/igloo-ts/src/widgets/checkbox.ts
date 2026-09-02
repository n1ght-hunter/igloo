import { Checkbox as WitCheckbox } from 'iced:app/checkbox@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushBool } from '../callbacks.js';

/**
 * Builder for creating Checkbox widgets.
 * A Checkbox is a box that can be checked.
 *
 * @example
 * ```typescript
 * const checkbox = Checkbox.new(state.isEnabled)
 *   .label('Enable feature')
 *   .onToggle((checked) => ({ type: 'enabledChanged', value: checked }));
 * ```
 *
 * @typeParam Msg - The message type this checkbox emits; inferred from `onToggle`.
 */
export class Checkbox<Msg = never> implements IntoElement<Msg> {
  private raw: WitCheckbox;

  private constructor(isChecked: boolean) {
    this.raw = new WitCheckbox(isChecked);
  }

  /** Create a new Checkbox builder with the given checked state */
  static new(isChecked: boolean): Checkbox<never> {
    return new Checkbox(isChecked);
  }

  /** Set the checkbox label */
  label(label: string): this {
    this.raw.label(label);
    return this;
  }

  /** Set the message to emit when the checkbox is toggled, given the new state */
  onToggle<const M>(mapper: (checked: boolean) => M): Checkbox<Msg | M> {
    this.raw.onToggle(pushBool(mapper));
    return this as unknown as Checkbox<Msg | M>;
  }

  /** Set the checkbox size in pixels */
  size(size: Pixels): this {
    this.raw.size(size);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the spacing between checkbox and label */
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
  intoElement(): Element<Msg> {
    return new Element(WitCheckbox.intoElement(this.raw));
  }
}
