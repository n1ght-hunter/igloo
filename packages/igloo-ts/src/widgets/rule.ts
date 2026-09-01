import { Rule as WitRule } from 'iced:app/rule@0.1.0';
import type { Pixels } from 'iced:app/shared@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating Rule widgets.
 * A Rule is a horizontal or vertical line for dividing content.
 *
 * @example
 * ```typescript
 * const hr = Rule.horizontal(1);
 * const vr = Rule.vertical(2);
 * ```
 */
export class Rule implements IntoElement {
  private raw: WitRule;

  private constructor(isHorizontal: boolean, thickness: Pixels) {
    this.raw = new WitRule(isHorizontal, thickness);
  }

  /** Create a horizontal rule with the given thickness */
  static horizontal(thickness: Pixels): Rule {
    return new Rule(true, thickness);
  }

  /** Create a vertical rule with the given thickness */
  static vertical(thickness: Pixels): Rule {
    return new Rule(false, thickness);
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitRule.intoElement(this.raw));
  }
}
