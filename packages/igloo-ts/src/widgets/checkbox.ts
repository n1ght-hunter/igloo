import type { Checkbox as WitCheckbox } from 'iced:app/checkbox@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { checkboxToElement } from 'iced:app/element@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating Checkbox widgets.
 * A Checkbox is a box that can be checked.
 *
 * @example
 * ```typescript
 * const checkbox = Checkbox.new(state.isEnabled)
 *   .label('Enable feature')
 *   .onToggle(messages, (msg) => {
 *     if (msg.tag === 'bool-type') {
 *       return { type: 'enabledChanged', value: msg.val };
 *     }
 *     return { type: 'noop' };
 *   })
 *   .build();
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Checkbox<Msg> implements IntoElement {
  private record: WitCheckbox;

  private constructor(isChecked: boolean) {
    this.record = { isChecked };
  }

  /** Create a new Checkbox builder with the given checked state */
  static new<Msg>(isChecked: boolean): Checkbox<Msg> {
    return new Checkbox(isChecked);
  }

  /** Set the checkbox label */
  label(label: string): this {
    this.record.label = label;
    return this;
  }

  /**
   * Set the message to emit when the checkbox is toggled.
   * The Message will have tag 'bool-type' with the new checked state.
   */
  onToggle<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): Checkbox<M> {
    this.record.onToggle = messages.register(handler);
    return this as unknown as Checkbox<M>;
  }

  /** Set the checkbox size in pixels */
  size(size: Pixels): this {
    this.record.size = size;
    return this;
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

  /** Set the spacing between checkbox and label */
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
    return new Element(checkboxToElement(this.record));
  }
}
