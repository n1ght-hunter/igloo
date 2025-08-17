declare module 'iced:app/length@0.1.0' {
  /**
   * The strategy used to fill space in a specific dimension.
   */
  export type Length = LengthFill | LengthFillPortion | LengthShrink | LengthFixed;
  /**
   * Fill all the remaining space
   */
  export interface LengthFill {
    tag: 'fill',
  }
  /**
   * Fill a portion of the remaining space relative to other elements.
   * 
   * Let's say we have two elements: one with `FillPortion(2)` and one with
   * `FillPortion(3)`. The first will get 2 portions of the available space,
   * while the second one would get 3.
   * 
   * `Length::Fill` is equivalent to `Length::FillPortion(1)`.
   */
  export interface LengthFillPortion {
    tag: 'fill-portion',
    val: number,
  }
  /**
   * Fill the least amount of space
   */
  export interface LengthShrink {
    tag: 'shrink',
  }
  /**
   * Fill a fixed amount of space
   */
  export interface LengthFixed {
    tag: 'fixed',
    val: number,
  }
}
