import { Slider as WitSlider } from 'iced:app/slider@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushF32, pushFixed } from '../callbacks.js';

/**
 * Builder for creating horizontal Slider widgets.
 * A Slider is a horizontal bar for selecting a value from a range.
 *
 * @example
 * ```typescript
 * const slider = Slider.new(0, 100, state.volume, (value) => ({
 *   type: 'volumeChanged',
 *   value,
 * })).step(1);
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Slider<Msg> implements IntoElement {
  private raw: WitSlider;

  private constructor(rangeStart: number, rangeEnd: number, value: number, onChange: CallbackId) {
    this.raw = new WitSlider(rangeStart, rangeEnd, value, onChange);
  }

  /**
   * Create a new Slider builder.
   * @param onChange - Handler called with the new value while dragging
   */
  static new<Msg>(
    rangeStart: number,
    rangeEnd: number,
    value: number,
    onChange: (value: number) => Msg,
  ): Slider<Msg> {
    return new Slider(rangeStart, rangeEnd, value, pushF32(onChange));
  }

  /** Set the default value (value to reset to on double-click) */
  default(defaultValue: number): this {
    this.raw.default(defaultValue);
    return this;
  }

  /** Set the message to emit when the slider is released */
  onRelease(msg: () => Msg): this {
    this.raw.onRelease(pushFixed(msg()));
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the height in pixels */
  height(height: Pixels): this {
    this.raw.height(height);
    return this;
  }

  /** Set the step size for normal dragging */
  step(step: number): this {
    this.raw.step(step);
    return this;
  }

  /** Set the step size when holding shift */
  shiftStep(step: number): this {
    this.raw.shiftStep(step);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitSlider.intoElement(this.raw));
  }
}
