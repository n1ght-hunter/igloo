/// <reference path="./iced-app-shared.d.ts" />
declare module 'iced:app/pane-grid@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  /**
   * A layout that can dynamically split its content into panes.
   */
  export interface PaneGrid {
    /**
     * The elements to display in the grid.
     */
    children: Array<Element>,
  }
}
