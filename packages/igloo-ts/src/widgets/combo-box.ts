import type { ComboBox as WitComboBox } from 'iced:app/combo-box@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { LineHeight } from 'iced:app/text@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { comboBoxToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating ComboBox widgets.
 * A ComboBox allows selection from a list of options with text input filtering.
 *
 * @example
 * ```typescript
 * const comboBox = ComboBox.new(['Apple', 'Banana', 'Cherry'], 'Search fruits...')
 *   .selected(state.selectedFruit)
 *   .onSelected(messages, (msg) => {
 *     if (msg.tag === 'string-type') {
 *       return { type: 'fruitSelected', fruit: msg.val };
 *     }
 *     return { type: 'noop' };
 *   })
 *   .build();
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class ComboBox<Msg> {
  private record: WitComboBox;

  private constructor(options: string[], placeholder: string, onSelected: bigint) {
    this.record = { options, placeholder, onSelected };
  }

  /**
   * Create a new ComboBox builder.
   * @param options - The list of options to display
   * @param placeholder - Placeholder text for the input
   * @param messages - MessageManager instance
   * @param onSelected - Handler called when an option is selected (receives string-type Message)
   */
  static new<Msg>(
    options: string[],
    placeholder: string,
    messages: MessageManager<Msg>,
    onSelected: (message: Message) => Msg
  ): ComboBox<Msg> {
    return new ComboBox(options, placeholder, messages.register(onSelected));
  }

  /** Set the currently selected option */
  selected(selected: string | undefined): this {
    this.record.selected = selected;
    return this;
  }

  /** Set the message to emit when text is input (for filtering) */
  onInput<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): ComboBox<M> {
    this.record.onInput = messages.register(handler);
    return this as unknown as ComboBox<M>;
  }

  /** Set the message to emit when an option is hovered */
  onOptionHovered<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): ComboBox<M> {
    this.record.onOptionHovered = messages.register(handler);
    return this as unknown as ComboBox<M>;
  }

  /** Set the message to emit when the combo box is opened */
  onOpen<M extends Msg>(messages: MessageManager<M>, handler: () => M): ComboBox<M> {
    this.record.onOpen = messages.on(handler);
    return this as unknown as ComboBox<M>;
  }

  /** Set the message to emit when the combo box is closed */
  onClose<M extends Msg>(messages: MessageManager<M>, handler: () => M): ComboBox<M> {
    this.record.onClose = messages.on(handler);
    return this as unknown as ComboBox<M>;
  }

  /** Set the padding */
  padding(padding: Padding): this {
    this.record.padding = padding;
    return this;
  }

  /** Set the text size */
  size(size: number): this {
    this.record.size = size;
    return this;
  }

  /** Set the line height */
  lineHeight(lineHeight: LineHeight): this {
    this.record.lineHeight = lineHeight;
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Build the ComboBox widget into an Element */
  build(): Element {
    return new Element(comboBoxToElement(this.record));
  }
}
