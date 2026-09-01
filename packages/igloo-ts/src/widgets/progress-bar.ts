import { ProgressBar as WitProgressBar } from 'iced:app/progress-bar@0.1.0';
import type { Length } from 'iced:app/length@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating ProgressBar widgets.
 * A ProgressBar displays progress within a range.
 *
 * @example
 * ```typescript
 * const progressBar = ProgressBar.new(0, 100, state.progress).length(Length.fill());
 * ```
 */
export class ProgressBar implements IntoElement {
  private raw: WitProgressBar;

  private constructor(rangeStart: number, rangeEnd: number, value: number) {
    this.raw = new WitProgressBar(rangeStart, rangeEnd, value);
  }

  /**
   * Create a new ProgressBar builder.
   * @param rangeStart - Start of the value range
   * @param rangeEnd - End of the value range
   * @param value - Current progress value
   */
  static new(rangeStart: number, rangeEnd: number, value: number): ProgressBar {
    return new ProgressBar(rangeStart, rangeEnd, value);
  }

  /** Set the length (width for horizontal, height for vertical) */
  length(length: Length): this {
    this.raw.length(length);
    return this;
  }

  /** Set the girth (height for horizontal, width for vertical) */
  girth(girth: Length): this {
    this.raw.girth(girth);
    return this;
  }

  /** Make the progress bar vertical */
  vertical(vertical: boolean = true): this {
    this.raw.vertical(vertical);
    return this;
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitProgressBar.intoElement(this.raw));
  }
}
