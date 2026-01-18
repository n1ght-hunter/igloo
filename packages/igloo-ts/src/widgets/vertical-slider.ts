import type { VerticalSlider as WitVerticalSlider } from 'iced:app/vertical-slider@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import type { Message } from 'iced:app/message@0.1.0';
import { verticalSliderToElement } from 'iced:app/element@0.1.0';
import { Element } from '../element.js';
import { MessageManager } from '../message.js';

/**
 * Builder for creating vertical Slider widgets.
 * A VerticalSlider is a vertical bar for selecting a value from a range.
 *
 * @example
 * ```typescript
 * const slider = VerticalSlider.new(0, 100, state.volume)
 *   .onChange(messages, (msg) => {
 *     if (msg.tag === 'f32-type') {
 *       return { type: 'volumeChanged', value: msg.val };
 *     }
 *     return { type: 'noop' };
 *   })
 *   .step(1)
 *   .build();
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class VerticalSlider<Msg> {
  private record: WitVerticalSlider;

  private constructor(rangeStart: number, rangeEnd: number, value: number, onChange: bigint) {
    this.record = { rangeStart, rangeEnd, value, onChange };
  }

  /**
   * Create a new VerticalSlider builder.
   * @param rangeStart - Start of the value range
   * @param rangeEnd - End of the value range
   * @param value - Current value
   * @param messages - MessageManager instance
   * @param onChange - Handler called when value changes (receives f32-type Message)
   */
  static new<Msg>(
    rangeStart: number,
    rangeEnd: number,
    value: number,
    messages: MessageManager<Msg>,
    onChange: (message: Message) => Msg
  ): VerticalSlider<Msg> {
    return new VerticalSlider(rangeStart, rangeEnd, value, messages.register(onChange));
  }

  /** Set the default value (value to reset to on double-click) */
  default(defaultValue: number): this {
    this.record.default = defaultValue;
    return this;
  }

  /**
   * Set the message to emit when the slider is released.
   */
  onRelease<M extends Msg>(messages: MessageManager<M>, handler: () => M): VerticalSlider<M> {
    this.record.onRelease = messages.on(handler);
    return this as unknown as VerticalSlider<M>;
  }

  /** Set the width in pixels */
  width(width: Pixels): this {
    this.record.width = width;
    return this;
  }

  /** Set the height */
  height(height: Length): this {
    this.record.height = height;
    return this;
  }

  /** Set the step size for normal dragging */
  step(step: number): this {
    this.record.step = step;
    return this;
  }

  /** Set the step size when holding shift */
  shiftStep(step: number): this {
    this.record.shiftStep = step;
    return this;
  }

  /** Build the VerticalSlider widget into an Element */
  build(): Element {
    return new Element(verticalSliderToElement(this.record));
  }
}
