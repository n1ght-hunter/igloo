import type { Float as WitFloat, Translation } from 'iced:app/float@0.1.0';
import { floatToElement } from 'iced:app/element@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

export type { Translation } from 'iced:app/float@0.1.0';

/**
 * Builder for creating Float widgets.
 * A Float displays floating content on top of the application.
 *
 * @example
 * ```typescript
 * const floating = Float.new(Text.new('Floating content'))
 *   .translation({ x: 100, y: 50 })
 *   .scale(0.8);
 * ```
 */
export class Float implements IntoElement {
  private record: WitFloat;

  private constructor(content: ElementLike) {
    this.record = { content: toElement(content).inner };
  }

  /** Create a new Float builder with the given content */
  static new(content: ElementLike): Float {
    return new Float(content);
  }

  /** Set the scale factor */
  scale(scale: number): this {
    this.record.scale = scale;
    return this;
  }

  /** Set the translation (offset) of the content */
  translation(translation: Translation): this {
    this.record.translation = translation;
    return this;
  }

  /** Set the translation by x and y values */
  translate(x: number, y: number): this {
    this.record.translation = { x, y };
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(floatToElement(this.record));
  }
}
