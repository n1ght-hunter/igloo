import type { TextInput as WitTextInput } from 'iced:app/text-input@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { LineHeight } from 'iced:app/text@0.1.0';
import type { Horizontal } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { textInputToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating TextInput widgets.
 * A TextInput is a field that can be filled with text.
 *
 * @example
 * ```typescript
 * const input = TextInput.new('Enter name...', state.name)
 *   .onInput(messages, (msg) => {
 *     if (msg.tag === 'string-type') {
 *       return { type: 'nameChanged', value: msg.val };
 *     }
 *     return { type: 'noop' };
 *   })
 *   .build();
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class TextInput<Msg> {
  private record: WitTextInput;

  private constructor(placeholder: string, value: string) {
    this.record = { placeholder, value };
  }

  /** Create a new TextInput builder with placeholder and current value */
  static new<Msg>(placeholder: string, value: string): TextInput<Msg> {
    return new TextInput(placeholder, value);
  }

  /** Make the text input secure (e.g., for passwords) */
  secure(secure: boolean = true): this {
    this.record.secure = secure;
    return this;
  }

  /**
   * Set the message to emit when the text changes.
   * The Message will have tag 'string-type' with the new text value.
   */
  onInput<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): TextInput<M> {
    this.record.onInput = messages.register(handler);
    return this as unknown as TextInput<M>;
  }

  /**
   * Set the message to emit when the user submits (e.g., presses Enter).
   */
  onSubmit<M extends Msg>(messages: MessageManager<M>, handler: () => M): TextInput<M> {
    this.record.onSubmit = messages.on(handler);
    return this as unknown as TextInput<M>;
  }

  /**
   * Set the message to emit when text is pasted.
   * The Message will have tag 'string-type' with the pasted text.
   */
  onPaste<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): TextInput<M> {
    this.record.onPaste = messages.register(handler);
    return this as unknown as TextInput<M>;
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

  /** Set the text size in pixels */
  size(size: Pixels): this {
    this.record.size = size;
    return this;
  }

  /** Set the line height */
  lineHeight(lineHeight: LineHeight): this {
    this.record.lineHeight = lineHeight;
    return this;
  }

  /** Set horizontal alignment of text */
  alignX(align: Horizontal): this {
    this.record.alignX = align;
    return this;
  }

  /** Build the TextInput widget into an Element */
  build(): Element {
    return new Element(textInputToElement(this.record));
  }
}
