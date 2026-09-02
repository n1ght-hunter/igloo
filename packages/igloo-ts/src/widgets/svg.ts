import { Svg as WitSvg } from 'iced:app/svg@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating SVG widgets.
 * An SVG displays a vector graphics image from a file path.
 *
 * @example
 * ```typescript
 * const icon = Svg.new('/path/to/icon.svg');
 * ```
 */
export class Svg implements IntoElement<never> {
  private raw: WitSvg;

  private constructor(path: string) {
    this.raw = new WitSvg(path);
  }

  /** Create a new SVG builder with the given file path */
  static new(path: string): Svg {
    return new Svg(path);
  }

  /** Convert to Element */
  intoElement(): Element<never> {
    return new Element(WitSvg.intoElement(this.raw));
  }
}
