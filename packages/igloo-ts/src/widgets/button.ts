import { Button as WitButton } from 'iced:app/button@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import { Element, toElement, type IntoElement, type ElementLike } from '../element.js';
import { pushFixed } from '../callbacks.js';

/**
 * Builder for creating Button widgets.
 *
 * @example
 * ```typescript
 * const button = Button.new(Text.new('Click me'))
 *   .onPress(() => ({ type: 'clicked' }))
 *   .padding(Padding.all(10));
 *
 * Column.new().push(button);
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Button<Msg> implements IntoElement {
  private raw: WitButton;

  private constructor(content: ElementLike) {
    this.raw = new WitButton(toElement(content).inner);
  }

  /** Create a new Button builder with the given content */
  static new<Msg>(content: ElementLike): Button<Msg> {
    return new Button(content);
  }

  /** Set the message to emit when the button is pressed */
  onPress(msg: () => Msg): this {
    this.raw.onPress(pushFixed(msg()));
    return this;
  }

  /** Set the button width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the button height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Set the button padding */
  padding(padding: Padding): this {
    this.raw.padding(padding);
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.raw.clip(clip);
    return this;
  }

  /** Convert to Element (implements IntoElement) */
  intoElement(): Element {
    return new Element(WitButton.intoElement(this.raw));
  }
}
