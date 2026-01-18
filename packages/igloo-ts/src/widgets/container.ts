import type { Container as WitContainer } from 'iced:app/container@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Horizontal, Vertical } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { containerToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';

/**
 * Builder for creating Container widgets.
 * A Container is a widget that wraps a single child with optional padding,
 * sizing, and alignment.
 *
 * @example
 * ```typescript
 * const centered = Container.new(Text.new('Centered').build())
 *   .center(Length.fill())
 *   .build();
 * ```
 */
export class Container {
  private record: WitContainer;

  private constructor(content: Element) {
    this.record = { content: content.inner };
  }

  /** Create a new Container builder with the given content element */
  static new(content: Element): Container {
    return new Container(content);
  }

  /** Set the padding around the content */
  padding(padding: Padding): this {
    this.record.padding = padding;
    return this;
  }

  /** Set the container width */
  width(width: Length): this {
    this.record.width = width;
    return this;
  }

  /** Set the container height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /** Set the maximum width in pixels */
  maxWidth(maxWidth: Pixels): this {
    this.record.maxWidth = maxWidth;
    return this;
  }

  /** Set the maximum height in pixels */
  maxHeight(maxHeight: Pixels): this {
    this.record.maxHeight = maxHeight;
    return this;
  }

  /** Center the content horizontally with the given width */
  centerX(width: Length): this {
    this.record.centerX = width;
    return this;
  }

  /** Center the content vertically with the given height */
  centerY(height: Length): this {
    this.record.centerY = height;
    return this;
  }

  /** Center the content both horizontally and vertically */
  center(size: Length): this {
    this.record.center = size;
    return this;
  }

  /** Align content to the left with the given width */
  alignLeft(width: Length): this {
    this.record.alignLeft = width;
    return this;
  }

  /** Align content to the right with the given width */
  alignRight(width: Length): this {
    this.record.alignRight = width;
    return this;
  }

  /** Align content to the top with the given height */
  alignTop(height: Length): this {
    this.record.alignTop = height;
    return this;
  }

  /** Align content to the bottom with the given height */
  alignBottom(height: Length): this {
    this.record.alignBottom = height;
    return this;
  }

  /** Set horizontal alignment */
  alignX(align: Horizontal): this {
    this.record.alignX = align;
    return this;
  }

  /** Set vertical alignment */
  alignY(align: Vertical): this {
    this.record.alignY = align;
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.record.clip = clip;
    return this;
  }

  /** Build the Container widget into an Element */
  build(): Element {
    return new Element(containerToElement(this.record));
  }
}
