import { TextInput as WitTextInput } from 'iced:app/text-input@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { LineHeight } from 'iced:app/text@0.1.0';
import type { Horizontal } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushFixed, pushString } from '../callbacks.js';

/**
 * Builder for creating TextInput widgets.
 * A TextInput is a field that can be filled with text.
 *
 * @example
 * ```typescript
 * const input = TextInput.new('Enter name...', state.name)
 *   .onInput((value) => ({ type: 'nameChanged', value }));
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class TextInput<Msg> implements IntoElement {
  private raw: WitTextInput;

  private constructor(placeholder: string, value: string) {
    this.raw = new WitTextInput(placeholder, value);
  }

  /** Create a new TextInput builder with placeholder and current value */
  static new<Msg>(placeholder: string, value: string): TextInput<Msg> {
    return new TextInput(placeholder, value);
  }

  /** Make the text input secure (e.g., for passwords) */
  secure(secure: boolean = true): this {
    this.raw.secure(secure);
    return this;
  }

  /** Set the message to emit when the text changes, given the new value */
  onInput(mapper: (value: string) => Msg): this {
    this.raw.onInput(pushString(mapper));
    return this;
  }

  /** Set the message to emit when the user submits (e.g., presses Enter) */
  onSubmit(msg: () => Msg): this {
    this.raw.onSubmit(pushFixed(msg()));
    return this;
  }

  /** Set the message to emit when text is pasted, given the pasted value */
  onPaste(mapper: (value: string) => Msg): this {
    this.raw.onPaste(pushString(mapper));
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

  /** Set the text size in pixels */
  size(size: Pixels): this {
    this.raw.size(size);
    return this;
  }

  /** Set the line height */
  lineHeight(lineHeight: LineHeight): this {
    this.raw.lineHeight(lineHeight);
    return this;
  }

  /** Set horizontal alignment of text */
  alignX(align: Horizontal): this {
    this.raw.alignX(align);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitTextInput.intoElement(this.raw));
  }
}
