declare module 'iced:app/alignment@0.1.0' {
  /**
   * The vertical [`alignment`] of some resource.
   * # Variants
   * 
   * ## `"top"`
   * 
   * Align top
   * ## `"center"`
   * 
   * Vertically centered
   * ## `"bottom"`
   * 
   * Align bottom
   */
  export type Vertical = 'top' | 'center' | 'bottom';
  /**
   * The horizontal [`alignment`] of some resource.
   * # Variants
   * 
   * ## `"left"`
   * 
   * Align left
   * ## `"center"`
   * 
   * Horizontally centered
   * ## `"right"`
   * 
   * Align right
   */
  export type Horizontal = 'left' | 'center' | 'right';
  /**
   * The alignment of some resource.
   * # Variants
   * 
   * ## `"start"`
   * 
   * Align start
   * ## `"center"`
   * 
   * Vertically centered
   * ## `"end"`
   * 
   * Align end
   */
  export type Alignment = 'start' | 'center' | 'end';
}
