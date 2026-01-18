import type { Button as WitButton } from 'iced:app/button@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { buttonToElement } from 'iced:app/element@0.1.0';
import { Element, toElement, type IntoElement, type ElementLike } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating Button widgets.
 *
 * @example
 * ```typescript
 * // Widgets can be passed directly - no .build() needed
 * const button = Button.new(Text.new('Click me'))
 *   .onPress(messages, () => ({ type: 'clicked' }))
 *   .padding(Padding.all(10));
 *
 * // Use in a Column directly
 * Column.new().push(button);
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Button<Msg> implements IntoElement {
  private record: WitButton;

  private constructor(content: ElementLike) {
    this.record = { content: toElement(content).inner };
  }

  /** Create a new Button builder with the given content */
  static new<Msg>(content: ElementLike): Button<Msg> {
    return new Button(content);
  }

  /**
   * Set the message to emit when the button is pressed.
   * @param messages - The MessageManager instance
   * @param handler - Function that receives the Message and returns the app message
   */
  onPressMsg<M extends Msg>(messages: MessageManager<M>, handler: (message: Message) => M): Button<M> {
    this.record.onPress = messages.register(handler);
    return this as unknown as Button<M>;
  }

  /**
   * Set the message to emit when the button is pressed (simple version).
   * @param messages - The MessageManager instance
   * @param msg - The message to emit
   */
  onPress<M extends Msg>(messages: MessageManager<M>, msg: () => M): Button<M> {
    this.record.onPress = messages.on(msg);
    return this as unknown as Button<M>;
  }

  /** Set the button width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the button height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /** Set the button padding */
  padding(padding: Padding): this {
    this.record.padding = padding;
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.record.clip = clip;
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(buttonToElement(this.record));
  }

  /** @deprecated Use intoElement() instead */
  build(): Element {
    return this.intoElement();
  }
}
