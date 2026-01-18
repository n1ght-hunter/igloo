import type { Svg as WitSvg } from 'iced:app/svg@0.1.0';
import { svgToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

/**
 * Builder for creating SVG widgets.
 * An SVG displays a vector graphics image from a file path.
 *
 * @example
 * ```typescript
 * const icon = Svg.new('/path/to/icon.svg').build();
 * ```
 */
export class Svg {
  private record: WitSvg;

  private constructor(path: string) {
    this.record = { path };
  }

  /** Create a new SVG builder with the given file path */
  static new(path: string): Svg {
    return new Svg(path);
  }

  /** Build the SVG widget into an Element */
  build(): Element {
    return new Element(svgToElement(this.record));
  }
}
