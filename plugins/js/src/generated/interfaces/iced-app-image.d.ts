/// <reference path="./iced-app-length.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/image@0.1.0' {
  export type Rotation = import('iced:app/shared@0.1.0').Rotation;
  export type ContentFit = import('iced:app/shared@0.1.0').ContentFit;
  export type FilterMethod = import('iced:app/shared@0.1.0').FilterMethod;
  export type Length = import('iced:app/length@0.1.0').Length;
  /**
   * A frame that displays an image.
   */
  export interface Image {
    /**
     * The path to the image file.
     */
    handle: string,
    width?: Length,
    height?: Length,
    expand?: boolean,
    contentFit?: ContentFit,
    filterMethod?: FilterMethod,
    rotation?: Rotation,
    opacity?: number,
    scale?: number,
  }
}
