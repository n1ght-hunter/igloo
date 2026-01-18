import type { Space as WitSpace } from 'iced:app/space@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import { spaceToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

/**
 * Builder for creating Space widgets.
 * A Space is an amount of empty space.
 *
 * @example
 * ```typescript
 * // Fixed size space
 * const fixedSpace = Space.new()
 *   .width(Length.fixed(20))
 *   .height(Length.fixed(10))
 *   .build();
 *
 * // Flexible space that fills remaining width
 * const flexSpace = Space.new()
 *   .width(Length.fill())
 *   .build();
 * ```
 */
export class Space {
  private record: WitSpace;

  private constructor() {
    this.record = {};
  }

  /** Create a new Space builder */
  static new(): Space {
    return new Space();
  }

  /** Create a space with the given width and height */
  static with(width: Length, height: Length): Space {
    const space = new Space();
    space.record.width = width;
    space.record.height = height;
    return space;
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

  /** Build the Space widget into an Element */
  build(): Element {
    return new Element(spaceToElement(this.record));
  }
}
