import type { Element as WitElement } from 'iced:app/shared@0.1.0';
import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { MessageValue } from 'iced:app/app-instance@0.1.0';
import { toElement, type ElementLike } from './element.js';
import { Frame, withFrame, resolve } from './callbacks.js';

/**
 * The interface for an Igloo application following the Elm architecture.
 *
 * @typeParam State - The application state type
 * @typeParam Msg - The application message type
 */
export interface App<State, Msg> {
  /** Initialize the application state */
  init(): State;

  /** Update the state in place based on a message */
  update(state: State, msg: Msg): void;

  /** Render the current state as an ElementLike (widget or Element) */
  view(state: State): ElementLike<Msg>;
}

/**
 * The host-facing resource shape required by `iced:app/app-instance`. The host
 * boundary is untyped by design — it deals in {@link WitElement} resources and
 * {@link CallbackId}s — so this interface carries no message type.
 */
export interface IApplication {
  /** Render the current view as a WitElement */
  view(): WitElement;

  /** Dispatch a widget interaction */
  update(id: CallbackId, value: MessageValue): void;
}

/**
 * Create the `Application` resource class required by the `iced:app/app-instance`
 * WIT interface from an {@link App} definition.
 *
 * Widget callbacks registered during `view()` are collected into a frame; the
 * two most recent frames are kept so a dispatch that races a re-render still
 * resolves. Anything older misses and is ignored.
 *
 * @example
 * ```typescript
 * const Application = createApp<State, Msg>({
 *   init: () => ({ count: 0 }),
 *   update: (state, msg) => {
 *     switch (msg.type) {
 *       case 'increment': state.count += 1; break;
 *       case 'decrement': state.count -= 1; break;
 *     }
 *   },
 *   view: (state) =>
 *     Column.new()
 *       .push(Text.new(`Count: ${state.count}`))
 *       .push(Button.new(Text.new('+')).onPress(() => ({ type: 'increment' }))),
 * });
 *
 * export const appInstance = { Application };
 * ```
 */
export function createApp<State, Msg>(app: App<State, Msg>): new () => IApplication {
  return class Application {
    private state: State = app.init();
    private current = new Frame<Msg>(0);
    private previous = new Frame<Msg>(0);

    /**
     * Render the current view, rotating the callback frames so the outgoing
     * frame stays reachable for one more generation.
     */
    view(): WitElement {
      const next = new Frame<Msg>(this.current.nextBase());
      const element = withFrame(next, () => toElement(app.view(this.state)).inner);
      this.previous = this.current;
      this.current = next;
      return element;
    }

    /**
     * Dispatch a widget interaction: resolve `id` against the current then the
     * previous frame, and apply the message it produces, if any.
     */
    update(id: CallbackId, value: MessageValue): void {
      const cb = this.current.get(id) ?? this.previous.get(id);
      if (cb === undefined) return;
      const msg = resolve(cb, value);
      if (msg !== undefined) {
        app.update(this.state, msg);
      }
    }
  };
}
