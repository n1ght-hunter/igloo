declare module 'iced:app/message@0.1.0' {
  export function cloneMessage(message: Message): Message;
  /**
   * resource message {
     *     /// clone the message
     *     clone: func() -> message;
     * }
     */
    export type Message = bigint;
  }
  