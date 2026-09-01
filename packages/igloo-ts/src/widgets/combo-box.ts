import { ComboBox as WitComboBox } from 'iced:app/combo-box@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { LineHeight } from 'iced:app/text@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushFixed, pushString } from '../callbacks.js';

/**
 * Builder for creating ComboBox widgets.
 * A ComboBox allows selection from a list of options with text input filtering.
 *
 * @example
 * ```typescript
 * const comboBox = ComboBox.new(
 *   ['Apple', 'Banana', 'Cherry'],
 *   'Search fruits...',
 *   (fruit) => ({ type: 'fruitSelected', fruit }),
 *   state.selectedFruit,
 * );
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class ComboBox<Msg> implements IntoElement {
  private raw: WitComboBox;

  private constructor(
    options: string[],
    placeholder: string,
    selected: string | undefined,
    onSelected: CallbackId,
  ) {
    this.raw = new WitComboBox(options, placeholder, selected, onSelected);
  }

  /**
   * Create a new ComboBox builder.
   * @param onSelected - Handler called with the selected option
   * @param selected - The currently selected option, if any
   */
  static new<Msg>(
    options: string[],
    placeholder: string,
    onSelected: (value: string) => Msg,
    selected?: string,
  ): ComboBox<Msg> {
    return new ComboBox(options, placeholder, selected, pushString(onSelected));
  }

  /** Set the message to emit when text is input (for filtering) */
  onInput(mapper: (value: string) => Msg): this {
    this.raw.onInput(pushString(mapper));
    return this;
  }

  /** Set the message to emit when an option is hovered */
  onOptionHovered(mapper: (value: string) => Msg): this {
    this.raw.onOptionHovered(pushString(mapper));
    return this;
  }

  /** Set the message to emit when the combo box is opened */
  onOpen(msg: () => Msg): this {
    this.raw.onOpen(pushFixed(msg()));
    return this;
  }

  /** Set the message to emit when the combo box is closed */
  onClose(msg: () => Msg): this {
    this.raw.onClose(pushFixed(msg()));
    return this;
  }

  /** Set the padding */
  padding(padding: Padding): this {
    this.raw.padding(padding);
    return this;
  }

  /** Set the text size */
  size(size: number): this {
    this.raw.size(size);
    return this;
  }

  /** Set the line height */
  lineHeight(lineHeight: LineHeight): this {
    this.raw.lineHeight(lineHeight);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitComboBox.intoElement(this.raw));
  }
}
