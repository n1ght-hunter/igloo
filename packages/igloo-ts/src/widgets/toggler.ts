import { Toggler as WitToggler } from 'iced:app/toggler@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import type { LineHeight, Shaping, Wrapping } from 'iced:app/text@0.1.0';
import type { Horizontal } from 'iced:app/alignment@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushBool } from '../callbacks.js';

/**
 * Builder for creating Toggler widgets.
 * A Toggler represents a setting that can be toggled on or off.
 *
 * @example
 * ```typescript
 * const toggler = Toggler.new(state.darkMode)
 *   .label('Dark Mode')
 *   .onToggle((enabled) => ({ type: 'darkModeChanged', enabled }));
 * ```
 *
 * @typeParam Msg - The message type this toggler emits; inferred from `onToggle`.
 */
export class Toggler<Msg = never> implements IntoElement<Msg> {
  private raw: WitToggler;

  private constructor(isToggled: boolean) {
    this.raw = new WitToggler(isToggled);
  }

  /** Create a new Toggler builder with the given toggled state */
  static new(isToggled: boolean): Toggler<never> {
    return new Toggler(isToggled);
  }

  /** Set the toggler label */
  label(label: string): this {
    this.raw.label(label);
    return this;
  }

  /** Set the message to emit when the toggler is toggled, given the new state */
  onToggle<const M>(mapper: (enabled: boolean) => M): Toggler<Msg | M> {
    this.raw.onToggle(pushBool(mapper));
    return this as unknown as Toggler<Msg | M>;
  }

  /** Set the toggler size in pixels */
  size(size: Pixels): this {
    this.raw.size(size);
    return this;
  }

  /** Set the width */
  width(width: Length): this {
    this.raw.width(width);
    return this;
  }

  /** Set the text size */
  textSize(size: Pixels): this {
    this.raw.textSize(size);
    return this;
  }

  /** Set the text line height */
  textLineHeight(lineHeight: LineHeight): this {
    this.raw.textLineHeight(lineHeight);
    return this;
  }

  /** Set the text alignment */
  textAlignment(alignment: Horizontal): this {
    this.raw.textAlignment(alignment);
    return this;
  }

  /** Set the text shaping */
  textShaping(shaping: Shaping): this {
    this.raw.textShaping(shaping);
    return this;
  }

  /** Set the text wrapping */
  textWrapping(wrapping: Wrapping): this {
    this.raw.textWrapping(wrapping);
    return this;
  }

  /** Set the spacing between toggler and label */
  spacing(spacing: Pixels): this {
    this.raw.spacing(spacing);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitToggler.intoElement(this.raw));
  }
}
