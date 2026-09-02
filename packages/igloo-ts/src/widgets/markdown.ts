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
 * @typeParam Msg - The message type this widget emits; inferred from `onLinkClick`.
 */
export class Markdown<Msg = never> implements IntoElement<Msg> {
  private raw: WitMarkdown;

  private constructor(content: string, onLinkClick: CallbackId) {
    this.raw = new WitMarkdown(content, onLinkClick);
  }

  /**
   * Create a new Markdown builder.
   * @param onLinkClick - Handler called with the URL of a clicked link
   */
  static new<const M>(content: string, onLinkClick: (url: string) => M): Markdown<M> {
    return new Markdown(content, pushString(onLinkClick));
  }

  /** Convert to Element */
  intoElement(): Element<Msg> {
    return new Element(WitMarkdown.intoElement(this.raw));
  }
}
