import { Container as WitContainer } from 'iced:app/container@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Padding } from 'iced:app/padding@0.1.0';
import type { Horizontal, Vertical } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, toElement, type ElementLike, type IntoElement } from '../element.js';

/**
 * Builder for creating Container widgets.
 * A Container wraps a single child with optional padding, sizing, and alignment.
 *
 * @example
 * ```typescript
 * const centered = Container.new(Text.new('Centered')).center(Length.fill());
 * ```
 *
 * @typeParam Msg - The message type of the wrapped content, inferred from `new`.
 */
export class Container<Msg = never> implements IntoElement<Msg> {
  private raw: WitContainer;

  private constructor(content: ElementLike<Msg>) {
    this.raw = new WitContainer(toElement(content).inner);
  }

  /** Create a new Container builder with the given content element */
  static new<const Msg = never>(content: ElementLike<Msg>): Container<Msg> {
    return new Container(content);
  }

  /** Set the padding around the content */
  padding(padding: Padding): this {
    this.raw.padding(padding);
    return this;
  }

  /** Set the container width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the container height */
  height(height: Length): this {
    this.raw.height(height);
    return this;
  }

  /** Set the maximum width in pixels */
  maxWidth(maxWidth: Pixels): this {
    this.raw.maxWidth(maxWidth);
    return this;
  }

  /** Set the maximum height in pixels */
  maxHeight(maxHeight: Pixels): this {
    this.raw.maxHeight(maxHeight);
    return this;
  }

  /** Center the content horizontally with the given width */
  centerX(width: Length): this {
    this.raw.centerX(width);
    return this;
  }

  /** Center the content vertically with the given height */
  centerY(height: Length): this {
    this.raw.centerY(height);
    return this;
  }

  /** Center the content both horizontally and vertically */
  center(size: Length): this {
    this.raw.center(size);
    return this;
  }

  /** Align content to the left with the given width */
  alignLeft(width: Length): this {
    this.raw.alignLeft(width);
    return this;
  }

  /** Align content to the right with the given width */
  alignRight(width: Length): this {
    this.raw.alignRight(width);
    return this;
  }

  /** Align content to the top with the given height */
  alignTop(height: Length): this {
    this.raw.alignTop(height);
    return this;
  }

  /** Align content to the bottom with the given height */
  alignBottom(height: Length): this {
    this.raw.alignBottom(height);
    return this;
  }

  /** Set horizontal alignment */
  alignX(align: Horizontal): this {
    this.raw.alignX(align);
    return this;
  }

  /** Set vertical alignment */
  alignY(align: Vertical): this {
    this.raw.alignY(align);
    return this;
  }

  /** Enable or disable clipping of content */
  clip(clip: boolean = true): this {
    this.raw.clip(clip);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitContainer.intoElement(this.raw));
  }
}
