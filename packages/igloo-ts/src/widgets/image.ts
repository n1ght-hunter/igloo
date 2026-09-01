import { Image as WitImage } from 'iced:app/image@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Rotation, ContentFit, FilterMethod } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';

export type { Rotation } from 'iced:app/shared@0.1.0';

/**
 * Builder for creating Image widgets.
 * An Image displays a raster image from a file path.
 *
 * @example
 * ```typescript
 * const image = Image.new('/path/to/image.png')
 *   .width(Length.fixed(200))
 *   .contentFit('contain');
 * ```
 */
export class Image implements IntoElement {
  private raw: WitImage;

  private constructor(handle: string) {
    this.raw = new WitImage(handle);
  }

  /** Create a new Image builder with the given file path */
  static new(handle: string): Image {
    return new Image(handle);
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

  /** Enable or disable expansion to fill available space */
  expand(expand: boolean = true): this {
    this.raw.expand(expand);
    return this;
  }

  /** Set how the image fits within its bounds */
  contentFit(fit: ContentFit): this {
    this.raw.contentFit(fit);
    return this;
  }

  /** Set the filtering method for scaling */
  filterMethod(method: FilterMethod): this {
    this.raw.filterMethod(method);
    return this;
  }

  /** Set the rotation */
  rotation(rotation: Rotation): this {
    this.raw.rotation(rotation);
    return this;
  }

  /** Set the opacity (0.0 to 1.0) */
  opacity(opacity: number): this {
    this.raw.opacity(opacity);
    return this;
  }

  /** Set the scale factor */
  scale(scale: number): this {
    this.raw.scale(scale);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitImage.intoElement(this.raw));
  }
}
