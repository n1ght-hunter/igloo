/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-message.d.ts" />
declare module 'iced:app/space@0.1.0' {
  export type MessageId = import('iced:app/message@0.1.0').MessageId;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * An amount of empty space.
   */
  export interface Space {
    width?: Length,
    height?: Length,
  }
}
