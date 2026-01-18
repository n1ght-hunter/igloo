import type { Element as WitElement } from 'iced:app/shared@0.1.0';
import type { MessageId, Message } from 'iced:app/message@0.1.0';
import { toElement, type ElementLike } from './element.js';
import { MessageManager } from './message.js';

/**
 * The interface for an Igloo application following the Elm architecture.
 *
 * @typeParam State - The application state type
 * @typeParam Msg - The application message type
 */
export interface App<State, Msg> {
  /** Initialize the application state */
  init(): State;

  /** Update the state based on a message */
  update(state: State, msg: Msg): State;

  /** Render the current state as an ElementLike (widget or Element) */
  view(state: State, messages: MessageManager<Msg>): ElementLike;
}

/**
 * Create an Igloo application from an App definition.
 * Returns the exports required by the WIT interface.
 *
 * @example
 * ```typescript
 * const app = createApp<State, Msg>({
 *   init: () => ({ count: 0 }),
 *   update: (state, msg) => {
 *     switch (msg.type) {
 *       case 'increment': return { count: state.count + 1 };
 *       case 'decrement': return { count: state.count - 1 };
 *     }
 *   },
 *   view: (state, messages) => {
 *     // No .build() calls needed - widgets implement IntoElement
 *     return Column.new()
 *       .push(Text.new(`Count: ${state.count}`))
 *       .push(
 *         Button.new(Text.new('+'))
 *           .onPress(messages, () => ({ type: 'increment' }))
 *       );
 *   }
 * });
 *
 * export const { update, view } = app;
 * ```
 */
export function createApp<State, Msg>(app: App<State, Msg>) {
  let state = app.init();
  const messages = new MessageManager<Msg>();

  return {
    /**
     * Handle an update message from the host.
     * Dispatches to the registered handler and updates state.
     */
    update(msgId: MessageId, message: Message): void {
      const msg = messages.dispatch(msgId, message);
      if (msg !== undefined) {
        state = app.update(state, msg);
      }
    },

    /**
     * Render the current view.
     * Clears message handlers before rendering to avoid stale references.
     */
    view(): WitElement {
      messages.clear();
      return toElement(app.view(state, messages)).inner;
    },
  };
}
