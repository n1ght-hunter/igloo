/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/rule@0.1.0' {
  export type Pixels = import('iced:app/shared@0.1.0').Pixels;
  /**
   * A horizontal or vertical rule used for dividing content.
   */
  export interface Rule {
    /**
     * Whether the rule is horizontal.
     */
    isHorizontal: boolean,
    /**
     * Thickness of the rule in pixels.
     */
    thickness: Pixels,
  }
}
