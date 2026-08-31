import { Markdown as WitMarkdown } from 'iced:app/markdown@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import { Element, type IntoElement } from '../element.js';
import { pushString } from '../callbacks.js';

/**
 * Builder for creating Markdown widgets.
 * A Markdown widget parses and displays Markdown content.
 *
 * @example
 * ```typescript
 * const md = Markdown.new('# Hello\n\n[link](https://example.com)', (url) => ({
 *   type: 'linkClicked',
 *   url,
 * }));
 * ```
 *
 * @typeParam Msg - The application message type
 */
export class Markdown<Msg> implements IntoElement {
  private raw: WitMarkdown;

  private constructor(content: string, onLinkClick: CallbackId) {
    this.raw = new WitMarkdown(content, onLinkClick);
  }

  /**
   * Create a new Markdown builder.
   * @param onLinkClick - Handler called with the URL of a clicked link
   */
  static new<Msg>(content: string, onLinkClick: (url: string) => Msg): Markdown<Msg> {
    return new Markdown(content, pushString(onLinkClick));
  }

  /** Convert to Element */
  intoElement(): Element {
    return new Element(WitMarkdown.intoElement(this.raw));
  }
}
