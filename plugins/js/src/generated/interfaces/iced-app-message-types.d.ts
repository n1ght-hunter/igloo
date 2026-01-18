declare module 'iced:app/message-types@0.1.0' {
  export type Pane = bigint;
  export type Split = bigint;
  export interface Size {
    width: number,
    height: number,
  }
  export interface Rectangle {
    x: number,
    y: number,
    width: number,
    height: number,
  }
  export interface Picked {
    pane: Pane,
  }
  export interface Canceled {
    pane: Pane,
  }
  /**
   * # Variants
   * 
   * ## `"top"`
   * 
   * ## `"left"`
   * 
   * ## `"right"`
   * 
   * ## `"bottom"`
   */
  export type Edge = 'top' | 'left' | 'right' | 'bottom';
  export type Region = RegionCenter | RegionEdge;
  export interface RegionCenter {
    tag: 'center',
  }
  export interface RegionEdge {
    tag: 'edge',
    val: Edge,
  }
  export type Target = TargetEdge | TargetPane;
  export interface TargetEdge {
    tag: 'edge',
    val: Edge,
  }
  export interface TargetPane {
    tag: 'pane',
    val: [Pane, Region],
  }
  export interface Dropped {
    pane: Pane,
    target: Target,
  }
  /**
   * An event produced during a drag and drop interaction of a PaneGrid.
   */
  export type DragEvent = DragEventPicked | DragEventDropped | DragEventCanceled;
  /**
   * A Pane was picked for dragging.
   */
  export interface DragEventPicked {
    tag: 'picked',
    val: Picked,
  }
  /**
   * A Pane was dropped on top of another Pane.
   */
  export interface DragEventDropped {
    tag: 'dropped',
    val: Dropped,
  }
  /**
   * A Pane was picked and then dropped outside of other Pane boundaries.
   */
  export interface DragEventCanceled {
    tag: 'canceled',
    val: Canceled,
  }
  export interface ResizeEvent {
    split: Split,
    ratio: number,
  }
  export interface AbsoluteOffset {
    x: number,
    y: number,
  }
  export interface RelativeOffset {
    x: number,
    y: number,
  }
  /**
   * The current Viewport of the Scrollable.
   */
  export interface Viewport {
    /**
     * Returns the AbsoluteOffset of the current Viewport.
     */
    absoluteOffset: AbsoluteOffset,
    /**
     * Returns the AbsoluteOffset of the current Viewport, but with its alignment reversed.
     * This method can be useful to switch the alignment of a Scrollable while maintaining its scrolling position.
     */
    absoluteOffsetReversed: AbsoluteOffset,
    /**
     * Returns the RelativeOffset of the current Viewport.
     */
    relativeOffset: RelativeOffset,
    /**
     * Returns the bounds of the current Viewport.
     */
    bounds: Rectangle,
    /**
     * Returns the content bounds of the current Viewport.
     */
    contentBounds: Rectangle,
  }
}
