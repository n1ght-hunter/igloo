/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/float@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  /**
   * A translation applied to content.
   */
  export interface Translation {
    /**
     * Horizontal offset.
     */
    x: number,
    /**
     * Vertical offset.
     */
    y: number,
  }
  /**
   * Displays floating content on top of the application.
   */
  export interface Float {
    /**
     * The content of the [`Float`].
     */
    content: Element,
    /**
     * The optional scale to apply to the content.
     */
    scale?: number,
    /**
     * The optional translation applied to the content.
     */
    translation?: Translation,
  }
}
