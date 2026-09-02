import { VerticalSlider as WitVerticalSlider } from 'iced:app/vertical-slider@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushF32, pushFixed } from '../callbacks.js';

/**
 * Builder for creating vertical Slider widgets.
 * A VerticalSlider is a vertical bar for selecting a value from a range.
 *
 * @example
 * ```typescript
 * const slider = VerticalSlider.new(0, 100, state.volume, (value) => ({
 *   type: 'volumeChanged',
 *   value,
 * })).step(1);
 * ```
 *
 * @typeParam Msg - The message type this slider emits; inferred from `onChange`.
 */
export class VerticalSlider<Msg = never> implements IntoElement<Msg> {
  private raw: WitVerticalSlider;

  private constructor(rangeStart: number, rangeEnd: number, value: number, onChange: CallbackId) {
    this.raw = new WitVerticalSlider(rangeStart, rangeEnd, value, onChange);
  }

  /**
   * Create a new VerticalSlider builder.
   * @param onChange - Handler called with the new value while dragging
   */
  static new<const M>(
    rangeStart: number,
    rangeEnd: number,
    value: number,
    onChange: (value: number) => M,
  ): VerticalSlider<M> {
    return new VerticalSlider(rangeStart, rangeEnd, value, pushF32(onChange));
  }

  /** Set the default value (value to reset to on double-click) */
  default(defaultValue: number): this {
    this.raw.default(defaultValue);
    return this;
  }

  /** Set the message to emit when the slider is released */
  onRelease<const M>(msg: () => M): VerticalSlider<Msg | M> {
    this.raw.onRelease(pushFixed(msg()));
    return this as unknown as VerticalSlider<Msg | M>;
  }

  /** Set the width in pixels */
  width(width: Pixels): this {
    this.raw.width(width);
    return this;
  }

  /** Set the height */
  height(height: Length): this {
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
  intoElement(): Element<Msg> {
    return new Element(WitVerticalSlider.intoElement(this.raw));
  }
}
