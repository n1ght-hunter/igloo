import type { MessageId, Message } from 'iced:app/message@0.1.0';

/**
 * Manages message registration and dispatch for the Elm-architecture pattern.
 *
 * The MessageManager maps MessageId values (bigint) to handler functions that
 * produce application messages when events occur (e.g., button clicks).
 *
 * @typeParam Msg - The application's message type
 */
export class MessageManager<Msg> {
  private nextId: bigint = 0n;
  private handlers: Map<bigint, (message: Message) => Msg> = new Map();

  /**
   * Register a handler that produces a message when triggered.
   * Returns a MessageId that can be passed to widgets.
   */
  register(handler: (message: Message) => Msg): MessageId {
    const id = this.nextId++;
    this.handlers.set(id, handler);
    return id;
  }

  /**
   * Register a simple handler that ignores the Message payload.
   * Useful for simple button clicks that don't need event data.
   */
  on(handler: () => Msg): MessageId {
    return this.register(() => handler());
  }

  /**
   * Dispatch a message by its ID, returning the application message if found.
   */
  dispatch(id: MessageId, message: Message): Msg | undefined {
    const handler = this.handlers.get(id);
    return handler?.(message);
  }

  /**
   * Clear all registered handlers. Called before each view() to avoid stale handlers.
   */
  clear(): void {
    this.handlers.clear();
    this.nextId = 0n;
  }
}
