declare module 'iced:app/alignment@0.1.0' {
  /**
   * The vertical [`alignment`] of some resource.
   */
  export type Vertical = VerticalTop | VerticalCenter | VerticalBottom;
  /**
   * Align top
   */
  export interface VerticalTop {
    tag: 'top',
  }
  /**
   * Vertically centered
   */
  export interface VerticalCenter {
    tag: 'center',
  }
  /**
   * Align bottom
   */
  export interface VerticalBottom {
    tag: 'bottom',
  }
  /**
   * The horizontal [`alignment`] of some resource.
   */
  export type Horizontal = HorizontalLeft | HorizontalCenter | HorizontalRight;
  /**
   * Align left
   */
  export interface HorizontalLeft {
    tag: 'left',
  }
  /**
   * Horizontally centered
   */
  export interface HorizontalCenter {
    tag: 'center',
  }
  /**
   * Align right
   */
  export interface HorizontalRight {
    tag: 'right',
  }
}
