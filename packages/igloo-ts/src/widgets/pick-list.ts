import { PickList as WitPickList } from 'iced:app/pick-list@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { LineHeight, Shaping } from 'iced:app/text@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushFixed, pushString } from '../callbacks.js';

/**
 * Builder for creating PickList widgets.
 * A PickList is a dropdown for selecting a value from a set of options.
 *
 * @example
 * ```typescript
 * const pickList = PickList.new(['Red', 'Green', 'Blue'], state.color, (color) => ({
 *   type: 'colorSelected',
 *   color,
 * })).placeholder('Select a color');
 * ```
 *
 * @typeParam Msg - The message type this pick list emits; inferred from its handlers.
 */
export class PickList<Msg = never> implements IntoElement<Msg> {
  private raw: WitPickList;

  private constructor(options: string[], selected: string | undefined, onSelect: CallbackId) {
    this.raw = new WitPickList(options, selected, onSelect);
  }

  /**
   * Create a new PickList builder.
   * @param onSelect - Handler called with the selected option
   */
  static new<const M>(
    options: string[],
    selected: string | undefined,
    onSelect: (value: string) => M,
  ): PickList<M> {
    return new PickList(options, selected, pushString(onSelect));
  }

  /** Set the placeholder text when no option is selected */
  placeholder(placeholder: string): this {
    this.raw.placeholder(placeholder);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the padding */
  padding(padding: Padding): this {
    this.raw.padding(padding);
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

  /** Set the text shaping */
  textShaping(shaping: Shaping): this {
    this.raw.textShaping(shaping);
    return this;
  }

  /** Set the message to emit when the pick list is opened */
  onOpen<const M>(msg: () => M): PickList<Msg | M> {
    this.raw.onOpen(pushFixed(msg()));
    return this as unknown as PickList<Msg | M>;
  }

  /** Set the message to emit when the pick list is closed */
  onClose<const M>(msg: () => M): PickList<Msg | M> {
    this.raw.onClose(pushFixed(msg()));
    return this as unknown as PickList<Msg | M>;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitPickList.intoElement(this.raw));
  }
}
