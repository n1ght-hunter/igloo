import type { Element as WitElement, Color } from 'iced:app/shared@0.1.0';

/**
 * Interface for types that can be converted to an Element.
 * Widgets implement this interface to allow automatic conversion.
 */
export interface IntoElement {
  /** Convert this widget into an Element */
  intoElement(): Element;
}

/**
 * Type that can be used where an Element is expected.
 * Either an Element directly, or any widget that implements IntoElement.
 */
export type ElementLike = Element | IntoElement;

/**
 * Convert an ElementLike to an Element.
 * If already an Element, returns it. Otherwise calls intoElement().
 */
export function toElement(value: ElementLike): Element {
  if (value instanceof Element) {
    return value;
  }
  return value.intoElement();
}

/**
 * Wrapper class for the WIT Element resource.
 * Provides a convenient interface for working with UI elements.
 */
export class Element implements IntoElement {
  constructor(public readonly inner: WitElement) {}

  /**
   * Returns itself (Element already is an Element).
   */
  intoElement(): Element {
    return this;
  }

  /**
   * Debug helper that draws a colored overlay on the element.
   * Useful for visualizing element bounds during development.
   */
  explain(color: Color): Element {
    return new Element(this.inner.explain(color));
  }
}
