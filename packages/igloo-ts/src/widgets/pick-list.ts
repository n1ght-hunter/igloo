import type { PickList as WitPickList } from 'iced:app/pick-list@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { LineHeight, Shaping } from 'iced:app/text@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { pickListToElement } from 'iced:app/element@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating PickList widgets.
 * A PickList is a dropdown for selecting a value from a set of options.
 *
 * @example
 * ```typescript
 * const pickList = PickList.new(
 *   ['Red', 'Green', 'Blue'],
 *   state.color,
 *   messages,
 *   (msg) => {
 *     if (msg.tag === 'string-type') {
 *       return { type: 'colorSelected', color: msg.val };
 *     }
 *     return { type: 'noop' };
 *   }
 * ).placeholder('Select a color');
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class PickList<Msg> implements IntoElement {
  private record: WitPickList;

  private constructor(options: string[], selected: string | undefined, onSelect: bigint) {
    this.record = { options, selected, onSelect };
  }

  /**
   * Create a new PickList builder.
   * @param options - The list of options to display
   * @param selected - The currently selected option (or undefined)
   * @param messages - MessageManager instance
   * @param onSelect - Handler called when an option is selected (receives string-type Message)
   */
  static new<Msg>(
    options: string[],
    selected: string | undefined,
    messages: MessageManager<Msg>,
    onSelect: (message: Message) => Msg
  ): PickList<Msg> {
    return new PickList(options, selected, messages.register(onSelect));
  }

  /** Set the placeholder text when no option is selected */
  placeholder(placeholder: string): this {
    this.record.placeholder = placeholder;
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the padding */
  padding(padding: Padding): this {
    this.record.padding = padding;
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

  /** Set the text shaping */
  textShaping(shaping: Shaping): this {
    this.record.textShaping = shaping;
    return this;
  }

  /** Set the message to emit when the pick list is opened */
  onOpen<M extends Msg>(messages: MessageManager<M>, handler: () => M): PickList<M> {
    this.record.onOpen = messages.on(handler);
    return this as unknown as PickList<M>;
  }

  /** Set the message to emit when the pick list is closed */
  onClose<M extends Msg>(messages: MessageManager<M>, handler: () => M): PickList<M> {
    this.record.onClose = messages.on(handler);
    return this as unknown as PickList<M>;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(pickListToElement(this.record));
  }
}
