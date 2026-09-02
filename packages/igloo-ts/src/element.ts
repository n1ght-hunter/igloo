import type { Element as WitElement, Color } from 'iced:app/shared@0.1.0';

declare const MSG: unique symbol;

/**
 * Interface for types that can be converted to an Element.
 * Widgets implement this interface to allow automatic conversion.
 *
 * @typeParam Msg - The message type this element (and its subtree) can emit.
 *   A widget that emits no messages is `IntoElement<never>`, which fits any tree.
 */
export interface IntoElement<Msg = never> {
  /**
   * Phantom marker; never present at runtime. Covariant in `Msg` so a widget
   * emitting a subset of the app's messages is assignable where the full union
   * is expected.
   */
  readonly [MSG]?: Msg;

  /** Convert this widget into an Element */
  intoElement(): Element<Msg>;
}

/**
 * Type that can be used where an Element is expected.
 * Either an Element directly, or any widget that implements IntoElement.
 */
export type ElementLike<Msg = never> = Element<Msg> | IntoElement<Msg>;

/**
 * Convert an ElementLike to an Element.
 * If already an Element, returns it. Otherwise calls intoElement().
 */
export function toElement<Msg>(value: ElementLike<Msg>): Element<Msg> {
  if (value instanceof Element) {
    return value;
  }
  return value.intoElement();
}

/**
 * Wrapper class for the WIT Element resource.
 * Provides a convenient interface for working with UI elements.
 *
 * @typeParam Msg - The message type this element's subtree can emit.
 */
export class Element<Msg = never> implements IntoElement<Msg> {
  declare readonly [MSG]?: Msg;

  constructor(public readonly inner: WitElement) {}

  /**
   * Returns itself (Element already is an Element).
   */
  intoElement(): Element<Msg> {
    return this;
  }

  /**
   * Debug helper that draws a colored overlay on the element.
   * Useful for visualizing element bounds during development.
   */
  explain(color: Color): Element<Msg> {
    return new Element(this.inner.explain(color));
  }
}
