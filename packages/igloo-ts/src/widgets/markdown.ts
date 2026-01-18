import type { Markdown as WitMarkdown } from 'iced:app/markdown@0.1.0';
import { markdownToElement } from 'iced:app/element@0.1.0';
import { Element, type IntoElement } from '../element.js';

/**
 * Builder for creating Markdown widgets.
 * A Markdown widget parses and displays Markdown content.
 *
 * @example
 * ```typescript
 * const md = Markdown.new('# Hello\n\nThis is **bold** text.').build();
 * ```
 */
export class Markdown implements IntoElement {
  private record: WitMarkdown;

  private constructor(content: string) {
    this.record = { content };
  }

  /** Create a new Markdown builder with the given content */
  static new(content: string): Markdown {
    return new Markdown(content);
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(markdownToElement(this.record));
  }
}
