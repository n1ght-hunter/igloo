import { Space as WitSpace } from 'iced:app/space@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating Space widgets.
 * A Space is an amount of empty space.
 *
 * @example
 * ```typescript
 * const flexSpace = Space.new().width(Length.fill());
 * ```
 */
export class Space implements IntoElement<never> {
  private raw: WitSpace;

  private constructor() {
    this.raw = new WitSpace();
  }

  /** Create a new Space builder */
  static new(): Space {
    return new Space();
  }

  /** Create a space with the given width and height */
  static with(width: Length, height: Length): Space {
    return new Space().width(width).height(height);
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<never> {
    return new Element(WitSpace.intoElement(this.raw));
  }
}
